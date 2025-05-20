<script lang="ts">
    import type { ProximityWidgetSettings } from "$lib/types/telemetry";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let settings = $state<ProximityWidgetSettings | undefined>(undefined);
    let enabled = $derived(settings?.enabled ?? false);
    let gap_width = $derived(settings?.gap_width ?? 0);
    let x = $derived(settings?.x ?? 0);
    let y = $derived(settings?.y ?? 0);

    onMount(() => {
        invoke<ProximityWidgetSettings>("get_proximity_widget_settings").then(
            (x) => {
                settings = x;
            },
        );
    });

    function handleEnabledChange(event: Event) {
        if (!settings) return;
        settings.enabled = (event.target as HTMLInputElement).checked;
        invoke("set_proximity_widget_settings", { settings: settings });
    }

    function handleWidthChange(event: Event) {
        if (!settings) return;
        settings.gap_width = parseInt((event.target as HTMLInputElement).value);
        invoke("set_proximity_widget_settings", { settings: settings });
    }

    function handleHorizontalOffsetChange(event: Event) {
        if (!settings) return;
        settings.x = parseInt((event.target as HTMLInputElement).value);
        invoke("set_proximity_widget_settings", { settings: settings });
    }

    function handleVerticalOffsetChange(event: Event) {
        if (!settings) return;
        settings.y = parseInt((event.target as HTMLInputElement).value);
        invoke("set_proximity_widget_settings", { settings: settings });
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
                <td class="text-sm font-bold text-right">Gap width</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={gap_width}
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
