<script lang="ts">
    import Badge from "../utils/Badge.svelte";
    import { getBadgeColor } from "$lib/utils";
    import { relative } from "$lib/backend/telemetry.svelte";
    import type { RelativeWidgetSettings } from "$lib/types/telemetry";

    let { settings }: { settings: RelativeWidgetSettings } = $props();
</script>

<div
    class="flex flex-row items-center justify-center"
    style="opacity: {settings.opacity / 100}"
>
    <table
        class="bg-secondary-content rounded-md"
        style="width: {settings.width}px"
    >
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
                            outlineClasses="text-center ring ring-2 ring-inset {rel?.is_player
                                ? 'ring-secondary'
                                : 'ring-primary'}"
                            textClasses="text-sm text-right {rel?.is_player
                                ? 'text-secondary'
                                : 'text-primary'}"
                            text={rel?.car_number ? "#" + rel?.car_number : ""}
                        />
                    </td>
                    <td class="text text-sm">
                        <span
                            class="text text-sm tracking-tight"
                            style="opacity: {rel?.is_player_car_class
                                ? 1
                                : 0.7}"
                            >{rel?.user_name ?? ""}
                        </span>
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
        </tbody>
    </table>
</div>

<style>
</style>
