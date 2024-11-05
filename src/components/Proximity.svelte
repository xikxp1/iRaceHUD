<script lang="ts">
    import { proximity } from "$lib/telemetry/telemetry.svelte";
    import type { Proximity } from "$lib/types/telemetry";
    import { onMount } from "svelte";

    let left_icon: HTMLImageElement;
    let right_icon: HTMLImageElement;

    function on_proximity(value: Proximity) {
        if (left_icon != null) {
            left_icon.style.opacity = value.is_left ? "1" : "0";
        }
        if (right_icon != null) {
            right_icon.style.opacity = value.is_right ? "1" : "0";
        }
    }

    onMount(() => {
        proximity.subscribe((value) => on_proximity(value));

        on_proximity($proximity);
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
