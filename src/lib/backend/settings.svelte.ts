import type { LapTimesOverlaySettings, MainOverlaySettings, ProximityOverlaySettings, RelativeOverlaySettings, StandingsOverlaySettings, SubTimerOverlaySettings, TelemetryOverlaySettings, TelemetryReferenceOverlaySettings, TimerOverlaySettings, TrackMapOverlaySettings } from "$lib/types/telemetry";
import { invoke } from "@tauri-apps/api/core";
import { readable } from "svelte/store";
import { wsClient } from './ws_client';

function createSettingsStore<T>(overlay: string) {
    return readable<T>(undefined, (set) => {
        invoke(`get_${overlay}_overlay_settings`).then((settings) => {
            set(settings as T);
        });

        // Subscribe to WebSocket updates for this overlay's settings
        wsClient.subscribe(`${overlay}_overlay_settings_changed`, (_data: T) => {
            invoke(`get_${overlay}_overlay_settings`).then((settings) => {
                set(settings as T);
            });
        });

        return () => {
            // Cleanup subscription when store is unsubscribed
            wsClient.unsubscribe(`${overlay}_overlay_settings_changed`);
        };
    });
}

export const lapTimesOverlaySettings = createSettingsStore<LapTimesOverlaySettings>("lap_times");
export const mainOverlaySettings = createSettingsStore<MainOverlaySettings>("main");
export const proximityOverlaySettings = createSettingsStore<ProximityOverlaySettings>("proximity");
export const relativeOverlaySettings = createSettingsStore<RelativeOverlaySettings>("relative");
export const standingsOverlaySettings = createSettingsStore<StandingsOverlaySettings>("standings");
export const subtimerOverlaySettings = createSettingsStore<SubTimerOverlaySettings>("subtimer");
export const telemetryOverlaySettings = createSettingsStore<TelemetryOverlaySettings>("telemetry");
export const telemetryReferenceOverlaySettings = createSettingsStore<TelemetryReferenceOverlaySettings>("telemetry_reference");
export const timerOverlaySettings = createSettingsStore<TimerOverlaySettings>("timer");
export const trackMapOverlaySettings = createSettingsStore<TrackMapOverlaySettings>("track_map");
