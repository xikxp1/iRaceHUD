<script lang="ts">
    import { page } from "$app/stores";
    import { onMount } from "svelte";

    let trackInfo: { [k: string]: any } = {};
    let path: SVGPathElement;
    let startFinishSvg: SVGElement;

    let lapPointPositionInput: number = 0;
    let lapPointPositionValue: number = 0;

    let cx: number;
    let cy: number;

    onMount(() => {
        fetch("/track_info/track_info.json")
            .then((response) => response.json())
            .then((data) => {
                trackInfo = data;
            });
    });

    function setPointOnPath(lapPointPosition: number) {
        const pathLength = path.getTotalLength();
        const pointOnPath = path.getPointAtLength(
            pathLength * lapPointPosition,
        );
        cx = pointOnPath.x;
        cy = pointOnPath.y;
    }

    function handleOnChange(event: Event) {
        lapPointPositionValue =
            Number((event.target as HTMLInputElement).value) / 1000.0;
        setPointOnPath(lapPointPositionValue);
    }
</script>

<div class="flex flex-row">
    <ul class="menu w-1/4">
        {#each Object.keys(trackInfo) as track_id}
            <li class="text-lg font-bold">
                <a href="/track_info/{track_id}">
                    {track_id}
                    {trackInfo[track_id].trackName} ({trackInfo[track_id]
                        .configName})
                </a>
            </li>
        {/each}
    </ul>
    {#if $page.params.track_id}
        <div class="flex flex-col w-3/4 items-center">
            <h3 class="text-lg font-bold">
                {$page.params.track_id}
                {trackInfo[$page.params.track_id].trackName} ({trackInfo[
                    $page.params.track_id
                ].configName})
            </h3>
            <svg
                fill="none"
                viewBox="0 0 1920 1080"
                overflow="visible"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    bind:this={path}
                    d={trackInfo[$page.params.track_id].activePath}
                    stroke="black"
                    stroke-width="30"
                />
                <image
                    href="/track_info/start_finish/{$page.params.track_id}.svg"
                    x="0"
                    y="0"
                    width="1920"
                    height="1080"
                />
                <circle {cx} {cy} r="32" fill="blue" />
            </svg>
            <input
                type="range"
                min="0"
                max="1000"
                value={lapPointPositionInput}
                class="range"
                on:change={handleOnChange}
                on:input={handleOnChange}
            />
            <span class="text">{lapPointPositionValue}</span>
        </div>
    {/if}
</div>

<style>
</style>
