import type { TelemetryReference } from "$lib/types/telemetry";
import { invoke } from "@tauri-apps/api/core";
import { readable } from "svelte/store";
import { trackID } from "./telemetry.svelte";

export const telemetryReferencePoints = readable<TelemetryReference[]>([], (set) => {
    const unsubscribe = trackID.subscribe((trackId) => {
        invoke("get_telemetry_reference_points", { trackId }).then((points) => {
            set(points as TelemetryReference[]);
        });
    });

    return () => {
        unsubscribe();
    };
});
