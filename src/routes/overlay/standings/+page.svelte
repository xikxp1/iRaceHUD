<script lang="ts">
    import { isLocked } from "$lib/backend/overlay_manager.svelte";
    import { standingsOverlaySettings } from "$lib/backend/settings.svelte";
    import { active } from "$lib/backend/telemetry.svelte";
    import { isTauri } from "$lib/backend/vr_mode";
    import { isOpenKneeboard, setPreferredPixelSize } from "$lib/backend/openkneeboard";
    import Standings from "../../../components/overlays/Standings.svelte";

    let settings = $derived(standingsOverlaySettings);

    let enabled = $derived($settings?.common_settings?.enabled ?? false);
    let opacity = $derived($settings?.common_settings?.opacity ?? 100);
    let width = $derived($settings?.common_settings?.width ?? 0);
    let height = $derived($settings?.common_settings?.height ?? 0);
    let scale = $derived(($settings?.common_settings?.scale ?? 100) / 100.0);

    let isOKB = isOpenKneeboard();

    $effect(() => {
        if (width > 0 && height > 0) {
            if (isTauri()) {
                import("@tauri-apps/api/window").then(({ getCurrentWindow, LogicalSize }) => {
                    const window = getCurrentWindow();
                    window.setResizable(true);
                    window.setSize(new LogicalSize(width * scale, height * scale));
                    window.setResizable(false);
                });
            } else if (isOKB) {
                setPreferredPixelSize(width * scale, height * scale);
            }
        }
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
            <span class="text-primary text-5xl text-center">Standings</span>
        </div>
    {/if}
    {#if enabled && $active}
        <div
            class="overlay-container"
            style="opacity: {opacity}%; width: {width}px; height: {height}px"
        >
            <Standings settings={$settings} />
        </div>
    {/if}
</div>

<style>
</style>
