<script lang="ts">
    import { onDestroy } from "svelte";
    import { trackInfo } from "$lib/stores/track_info";
    import { trackSettings } from "$lib/stores/track_settings";
    import { page } from "$app/stores";

    let trackInfoData: { [k: string]: any } = {};
    let trackSettingsData: { [k: string]: any } = {};

    let path: SVGPathElement;

    let lapPointPositionInput: number = 0;
    let lapPointPositionValue: number = 0;

    let cx: number;
    let cy: number;

    const unsubscribeInfo = trackInfo.subscribe((value) => {
        trackInfoData = value;
    });

    const unsubscribeSettings = trackSettings.subscribe((value) => {
        trackSettingsData = value;
    });

    onDestroy(() => {
        unsubscribeInfo();
        unsubscribeSettings();
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

{#if $page.params.track_id}
    <div class="flex flex-col items-center">
        <h3 class="text-lg font-bold">
            {$page.params.track_id}
            {trackInfoData[$page.params.track_id].trackName} ({trackInfoData[
                $page.params.track_id
            ].configName})
        </h3>
        <span class="text-lg">
            Offset: {trackSettingsData[$page.params.track_id]?.offset} / Direction:
            {trackSettingsData[$page.params.track_id]?.direction}
        </span>
        <svg
            fill="none"
            viewBox="0 0 1920 1080"
            overflow="visible"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                bind:this={path}
                d={trackInfoData[$page.params.track_id].activePath}
                stroke="black"
                stroke-width="30"
            />
            <image
                href="/track_info_data/start_finish/{$page.params.track_id}.svg"
                x="0"
                y="0"
                width="1920"
                height="1080"
            />
            <circle {cx} {cy} r="32" fill="none" stroke="blue" stroke-width="16"/>
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
