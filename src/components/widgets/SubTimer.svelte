<script lang="ts">
    import {
        gapNext,
        gapPrev,
        sessionState,
    } from "$lib/telemetry/telemetry.svelte";
    import { subtimerWidgetSettings } from "$lib/settings/settings.svelte";
</script>

<div
    class="flex flex-row items-center justify-center"
    style="opacity: {$subtimerWidgetSettings?.opacity / 100}"
>
    <div class="join flex flex-row bg-primary-content rounded-md">
        {#if $subtimerWidgetSettings?.gap_enabled}
            <div
                class="join-item flex flex-col items-end justify-center"
                style="width: {$subtimerWidgetSettings?.gap_width}px"
            >
                <div class="text-primary text-xl">{$gapNext}</div>
            </div>
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
        {/if}
        <div
            class="join-item flex flex-col items-center justify-center"
            style="width: {$subtimerWidgetSettings?.session_state_width}px"
        >
            <div class="text-primary text-xl">{$sessionState}</div>
        </div>
        {#if $subtimerWidgetSettings?.gap_enabled}
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
            <div
                class="join-item flex flex-col items-start justify-center"
                style="width: {$subtimerWidgetSettings?.gap_width}px"
            >
                <div class="text-primary text-xl">{$gapPrev}</div>
            </div>
        {/if}
    </div>
</div>

<style>
</style>
