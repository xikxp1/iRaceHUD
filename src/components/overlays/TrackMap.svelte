<script lang="ts">
    import type {
        TrackId,
        TrackMap,
        TrackMapDriver,
        TrackMapOverlaySettings,
    } from "$lib/types/telemetry";
    import { trackID, trackMap } from "$lib/backend/telemetry.svelte";
    import { onDestroy, onMount } from "svelte";

    let { settings }: { settings: TrackMapOverlaySettings } = $props();

    type TrackMapLocal = TrackMapDriver & {
        transform: string;
    };

    // Cache for point calculations
    const pointCache = new Map<number, { x: number; y: number }>();
    const CACHE_SIZE = 10000;

    let trackPathElement: SVGPathElement | undefined = $state();
    let trackPathLength: number = 0;
    let trackBounds: { x: number; y: number; width: number; height: number } =
        $state({ x: 0, y: 0, width: 0, height: 0 });

    let trackInfo: { [k: number]: any } = {};
    let trackSettings: { [k: number]: any } = {};

    let track_id: TrackId;
    let trackPath: string | undefined = $state();
    let trackMapCars: TrackMapLocal[] = $state([]);
    let trackMapCarsById: Map<number, TrackMapLocal> = $state(new Map());
    let animationFrameId: number | null = null;
    let pendingTrackMapUpdate: TrackMap | null = null;

    let offset: number = 0;
    let direction: number = 1;

    let startFinishPoint: { x: number; y: number } | null = $state(null);
    let startFinishPerp: { x: number; y: number } | null = $state(null);
    const START_LINE_LENGTH = 50;

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

    function getPointAtLength(length: number): { x: number; y: number } {
        if (!trackPathElement || !trackPathLength) return { x: 0, y: 0 };

        // Check cache first
        const cacheKey = Math.round(length * CACHE_SIZE) / CACHE_SIZE; // Round to 4 decimal places
        if (pointCache.has(cacheKey)) {
            return pointCache.get(cacheKey)!;
        }

        // Calculate new point
        const point = trackPathElement.getPointAtLength(length);

        // Update cache
        if (pointCache.size >= CACHE_SIZE) {
            // Remove oldest entry if cache is full
            const firstKey = pointCache.keys().next().value;
            if (firstKey != null) {
                pointCache.delete(firstKey);
            }
        }
        pointCache.set(cacheKey, point);

        return point;
    }

    function calculateStartFinishLine() {
        if (!trackPathElement) return;

        const point = getPointAtLength(offset * trackPathLength);
        const delta = 0.001;
        const nextPoint = getPointAtLength((offset + delta) * trackPathLength);

        const dx = nextPoint.x - point.x;
        const dy = nextPoint.y - point.y;
        const length = Math.sqrt(dx * dx + dy * dy);

        const perpX = -dy / length;
        const perpY = dx / length;

        startFinishPoint = point;
        startFinishPerp = { x: perpX, y: perpY };
    }

    function calculateTrackBounds() {
        if (!trackPathElement) return;

        const bbox = trackPathElement.getBBox();
        const padding = 200;
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
        if (trackPathElement == null || trackPathLength === 0) return;
        pendingTrackMapUpdate = value;

        if (animationFrameId === null) {
            animationFrameId = requestAnimationFrame(updateTrackMap);
        }
    }

    function updateTrackMap() {
        if (
            !pendingTrackMapUpdate ||
            trackPathElement == null ||
            trackPathLength === 0
        ) {
            animationFrameId = null;
            return;
        }

        // Only update cars that have changed
        const track_map: TrackMapLocal[] = [];
        const newTrackMapCarsById = new Map<number, TrackMapLocal>();

        for (let car of pendingTrackMapUpdate) {
            const offsetedLapDistPct =
                (1 + offset + direction * car.lap_dist_pct) % 1;
            const point = getPointAtLength(
                offsetedLapDistPct * trackPathLength,
            );

            // Only update if position changed significantly
            const existingCar = trackMapCarsById.get(car.car_id);
            if (existingCar) {
                const dx =
                    point.x -
                    parseFloat(
                        existingCar.transform.split("(")[1].split(",")[0],
                    );
                const dy =
                    point.y -
                    parseFloat(
                        existingCar.transform.split(")")[0].split(",")[1],
                    );
                if (Math.sqrt(dx * dx + dy * dy) < 1) {
                    // Skip if movement is less than 1 pixel
                    track_map.push(existingCar);
                    newTrackMapCarsById.set(car.car_id, existingCar);
                    continue;
                }
            }

            let new_car: TrackMapLocal = {
                ...car,
                transform: `translate(${point.x}, ${point.y})`,
            };
            track_map.push(new_car);
            newTrackMapCarsById.set(car.car_id, new_car);
        }
        trackMapCars = track_map;
        trackMapCarsById = newTrackMapCarsById;
        pendingTrackMapUpdate = null;
        animationFrameId = null;
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
        pointCache.clear();
        if (animationFrameId !== null) {
            cancelAnimationFrame(animationFrameId);
        }
    });
