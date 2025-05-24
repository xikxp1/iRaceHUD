<script lang="ts">
    import type { TimerOverlaySettings } from "$lib/types/telemetry";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let settings = $state<TimerOverlaySettings | undefined>(undefined);
    let enabled = $derived(settings?.enabled ?? false);
    let opacity = $derived(settings?.opacity ?? 0);
    let delta_enabled = $derived(settings?.delta_enabled ?? false);
    let delta_width = $derived(settings?.delta_width ?? 0);
    let lap_time_width = $derived(settings?.lap_time_width ?? 0);
    let x = $derived(settings?.x ?? 0);
    let y = $derived(settings?.y ?? 0);

    onMount(() => {
        invoke<TimerOverlaySettings>("get_timer_overlay_settings").then((x) => {
            settings = x;
        });
    });

    async function handleEnabledChange(event: Event) {
        if (!settings) return;
        settings.enabled = (event.target as HTMLInputElement).checked;
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    async function handleLapTimeWidthChange(event: Event) {
        if (!settings) return;
        settings.lap_time_width = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    async function handleDeltaEnabledChange(event: Event) {
        if (!settings) return;
        settings.delta_enabled = (event.target as HTMLInputElement).checked;
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    async function handleDeltaWidthChange(event: Event) {
        if (!settings) return;
        settings.delta_width = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    async function handleHorizontalOffsetChange(event: Event) {
        if (!settings) return;
        settings.x = parseInt((event.target as HTMLInputElement).value);
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    async function handleVerticalOffsetChange(event: Event) {
        if (!settings) return;
        settings.y = parseInt((event.target as HTMLInputElement).value);
        invoke("set_timer_overlay_settings", { settings: settings });
    }

    async function handleOpacityChange(event: Event) {
        if (!settings) return;
        settings.opacity = parseInt((event.target as HTMLInputElement).value);
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
                <td class="text-sm font-bold text-right">Lap time width</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={lap_time_width}
                        onchange={handleLapTimeWidthChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Delta enabled</td>
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
                <td class="text-sm font-bold text-right">Delta width</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={delta_width}
                        onchange={handleDeltaWidthChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Horizontal offset</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={x}
                        onchange={handleHorizontalOffsetChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Vertical offset</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={y}
                        onchange={handleVerticalOffsetChange}
                    />
                </td>
            </tr>
        </tbody>
    </table>
</div>

<style>
</style>
