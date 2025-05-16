<script lang="ts">
    import { getTrackInfoState } from "$lib/stores/TrackInfoState.svelte";

    const trackInfoState = getTrackInfoState();
</script>

{#if trackInfoState.isLoading}
    <div class="flex justify-center items-center h-full">
        <div class="loading loading-spinner loading-lg"></div>
    </div>
{:else}
    <ul class="menu">
        {#each Object.entries(trackInfoState.data) as [id, track]}
            <li class="text-lg font-bold">
                <a href="/track_info/{id}">
                    {id}
                    {track.trackName} ({track.configName})
                </a>
            </li>
        {/each}
    </ul>
{/if}

<style>
</style>
