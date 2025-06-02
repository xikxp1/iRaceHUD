<script lang="ts">
    import type { TimerOverlaySettings } from "$lib/types/telemetry";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let settings = $state<TimerOverlaySettings | undefined>(undefined);
    let enabled = $derived(settings?.common_settings.enabled ?? false);
    let opacity = $derived(settings?.common_settings.opacity ?? 100);
    let scale = $derived(settings?.common_settings.scale ?? 100);
    let delta_enabled = $derived(settings?.delta_enabled ?? false);
    let delta_width = $derived(settings?.delta_width ?? 0);

    onMount(() => {
        invoke<TimerOverlaySettings>("get_timer_overlay_settings").then((x) => {
            settings = x;
        });
    });

    function handleEnabledChange(event: Event) {
        if (!settings) return;
        settings.common_settings.enabled = (
            event.target as HTMLInputElement
        ).checked;
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    function handleOpacityChange(event: Event) {
        if (!settings) return;
        settings.common_settings.opacity = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    function handleScaleChange(event: Event) {
        if (!settings) return;
        settings.common_settings.scale = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    function handleDeltaEnabledChange(event: Event) {
        if (!settings) return;
        settings.delta_enabled = (event.target as HTMLInputElement).checked;
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    function handleDeltaWidthChange(event: Event) {
        if (!settings) return;
        settings.delta_width = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_timer_overlay_settings", { settings: settings });
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
                <td class="text-sm font-bold text-right">Delta Enabled</td>
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-3"
                        bind:checked={delta_enabled}
                        onchange={handleDeltaEnabledChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Delta Width</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={delta_width}
                        onchange={handleDeltaWidthChange}
                        min="0"
                    />
                </td>
            </tr>
        </tbody>
    </table>
</div>

<style>
</style>
