<script lang="ts">
    import {
        gapNext,
        gapPrev,
        sessionState,
        sessionType,
    } from "$lib/backend/telemetry.svelte";
    import type { SubTimerOverlaySettings } from "$lib/types/telemetry";

    let { settings }: { settings: SubTimerOverlaySettings } = $props();
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
                <div class="text-primary text-xl">{$gapNext}</div>
            </div>
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
        {/if}
        <div
            class="join-item flex flex-col items-center justify-center"
            style="width: {settings.session_state_width}px"
        >
            <div class="text-primary text-xl">{$sessionState}</div>
        </div>
        {#if settings.gap_enabled && $sessionType === "Race"}
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
            <div
                class="join-item flex flex-col items-start justify-center"
                style="width: {settings.gap_width}px"
            >
                <div class="text-primary text-xl">{$gapPrev}</div>
            </div>
        {/if}
    </div>
</div>

<style>
</style>
