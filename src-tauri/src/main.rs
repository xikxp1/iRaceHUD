// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod emitter;
pub mod session;
pub mod settings;
pub mod telemetry;
pub mod util;
pub mod websocket;

use eyre::{eyre, OptionExt, Result};
use log::{debug, error, info, warn};
use simetry::iracing::Client;
use std::{backtrace::Backtrace, sync::OnceLock, time::Duration};
use tauri::{
    async_runtime,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_log::{Target, TargetKind};
use tokio::sync::Mutex;
use websocket::WebSocketServer;

use crate::emitter::telemetry_emitter::TelemetryEmitter;
use crate::session::session_data::SessionData;
use crate::settings::overlays::lap_times::LapTimesOverlaySettings;
use crate::settings::overlays::main::MainOverlaySettings;
use crate::settings::overlays::proximity::ProximityOverlaySettings;
use crate::settings::overlays::relative::RelativeOverlaySettings;
use crate::settings::overlays::standings::StandingsOverlaySettings;
use crate::settings::overlays::subtimer::SubTimerOverlaySettings;
use crate::settings::overlays::telemetry::TelemetryOverlaySettings;
use crate::settings::overlays::timer::TimerOverlaySettings;
use crate::settings::overlays::track_map::TrackMapOverlaySettings;
use crate::util::settings_helper::{get_settings, set_settings};

#[cfg(not(debug_assertions))]
use tauri_plugin_updater::UpdaterExt;

static WINDOW: OnceLock<tauri::WebviewWindow> = OnceLock::new();
static WS_SERVER: OnceLock<WebSocketServer> = OnceLock::new();
const RETRY_TIMEOUT_SECS: u64 = 5;
const SESSION_UPDATE_PERIOD_MILLIS: u64 = 25;
const SLOW_VAR_RESET_TICKS: u32 = 50;

fn open_settings_window(app_handle: tauri::AppHandle) {
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
            match WebviewWindowBuilder::new(
                &app_handle,
                "settings",
                WebviewUrl::App("/settings".into()),
            )
            .title("iRaceHUD Settings")
            .focused(true)
            .build()
            {
                Ok(window) => {
                    info!("Settings window opened");
                    window.set_focus().unwrap();
                }
                Err(err) => {
                    error!("Failed to build settings window: {:?}", err);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let _ = color_eyre::install();

    #[cfg(debug_assertions)]
    specta::export::ts("../src/lib/types/telemetry.ts").expect("Failed to export types");

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let ctrl_f11_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::F11);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if shortcut == &ctrl_f11_shortcut && event.state() == ShortcutState::Pressed {
                        info!("Ctrl+F11 shortcut pressed, opening settings");
                        open_settings_window(app.clone());
                    }
                })
                .build(),
        )
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
        .plugin(
            tauri_plugin_prevent_default::Builder::new()
                .with_flags(tauri_plugin_prevent_default::Flags::debug())
                .platform(tauri_plugin_prevent_default::PlatformOptions {
                    general_autofill: false,
                    password_autosave: false,
                })
                .build(),
        )
        .setup(move |app| {
            let default_panic = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                let backtrace = Backtrace::force_capture();
                log::error!("Panic: {info}\n{backtrace}");
                default_panic(info);
            }));

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
            let settings =
                MenuItemBuilder::with_id("settings", "Settings (Ctrl+F11)").build(app)?;
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
                            open_settings_window(app_handle.clone());
                        }
                    }
                })
                .title("iRaceHUD")
                .build(app)?;

            let emitter = TelemetryEmitter::default();
            app.manage(Mutex::new(emitter));

            // Initialize WebSocket server
            let server = WebSocketServer::new();
            let server_clone = server.clone();
            let _ = WS_SERVER
                .set(server)
                .map_err(|err| error!("Failed to set WebSocket server: {:?}", err));

            // Run WebSocket server in a separate task
            tokio::spawn(async move {
                server_clone.run("127.0.0.1:0").await;
            });

            let window = app
                .get_webview_window("main")
                .ok_or_eyre("Failed to get window")?;

            #[cfg(debug_assertions)]
            window.open_devtools();

            window
                .set_ignore_cursor_events(true)
                .map_err(|err| eyre!("Failed to set ignore cursor events: {:?}", err))?;

            WINDOW
                .set(window)
                .map_err(|err| eyre!("Failed to set window: {:?}", err))?;

            async_runtime::spawn(async move {
                if let Err(err) = connect().await {
                    error!("Failed to connect: {:?}", err);
                }
            });

            app.global_shortcut().register(ctrl_f11_shortcut)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register_event_emitter,
            unregister_event_emitter,
            set_autostart,
            get_autostart,
            get_ws_port,
            get_lap_times_overlay_settings,
            set_lap_times_overlay_settings,
            get_main_overlay_settings,
            set_main_overlay_settings,
            get_proximity_overlay_settings,
            set_proximity_overlay_settings,
            get_relative_overlay_settings,
            set_relative_overlay_settings,
            get_standings_overlay_settings,
            set_standings_overlay_settings,
            get_subtimer_overlay_settings,
            set_subtimer_overlay_settings,
            get_telemetry_overlay_settings,
            set_telemetry_overlay_settings,
            get_timer_overlay_settings,
            set_timer_overlay_settings,
            get_track_map_overlay_settings,
            set_track_map_overlay_settings,
            get_app_version,
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

