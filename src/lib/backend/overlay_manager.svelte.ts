import { invoke } from "@tauri-apps/api/core";
import { readable } from "svelte/store";
import { wsClient } from "./ws_client";

export const isLocked = readable<boolean>(false, (set) => {
    invoke(`get_overlays_locked`).then((isLocked) => {
        set(isLocked as boolean);
    });

    // Subscribe to WebSocket updates for this overlay's settings
    wsClient.subscribe(`overlay_locked_unlocked`, (data: boolean) => {
        set(data);
    });

    return () => {
        // Cleanup subscription when store is unsubscribed
        wsClient.unsubscribe(`overlay_locked_unlocked`);
    };
});
