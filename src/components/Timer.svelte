<script lang="ts">
    import type { DeltaTime, LapTime } from "$lib/types/telemetry";
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { onMount, onDestroy } from "svelte";

    let lap_time = "–:––.–––";
    let delta_last_time = "–";
    let delta_optimal_time = "–";

    let lap_time_channel = new Channel<LapTime>();
    let delta_last_time_channel = new Channel<DeltaTime>();
    let delta_optimal_time_channel = new Channel<DeltaTime>();

    onMount(() => {
        lap_time_channel.onmessage = (message) => {
            lap_time = message;
        };

        delta_last_time_channel.onmessage = (message) => {
            delta_last_time = message;
        };

        delta_optimal_time_channel.onmessage = (message) => {
            delta_optimal_time = message;
        };

        invoke("register_event_emitter", {
            event: "lap_time",
            onEvent: lap_time_channel,
        });

        invoke("register_event_emitter", {
            event: "delta_last_time",
            onEvent: delta_last_time_channel,
        });

        invoke("register_event_emitter", {
            event: "delta_optimal_time",
            onEvent: delta_optimal_time_channel,
        });
    });

    onDestroy(() => {
        lap_time_channel.onmessage = () => {};
        delta_last_time_channel.onmessage = () => {};
        delta_optimal_time_channel.onmessage = () => {};

        invoke("unregister_event_emitter", {
            event: "lap_time",
        });

        invoke("unregister_event_emitter", {
            event: "delta_last_time",
        });

        invoke("unregister_event_emitter", {
            event: "delta_optimal_time",
        });
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="join flex flex-row bg-primary-content rounded-md">
        <div class="join-item flex flex-col items-end justify-center w-[75px]">
            <div class="text-primary text-xl font-square">
                {delta_optimal_time}
            </div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-center justify-center w-[160px]"
        >
            <div class="text-primary text-3xl font-square">{lap_time}</div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-start justify-center w-[75px]"
        >
            <div class="text-primary text-xl font-square">
                {delta_last_time}
            </div>
        </div>
    </div>
</div>

<style>
</style>
