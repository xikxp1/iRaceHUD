<script lang="ts">
    import type { LapTimes } from "$lib/types/telemetry";
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";

    let player_lap_times: LapTimes = [];

    let channel = new Channel<LapTimes>();

    onMount(() => {
        channel.onmessage = (message) => {
            player_lap_times = message;
        };

        invoke("register_event_emitter", {
            event: "player_lap_times",
            onEvent: channel,
        });
    });

    onDestroy(() => {
        channel.onmessage = () => {};

        invoke("unregister_event_emitter", {
            event: "player_lap_times",
        });
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <table class="bg-secondary-content rounded-md w-[110px]">
        {#each player_lap_times as { lap, lap_time }}
            <tr
                class="divide-x-2 divide-secondary even:bg-primary-content odd:bg-secondary-content"
            >
                <td class="text text-primary text-sm text-right pr-2 w-[30%]">
                    {lap}
                </td>
                <td class="text text-primary text-sm text-right pr-1 w-[70%]">
                    {lap_time}
                </td>
            </tr>
        {/each}
    </table>
</div>

<style>
    table tr {
        clip-path: xywh(0 0 100% 100% round 0.375em);
    }
</style>
