<script lang="ts">
    import {
        deltaLastTime,
        deltaOptimalTime,
        lapTime,
    } from "$lib/telemetry/telemetry.svelte";
    import { Duration } from "luxon";
    import { timerWidgetSettings } from "$lib/settings/settings.svelte";

    let lapTimeFormatted = $derived(
        Duration.fromObject({ seconds: $lapTime }).toFormat("m:ss.SSS"),
    );
</script>

<div
    class="flex flex-row items-center justify-center"
    style="opacity: {$timerWidgetSettings?.opacity / 100}"
>
    <div class="join flex flex-row bg-primary-content rounded-md">
        {#if $timerWidgetSettings?.delta_enabled}
            <div
                class="join-item flex flex-col items-end justify-center"
                style="width: {$timerWidgetSettings?.delta_width}px"
            >
                <div class="text-primary text-xl font-square">
                    {$deltaOptimalTime}
                </div>
            </div>
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
        {/if}
        <div
            class="join-item flex flex-col items-center justify-center"
            style="width: {$timerWidgetSettings?.lap_time_width}px"
        >
            <div class="text-primary text-3xl font-square">
                {lapTimeFormatted}
            </div>
        </div>
        {#if $timerWidgetSettings?.delta_enabled}
            <div
                class="join-item divider divider-horizontal divider-primary w-[2px]"
            ></div>
            <div
                class="join-item flex flex-col items-start justify-center"
                style="width: {$timerWidgetSettings?.delta_width}px"
            >
                <div class="text-primary text-xl font-square">
                    {$deltaLastTime}
                </div>
            </div>
        {/if}
    </div>
</div>

<style>
</style>
