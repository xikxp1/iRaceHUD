<script lang="ts">
    import { proximity } from "$lib/telemetry/telemetry.svelte";
    import { onMount } from "svelte";

    let left_icon: HTMLImageElement;
    let right_icon: HTMLImageElement;

    onMount(() => {
        proximity.subscribe((value) => {
            left_icon.style.opacity = value.is_left ? "1" : "0";
            right_icon.style.opacity = value.is_right ? "1" : "0";
        });
    });
</script>

<div class="flex flex-row items-center justify-center">
    <div class="flex flex-row">
        <img
            bind:this={left_icon}
            src="/icons/alert.svg"
            alt=""
            style="opacity: 0;"
        />
        <div class="flex w-[650px]"></div>
        <img
            bind:this={right_icon}
            src="/icons/alert.svg"
            alt=""
            style="opacity: 0;"
        />
    </div>
</div>

<style>
</style>
