<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";
    import { flip } from "svelte/animate";

    let stength_of_field = 0;
    let current_time = "––:––";
    let driver_count = 0;

    let standings: Standings[] = [];

    interface Standings {
        car_id: number;
        position: number;
        user_name: string;
        car_number: string;
        irating: string;
        leader_gap: string;
        best_lap: string;
        last_lap: string;
        is_player: boolean;
        is_in_pits: boolean;
    }

    const SWITCH_INTERVAL = 10000;
    let show_best_lap = false;
    let interval: NodeJS.Timeout;

    onMount(() => {
        interval = setInterval(() => {
            show_best_lap = !show_best_lap;
        }, SWITCH_INTERVAL);
    });

    let unlistens = [];

    unlistens.push(
        listen("strength_of_field", (event) => {
            stength_of_field = event.payload as number;
        }),
    );

    unlistens.push(
        listen("current_time", (event) => {
            current_time = event.payload as string;
        }),
    );

    unlistens.push(
        listen("standings", (event) => {
            let newStandings = event.payload as Standings[];
            //TODO: Calculate changes
            standings = newStandings;
            driver_count = standings.length;
        }),
    );

    onDestroy(() => {
        clearInterval(interval);
        unlistens.forEach(async (unlisten) => (await unlisten)());
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="flex flex-col bg-primary-content rounded-l-md w-[360px]">
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
                    <td class="text text-sm text-right pr-2 w-[30px]">
                        {st?.car_number ?? ""}
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
                        {st?.irating}
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
