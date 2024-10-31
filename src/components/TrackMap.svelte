<script lang="ts">
    import { trackID, trackMap } from "$lib/telemetry/telemetry.svelte";
    import type { TrackID, TrackMap } from "$lib/types/telemetry";
    import { onMount } from "svelte";

    let trackPathElement: SVGPathElement;

    let trackInfo: { [k: number]: any } = {};
    let trackSettings: { [k: number]: any } = {};

    let track_id: TrackID;
    let trackPath: string | undefined;
    let trackMapCars: TrackMap = [];

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

    onMount(() => {
        fetch("/track_info_data/track_info.json")
            .then((response) => response.json())
            .then((data) => {
                trackInfo = data;
            });
        fetch("/track_info_data/track_settings.json")
            .then((response) => response.json())
            .then((data) => {
                trackSettings = data;
            });

        trackID.subscribe((value) => {
            track_id = value;
            let path = trackInfo[track_id]?.activePath;
            if (path !== undefined) {
                trackPath = path;
            }
            offset = trackSettings[track_id]?.offset ?? 0;
            direction = trackSettings[track_id]?.direction ?? 1;
        });

        trackMap.subscribe((value) => {
            if (trackPathElement === undefined) {
                return;
            }
            const pathLength = trackPathElement.getTotalLength();
            if (pathLength === 0) {
                return;
            }
            const track_map = value;
            for (let car of track_map) {
                const offsetedLapDistPct =
                    (1 + offset + direction * car.lap_dist_pct) % 1;
                const point = trackPathElement.getPointAtLength(
                    offsetedLapDistPct * pathLength,
                );
                car.x = point.x;
                car.y = point.y;
            }
            trackMapCars = track_map;
        });
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="flex flex-col items-center justify-center w-[600px]">
        <svg
            fill="none"
            viewBox="0 0 1920 1080"
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
