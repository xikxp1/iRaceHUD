<script lang="ts">
    import type {
        TrackId,
        TrackMap,
        TrackMapDriver,
    } from "$lib/types/telemetry";
    import { trackID, trackMap } from "$lib/telemetry/telemetry.svelte";
    import { onMount } from "svelte";

    type TrackMapLocal = TrackMapDriver & {
        x: number;
        y: number;
    };

    let trackPathElement: SVGPathElement;
    let trackPathLength: number = 0;

    let trackInfo: { [k: number]: any } = {};
    let trackSettings: { [k: number]: any } = {};

    let track_id: TrackId;
    let trackPath: string | undefined;
    let trackMapCars: TrackMapLocal[] = [];

    let offset: number = 0;
    let direction: number = 1;

    const css = window.getComputedStyle(document.documentElement);

    const textColor: string = `oklch(${css.getPropertyValue("--pc")})`;
    const trackColor: string = `oklch(${css.getPropertyValue("--sc")})`;
    const trackBorderColor: string = `oklch(${css.getPropertyValue("--p")})`;
    const carCircleColor: string = `oklch(${css.getPropertyValue("--p")})`;
    const leaderCircleColor: string = `oklch(${css.getPropertyValue("--in")})`;
    const playerCircleColor: string = `oklch(${css.getPropertyValue("--s")})`;
    const offtrackCircleColor: string = `oklch(${css.getPropertyValue("--er")})`;
    const inPitsCircleColor: string = `oklch(${css.getPropertyValue("--su")})`;

    function onTrackId(trackId: TrackId) {
        track_id = trackId;
        let path = trackInfo[track_id]?.activePath;
        if (path != null) {
            trackPath = path;
        }
        offset = trackSettings[track_id]?.offset ?? 0;
        direction = trackSettings[track_id]?.direction ?? 1;
        if (trackPathElement == null) {
            return;
        }
        trackPathLength = trackPathElement.getTotalLength();
    }

    function onTrackMap(value: TrackMap) {
        if (trackPathLength === 0) {
            return;
        }
        const track_map: TrackMapLocal[] = [];
        for (let car of value) {
            const offsetedLapDistPct =
                (1 + offset + direction * car.lap_dist_pct) % 1;
            const point = trackPathElement.getPointAtLength(
                offsetedLapDistPct * trackPathLength,
            );
            let new_car: TrackMapLocal = {
                ...car,
                x: point.x,
                y: point.y,
            };
            track_map.push(new_car);
        }
        trackMapCars = track_map;
    }

    onMount(async () => {
        await Promise.all([
            fetch("/track_info_data/track_info.json")
                .then((response) => response.json())
                .then((data) => {
                    trackInfo = data;
                }),
            fetch("/track_info_data/track_settings.json")
                .then((response) => response.json())
                .then((data) => {
                    trackSettings = data;
                }),
        ]);

        trackID.subscribe((value) => {
            onTrackId(value);
        });

        onTrackId($trackID);

        trackMap.subscribe((value) => onTrackMap(value));

        onTrackMap($trackMap);
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="flex flex-col items-center justify-center w-[550px]">
        <svg
            fill="none"
            viewBox="0 0 1920 1080"
            width="100%"
            height="100%"
            overflow="visible"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path d={trackPath} stroke={trackBorderColor} stroke-width="30" />
            <path
                bind:this={trackPathElement}
                d={trackPath}
                stroke={trackColor}
                stroke-width="20"
            />
            {#if track_id != undefined && track_id != 0}
                <image
                    href="/track_info_data/start_finish/{track_id}.svg"
                    x="0"
                    y="0"
                    width="1920"
                    height="1080"
                >
                </image>
            {/if}
            {#each trackMapCars as car}
                {#if !car.is_off_world && !car.is_leader && !car.is_player}
                    <g>
                        <circle
                            cx={car.x}
                            cy={car.y}
                            r="32"
                            fill={carCircleColor}
                        />
                        {#if car.is_off_track || car.is_in_pits}
                            <circle
                                cx={car.x}
                                cy={car.y}
                                r="36"
                                stroke={car.is_off_track
                                    ? offtrackCircleColor
                                    : inPitsCircleColor}
                                stroke-width="6"
                            />
                        {/if}
                        <text
                            x={car.x}
                            y={car.y + 5}
                            fill={textColor}
                            font-size="60"
                            text-anchor="middle"
                            alignment-baseline="middle"
                        >
                            {car.position}
                        </text>
                    </g>
                {/if}
            {/each}
            {#each trackMapCars as car}
                {#if !car.is_off_world && car.is_leader && !car.is_player}
                    <g>
                        <circle
                            cx={car.x}
                            cy={car.y}
                            r="32"
                            fill={leaderCircleColor}
                        />
                        {#if car.is_off_track || car.is_in_pits}
                            <circle
                                cx={car.x}
                                cy={car.y}
                                r="36"
                                stroke={car.is_off_track
                                    ? offtrackCircleColor
                                    : inPitsCircleColor}
                                stroke-width="6"
                            />
                        {/if}
                        <text
                            x={car.x}
                            y={car.y + 5}
                            fill={textColor}
                            font-size="60"
                            text-anchor="middle"
                            alignment-baseline="middle"
                        >
                            {car.position}
                        </text>
                    </g>
                {/if}
            {/each}
            {#each trackMapCars as car}
                {#if !car.is_off_world && car.is_player}
                    <g>
                        <circle
                            cx={car.x}
                            cy={car.y}
                            r="32"
                            fill={playerCircleColor}
                        />
                        {#if car.is_off_track || car.is_in_pits}
                            <circle
                                cx={car.x}
                                cy={car.y}
                                r="36"
                                stroke={car.is_off_track
                                    ? offtrackCircleColor
                                    : inPitsCircleColor}
                                stroke-width="6"
                            />
                        {/if}
                        <text
                            x={car.x}
                            y={car.y + 5}
                            fill={textColor}
                            font-size="60"
                            text-anchor="middle"
                            alignment-baseline="middle"
                        >
                            {car.position}
                        </text>
                    </g>
                {/if}
            {/each}
        </svg>
    </div>
</div>

<style>
</style>
