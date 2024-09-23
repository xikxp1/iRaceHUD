<script lang="ts">
    import { onMount } from "svelte";
    import { trackInfo } from "$lib/stores/track_info";
    import { trackSettings } from "$lib/stores/track_settings";

    let trackInfoData: { [k: string]: any } = {};

    onMount(() => {
        fetch("/track_info_data/track_info.json")
            .then((response) => response.json())
            .then((data) => {
                trackInfoData = data;
                trackInfo.set(data);
            });

        fetch("/track_info_data/track_settings.json")
            .then((response) => response.json())
            .then((data) => {
                trackSettings.set(data);
            });
    });
</script>

<ul class="menu">
    {#each Object.keys(trackInfoData) as track_id}
        <li class="text-lg font-bold">
            <a href="/track_info/{track_id}">
                {track_id}
                {trackInfoData[track_id].trackName} ({trackInfoData[track_id]
                    .configName})
            </a>
        </li>
    {/each}
</ul>

<style>
</style>
