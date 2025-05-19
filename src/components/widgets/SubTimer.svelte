<script lang="ts">
    import {
        gapNext,
        gapPrev,
        sessionState,
    } from "$lib/telemetry/telemetry.svelte";
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="join flex flex-row bg-primary-content rounded-md">
        <div class="join-item flex flex-col items-end justify-center w-[50px]">
            <div class="text-primary text-xl">{$gapNext}</div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-center justify-center w-[100px]"
        >
            <div class="text-primary text-xl">{$sessionState}</div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-start justify-center w-[50px]"
        >
            <div class="text-primary text-xl">{$gapPrev}</div>
        </div>
    </div>
</div>

<style>
</style>
