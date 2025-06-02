<script lang="ts">
    import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
    import { isLocked } from "$lib/backend/overlay_manager.svelte";
    import { proximityOverlaySettings } from "$lib/backend/settings.svelte";
    import { active } from "$lib/backend/telemetry.svelte";
    import Proximity from "../../../components/overlays/Proximity.svelte";

    const window = getCurrentWindow();

    let settings = $derived(proximityOverlaySettings);

    let enabled = $derived($settings?.common_settings?.enabled ?? false);
    let opacity = $derived($settings?.common_settings?.opacity ?? 100);
    let width = $derived($settings?.common_settings?.width ?? 0);
    let height = $derived($settings?.common_settings?.height ?? 0);
    let scale = $derived(($settings?.common_settings?.scale ?? 100) / 100.0);

    $effect(() => {
        window.setResizable(true);
        window.setSize(new LogicalSize(width * scale, height * scale));
        window.setResizable(false);
    });
</script>

<div
    class="scale-container"
    style="transform: scale({scale}); transform-origin: top left;"
>
    {#if !$isLocked}
        <div
            class="drag-region bg-primary-content bg-opacity-20"
            style="width: {width}px; height: {height}px"
        ></div>
        <div
            class="drag-region-text"
            style="width: {width}px; height: {height}px"
        >
            <span class="text-primary text-5xl text-center">Proximity</span>
        </div>
    {/if}
    {#if enabled && $active}
        <div
            class="overlay-container"
            style="width: {width}px; height: {height}px"
        >
            <Proximity settings={$settings} />
        </div>
    {/if}
</div>

<style>
</style>
