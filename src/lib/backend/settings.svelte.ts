import type { LapTimesWidgetSettings, MainWidgetSettings, ProximityWidgetSettings, RelativeWidgetSettings, StandingsWidgetSettings, SubTimerWidgetSettings, TelemetryWidgetSettings, TimerWidgetSettings, TrackMapWidgetSettings } from "$lib/types/telemetry";
import { invoke } from "@tauri-apps/api/core";
import { readable } from "svelte/store";
import { wsClient } from './ws_client';

function createSettingsStore<T>(widget: string) {
    return readable<T>(undefined, (set) => {
        invoke(`get_${widget}_widget_settings`).then((settings) => {
            set(settings as T);
        });

        // Subscribe to WebSocket updates for this widget's settings
        wsClient.subscribe(`${widget}_widget_settings_changed`, (_data: T) => {
            invoke(`get_${widget}_widget_settings`).then((settings) => {
                console.log(`${widget}_widget_settings_changed`, settings);
                set(settings as T);
            });
        });

        return () => {
            // Cleanup subscription when store is unsubscribed
            wsClient.unsubscribe(`${widget}_widget_settings_changed`);
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
