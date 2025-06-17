<script lang="ts">
    import type { TelemetryReferenceOverlaySettings } from "$lib/types/telemetry";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let settings = $state<TelemetryReferenceOverlaySettings | undefined>(
        undefined,
    );
    let enabled = $derived(settings?.common_settings.enabled ?? false);
    let opacity = $derived(settings?.common_settings.opacity ?? 100);
    let scale = $derived(settings?.common_settings.scale ?? 100);
    let showThrottle = $derived(settings?.show_throttle ?? true);
    let showSteering = $derived(settings?.show_steering ?? true);
    let brakeQue0Enabled = $derived(settings?.brake_que_0_enabled ?? true);
    let brakeQue1Enabled = $derived(settings?.brake_que_1_enabled ?? true);
    let brakeQue2Enabled = $derived(settings?.brake_que_2_enabled ?? true);
    let brakeQue3Enabled = $derived(settings?.brake_que_3_enabled ?? true);
    let brakeQue1Distance = $derived(settings?.brake_que_1_distance ?? 0);
    let brakeQue2Distance = $derived(settings?.brake_que_2_distance ?? 0);
    let brakeQue3Distance = $derived(settings?.brake_que_3_distance ?? 0);

    onMount(() => {
        invoke<TelemetryReferenceOverlaySettings>(
            "get_telemetry_reference_overlay_settings",
        ).then((x) => {
            settings = x;
        });
    });

    function handleEnabledChange(event: Event) {
        if (!settings) return;
        settings.common_settings.enabled = (
            event.target as HTMLInputElement
        ).checked;
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleOpacityChange(event: Event) {
        if (!settings) return;
        settings.common_settings.opacity = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleScaleChange(event: Event) {
        if (!settings) return;
        settings.common_settings.scale = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleShowThrottleChange(event: Event) {
        if (!settings) return;
        settings.show_throttle = (event.target as HTMLInputElement).checked;
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleShowSteeringChange(event: Event) {
        if (!settings) return;
        settings.show_steering = (event.target as HTMLInputElement).checked;
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleBrakeQue0EnabledChange(event: Event) {
        if (!settings) return;
        settings.brake_que_0_enabled = (
            event.target as HTMLInputElement
        ).checked;
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleBrakeQue1EnabledChange(event: Event) {
        if (!settings) return;
        settings.brake_que_1_enabled = (
            event.target as HTMLInputElement
        ).checked;
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleBrakeQue2EnabledChange(event: Event) {
        if (!settings) return;
        settings.brake_que_2_enabled = (
            event.target as HTMLInputElement
        ).checked;
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleBrakeQue3EnabledChange(event: Event) {
        if (!settings) return;
        settings.brake_que_3_enabled = (
            event.target as HTMLInputElement
        ).checked;
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleBrakeQue1DistanceChange(event: Event) {
        if (!settings) return;
        settings.brake_que_1_distance = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleBrakeQue2DistanceChange(event: Event) {
        if (!settings) return;
        settings.brake_que_2_distance = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    function handleBrakeQue3DistanceChange(event: Event) {
        if (!settings) return;
        settings.brake_que_3_distance = parseInt(
            (event.target as HTMLInputElement).value,
        );
        invoke("set_telemetry_reference_overlay_settings", {
            settings: settings,
        });
    }

    async function handleRecordTelemetry() {
        invoke("record_telemetry");
    }
</script>

<div class="flex w-full h-full items-center justify-center flex-col">
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
                <td class="text-sm font-bold text-right">Show throttle</td>
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-3"
                        bind:checked={showThrottle}
                        onchange={handleShowThrottleChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Show steering</td>
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-3"
                        bind:checked={showSteering}
                        onchange={handleShowSteeringChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Brake que 0 enabled</td
                >
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-3"
                        bind:checked={brakeQue0Enabled}
                        onchange={handleBrakeQue0EnabledChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Brake que 1 enabled</td
                >
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-3"
                        bind:checked={brakeQue1Enabled}
                        onchange={handleBrakeQue1EnabledChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right"
                    >Brake que 1 distance</td
                >
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={brakeQue1Distance}
                        onchange={handleBrakeQue1DistanceChange}
                        min="0"
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Brake que 2 enabled</td
                >
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-3"
                        bind:checked={brakeQue2Enabled}
                        onchange={handleBrakeQue2EnabledChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right"
                    >Brake que 2 distance</td
                >
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={brakeQue2Distance}
                        onchange={handleBrakeQue2DistanceChange}
                        min="0"
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right">Brake que 3 enabled</td
                >
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-3"
                        bind:checked={brakeQue3Enabled}
                        onchange={handleBrakeQue3EnabledChange}
                    />
                </td>
            </tr>
            <tr>
                <td class="text-sm font-bold text-right"
                    >Brake que 3 distance</td
                >
                <td>
                    <input
                        type="number"
                        class="input input-sm w-24"
                        bind:value={brakeQue3Distance}
                        onchange={handleBrakeQue3DistanceChange}
                        min="0"
                    />
                </td>
            </tr>
        </tbody>
    </table>
    <button
        class="btn btn-secondary btn-sm mt-4"
        onclick={handleRecordTelemetry}
    >
        Record Telemetry
    </button>
</div>

<style>
</style>
