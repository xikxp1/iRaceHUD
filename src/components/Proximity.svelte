<script lang="ts">
    import type { Proximity } from "$lib/types/telemetry";
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy } from "svelte";

    let left_icon: HTMLImageElement;
    let right_icon: HTMLImageElement;

    let unlistens = [];

    unlistens.push(
        listen("proximity", (event) => {
            let payload = event.payload as Proximity;
            left_icon.style.opacity = payload.is_left ? "1" : "0";
            right_icon.style.opacity = payload.is_right ? "1" : "0";
        }),
    );

    onDestroy(() => {
        unlistens.forEach(async (unlisten) => (await unlisten)());
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
        <div class="flex w-[650px]" />
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
