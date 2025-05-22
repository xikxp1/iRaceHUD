<script lang="ts">
    import type { LapTimesWidgetSettings } from "$lib/types/telemetry";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let settings = $state<LapTimesWidgetSettings | undefined>(undefined);
    let enabled = $derived(settings?.enabled ?? false);
    let opacity = $derived(settings?.opacity ?? 0);
    let width = $derived(settings?.width ?? 0);
    let x = $derived(settings?.x ?? 0);
    let y = $derived(settings?.y ?? 0);

    onMount(() => {
        invoke<LapTimesWidgetSettings>("get_lap_times_widget_settings").then(
            (x) => {
                settings = x;
            },
        );
    });

    function handleEnabledChange(event: Event) {
        if (!settings) return;
        settings.enabled = (event.target as HTMLInputElement).checked;
        invoke("set_lap_times_widget_settings", { settings: settings });
    }

    function handleWidthChange(event: Event) {
        if (!settings) return;
        settings.width = parseInt((event.target as HTMLInputElement).value);
        invoke("set_lap_times_widget_settings", { settings: settings });
    }

    function handleHorizontalOffsetChange(event: Event) {
        if (!settings) return;
        settings.x = parseInt((event.target as HTMLInputElement).value);
        invoke("set_lap_times_widget_settings", { settings: settings });
    }

    function handleVerticalOffsetChange(event: Event) {
        if (!settings) return;
        settings.y = parseInt((event.target as HTMLInputElement).value);
        invoke("set_lap_times_widget_settings", { settings: settings });
    }

    function handleOpacityChange(event: Event) {
        if (!settings) return;
        settings.opacity = parseInt((event.target as HTMLInputElement).value);
        invoke("set_lap_times_widget_settings", { settings: settings });
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
                <td class="text-sm font-bold text-right">Width</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={width}
                        onchange={handleWidthChange}
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
