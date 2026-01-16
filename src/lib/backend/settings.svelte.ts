import type { LapTimesOverlaySettings, MainOverlaySettings, ProximityOverlaySettings, RelativeOverlaySettings, StandingsOverlaySettings, SubTimerOverlaySettings, TelemetryOverlaySettings, TelemetryReferenceOverlaySettings, TimerOverlaySettings, TrackMapOverlaySettings } from "$lib/types/telemetry";
import { readable } from "svelte/store";
import { wsClient } from './ws_client';
import { isTauri, getVrHost } from './vr_mode';

// Get settings via Tauri IPC
async function getSettingsViaTauri<T>(overlay: string): Promise<T> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<T>(`get_${overlay}_overlay_settings`);
}

// Get settings via HTTP API (for VR mode)
async function getSettingsViaHttp<T>(overlay: string): Promise<T | undefined> {
    try {
        const host = getVrHost();
        const response = await fetch(`${host}/api/settings/${overlay}`);
        if (response.ok) {
            return await response.json() as T;
        }
    } catch (error) {
        console.error(`Failed to fetch ${overlay} settings via HTTP:`, error);
    }
    return undefined;
}

// Get settings using appropriate method
async function getSettings<T>(overlay: string): Promise<T | undefined> {
    if (isTauri()) {
        return getSettingsViaTauri<T>(overlay);
    } else {
        return getSettingsViaHttp<T>(overlay);
    }
}

function createSettingsStore<T>(overlay: string) {
    return readable<T>(undefined, (set) => {
        getSettings<T>(overlay).then((settings) => {
            if (settings) {
                set(settings);
            }
        });

        // Subscribe to WebSocket updates for this overlay's settings
        wsClient.subscribe(`${overlay}_overlay_settings_changed`, (_data: T) => {
            getSettings<T>(overlay).then((settings) => {
                if (settings) {
                    set(settings);
                }
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
