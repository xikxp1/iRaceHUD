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
    } from "$lib/telemetry/telemetry.svelte";
    import type { Rpm } from "$lib/types/telemetry";
    import { onMount } from "svelte";

    let gear_indicator: HTMLDivElement;
    let rpm_indicator: HTMLProgressElement;

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

    onMount(() => {
        rpm.subscribe((value) => {
            on_rpm(value);
        });

        on_rpm($rpm);
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="join w-[17%] bg-primary-content">
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
                        {$lap}{#if $lapsTotal > 0}/{$lapsTotal}{/if}
                    </div>
                    <div class="text-secondary text-xs">lap</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary">
                        {#if $position > 0}
                            {$position}{#if $positionsTotal > 0}/{$positionsTotal}{/if}
                        {:else}
                            -
                        {/if}
                    </div>
                    <div class="text-secondary text-xs">pos</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary">
                        {$incidents}{#if $incidentLimit > 0}/{$incidentLimit}{/if}
                    </div>
                    <div class="text-secondary text-xs">inc</div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
