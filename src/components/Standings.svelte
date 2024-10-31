<script lang="ts">
    import type { Standings } from "$lib/types/telemetry";
    import { onDestroy, onMount } from "svelte";
    import { flip } from "svelte/animate";
    import Badge from "./utils/Badge.svelte";
    import { getBadgeColor } from "$lib/utils";
    import {
        currentTime,
        positionsTotal,
        raceLaps,
        standings,
        strengthOfField,
    } from "$lib/telemetry/telemetry.svelte";

    let current_standings: Standings = [];

    const SWITCH_INTERVAL = 10000;
    let show_best_lap = false;
    let interval: NodeJS.Timeout;

    onMount(() => {
        interval = setInterval(() => {
            show_best_lap = !show_best_lap;
        }, SWITCH_INTERVAL);

        standings.subscribe((value) => {
            let new_standings = value;
            new_standings.forEach((st) => {
                let old_st = current_standings.find(
                    (old_st) => old_st.car_id === st.car_id,
                );
                st.position_change =
                    st.position - (old_st?.position ?? st.position);
            });
            current_standings = new_standings;
        });
    });

    onDestroy(() => {
        clearInterval(interval);
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="flex flex-col bg-primary-content rounded-l-md w-[375px]">
        <div class="flex flex-row items-center justify-center">
            <div class="flex flex-row items-center justify-start w-1/6 pl-2">
                <span class="text text-secondary">SoF&nbsp</span>
                <span class="text text-primary">{$strengthOfField}</span>
            </div>
            <div class="flex flex-row items-center justify-center w-4/6">
                <span class="text text-primary">{$currentTime}</span>
            </div>
            <div class="flex flex-row items-center justify-end w-1/6 pr-2">
                <span class="text text-primary">{$positionsTotal}&nbsp;</span>
                <img src="/icons/helmet.svg" alt="" />
            </div>
        </div>
        <table class="bg-secondary-content rounded-l-md">
            <tbody>
                {#each current_standings as st, index (st.car_id)}
                    <tr
                        class={st?.is_player
                            ? "bg-secondary text-primary-content"
                            : "odd:bg-secondary-content even:bg-primary-content text-primary"}
                        animate:flip
                    >
                        <td
                            class="transition-colors duration-700 text text-sm text-right pr-2 w-[25px] {st?.position_change >
                            0
                                ? 'bg-error'
                                : st?.position_change < 0
                                  ? 'bg-success'
                                  : ''}"
                        >
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
                                text={st?.car_number
                                    ? "#" + st?.car_number
                                    : ""}
                            />
                        </td>
                        <td class="text text-sm">
                            <span class="text text-sm"
                                >{st?.user_name ?? ""}</span
                            >
                            {#if st?.is_in_pits}
                                <span
                                    class="text text-sm text-success text-right"
                                    >&nbspPIT</span
                                >
                            {/if}
                        </td>
                        <td class="text text-sm text-right pr-1 w-[40px]">
                            {#if st?.license && st?.irating}
                                <Badge
                                    colorClasses={getBadgeColor(
                                        st?.license ?? "",
                                    )}
                                    outlineClasses="text-center"
                                    textClasses="text text-sm text-right text-primary"
                                    text={st?.irating}
                                />
                            {/if}
                        </td>
                        <td class="text text-sm text-right pr-1 w-[40px]">
                            {#if st?.is_leader && $raceLaps > 0}
                                L{$raceLaps}
                            {:else}
                                {st?.leader_gap ?? ""}
                            {/if}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
    <div class="flex flex-col bg-primary-content rounded-r-md w-[100px]">
        <div class="flex flex-row items-center justify-end">
            <span class="text text-primary text-right pr-1">
                {show_best_lap ? "BEST" : "LAST"}
            </span>
        </div>
        <table class="bg-secondary-content rounded-r-md">
            <tbody>
                {#each current_standings as st, index (st.car_id)}
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
            </tbody>
        </table>
    </div>
</div>

<style>
</style>
