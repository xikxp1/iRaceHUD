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

<div
    class="flex flex-row items-center justify-center"
    style="opacity: {settings.opacity / 100}"
>
    <div class="join flex flex-row bg-primary-content rounded-md">
        {#if settings.gap_enabled && $sessionType === "Race"}
            <div
                class="join-item flex flex-col items-end justify-center"
                style="width: {settings.gap_width}px"
            >
                <div class="text-primary text-xl">{gapNextValue}</div>
            </div>
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
        {/if}
        <div
            class="join-item flex flex-col items-center justify-center"
            style="width: {settings.session_state_width}px"
        >
            <div class="text-primary text-xl">{sessionStateValue}</div>
        </div>
        {#if settings.gap_enabled && $sessionType === "Race"}
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
            <div
                class="join-item flex flex-col items-start justify-center"
                style="width: {settings.gap_width}px"
            >
                <div class="text-primary text-xl">{gapPrevValue}</div>
            </div>
        {/if}
    </div>
</div>

<style>
</style>
