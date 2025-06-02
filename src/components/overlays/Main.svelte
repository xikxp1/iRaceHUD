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
    import type { MainOverlaySettings, Rpm } from "$lib/types/telemetry";
    import NumberFlow, { continuous } from "@number-flow/svelte";
    import { onDestroy, onMount } from "svelte";

    let { settings }: { settings: MainOverlaySettings } = $props();

    let gear_indicator: HTMLDivElement | undefined = $state();
    let rpm_indicator: HTMLProgressElement | undefined = $state();
    let animationFrameId: number;
    let gearValue = $state($gear);
    let rpmValue = $state($rpm);
    let speedValue = $state($speed);
    let positionValue = $state($position);
    let incidentsValue = $state($incidents);
    let lapValue = $state($lap);
    let lapsTotalValue = $state($lapsTotal);
    let positionsTotalValue = $state($positionsTotal);
    let incidentLimitValue = $state($incidentLimit);

    function onRPM(rpm: Rpm) {
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

    function update() {
        gearValue = $gear;
        rpmValue = $rpm;
        speedValue = $speed;
        positionValue = $position;
        incidentsValue = $incidents;
        lapValue = $lap;
        lapsTotalValue = $lapsTotal;
        positionsTotalValue = $positionsTotal;
        incidentLimitValue = $incidentLimit;
        onRPM(rpmValue);
        animationFrameId = requestAnimationFrame(update);
    }

    onMount(() => {
        animationFrameId = requestAnimationFrame(update);
    });

    onDestroy(() => {
        if (animationFrameId) {
            cancelAnimationFrame(animationFrameId);
        }
    });
</script>

<div class="join bg-primary-content w-full h-full">
    <div
        class="join-item flex flex-col items-center justify-center rounded-md w-1/4"
    >
        <div
            bind:this={gear_indicator}
            class="text-secondary text-5xl font-square"
        >
            {gearValue}
        </div>
        <div class="text-primary text-2xl -mt-1">{speedValue}</div>
    </div>
    <div
        class="join-item flex flex-col items-center justify-evenly rounded-md w-3/4"
    >
        <progress
            bind:this={rpm_indicator}
            class="progress progress-secondary w-5/6 outline outline-2 outline-offset-2 outline-primary"
            value={rpmValue}
            max={$gearBlinkRPM}
        ></progress>
        <div class="flex flex-row items-center justify-evenly">
            <div class="flex flex-col items-center justify-evenly">
                <div class="text-primary">
                    <NumberFlow value={lapValue} plugins={[continuous]}
                    ></NumberFlow>{#if lapsTotalValue > 0}/{lapsTotalValue}{/if}
                </div>
                <div class="text-secondary text-xs">lap</div>
            </div>
            <div class="divider divider-horizontal divider-primary"></div>
            <div class="flex flex-col items-center justify-evenly">
                <div class="text-primary">
                    {#if $position > 0}
                        <NumberFlow value={positionValue} plugins={[continuous]}
                        ></NumberFlow>{#if positionsTotalValue > 0}/{positionsTotalValue}{/if}
                    {:else}
                        -
                    {/if}
                </div>
                <div class="text-secondary text-xs">pos</div>
            </div>
            <div class="divider divider-horizontal divider-primary"></div>
            <div class="flex flex-col items-center justify-evenly">
                <div class="text-primary">
                    <NumberFlow value={incidentsValue} plugins={[continuous]}
                    ></NumberFlow>{#if incidentLimitValue > 0}/{incidentLimitValue}{/if}x
                </div>
                <div class="text-secondary text-xs">inc</div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
