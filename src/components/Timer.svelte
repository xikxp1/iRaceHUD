<script lang="ts">
    import {
        deltaLastTime,
        deltaOptimalTime,
        lapTime,
    } from "$lib/telemetry/telemetry.svelte";
    import { Duration } from "luxon";

    let lapTimeFormatted = $derived(
        Duration.fromObject({ seconds: $lapTime }).toFormat("m:ss.SSS"),
    );
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="join flex flex-row bg-primary-content rounded-md">
        <div class="join-item flex flex-col items-end justify-center w-[75px]">
            <div class="text-primary text-xl font-square">
                {$deltaOptimalTime}
            </div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-center justify-center w-[160px]"
        >
            <div class="text-primary text-3xl font-square">
                {lapTimeFormatted}
            </div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-start justify-center w-[75px]"
        >
            <div class="text-primary text-xl font-square">
                {$deltaLastTime}
            </div>
        </div>
    </div>
</div>

<style>
</style>
