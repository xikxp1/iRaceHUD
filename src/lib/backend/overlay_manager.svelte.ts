import { readable } from "svelte/store";
import { wsClient } from "./ws_client";
import { isTauri } from "./vr_mode";

// Get locked state via Tauri IPC
async function getOverlaysLocked(): Promise<boolean> {
    if (isTauri()) {
        const { invoke } = await import('@tauri-apps/api/core');
        return invoke<boolean>('get_overlays_locked');
    }
    // In VR mode, overlays are never "locked" - they're always interactive web pages
    return false;
}

export const isLocked = readable<boolean>(false, (set) => {
    getOverlaysLocked().then((isLocked) => {
        set(isLocked);
    });

    // Subscribe to WebSocket updates for this overlay's settings
    wsClient.subscribe(`overlay_locked_unlocked`, (data: boolean) => {
        // Only update in Tauri mode; in VR mode, keep always unlocked
        if (isTauri()) {
            set(data);
        }
    });

    return () => {
        // Cleanup subscription when store is unsubscribed
        wsClient.unsubscribe(`overlay_locked_unlocked`);
    };
});
