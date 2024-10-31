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
    import { onMount } from "svelte";

    let gear_indicator: HTMLDivElement;
    let rpm_incidator: HTMLProgressElement;

    onMount(() => {
        rpm.subscribe((value) => {
            if (value >= $gearBlinkRPM) {
                gear_indicator.classList.remove("text-secondary");
                gear_indicator.classList.remove("text-info");
                gear_indicator.classList.add("text-error");

                rpm_incidator.classList.remove("progress-secondary");
                rpm_incidator.classList.remove("progress-info");
                rpm_incidator.classList.add("progress-error");
            } else if (value >= $gearShiftRPM) {
                gear_indicator.classList.remove("text-secondary");
                gear_indicator.classList.add("text-info");
                gear_indicator.classList.remove("text-error");

                rpm_incidator.classList.remove("progress-secondary");
                rpm_incidator.classList.add("progress-info");
                rpm_incidator.classList.remove("progress-error");
            } else {
                gear_indicator.classList.add("text-secondary");
                gear_indicator.classList.remove("text-info");
                gear_indicator.classList.remove("text-error");

                rpm_incidator.classList.add("progress-secondary");
                rpm_incidator.classList.remove("progress-info");
                rpm_incidator.classList.remove("progress-error");
            }
        });
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
                bind:this={rpm_incidator}
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
