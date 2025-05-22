<script lang="ts">
    import type { StandingsWidgetSettings } from "$lib/types/telemetry";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { z } from "zod/v4";

    const standingsSchema = z
        .object({
            enabled: z.boolean(),
            x: z.number(),
            y: z.number(),
            width: z.number().min(1),
            opacity: z.number().min(0).max(100),
            max_drivers: z.number().min(1),
            top_drivers: z.number().min(1),
        })
        .refine((data) => data.top_drivers <= data.max_drivers, {
            message: "Top drivers count cannot exceed max drivers count",
            path: ["top_drivers"],
        });

    let settings = $state<StandingsWidgetSettings | undefined>(undefined);
    let enabled = $derived(settings?.enabled ?? false);
    let opacity = $derived(settings?.opacity ?? 0);
    let width = $derived(settings?.width ?? 0);
    let x = $derived(settings?.x ?? 0);
    let y = $derived(settings?.y ?? 0);
    let maxDrivers = $derived(settings?.max_drivers ?? 0);
    let topDrivers = $derived(settings?.top_drivers ?? 0);
    let error = $state<string | null>(null);

    onMount(() => {
        invoke<StandingsWidgetSettings>("get_standings_widget_settings").then(
            (x) => {
                settings = x;
            },
        );
    });

    function validateSettings(newSettings: StandingsWidgetSettings) {
        try {
            standingsSchema.parse(newSettings);
            error = null;
            return true;
        } catch (e) {
            if (e instanceof z.ZodError) {
                error = e.issues.map((issue) => issue.message).join("\n");
            }
            return false;
        }
    }

    function handleEnabledChange(event: Event) {
        if (!settings) return;
        settings.enabled = (event.target as HTMLInputElement).checked;
        if (validateSettings(settings)) {
            invoke("set_standings_widget_settings", { settings: settings });
        }
    }

    function handleWidthChange(event: Event) {
        if (!settings) return;
        settings.width = parseInt((event.target as HTMLInputElement).value);
        if (validateSettings(settings)) {
            invoke("set_standings_widget_settings", { settings: settings });
        }
    }

    function handleHorizontalOffsetChange(event: Event) {
        if (!settings) return;
        settings.x = parseInt((event.target as HTMLInputElement).value);
        if (validateSettings(settings)) {
            invoke("set_standings_widget_settings", { settings: settings });
        }
    }

    function handleVerticalOffsetChange(event: Event) {
        if (!settings) return;
        settings.y = parseInt((event.target as HTMLInputElement).value);
        if (validateSettings(settings)) {
            invoke("set_standings_widget_settings", { settings: settings });
        }
    }

    function handleOpacityChange(event: Event) {
        if (!settings) return;
        settings.opacity = parseInt((event.target as HTMLInputElement).value);
        if (validateSettings(settings)) {
            invoke("set_standings_widget_settings", { settings: settings });
        }
    }

    function handleMaxDriversChange(event: Event) {
        if (!settings) return;
        settings.max_drivers = parseInt(
            (event.target as HTMLInputElement).value,
        );
        if (validateSettings(settings)) {
            invoke("set_standings_widget_settings", { settings: settings });
        }
    }

    function handleTopDriversChange(event: Event) {
        if (!settings) return;
        settings.top_drivers = parseInt(
            (event.target as HTMLInputElement).value,
        );
        if (validateSettings(settings)) {
            invoke("set_standings_widget_settings", { settings: settings });
        }
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
            <tr>
                <td class="text-sm font-bold text-right">Max drivers count</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={maxDrivers}
                        onchange={handleMaxDriversChange}
                        min="1"
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Top drivers count</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={topDrivers}
                        onchange={handleTopDriversChange}
                        min="1"
                    />
                </td>
            </tr>
        </tbody>
    </table>
    {#if error}
        <div class="toast toast-end">
            <div class="alert alert-error">
                <span class="text-xs">{error}</span>
            </div>
        </div>
    {/if}
</div>

<style>
</style>
