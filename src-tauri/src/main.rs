// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod emitter;
pub mod session;
pub mod telemetry;
pub mod util;
pub mod websocket;

use eframe::egui;
use eyre::{eyre, OptionExt, Result};
use log::{debug, error, info, warn};
use simetry::iracing::Client;
use std::{sync::OnceLock, time::Duration};
use tauri::{
    async_runtime,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_log::{Target, TargetKind};
use tokio::sync::Mutex;

use crate::emitter::telemetry_emitter::TelemetryEmitter;
use crate::session::session_data::SessionData;

#[cfg(not(debug_assertions))]
use tauri_plugin_updater::UpdaterExt;

static WINDOW: OnceLock<tauri::WebviewWindow> = OnceLock::new();
const RETRY_TIMEOUT_SECS: u64 = 5;
const SESSION_UPDATE_PERIOD_MILLIS: u64 = 25;
const SLOW_VAR_RESET_TICKS: u32 = 50;

#[tokio::main]
async fn main() {
    let _ = color_eyre::install();

    #[cfg(debug_assertions)]
    specta::export::ts("../src/lib/types/telemetry.ts").expect("Failed to export types");

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, args, cwd| {
            info!(
                "Single instance started with args: {:?}, cwd: {:?}",
                args, cwd
            );
        }))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(
            tauri_plugin_log::Builder::default()
                .target(Target::new(TargetKind::LogDir { file_name: None }))
                .target(Target::new(TargetKind::Stdout))
                .target(Target::new(TargetKind::Webview))
                .filter(|metadata| !metadata.target().contains("tungstenite"))
                .build(),
        )
        .setup(|app| {
            #[cfg(not(debug_assertions))]
            {
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    if let Err(err) = update(handle).await {
                        error!("Failed to update: {:?}", err);
                    }
                });
            }

            let version = MenuItemBuilder::with_id(
                "version",
                format!("iRaceHUD v{}", app.package_info().version),
            )
            .enabled(false)
            .build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let settings = MenuItemBuilder::with_id("settings", "Settings").build(app)?;
            let tray_menu = MenuBuilder::new(app)
                .item(&version)
                .item(&settings)
                .separator()
                .item(&quit)
                .build()?;
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event({
                    let app_handle = app.handle().clone();
                    move |_, event| {
                        if event.id().as_ref() == "quit" {
                            info!("Quit menu item clicked, quitting application");
                            app_handle.exit(0);
                        } else if event.id().as_ref() == "settings" {
                            info!("Settings menu item clicked, opening settings");
                            match app_handle.get_webview_window("settings") {
                                Some(window) => {
                                    info!("Settings window already open");
                                    if let Err(err) = window.unminimize() {
                                        warn!("Failed to unminimize settings window: {:?}", err);
                                    };
                                    if let Err(err) = window.set_focus() {
                                        warn!("Failed to focus settings window: {:?}", err);
                                    };
                                }
                                None => {
                                    if let Err(err) = WebviewWindowBuilder::new(
                                        &app_handle,
                                        "settings",
                                        WebviewUrl::App("/settings".into()),
                                    )
                                    .title("iRaceHUD Settings")
                                    .resizable(false)
                                    .center()
                                    .build()
                                    {
                                        error!("Failed to build settings window: {:?}", err);
                                    }
                                }
                            }
                        }
                    }
                })
                .title("iRaceHUD")
                .build(app)?;

            let emitter = TelemetryEmitter::default();
            app.manage(Mutex::new(emitter));

            tokio::spawn(async move {
                TelemetryEmitter::init().await;
            });

            let options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default()
                    .with_active(false)
                    .with_always_on_top()
                    .with_close_button(false)
                    .with_decorations(false)
                    .with_drag_and_drop(false)
                    .with_inner_size((f32::INFINITY, f32::INFINITY))
                    .with_fullscreen(true)
                    .with_maximize_button(false)
                    .with_minimize_button(false)
                    .with_mouse_passthrough(true)
                    .with_position((0.0, 0.0))
                    .with_resizable(false)
                    .with_transparent(true)
                    .build(),
                ..Default::default()
            };

            eframe::run_native(
                "iRaceHUD",
                options,
                Box::new(|cc| Ok(Box::new(OverlayWindow::new(cc)))),
            );

            let window = app
                .get_webview_window("main")
                .ok_or_eyre("Failed to get window")?;

            // #[cfg(debug_assertions)]
            // window.open_devtools();

            window
                .set_ignore_cursor_events(true)
                .map_err(|err| eyre!("Failed to set ignore cursor events: {:?}", err))?;

            async_runtime::spawn(async move {
                if let Err(err) = connect().await {
                    error!("Failed to connect: {:?}", err);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register_event_emitter,
            unregister_event_emitter,
            set_autostart,
            get_autostart,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

async fn connect() -> Result<()> {
    loop {
        info!("Start iRacing");
        let mut client = Client::connect(Duration::from_secs(RETRY_TIMEOUT_SECS)).await;
        let mut data = SessionData::default();
        let mut slow_var_ticks: u32 = SLOW_VAR_RESET_TICKS;
        while let Some(sim_state) = client.next_sim_state().await {
            slow_var_ticks += 1;

            let should_process_slow = slow_var_ticks >= SLOW_VAR_RESET_TICKS;

            let result = data.process_tick(&sim_state, should_process_slow);

            if should_process_slow {
                slow_var_ticks = 0;
            }

            let window = WINDOW.get().ok_or_eyre("Failed to get window")?;
            let emitter_state = window.app_handle().state::<Mutex<TelemetryEmitter>>();
            let mut emitter = emitter_state.lock().await;

            if result == session::session_data::ProcessTickResult::StateChanged {
                emitter.reset();
            }

            emitter.emit_all(&data)?;

            tokio::time::sleep(Duration::from_millis(SESSION_UPDATE_PERIOD_MILLIS)).await;
        }
    }
}

#[cfg(not(debug_assertions))]
async fn update(app: tauri::AppHandle) -> Result<()> {
    let updater = app
        .updater_builder()
        .version_comparator(|current, update| update.version != current)
        .build();
    if updater.is_err() {
        info!("Updater not available, skipping update check");
        return Ok(());
    }
    let updater_result = updater.unwrap().check().await;
    if updater_result.is_err() {
        info!("Failed to check for updates");
        return Ok(());
    }
    if let Some(update) = updater_result.unwrap() {
        info!("Update found: {:?}", update.version);

        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    info!("Downloaded {downloaded} from {content_length:?}");
                },
                || {
                    info!("Download finished");
                },
            )
            .await?;

        info!("Update installed, restarting app");
        app.restart();
    }

    Ok(())
}

#[derive(Default)]
struct OverlayWindow {}

impl OverlayWindow {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for OverlayWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("iRaceHUD");
        });
    }
}

#[tauri::command]
async fn register_event_emitter(app: tauri::AppHandle, event: String) {
    debug!("Registering event emitter for {}", event);
    let emitter_state = app.app_handle().state::<Mutex<TelemetryEmitter>>();
    let mut emitter = emitter_state.lock().await;
    emitter.register(&event);
}

#[tauri::command]
async fn unregister_event_emitter(app: tauri::AppHandle, event: String) {
    let emitter_state = app.app_handle().state::<Mutex<TelemetryEmitter>>();
    let mut emitter = emitter_state.lock().await;
    emitter.unregister(&event);
}

#[tauri::command]
async fn set_autostart(app: tauri::AppHandle, enabled: bool) {
    info!("Setting autostart to {}", enabled);
    let autostart_manager = app.autolaunch();
    if enabled {
        autostart_manager
            .enable()
            .expect("Failed to enable autostart");
    } else {
        autostart_manager
            .disable()
            .expect("Failed to disable autostart");
    }
}

#[tauri::command]
async fn get_autostart(app: tauri::AppHandle) -> bool {
    info!("Getting autostart status");
    app.autolaunch().is_enabled().unwrap_or(false)
}
