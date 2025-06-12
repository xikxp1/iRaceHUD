use std::{collections::HashMap, sync::OnceLock};

use log::info;
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::emitter::overlay_manager_emitter::emit_overlay_locked_unlocked;
use crate::settings::overlays::common_settings::CommonSettings;
use crate::settings::overlays::{
    lap_times::LapTimesOverlaySettings, main::MainOverlaySettings,
    proximity::ProximityOverlaySettings, relative::RelativeOverlaySettings,
    standings::StandingsOverlaySettings, subtimer::SubTimerOverlaySettings,
    telemetry::TelemetryOverlaySettings, telemetry_reference::TelemetryReferenceOverlaySettings,
    timer::TimerOverlaySettings, track_map::TrackMapOverlaySettings,
};
use crate::util::settings_helper::{get_common_settings, get_settings, set_settings};

pub const AVAILABLE_OVERLAYS: [&str; 10] = [
    "main",
    "standings",
    "lap_times",
    "proximity",
    "relative",
    "subtimer",
    "telemetry",
    "telemetry_reference",
    "timer",
    "track_map",
];

pub static OVERLAY_MANAGER: OnceLock<OverlayManager> = OnceLock::new();

#[derive(Debug)]
pub struct OverlayManager {
    overlay_windows: HashMap<String, WebviewWindow>,
    locked: bool,
}

impl OverlayManager {
    pub fn new() -> Self {
        Self {
            overlay_windows: HashMap::new(),
            locked: true,
        }
    }

    pub fn register_overlay(&mut self, app_handle: &tauri::AppHandle, overlay_name: &str) {
        if !AVAILABLE_OVERLAYS.contains(&overlay_name) {
            panic!("Unknown overlay type: {}", overlay_name);
        }

        if self.overlay_windows.contains_key(overlay_name) {
            info!("Overlay {} is already registered", overlay_name);
            return;
        }

        let settings: CommonSettings = match overlay_name {
            "main" => get_common_settings::<MainOverlaySettings>(app_handle.clone(), overlay_name),
            "standings" => {
                get_common_settings::<StandingsOverlaySettings>(app_handle.clone(), overlay_name)
            }
            "lap_times" => {
                get_common_settings::<LapTimesOverlaySettings>(app_handle.clone(), overlay_name)
            }
            "proximity" => {
                get_common_settings::<ProximityOverlaySettings>(app_handle.clone(), overlay_name)
            }
            "relative" => {
                get_common_settings::<RelativeOverlaySettings>(app_handle.clone(), overlay_name)
            }
            "subtimer" => {
                get_common_settings::<SubTimerOverlaySettings>(app_handle.clone(), overlay_name)
            }
            "telemetry" => {
                get_common_settings::<TelemetryOverlaySettings>(app_handle.clone(), overlay_name)
            }
            "telemetry_reference" => get_common_settings::<TelemetryReferenceOverlaySettings>(
                app_handle.clone(),
                overlay_name,
            ),
            "timer" => {
                get_common_settings::<TimerOverlaySettings>(app_handle.clone(), overlay_name)
            }
            "track_map" => {
                get_common_settings::<TrackMapOverlaySettings>(app_handle.clone(), overlay_name)
            }
            _ => panic!("Unknown overlay type: {}", overlay_name),
        };

        if !settings.enabled {
            info!("Overlay {} is disabled", overlay_name);
            return;
        }

        let width = settings.width;
        let height = settings.height;
        let scale = settings.scale;

        let window = WebviewWindowBuilder::new(
            app_handle,
            overlay_name,
            WebviewUrl::App(format!("/overlay/{}", overlay_name).into()),
        )
        .title(format!("iRaceHUD - {}", overlay_name))
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .closable(false)
        .transparent(true)
        .decorations(false)
        .drag_and_drop(false)
        .focused(false)
        .inner_size(
            (width as f64 * scale as f64) / 100.0,
            (height as f64 * scale as f64) / 100.0,
        )
        .position(settings.x as f64, settings.y as f64)
        .accept_first_mouse(true)
        .visible(true)
        .shadow(false)
        .build()
        .unwrap();

        let _ = window.set_ignore_cursor_events(self.locked);

        self.overlay_windows
            .insert(overlay_name.to_string(), window);
    }

    pub fn set_locked_unlocked(&mut self) {
        self.locked = !self.locked;
        for window in self.overlay_windows.values_mut() {
            let _ = window.set_ignore_cursor_events(self.locked);
        }
        emit_overlay_locked_unlocked(self.locked);
    }

    pub fn save_positions(&self, app_handle: &tauri::AppHandle) {
        for (overlay_name, window) in self.overlay_windows.iter() {
            let position = window.inner_position();
            if position.is_ok() {
                let position = position.unwrap();
                match overlay_name.as_str() {
                    "main" => {
                        let mut settings =
                            get_settings::<MainOverlaySettings>(app_handle.clone(), overlay_name);
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    "standings" => {
                        let mut settings = get_settings::<StandingsOverlaySettings>(
                            app_handle.clone(),
                            overlay_name,
                        );
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    "lap_times" => {
                        let mut settings = get_settings::<LapTimesOverlaySettings>(
                            app_handle.clone(),
                            overlay_name,
                        );
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    "proximity" => {
                        let mut settings = get_settings::<ProximityOverlaySettings>(
                            app_handle.clone(),
                            overlay_name,
                        );
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    "relative" => {
                        let mut settings = get_settings::<RelativeOverlaySettings>(
                            app_handle.clone(),
                            overlay_name,
                        );
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    "subtimer" => {
                        let mut settings = get_settings::<SubTimerOverlaySettings>(
                            app_handle.clone(),
                            overlay_name,
                        );
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    "telemetry" => {
                        let mut settings = get_settings::<TelemetryOverlaySettings>(
                            app_handle.clone(),
                            overlay_name,
                        );
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    "telemetry_reference" => {
                        let mut settings = get_settings::<TelemetryReferenceOverlaySettings>(
                            app_handle.clone(),
                            overlay_name,
                        );
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    "timer" => {
                        let mut settings =
                            get_settings::<TimerOverlaySettings>(app_handle.clone(), overlay_name);
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    "track_map" => {
                        let mut settings = get_settings::<TrackMapOverlaySettings>(
                            app_handle.clone(),
                            overlay_name,
                        );
                        settings.common_settings.x = position.x as u32;
                        settings.common_settings.y = position.y as u32;
                        set_settings(app_handle.clone(), overlay_name, settings);
                    }
                    _ => panic!("Unknown overlay type: {}", overlay_name),
                }
            }
        }
    }

    pub fn get_locked(&self) -> bool {
        self.locked
    }
}

impl Default for OverlayManager {
    fn default() -> Self {
        Self::new()
    }
}
