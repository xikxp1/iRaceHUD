<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    let trackPaths: { [k: number]: string } = {};

    let track_id: number;
    let trackPath: string | undefined;

    const css = window.getComputedStyle(document.documentElement);

    const trackColor: string = `oklch(${css.getPropertyValue("--sc")})`;
    const trackStrokeColor: string = `oklch(${css.getPropertyValue("--p")})`;

    onMount(() => {
        fetch("/track_paths/track_paths.json")
            .then((response) => response.json())
            .then((data) => {
                trackPaths = data;
            });
    });

    listen("track_id", (event) => {
        track_id = event.payload as number;
        trackPath = trackPaths[track_id];
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <svg
        width="500"
        height="100%"
        viewBox="0 0 1800 1000"
        fill="none"
        overflow="visible"
        xmlns="http://www.w3.org/2000/svg"
    >
        <path
            d={trackPath}
            fill={trackColor}
            stroke={trackStrokeColor}
            stroke-width="5"
        />
    </svg>
</div>

<style>
</style>