#[tauri::command]
async fn get_ws_port() -> Option<u16> {
    websocket::WebSocketServer::get_port()
}

#[tauri::command]
async fn get_lap_times_overlay_settings(app: tauri::AppHandle) -> LapTimesOverlaySettings {
    get_settings(app, "lap_times")
}

#[tauri::command]
async fn set_lap_times_overlay_settings(app: tauri::AppHandle, settings: LapTimesOverlaySettings) {
    set_settings(app, "lap_times", settings);
}

#[tauri::command]
async fn get_main_overlay_settings(app: tauri::AppHandle) -> MainOverlaySettings {
    get_settings(app, "main")
}

#[tauri::command]
async fn set_main_overlay_settings(app: tauri::AppHandle, settings: MainOverlaySettings) {
    set_settings(app, "main", settings);
}

#[tauri::command]
async fn get_proximity_overlay_settings(app: tauri::AppHandle) -> ProximityOverlaySettings {
    get_settings(app, "proximity")
}

#[tauri::command]
async fn set_proximity_overlay_settings(app: tauri::AppHandle, settings: ProximityOverlaySettings) {
    set_settings(app, "proximity", settings);
}

#[tauri::command]
async fn get_relative_overlay_settings(app: tauri::AppHandle) -> RelativeOverlaySettings {
    get_settings(app, "relative")
}

#[tauri::command]
async fn set_relative_overlay_settings(app: tauri::AppHandle, settings: RelativeOverlaySettings) {
    set_settings(app, "relative", settings);
}

#[tauri::command]
async fn get_standings_overlay_settings(app: tauri::AppHandle) -> StandingsOverlaySettings {
    get_settings(app, "standings")
}

#[tauri::command]
async fn set_standings_overlay_settings(app: tauri::AppHandle, settings: StandingsOverlaySettings) {
    set_settings(app, "standings", settings);
}

#[tauri::command]
async fn get_subtimer_overlay_settings(app: tauri::AppHandle) -> SubTimerOverlaySettings {
    get_settings(app, "subtimer")
}

#[tauri::command]
async fn set_subtimer_overlay_settings(app: tauri::AppHandle, settings: SubTimerOverlaySettings) {
    set_settings(app, "subtimer", settings);
}

#[tauri::command]
async fn get_telemetry_overlay_settings(app: tauri::AppHandle) -> TelemetryOverlaySettings {
    get_settings(app, "telemetry")
}

#[tauri::command]
async fn set_telemetry_overlay_settings(app: tauri::AppHandle, settings: TelemetryOverlaySettings) {
    set_settings(app, "telemetry", settings);
}

#[tauri::command]
async fn get_timer_overlay_settings(app: tauri::AppHandle) -> TimerOverlaySettings {
    get_settings(app, "timer")
}

#[tauri::command]
async fn set_timer_overlay_settings(app: tauri::AppHandle, settings: TimerOverlaySettings) {
    set_settings(app, "timer", settings);
}

#[tauri::command]
async fn get_track_map_overlay_settings(app: tauri::AppHandle) -> TrackMapOverlaySettings {
    get_settings(app, "track_map")
}

#[tauri::command]
async fn set_track_map_overlay_settings(app: tauri::AppHandle, settings: TrackMapOverlaySettings) {
    set_settings(app, "track_map", settings);
}

#[tauri::command]
async fn get_app_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}
