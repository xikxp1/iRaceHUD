import type { LapTimesWidgetSettings, MainWidgetSettings, ProximityWidgetSettings, RelativeWidgetSettings, StandingsWidgetSettings, SubTimerWidgetSettings, TelemetryWidgetSettings, TimerWidgetSettings, TrackMapWidgetSettings } from "$lib/types/telemetry";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { readable } from "svelte/store";

function createSettingsStore<T>(widget: string) {
    return readable<T>(undefined, (set) => {
        invoke(`get_${widget}_widget_settings`).then((settings) => {
            set(settings as T);
        });

        let unlisten: () => void;

        listen<T>(`${widget}_widget_settings_changed`, (event) => {
            set(event.payload);
        }).then((fn) => {
            unlisten = fn;
        });

        return () => {
            if (unlisten) {
                unlisten();
            }
        };
    });
}

export const lapTimesWidgetSettings = createSettingsStore<LapTimesWidgetSettings>("lap_times");
export const mainWidgetSettings = createSettingsStore<MainWidgetSettings>("main");
export const proximityWidgetSettings = createSettingsStore<ProximityWidgetSettings>("proximity");
export const relativeWidgetSettings = createSettingsStore<RelativeWidgetSettings>("relative");
export const standingsWidgetSettings = createSettingsStore<StandingsWidgetSettings>("standings");
export const subtimerWidgetSettings = createSettingsStore<SubTimerWidgetSettings>("subtimer");
export const telemetryWidgetSettings = createSettingsStore<TelemetryWidgetSettings>("telemetry");
export const timerWidgetSettings = createSettingsStore<TimerWidgetSettings>("timer");
export const trackMapWidgetSettings = createSettingsStore<TrackMapWidgetSettings>("track_map");