</script>

<div class="flex flex-col items-center justify-center w-full h-full">
    <svg
        fill="none"
        viewBox="{trackBounds.x} {trackBounds.y} {trackBounds.width} {trackBounds.height}"
        width="70%"
        height="70%"
        preserveAspectRatio="xMidYMid meet"
        xmlns="http://www.w3.org/2000/svg"
    >
        <filter id="shadow">
            <feDropShadow dx="2" dy="2" stdDeviation="2" flood-opacity="0.3" />
        </filter>

        <!-- Track background with shadow -->
        <path
            d={trackPath}
            stroke={trackBorderColor}
            stroke-width="30"
            filter="url(#shadow)"
        />

        <defs>
            <linearGradient
                id="trackGradient"
                x1="0%"
                y1="0%"
                x2="100%"
                y2="0%"
            >
                <stop
                    offset="0%"
                    style="stop-color: {trackColor}; stop-opacity: 0.8"
                />
                <stop
                    offset="100%"
                    style="stop-color: {trackColor}; stop-opacity: 1"
                />
            </linearGradient>
        </defs>

        <!-- Track surface -->
        <path
            bind:this={trackPathElement}
            d={trackPath}
            stroke="url(#trackGradient)"
            stroke-width="20"
        />

        <!-- Start/Finish line -->
        {#if startFinishPoint && startFinishPerp}
            <line
                x1={startFinishPoint.x - startFinishPerp.x * START_LINE_LENGTH}
                y1={startFinishPoint.y - startFinishPerp.y * START_LINE_LENGTH}
                x2={startFinishPoint.x + startFinishPerp.x * START_LINE_LENGTH}
                y2={startFinishPoint.y + startFinishPerp.y * START_LINE_LENGTH}
                stroke={trackColor}
                stroke-width="16"
            />
            <line
                x1={startFinishPoint.x - startFinishPerp.x * START_LINE_LENGTH}
                y1={startFinishPoint.y - startFinishPerp.y * START_LINE_LENGTH}
                x2={startFinishPoint.x + startFinishPerp.x * START_LINE_LENGTH}
                y2={startFinishPoint.y + startFinishPerp.y * START_LINE_LENGTH}
                stroke={startFinishColor}
                stroke-width="12"
            />
        {/if}
        {#each trackMapCars as car}
            {#if !car.is_off_world && !car.is_leader && !car.is_player}
                <g transform={car.transform} style="will-change: transform;">
                    <circle r="32" fill={carCircleColor} />
                    {#if car.is_off_track || car.is_in_pits}
                        <circle
                            r="36"
                            stroke={car.is_off_track
                                ? offtrackCircleColor
                                : inPitsCircleColor}
                            stroke-width="6"
                        />
                    {/if}
                    <text
                        y="5"
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
                <g transform={car.transform} style="will-change: transform;">
                    <circle r="32" fill={leaderCircleColor} />
                    {#if car.is_off_track || car.is_in_pits}
                        <circle
                            r="36"
                            stroke={car.is_off_track
                                ? offtrackCircleColor
                                : inPitsCircleColor}
                            stroke-width="6"
                        />
                    {/if}
                    <text
                        y="5"
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
                <g transform={car.transform} style="will-change: transform;">
                    <circle r="32" fill={playerCircleColor} />
                    {#if car.is_off_track || car.is_in_pits}
                        <circle
                            r="36"
                            stroke={car.is_off_track
                                ? offtrackCircleColor
                                : inPitsCircleColor}
                            stroke-width="6"
                        />
                    {/if}
                    <text
                        y="5"
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

<style>
</style>
