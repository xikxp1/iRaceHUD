<script lang="ts">
    import type { SubTimerOverlaySettings } from "$lib/types/telemetry";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let settings = $state<SubTimerOverlaySettings | undefined>(undefined);
    let enabled = $derived(settings?.common_settings.enabled ?? false);
    let opacity = $derived(settings?.common_settings.opacity ?? 100);
    let scale = $derived(settings?.common_settings.scale ?? 100);
    let gap_enabled = $derived(settings?.gap_enabled ?? false);
    let gap_width = $derived(settings?.gap_width ?? 0);

    onMount(() => {
        invoke<SubTimerOverlaySettings>("get_subtimer_overlay_settings").then(
            (x) => {
                settings = x;
            },
        );
    });

    function handleEnabledChange(event: Event) {
        if (!settings) return;
        settings.common_settings.enabled = (
            event.target as HTMLInputElement
        ).checked;
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }

    function handleOpacityChange(event: Event) {
        if (!settings) return;
        settings.common_settings.opacity = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }

    function handleScaleChange(event: Event) {
        if (!settings) return;
        settings.common_settings.scale = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_lap_times_overlay_settings", { settings: settings });
    }

    function handleGapEnabledChange(event: Event) {
        if (!settings) return;
        settings.gap_enabled = (event.target as HTMLInputElement).checked;
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }

    function handleGapWidthChange(event: Event) {
        if (!settings) return;
        settings.gap_width = parseInt((event.target as HTMLInputElement).value);
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }
</script>

<div class="flex w-full h-full items-center justify-center">
    <table>
        <tbody>
            <tr>
                <td class="text-sm font-bold text-right">Enabled</td>
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-3"
                        bind:checked={enabled}
                        onchange={handleEnabledChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Opacity</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={opacity}
                        onchange={handleOpacityChange}
                        min="0"
                        max="100"
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Scale</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={scale}
                        onchange={handleScaleChange}
                        min="20"
                        max="500"
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Gap Enabled</td>
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-3"
                        bind:checked={gap_enabled}
                        onchange={handleGapEnabledChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Gap Width</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={gap_width}
                        onchange={handleGapWidthChange}
                        min="0"
                    />
                </td>
            </tr>
        </tbody>
    </table>
</div>

<style>
</style>
