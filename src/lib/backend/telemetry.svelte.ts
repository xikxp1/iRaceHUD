import type {
    Position, CurrentTime, PlayerLapTimes, Standings, StrengthOfField, Lap, Proximity, Relative,
    LapTime, DeltaBestTime, DeltaLastTime, TelemetryGraph, SessionState, GapNext, GapPrev,
    TrackId, TrackMap, Gear, Speed, Rpm, Active, GearShiftRpm, GearBlinkRpm, Incidents, RaceLaps,
    LapsTotal, PlayerCarClass
} from "$lib/types/telemetry";
import { invoke } from "@tauri-apps/api/core";
import { readable } from 'svelte/store';
import { wsClient } from './ws_client';

function createTelemetryStore<T>(event: string, initialValue: T) {
    return readable<T>(initialValue, (set) => {
        invoke("register_event_emitter", { event });

        // Set up WebSocket subscription
        wsClient.subscribe(event, (message: T) => {
            set(message);
        });

        return () => {
            // Cleanup
            invoke("unregister_event_emitter", { event });
            wsClient.unsubscribe(event);
        };
    });
}

export const active = createTelemetryStore<Active>("active", false);
export const currentTime = createTelemetryStore<CurrentTime>("current_time", "--:--");
export const lapTimes = createTelemetryStore<PlayerLapTimes>("player_lap_times", []);
export const standings = createTelemetryStore<Standings>("standings", []);
export const strengthOfField = createTelemetryStore<StrengthOfField>("strength_of_field", 0);
export const positionsTotal = createTelemetryStore<Position>("positions_total", 0);
export const raceLaps = createTelemetryStore<RaceLaps>("race_laps", 0);
export const proximity = createTelemetryStore<Proximity>("proximity", { is_left: false, is_right: false });
export const relative = createTelemetryStore<Relative>("relative", []);
export const lapTime = createTelemetryStore<LapTime>("lap_time", 0);
export const deltaBestTime = createTelemetryStore<DeltaBestTime>("delta_best_time", "–");
export const deltaLastTime = createTelemetryStore<DeltaLastTime>("delta_last_time", "–");
export const telemetry = createTelemetryStore<TelemetryGraph>("telemetry_graph", { ts: 0, throttle: 0, brake: 0, abs_active: false });
export const sessionState = createTelemetryStore<SessionState>("session_state", "");
export const gapNext = createTelemetryStore<GapNext>("gap_next", "-");
export const gapPrev = createTelemetryStore<GapPrev>("gap_prev", "-");
export const trackID = createTelemetryStore<TrackId>("track_id", 0);
export const trackMap = createTelemetryStore<TrackMap>("track_map", []);
export const gear = createTelemetryStore<Gear>("gear", "N");
export const speed = createTelemetryStore<Speed>("speed", 0);
export const rpm = createTelemetryStore<Rpm>("rpm", 0);
export const gearShiftRPM = createTelemetryStore<GearShiftRpm>("gear_shift_rpm", 0);
export const gearBlinkRPM = createTelemetryStore<GearBlinkRpm>("gear_blink_rpm", 0);
export const lap = createTelemetryStore<Lap>("lap", 0);
export const lapsTotal = createTelemetryStore<LapsTotal>("laps_total", 0);
export const position = createTelemetryStore<Position>("position", 0);
export const incidents = createTelemetryStore<Incidents>("incidents", 0);
export const incidentLimit = createTelemetryStore<Incidents>("incident_limit", 0);
export const playerCarClass = createTelemetryStore<PlayerCarClass>("player_car_class", "");
