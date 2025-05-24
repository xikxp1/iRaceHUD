<script lang="ts">
    import type { SubTimerOverlaySettings } from "$lib/types/telemetry";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let settings = $state<SubTimerOverlaySettings | undefined>(undefined);
    let enabled = $derived(settings?.enabled ?? false);
    let opacity = $derived(settings?.opacity ?? 0);
    let session_state_width = $derived(settings?.session_state_width ?? 0);
    let gap_enabled = $derived(settings?.gap_enabled ?? false);
    let gap_width = $derived(settings?.gap_width ?? 0);
    let x = $derived(settings?.x ?? 0);
    let y = $derived(settings?.y ?? 0);

    onMount(() => {
        invoke<SubTimerOverlaySettings>("get_subtimer_overlay_settings").then(
            (x) => {
                settings = x;
            },
        );
    });

    async function handleEnabledChange(event: Event) {
        if (!settings) return;
        settings.enabled = (event.target as HTMLInputElement).checked;
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }

    async function handleSessionStateWidthChange(event: Event) {
        if (!settings) return;
        settings.session_state_width = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }

    async function handleGapEnabledChange(event: Event) {
        if (!settings) return;
        settings.gap_enabled = (event.target as HTMLInputElement).checked;
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }

    async function handleGapWidthChange(event: Event) {
        if (!settings) return;
        settings.gap_width = parseInt((event.target as HTMLInputElement).value);
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }

    async function handleHorizontalOffsetChange(event: Event) {
        if (!settings) return;
        settings.x = parseInt((event.target as HTMLInputElement).value);
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }

    async function handleVerticalOffsetChange(event: Event) {
        if (!settings) return;
        settings.y = parseInt((event.target as HTMLInputElement).value);
        invoke("set_subtimer_overlay_settings", { settings: settings });
    }

    async function handleOpacityChange(event: Event) {
        if (!settings) return;
        settings.opacity = parseInt((event.target as HTMLInputElement).value);
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
                <td class="text-sm font-bold text-right">Session state width</td
                >
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={session_state_width}
                        onchange={handleSessionStateWidthChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Gap enabled</td>
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
                <td class="text-sm font-bold text-right">Gap width</td>
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={gap_width}
                        onchange={handleGapWidthChange}
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
