<script lang="ts">
    import {
        gapNext,
        gapPrev,
        sessionState,
        sessionType,
    } from "$lib/backend/telemetry.svelte";
    import type { SubTimerOverlaySettings } from "$lib/types/telemetry";
    import { onMount, onDestroy } from "svelte";

    let { settings }: { settings: SubTimerOverlaySettings } = $props();
    let animationFrameId: number;
    let gapNextValue = $state($gapNext);
    let gapPrevValue = $state($gapPrev);
    let sessionStateValue = $state($sessionState);

    function update() {
        gapNextValue = $gapNext;
        gapPrevValue = $gapPrev;
        sessionStateValue = $sessionState;
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

<div class="join flex flex-row bg-primary-content rounded-md w-full h-fit">
    {#if settings.gap_enabled && $sessionType === "Race"}
        <div
            class="join-item flex flex-col items-end justify-center border-r-primary border-r-2"
            style="width: {settings.gap_width}px"
        >
            <span class="text-primary text-xl mr-2">{gapNextValue}</span>
        </div>
    {/if}
    <div class="join-item flex flex-col flex-grow items-center justify-center">
        <span class="text-primary text-xl ml-1 mr-1">{sessionStateValue}</span>
    </div>
    {#if settings.gap_enabled && $sessionType === "Race"}
        <div
            class="join-item flex flex-col items-start justify-center border-l-primary border-l-2"
            style="width: {settings.gap_width}px"
        >
            <span class="text-primary text-xl ml-2">{gapPrevValue}</span>
        </div>
    {/if}
</div>

<style>
</style>
