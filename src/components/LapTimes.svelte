<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy } from "svelte";

    interface LapTime {
        lap: number;
        lap_time: string;
    }

    let player_lap_times: LapTime[] = [];

    let unlistens = [];

    unlistens.push(
        listen("player_lap_times", (event) => {
            player_lap_times = event.payload as {
                lap: number;
                lap_time: string;
            }[];
        }),
    );

    onDestroy(() => {
        unlistens.forEach(async (unlisten) => (await unlisten)());
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
