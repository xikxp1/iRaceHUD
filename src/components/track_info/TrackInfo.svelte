<script lang="ts">
    import { page } from "$app/state";
    import { getTrackInfoState } from "$lib/stores/TrackInfoState.svelte";
    import { getTrackSettingsState } from "$lib/stores/TrackSettingsState.svelte";

    let trackId: number = $derived(Number(page.params.track_id));

    const trackInfoState = getTrackInfoState();
    const trackSettingsState = getTrackSettingsState();

    let path: SVGPathElement | undefined = $state();

    let lapPointPositionInput: number = $state(0);
    let lapPointPositionValue: number = $state(0);

    let cx: number = $state(0);
    let cy: number = $state(0);

    function setPointOnPath(lapPointPosition: number) {
        if (!path) return;
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

{#if trackInfoState.isLoading || trackSettingsState.isLoading}
    <div class="flex justify-center items-center h-full">
        <div class="loading loading-spinner loading-lg"></div>
    </div>
{:else if page.params.track_id}
    <div class="flex flex-col items-center">
        <h3 class="text-lg font-bold">
            {trackId}
            {trackInfoState.data[trackId].trackName} ({trackInfoState.data[
                trackId
            ].configName})
        </h3>
        <span class="text-lg">
            Offset: {trackSettingsState.data[trackId]?.offset} / Direction:
            {trackSettingsState.data[trackId]?.direction}
        </span>
        <svg
            fill="none"
            viewBox="0 0 1920 1080"
            overflow="visible"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                bind:this={path}
                d={trackInfoState.data[trackId].activePath}
                stroke="black"
                stroke-width="30"
            />
            {#each trackInfoState.data[trackId].turns as turn}
                <text
                    fill="black"
                    transform={turn.transform}
                    font-size={parseInt(turn.textContent) ? "36" : "0"}
                    font-weight="bold"
                    stroke="white"
                    stroke-width="2"
                    paint-order="stroke"
                    text-anchor="middle"
                    dominant-baseline="middle"
                >
                    {turn.textContent}
                </text>
            {/each}
            <image
                href="/track_info_data/start_finish/{trackId}.svg"
                x="0"
                y="0"
                width="1920"
                height="1080"
            />
            <circle
                {cx}
                {cy}
                r="32"
                fill="none"
                stroke="blue"
                stroke-width="16"
            />
        </svg>
        <input
            type="range"
            min="0"
            max="1000"
            value={lapPointPositionInput}
            class="range"
            onchange={handleOnChange}
            oninput={handleOnChange}
        />
        <span class="text">{lapPointPositionValue}</span>
    </div>
{/if}
