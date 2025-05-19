<script lang="ts">
    import { lapTimes } from "$lib/telemetry/telemetry.svelte";
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <table class="bg-secondary-content rounded-md w-[110px]">
        <tbody>
            {#each $lapTimes as { lap, lap_time }}
                <tr
                    class="divide-x-2 divide-secondary even:bg-primary-content odd:bg-secondary-content"
                >
                    <td
                        class="text text-primary text-sm text-right pr-2 w-[30%]"
                    >
                        {lap}
                    </td>
                    <td
                        class="text text-primary text-sm text-right pr-1 w-[70%]"
                    >
                        {lap_time}
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

<style>
    table tr {
        clip-path: xywh(0 0 100% 100% round 0.375em);
    }
</style>
