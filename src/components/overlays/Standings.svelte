<script lang="ts">
    import type {
        StandingsDriver,
        StandingsOverlaySettings,
    } from "$lib/types/telemetry";
    import { onDestroy, onMount } from "svelte";
    import { flip } from "svelte/animate";
    import Badge from "../utils/Badge.svelte";
    import { getBadgeColor } from "$lib/utils";
    import {
        positionsTotal,
        raceLaps,
        standings,
        strengthOfField,
        playerCarClass,
        sessionType,
    } from "$lib/backend/telemetry.svelte";

    let { settings }: { settings: StandingsOverlaySettings } = $props();

    let sessionTypeText = $derived($sessionType.at(0));

    type LocalStandings = StandingsDriver & {
        position_change: number;
    };

    let current_standings: LocalStandings[] = $state([]);

    // TODO: Make this configurable
    const SWITCH_INTERVAL = 10000;
    let show_best_lap = $state(false);
    let interval: NodeJS.Timeout;

    const css = window.getComputedStyle(document.documentElement);
    const splitBorderColor: string = `oklch(${css.getPropertyValue("--a")})`;

    function on_standings(value: StandingsDriver[]) {
        let new_standings: LocalStandings[] = [];
        value.forEach((st) => {
            let new_value: LocalStandings = {
                ...st,
                position_change: 0,
            };
            let old_st = current_standings.find(
                (old_st) => old_st.car_id === new_value.car_id,
            );
            new_value.position_change =
                new_value.position - (old_st?.position ?? new_value.position);
            new_standings.push(new_value);
        });
        current_standings = new_standings;
    }

    let unsubscribe_standings: () => void = () => {};
    let unsubscribe_session_type: () => void = () => {};

    onMount(() => {
        unsubscribe_standings = standings.subscribe((value) =>
            on_standings(value),
        );

        unsubscribe_session_type = sessionType.subscribe((value) => {
            if (value != "Qualify") {
                interval = setInterval(() => {
                    show_best_lap = !show_best_lap;
                }, SWITCH_INTERVAL);
            } else {
                show_best_lap = true;
            }
        });

        on_standings($standings);
    });

    onDestroy(() => {
        clearInterval(interval);
        unsubscribe_standings();
        unsubscribe_session_type();
    });
</script>

<div class="flex flex-row w-full h-fit">
    <div class="flex flex-col bg-primary-content rounded-l-md w-full">
        <div
            class="flex flex-row items-center justify-center border-b-2 border-accent"
        >
            <div class="flex flex-row items-center justify-start w-5/6 pl-2">
                <span class="text text-primary font-square"
                    >{sessionTypeText}</span
                >
                <span class="text text-primary text-opacity-70"
                    >&nbsp;|&nbsp;</span
                >
                <span class="text text-primary">{$playerCarClass}</span>
                <span class="text text-primary text-opacity-70"
                    >&nbsp;|&nbsp;</span
                >
                <span class="text text-primary text-opacity-70">SoF:&nbsp;</span
                >
                <span class="text text-primary">{$strengthOfField}</span>
                <span class="text text-primary text-opacity-70"
                    >&nbsp;|&nbsp;</span
                >
                <span class="text text-primary text-opacity-70">Lap:&nbsp;</span
                >
                <span class="text text-primary">{$raceLaps}</span>
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
                        class="{st?.is_player
                            ? 'text-secondary'
                            : 'text-primary'} odd:bg-secondary-content even:bg-primary-content h-[22px] {st?.is_off_world
                            ? 'text-opacity-70'
                            : ''} {st?.split_after
                            ? 'border-b-2 border-accent'
                            : ''}"
                        animate:flip
                    >
                        <td
                            class="transition-colors duration-700 text text-sm text-right pr-2 w-[30px] {st?.position_change >
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
                                    ? 'ring-secondary'
                                    : 'ring-primary'}"
                                textClasses="text-sm {st?.is_player
                                    ? 'text-secondary'
                                    : 'text-primary'}"
                                text={st?.car_number
                                    ? "#" + st?.car_number
                                    : ""}
                            />
                        </td>
                        <td class="text text-sm">
                            <div
                                class="flex flex-row items-center justify-between"
                            >
                                <span class="text text-sm tracking-tight"
                                    >{st?.user_name ?? ""}</span
                                >
                                <div class="flex flex-row gap-1 ml-2 mr-1">
                                    {#if st?.is_off_track}
                                        <Badge
                                            outlineClasses="ring ring-2 ring-inset ring-warning text-center"
                                            textClasses="text text-xs tracking-tight text-warning ml-1 mr-1"
                                            text="OFF"
                                        />
                                    {/if}
                                    {#if st?.is_in_pits}
                                        <Badge
                                            outlineClasses="ring ring-2 ring-inset ring-primary text-center"
                                            textClasses="text text-xs text-primary ml-1 mr-1"
                                            text="PIT"
                                        />
                                    {/if}
                                </div>
                            </div>
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
                        {#if $sessionType === "Race"}
                            <td class="text text-sm text-right pr-1 w-[35px]">
                                {st?.leader_gap ?? ""}
                            </td>
                        {/if}
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
    <div class="flex flex-col bg-primary-content rounded-r-md w-[75px]">
        <div
            class="flex flex-row items-center justify-end border-b-2 border-accent"
        >
            <span class="text text-primary text-right pr-1">
                {show_best_lap ? "BEST" : "LAST"}
            </span>
        </div>
        <table class="bg-secondary-content rounded-r-md">
            <tbody>
                {#each current_standings as st, index (st.car_id)}
                    <tr
                        class="{st?.is_player
                            ? 'text-secondary'
                            : 'text-primary'} odd:bg-secondary-content even:bg-primary-content h-[22px]"
                        style={st?.split_after
                            ? `border-bottom: solid 2px ${splitBorderColor};`
                            : ""}
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
