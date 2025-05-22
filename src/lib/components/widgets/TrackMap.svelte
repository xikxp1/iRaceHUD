<script lang="ts">
    import type {
        TrackId,
        TrackMap,
        TrackMapDriver,
        TrackMapWidgetSettings,
    } from "$lib/types/telemetry";
    import { trackID, trackMap } from "$lib/backend/telemetry.svelte";
    import { onDestroy, onMount } from "svelte";

    let { settings }: { settings: TrackMapWidgetSettings } = $props();

    type TrackMapLocal = TrackMapDriver & {
        x: number;
        y: number;
    };

    let trackPathElement: SVGPathElement | undefined = $state();
    let trackPathLength: number = 0;
    let trackBounds: { x: number; y: number; width: number; height: number } =
        $state({ x: 0, y: 0, width: 0, height: 0 });

    let trackInfo: { [k: number]: any } = {};
    let trackSettings: { [k: number]: any } = {};

    let track_id: TrackId;
    let trackPath: string | undefined = $state();
    let trackMapCars: TrackMapLocal[] = $state([]);

    let offset: number = 0;
    let direction: number = 1;

    let startFinishPoint: { x: number; y: number } | null = $state(null);
    let startFinishPerp: { x: number; y: number } | null = $state(null);
    const START_LINE_LENGTH = 50; // Length of the start/finish line in pixels

    const css = window.getComputedStyle(document.documentElement);

    const textColor: string = `oklch(${css.getPropertyValue("--pc")})`;
    const trackColor: string = `oklch(${css.getPropertyValue("--sc")})`;
    const startFinishColor: string = `oklch(${css.getPropertyValue("--s")})`;
    const trackBorderColor: string = `oklch(${css.getPropertyValue("--p")})`;
    const carCircleColor: string = `oklch(${css.getPropertyValue("--p")})`;
    const leaderCircleColor: string = `oklch(${css.getPropertyValue("--in")})`;
    const playerCircleColor: string = `oklch(${css.getPropertyValue("--s")})`;
    const offtrackCircleColor: string = `oklch(${css.getPropertyValue("--er")})`;
    const inPitsCircleColor: string = `oklch(${css.getPropertyValue("--su")})`;

    function calculateStartFinishLine() {
        if (!trackPathElement) return;

        // Get point at the offset position
        const point = trackPathElement.getPointAtLength(
            offset * trackPathLength,
        );

        // Get point slightly ahead to calculate direction
        const delta = 0.001;
        const nextPoint = trackPathElement.getPointAtLength(
            (offset + delta) * trackPathLength,
        );

        // Calculate tangent vector
        const dx = nextPoint.x - point.x;
        const dy = nextPoint.y - point.y;
        const length = Math.sqrt(dx * dx + dy * dy);

        // Calculate perpendicular vector
        const perpX = -dy / length;
        const perpY = dx / length;

        startFinishPoint = point;
        startFinishPerp = { x: perpX, y: perpY };
    }

    function calculateTrackBounds() {
        if (!trackPathElement) return;

        const bbox = trackPathElement.getBBox();
        const padding = 200; // Add some padding around the track
        trackBounds = {
            x: bbox.x - padding,
            y: bbox.y - padding,
            width: bbox.width + padding * 2,
            height: bbox.height + padding * 2,
        };
    }

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
        if (trackPath == null) {
            return;
        }
        trackPathElement.setAttribute("d", trackPath);
        trackPathLength = trackPathElement.getTotalLength();
        calculateStartFinishLine();
        calculateTrackBounds();
    }

    function onTrackMap(value: TrackMap) {
        if (trackPathElement == null) {
            return;
        }
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

    let unsubscribe_track_id: () => void = () => {};
    let unsubscribe_track_map: () => void = () => {};

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

        unsubscribe_track_id = trackID.subscribe((value) => {
            onTrackId(value);
        });

        onTrackId($trackID);

        unsubscribe_track_map = trackMap.subscribe((value) =>
            onTrackMap(value),
        );

        onTrackMap($trackMap);
    });

    onDestroy(() => {
        unsubscribe_track_id();
        unsubscribe_track_map();
    });
</script>

<div
    class="flex flex-row items-center justify-center"
    style="opacity: {settings.opacity / 100}"
>
    <div
        class="flex flex-col items-center justify-center"
        style="width: {settings.width}px"
    >
        <svg
            fill="none"
            viewBox="{trackBounds.x} {trackBounds.y} {trackBounds.width} {trackBounds.height}"
            width="70%"
            height="70%"
            preserveAspectRatio="xMidYMid meet"
            xmlns="http://www.w3.org/2000/svg"
        >
            <!-- Add a subtle shadow for depth -->
            <filter id="shadow">
                <feDropShadow dx="2" dy="2" stdDeviation="2" flood-opacity="0.3"/>
            </filter>
            
            <!-- Track border with shadow -->
            <path 
                d={trackPath} 
                stroke={trackBorderColor} 
                stroke-width="30"
                filter="url(#shadow)"
            />
            
            <!-- Track with gradient -->
            <defs>
                <linearGradient id="trackGradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: {trackColor}; stop-opacity: 0.8"/>
                    <stop offset="100%" style="stop-color: {trackColor}; stop-opacity: 1"/>
                </linearGradient>
            </defs>
            
            <path
                bind:this={trackPathElement}
                d={trackPath}
                stroke="url(#trackGradient)"
                stroke-width="20"
            />
            {#if startFinishPoint && startFinishPerp}
                <!-- Outline line -->
                <line
                    x1={startFinishPoint.x -
                        startFinishPerp.x * START_LINE_LENGTH}
                    y1={startFinishPoint.y -
                        startFinishPerp.y * START_LINE_LENGTH}
                    x2={startFinishPoint.x +
                        startFinishPerp.x * START_LINE_LENGTH}
                    y2={startFinishPoint.y +
                        startFinishPerp.y * START_LINE_LENGTH}
                    stroke={trackColor}
                    stroke-width="16"
                />
                <!-- Main line -->
                <line
                    x1={startFinishPoint.x -
                        startFinishPerp.x * START_LINE_LENGTH}
                    y1={startFinishPoint.y -
                        startFinishPerp.y * START_LINE_LENGTH}
                    x2={startFinishPoint.x +
                        startFinishPerp.x * START_LINE_LENGTH}
                    y2={startFinishPoint.y +
                        startFinishPerp.y * START_LINE_LENGTH}
                    stroke={startFinishColor}
                    stroke-width="12"
                />
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
