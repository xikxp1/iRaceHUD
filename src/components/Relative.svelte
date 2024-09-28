<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy } from "svelte";
    import Badge from "./utils/Badge.svelte";

    interface Relative {
        car_id: number;
        position: number;
        user_name: string;
        car_number: string;
        irating: string;
        license: string;
        player_relative_gap: string;
        is_player: boolean;
    }

    let relative: (Relative | null)[] = [];

    let unlistens = [];

    unlistens.push(
        listen("relative", (event) => {
            relative = event.payload as (Relative | null)[];
        }),
    );

    onDestroy(() => {
        unlistens.forEach(async (unlisten) => (await unlisten)());
    });
</script>

<div class="flex flex-row items-center justify-center opacity-90">
    <table class="bg-secondary-content rounded-md w-[400px]">
        {#each relative as rel}
            <tr
                class="{rel?.is_player
                    ? 'bg-secondary text-primary-content'
                    : 'odd:bg-secondary-content even:bg-primary-content text-primary'} h-[18px]"
            >
                <td class="text text-sm text-right pr-2 w-[25px]">
                    {rel?.position ?? "*"}</td
                >
                <td class="text text-sm text-right pr-2 w-[30px]">
                    {rel?.car_number ?? ""}</td
                >
                <td class="text text-sm">
                    {rel?.user_name ?? ""}
                </td>
                <td class="text text-sm text-right pr-1 w-[100px]">
                    <Badge
                        license={rel?.license ?? ""}
                        irating={rel?.irating ?? ""}
                    />
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
