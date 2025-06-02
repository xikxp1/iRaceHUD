<script lang="ts">
    import Badge from "../utils/Badge.svelte";
    import { getBadgeColor, getCarClassColors } from "$lib/utils";
    import {
        relative,
        fastestLap,
        currentTime,
    } from "$lib/backend/telemetry.svelte";
    import type { RelativeOverlaySettings } from "$lib/types/telemetry";

    let { settings }: { settings: RelativeOverlaySettings } = $props();

    const css = window.getComputedStyle(document.documentElement);
    const carClassColors = getCarClassColors(css);

    function getCarClassColor(carClassColor: number): string {
        const color = `#${carClassColor.toString(16)}`;
        const colorString = carClassColors[color];
        if (colorString) {
            return colorString;
        }
        return carClassColors["#ffffff"];
    }
</script>

<div class="flex flex-col bg-primary-content rounded-md w-full h-fit">
    <div class="flex flex-row h-[22px] border-b-2 border-accent">
        <div class="flex flex-row items-center justify-start w-3/4 pl-2">
            <span class="text-sm text-primary text-opacity-70">Best Lap: </span>
            <span class="text text-sm text-primary">&nbsp;{$fastestLap}</span>
        </div>
        <div class="flex flex-row items-center justify-end w-1/4 pr-2">
            <img src="/icons/clock.svg" alt="" class="h-[16px]" />
            <span class="text text-primary">&nbsp;{$currentTime}</span>
        </div>
    </div>
    <table>
        <tbody>
            {#each $relative as rel}
                <tr
                    class="{rel?.is_player
                        ? 'text-secondary'
                        : 'text-primary'} odd:bg-secondary-content even:bg-primary-content h-[22px]"
                >
                    <td class="text text-sm text-right pr-2 w-[30px]">
                        {#if rel?.position == 0}
                            *
                        {:else}
                            {rel?.position}
                        {/if}
                    </td>
                    <td class="text text-sm text-right pr-2 w-[48px]">
                        <Badge
                            outlineClasses="text-center ring ring-2 ring-inset ring-current"
                            textClasses="text-sm text-right"
                            extraStyles={`color: ${getCarClassColor(rel?.car_class_color ?? 0)}`}
                            text={rel?.car_number ? "#" + rel?.car_number : ""}
                        />
                    </td>
                    <td class="text text-sm">
                        <div class="flex flex-row items-center justify-between">
                            <span
                                class="text text-sm tracking-tight truncate {rel?.is_player_car_class
                                    ? ''
                                    : 'text-opacity-70'} {rel?.is_ahead
                                    ? 'text-warning'
                                    : ''} {rel?.is_behind ? 'text-info' : ''}"
                                >{rel?.user_name ?? ""}
                            </span>
                            <div class="flex flex-row gap-1 ml-2 mr-1">
                                {#if rel?.is_off_track}
                                    <Badge
                                        outlineClasses="ring ring-2 ring-inset ring-warning text-center"
                                        textClasses="text text-xs tracking-tight text-warning ml-1 mr-1"
                                        text="OFF"
                                    />
                                {/if}
                                {#if rel?.is_in_pits}
                                    <Badge
                                        outlineClasses="ring ring-2 ring-inset ring-primary text-center"
                                        textClasses="text text-xs text-primary ml-1 mr-1"
                                        text="PIT"
                                    />
                                {/if}
                            </div>
                        </div>
                    </td>
                    <td class="text text-sm text-right pr-1 w-[80px]">
                        {#if rel?.license && rel?.irating}
                            <Badge
                                colorClasses={getBadgeColor(rel?.license ?? "")}
                                outlineClasses="text-center"
                                textClasses="text text-sm text-right text-primary"
                                text="{(rel?.license).substring(
                                    0,
                                    5,
                                )}&nbsp|&nbsp{rel?.irating}"
                            />
                        {/if}
                    </td>
                    <td class="text text-sm text-right pr-1 w-[40px]"
                        >{rel?.player_relative_gap ?? ""}
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

<style>
</style>
