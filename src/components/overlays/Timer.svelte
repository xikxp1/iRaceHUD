<script lang="ts">
    import {
        deltaBestTime,
        deltaLastTime,
        lapTime,
    } from "$lib/backend/telemetry.svelte";
    import { Duration } from "luxon";
    import type { TimerOverlaySettings } from "$lib/types/telemetry";
    import { onMount, onDestroy } from "svelte";

    let { settings }: { settings: TimerOverlaySettings } = $props();
    let animationFrameId: number;
    let lTime = $state($lapTime);
    let dLastTime = $state($deltaLastTime);
    let dBestTime = $state($deltaBestTime);

    function update() {
        lTime = $lapTime;
        dLastTime = $deltaLastTime;
        dBestTime = $deltaBestTime;
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

    let lapTimeFormatted = $derived(
        Duration.fromObject({ seconds: lTime }).toFormat("m:ss.SSS"),
    );
</script>

<div
    class="flex flex-row items-center justify-center"
    style="opacity: {settings.opacity / 100}"
>
    <div class="join flex flex-row bg-primary-content rounded-md">
        {#if settings.delta_enabled}
            <div
                class="join-item flex flex-col items-end justify-center"
                style="width: {settings.delta_width}px"
            >
                <div class="text-primary text-xl font-square">
                    {dBestTime}
                </div>
            </div>
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
        {/if}
        <div
            class="join-item flex flex-col items-center justify-center"
            style="width: {settings.lap_time_width}px"
        >
            <div class="text-primary text-3xl font-square">
                {lapTimeFormatted}
            </div>
        </div>
        {#if settings.delta_enabled}
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
            <div
                class="join-item flex flex-col items-start justify-center"
                style="width: {settings.delta_width}px"
            >
                <div class="text-primary text-xl font-square">
                    {dLastTime}
                </div>
            </div>
        {/if}
    </div>
</div>

<style>
</style>
