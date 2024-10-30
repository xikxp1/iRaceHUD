<script lang="ts">
    import type {
        CurrentTime,
        Position,
        Standings,
        StrengthOfField,
    } from "$lib/types/telemetry";
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";
    import { flip } from "svelte/animate";
    import Badge from "./utils/Badge.svelte";
    import { getBadgeColor } from "$lib/utils";

    let stength_of_field = 0;
    let current_time = "––:––";
    let driver_count = 0;

    let standings: Standings = [];

    const SWITCH_INTERVAL = 10000;
    let show_best_lap = false;
    let interval: NodeJS.Timeout;

    let standings_channel = new Channel<Standings>();
    let current_time_channel = new Channel<CurrentTime>();
    let stength_of_field_channel = new Channel<StrengthOfField>();
    let positions_total_channel = new Channel<Position>();

    onMount(() => {
        interval = setInterval(() => {
            show_best_lap = !show_best_lap;
        }, SWITCH_INTERVAL);

        standings_channel.onmessage = (message) => {
            standings = message;
        };

        current_time_channel.onmessage = (message) => {
            current_time = message;
        };

        stength_of_field_channel.onmessage = (message) => {
            stength_of_field = message;
        };

        positions_total_channel.onmessage = (message) => {
            driver_count = message;
        };

        invoke("register_event_emitter", {
            event: "standings",
            onEvent: standings_channel,
        });

        invoke("register_event_emitter", {
            event: "current_time",
            onEvent: current_time_channel,
        });

        invoke("register_event_emitter", {
            event: "strength_of_field",
            onEvent: stength_of_field_channel,
        });

        invoke("register_event_emitter", {
            event: "positions_total",
            onEvent: positions_total_channel,
        });
    });

    onDestroy(() => {
        clearInterval(interval);
        standings_channel.onmessage = () => {};
        current_time_channel.onmessage = () => {};
        stength_of_field_channel.onmessage = () => {};
        positions_total_channel.onmessage = () => {};

        invoke("unregister_event_emitter", {
            event: "standings",
        });

        invoke("unregister_event_emitter", {
            event: "current_time",
        });

        invoke("unregister_event_emitter", {
            event: "strength_of_field",
        });

        invoke("unregister_event_emitter", {
            event: "positions_total",
        });
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="flex flex-col bg-primary-content rounded-l-md w-[375px]">
        <div class="flex flex-row items-center justify-center">
            <div class="flex flex-row items-center justify-start w-1/6 pl-2">
                <span class="text text-secondary">SoF&nbsp</span>
                <span class="text text-primary">{stength_of_field}</span>
            </div>
            <div class="flex flex-row items-center justify-center w-4/6">
                <span class="text text-primary">{current_time}</span>
            </div>
            <div class="flex flex-row items-center justify-end w-1/6 pr-2">
                <span class="text text-primary">{driver_count}&nbsp;</span>
                <img src="/icons/helmet.svg" alt="" />
            </div>
        </div>
        <table class="bg-secondary-content rounded-l-md">
            {#each standings as st, index (st.car_id)}
                <tr
                    class={st?.is_player
                        ? "bg-secondary text-primary-content"
                        : "odd:bg-secondary-content even:bg-primary-content text-primary"}
                    animate:flip
                >
                    <td class="text text-sm text-right pr-2 w-[25px]">
                        {st?.position ?? ""}
                    </td>
                    <td class="text text-sm pr-2 w-[48px]">
                        <Badge
                            outlineClasses="text-center ring ring-2 ring-inset {st?.is_player
                                ? 'ring-primary-content'
                                : 'ring-primary'}"
                            textClasses="text-sm {st?.is_player
                                ? 'text-primary-content'
                                : 'text-primary'}"
                            text={st?.car_number ? "#" + st?.car_number : ""}
                        />
                    </td>
                    <td class="text text-sm">
                        <span class="text text-sm">{st?.user_name ?? ""}</span>
                        {#if st?.is_in_pits}
                            <span class="text text-sm text-success text-right"
                                >&nbspPIT</span
                            >
                        {/if}
                    </td>
                    <td class="text text-sm text-right pr-1 w-[40px]">
                        {#if st?.license && st?.irating}
                            <Badge
                                colorClasses={getBadgeColor(st?.license ?? "")}
                                outlineClasses="text-center"
                                textClasses="text text-sm text-right text-primary"
                                text={st?.irating}
                            />
                        {/if}
                    </td>
                    <td class="text text-sm text-right pr-1 w-[40px]">
                        {st?.leader_gap ?? ""}
                    </td>
                </tr>
            {/each}
        </table>
    </div>
    <div class="flex flex-col bg-primary-content rounded-r-md w-[100px]">
        <div class="flex flex-row items-center justify-end">
            <span class="text text-primary text-right pr-1">
                {show_best_lap ? "BEST" : "LAST"}
            </span>
        </div>
        <table class="bg-secondary-content rounded-r-md">
            {#each standings as st, index (st.car_id)}
                <tr
                    class="{st?.is_player
                        ? 'bg-secondary text-primary-content'
                        : 'odd:bg-secondary-content even:bg-primary-content text-primary'} h-[22px]"
                    animate:flip
                >
                    <td class="text text-sm text-right pr-1">
                        {show_best_lap
                            ? (st?.best_lap ?? "")
                            : (st?.last_lap ?? "")}
                    </td>
                </tr>
            {/each}
        </table>
    </div>
</div>

<style>
</style>
