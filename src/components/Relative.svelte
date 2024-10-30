<script lang="ts">
    import type { Relative } from "$lib/types/telemetry";
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";
    import Badge from "./utils/Badge.svelte";
    import { getBadgeColor } from "$lib/utils";

    let relative: Relative = [];

    let channel = new Channel<Relative>();

    onMount(() => {
        channel.onmessage = (message) => {
            relative = message as Relative;
        };

        invoke("register_event_emitter", {
            event: "relative",
            onEvent: channel,
        });
    });

    onDestroy(() => {
        channel.onmessage = () => {};

        invoke("unregister_event_emitter", {
            event: "relative",
        });
    });
</script>

<div class="flex flex-row items-center justify-center opacity-90">
    <table class="bg-secondary-content rounded-md w-[410px]">
        {#each relative as rel}
            <tr
                class="{rel?.is_player
                    ? 'bg-secondary text-primary-content'
                    : 'odd:bg-secondary-content even:bg-primary-content text-primary'} h-[22px]"
            >
                <td class="text text-sm text-right pr-2 w-[25px]">
                    {#if rel?.position == 0}
                        *
                    {:else}
                        {rel?.position}
                    {/if}
                </td>
                <td class="text text-sm text-right pr-2 w-[48px]">
                    <Badge
                        outlineClasses="text-center ring ring-2 ring-inset {rel?.is_player
                            ? 'ring-primary-content'
                            : 'ring-primary'}"
                        textClasses="text-sm text-right {rel?.is_player ? 'text-primary-content' : 'text-primary'}"
                        text={rel?.car_number ? "#" + rel?.car_number : ""}
                    />
                </td><td class="text text-sm">
                    <span class="text text-sm">{rel?.user_name ?? ""}</span>
                    {#if rel?.is_off_track}
                        <span class="text text-sm text-error text-right"
                            >&nbspOFF</span
                        >
                    {/if}
                    {#if rel?.is_in_pits}
                        <span class="text text-sm text-success text-right"
                            >&nbspPIT</span
                        >
                    {/if}
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
    </table>
</div>

<style>
    table tr {
        clip-path: xywh(0 0 100% 100% round 0.375em);
    }
</style>
