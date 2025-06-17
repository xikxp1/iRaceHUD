import type { TelemetryReferenceOutput } from "$lib/types/telemetry";
import { invoke } from "@tauri-apps/api/core";
import { readable } from "svelte/store";
import { trackID } from "./telemetry.svelte";

export const telemetryReferencePoints = readable<TelemetryReferenceOutput>(
    { reference: [], brake_points: [] },
    (set) => {
        const unsubscribe = trackID.subscribe((trackId) => {
            invoke("get_telemetry_reference_points", { trackId }).then((points) => {
                set(points as TelemetryReferenceOutput);
            });
        });

        return () => {
            unsubscribe();
        };
    });
