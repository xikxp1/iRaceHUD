<script lang="ts">
    import {
        gear,
        gearBlinkRPM,
        gearShiftRPM,
        incidentLimit,
        incidents,
        lap,
        lapsTotal,
        position,
        positionsTotal,
        rpm,
        speed,
    } from "$lib/backend/telemetry.svelte";
    import type { MainWidgetSettings, Rpm } from "$lib/types/telemetry";
    import NumberFlow, { continuous } from "@number-flow/svelte";
    import { onDestroy, onMount } from "svelte";

    let { settings }: { settings: MainWidgetSettings } = $props();

    let gear_indicator: HTMLDivElement | undefined = $state();
    let rpm_indicator: HTMLProgressElement | undefined = $state();

    function on_rpm(rpm: Rpm) {
        if (rpm >= $gearBlinkRPM) {
            if (gear_indicator != null) {
                gear_indicator.classList.remove("text-secondary");
                gear_indicator.classList.remove("text-info");
                gear_indicator.classList.add("text-error");
            }

            if (rpm_indicator != null) {
                rpm_indicator.classList.remove("progress-secondary");
                rpm_indicator.classList.remove("progress-info");
                rpm_indicator.classList.add("progress-error");
            }
        } else if (rpm >= $gearShiftRPM) {
            if (gear_indicator != null) {
                gear_indicator.classList.remove("text-secondary");
                gear_indicator.classList.add("text-info");
                gear_indicator.classList.remove("text-error");
            }

            if (rpm_indicator != null) {
                rpm_indicator.classList.remove("progress-secondary");
                rpm_indicator.classList.add("progress-info");
                rpm_indicator.classList.remove("progress-error");
            }
        } else {
            if (gear_indicator != null) {
                gear_indicator.classList.add("text-secondary");
                gear_indicator.classList.remove("text-info");
                gear_indicator.classList.remove("text-error");
            }

            if (rpm_indicator != null) {
                rpm_indicator.classList.add("progress-secondary");
                rpm_indicator.classList.remove("progress-info");
                rpm_indicator.classList.remove("progress-error");
            }
        }
    }

    let unsubscribe_rpm: () => void = () => {};

    onMount(() => {
        unsubscribe_rpm = rpm.subscribe((value) => {
            on_rpm(value);
        });

        on_rpm($rpm);
    });

    onDestroy(() => {
        unsubscribe_rpm();
    });
</script>

<div
    class="flex flex-row items-center justify-center"
    style="opacity: {settings.opacity / 100}"
>
    <div class="join bg-primary-content" style="width: {settings.width}px">
        <div
            class="join-item flex flex-col items-center justify-center rounded-md w-1/4"
        >
            <div
                bind:this={gear_indicator}
                class="text-secondary text-5xl font-square"
            >
                {$gear}
            </div>
            <div class="text-primary text-2xl">{$speed}</div>
        </div>
        <div
            class="join-item flex flex-col items-center justify-evenly rounded-md w-3/4"
        >
            <progress
                bind:this={rpm_indicator}
                class="progress progress-secondary w-5/6 outline outline-2 outline-offset-2 outline-primary"
                value={$rpm}
                max={$gearBlinkRPM}
            ></progress>
            <div class="flex flex-row items-center justify-evenly">
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary">
                        <NumberFlow value={$lap} plugins={[continuous]}
                        ></NumberFlow>{#if $lapsTotal > 0}/{$lapsTotal}{/if}
                    </div>
                    <div class="text-secondary text-xs">lap</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary">
                        {#if $position > 0}
                            <NumberFlow value={$position} plugins={[continuous]}
                            ></NumberFlow>{#if $positionsTotal > 0}/{$positionsTotal}{/if}
                        {:else}
                            -
                        {/if}
                    </div>
                    <div class="text-secondary text-xs">pos</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary">
                        <NumberFlow value={$incidents} plugins={[continuous]}
                        ></NumberFlow>{#if $incidentLimit > 0}/{$incidentLimit}{/if}x
                    </div>
                    <div class="text-secondary text-xs">inc</div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
