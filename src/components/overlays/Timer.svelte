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

<div class="join flex flex-row bg-primary-content rounded-md w-full h-fit">
    {#if settings.delta_enabled}
        <div
            class="join-item flex flex-col items-end justify-center border-r-primary border-r-2"
            style="width: {settings.delta_width}px"
        >
            <span class="text-primary text-xl font-square mr-3">
                {dBestTime}
            </span>
        </div>
    {/if}
    <div class="join-item flex flex-col flex-grow items-center justify-center">
        <span class="text-primary text-3xl font-square ml-1 mr-1">
            {lapTimeFormatted}
        </span>
    </div>
    {#if settings.delta_enabled}
        <div
            class="join-item flex flex-col items-start justify-center border-l-primary border-l-2"
            style="width: {settings.delta_width}px"
        >
            <span class="text-primary text-xl font-square ml-3">
                {dLastTime}
            </span>
        </div>
    {/if}
</div>

<style>
</style>
