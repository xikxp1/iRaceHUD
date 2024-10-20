<script lang="ts">
    import type { Proximity } from "$lib/types/telemetry";
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";

    let left_icon: HTMLImageElement;
    let right_icon: HTMLImageElement;

    let channel = new Channel<Proximity>();

    onMount(() => {
        channel.onmessage = (message) => {
            let payload = message as Proximity;
            left_icon.style.opacity = payload.is_left ? "1" : "0";
            right_icon.style.opacity = payload.is_right ? "1" : "0";
        };

        invoke("register_event_emitter", {
            event: "proximity",
            onEvent: channel,
        });
    });

    onDestroy(() => {
        channel.onmessage = () => {};

        invoke("unregister_event_emitter", {
            event: "proximity",
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
