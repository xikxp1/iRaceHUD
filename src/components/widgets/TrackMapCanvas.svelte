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

    // Cache for point calculations
    const pointCache = new Map<number, { x: number; y: number }>();
    const CACHE_SIZE = 10000;

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D;
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
        const cacheKey = Math.round(length * CACHE_SIZE) / CACHE_SIZE;
        if (pointCache.has(cacheKey)) {
            return pointCache.get(cacheKey)!;
        }

        // Calculate new point
        const point = trackPathElement.getPointAtLength(length);

        // Update cache
        if (pointCache.size >= CACHE_SIZE) {
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
        const padding = 100;
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

    function updateTrackMap() {
        if (
            !pendingTrackMapUpdate ||
            trackPathElement == null ||
            trackPathLength === 0
        ) {
            animationFrameId = null;
            return;
        }

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
                const dx = point.x - existingCar.x;
                const dy = point.y - existingCar.y;
                if (Math.sqrt(dx * dx + dy * dy) < 1) {
                    // Skip if movement is less than 1 pixel
                    track_map.push(existingCar);
                    newTrackMapCarsById.set(car.car_id, existingCar);
                    continue;
                }
            }

            let new_car: TrackMapLocal = {
                ...car,
                x: point.x,
                y: point.y,
            };
            track_map.push(new_car);
            newTrackMapCarsById.set(car.car_id, new_car);
        }
        trackMapCars = track_map;
        trackMapCarsById = newTrackMapCarsById;
        pendingTrackMapUpdate = null;
        animationFrameId = null;

        // Trigger redraw
        draw();
    }

    function draw() {
        if (!ctx || !trackPathElement) return;

        // Clear canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Calculate scale to fit the track in the canvas
        const scale =
            Math.min(
                canvas.width / trackBounds.width,
                canvas.height / trackBounds.height,
            ) * 0.8; // 80% of the available space

        // Set up transform to center and scale the track
        ctx.setTransform(
            scale,
            0,
            0,
            scale,
            (canvas.width - trackBounds.width * scale) / 2 -
                trackBounds.x * scale,
            (canvas.height - trackBounds.height * scale) / 2 -
                trackBounds.y * scale,
        );

        // Draw track background with shadow
        ctx.shadowColor = "rgba(0, 0, 0, 0.3)";
        ctx.shadowBlur = 4;
        ctx.shadowOffsetX = 2;
        ctx.shadowOffsetY = 2;
        ctx.strokeStyle = trackBorderColor;
        ctx.lineWidth = 30;
        ctx.stroke(new Path2D(trackPath || ""));

        // Draw track surface
        ctx.shadowColor = "transparent";
        ctx.strokeStyle = trackColor;
        ctx.lineWidth = 20;
        ctx.stroke(new Path2D(trackPath || ""));

        // Draw start/finish line
        if (startFinishPoint && startFinishPerp) {
            ctx.beginPath();
            ctx.moveTo(
                startFinishPoint.x - startFinishPerp.x * START_LINE_LENGTH,
                startFinishPoint.y - startFinishPerp.y * START_LINE_LENGTH,
            );
            ctx.lineTo(
                startFinishPoint.x + startFinishPerp.x * START_LINE_LENGTH,
                startFinishPoint.y + startFinishPerp.y * START_LINE_LENGTH,
            );
            ctx.strokeStyle = trackColor;
            ctx.lineWidth = 16;
            ctx.stroke();

            ctx.beginPath();
            ctx.moveTo(
                startFinishPoint.x - startFinishPerp.x * START_LINE_LENGTH,
                startFinishPoint.y - startFinishPerp.y * START_LINE_LENGTH,
            );
            ctx.lineTo(
                startFinishPoint.x + startFinishPerp.x * START_LINE_LENGTH,
                startFinishPoint.y + startFinishPerp.y * START_LINE_LENGTH,
            );
            ctx.strokeStyle = startFinishColor;
            ctx.lineWidth = 12;
            ctx.stroke();
        }

        // Helper function to check if cars are close to each other
        function areCarsClose(
            car1: TrackMapLocal,
            car2: TrackMapLocal,
        ): boolean {
            const dx = car1.x - car2.x;
            const dy = car1.y - car2.y;
            return Math.sqrt(dx * dx + dy * dy) < 50;
        }

        // Helper function to draw a car circle without text
        function drawCarCircle(car: TrackMapLocal) {
            ctx.save();
            ctx.translate(car.x, car.y);

            // Draw car circle
            ctx.beginPath();
            ctx.arc(0, 0, 32, 0, Math.PI * 2);
            ctx.fillStyle = car.is_player
                ? playerCircleColor
                : car.is_leader
                  ? leaderCircleColor
                  : carCircleColor;
            ctx.fill();

            // Draw status circle if needed
            if (car.is_off_track || car.is_in_pits) {
                ctx.beginPath();
                ctx.arc(0, 0, 36, 0, Math.PI * 2);
                ctx.strokeStyle = car.is_off_track
                    ? offtrackCircleColor
                    : inPitsCircleColor;
                ctx.lineWidth = 6;
                ctx.stroke();
            }

            ctx.restore();
        }

        // Helper function to draw position number
        function drawPositionNumber(car: TrackMapLocal) {
            ctx.save();
            ctx.translate(car.x, car.y);
            ctx.fillStyle = textColor;
            ctx.font = "52px iracing";
            ctx.textAlign = "center";
            ctx.textBaseline = "middle";
            ctx.fillText(car.position.toString(), 0, 5);
            ctx.restore();
        }

        // Draw cars in order
        const visibleCars = trackMapCars.filter((car) => !car.is_off_world);

        // First draw regular cars
        const regularCars = visibleCars.filter(
            (car) => !car.is_leader && !car.is_player,
        );
        regularCars.sort((a, b) => a.position - b.position);

        // Identify clusters
        const clusters: TrackMapLocal[][] = [];
        let currentCluster: TrackMapLocal[] = [];

        for (let i = 0; i < regularCars.length; i++) {
            const car = regularCars[i];

            if (currentCluster.length === 0) {
                currentCluster.push(car);
            } else {
                const lastCar = currentCluster[currentCluster.length - 1];
                if (areCarsClose(car, lastCar)) {
                    currentCluster.push(car);
                } else {
                    clusters.push([...currentCluster]);
                    currentCluster = [car];
                }
            }
        }
        if (currentCluster.length > 0) {
            clusters.push(currentCluster);
        }

        // First draw all car circles
        for (const cluster of clusters) {
            for (const car of cluster) {
                drawCarCircle(car);
            }
        }

        // Then draw all position numbers in reverse order
        for (let i = clusters.length - 1; i >= 0; i--) {
            const cluster = clusters[i];
            if (areCarsClose(cluster[0], cluster[cluster.length - 1])) {
                drawPositionNumber(cluster[0]);
            } else {
                drawPositionNumber(cluster[cluster.length - 1]);
                drawPositionNumber(cluster[0]);
            }
        }

        // Draw leader car circle
        const leaderCar = visibleCars.find(
            (car) => car.is_leader && !car.is_player,
        );
        if (leaderCar) {
            drawCarCircle(leaderCar);
        }

        // Draw player car circle
        const playerCar = visibleCars.find((car) => car.is_player);
        if (playerCar) {
            drawCarCircle(playerCar);
        }

        // Draw leader and player positions
        if (leaderCar) {
            drawPositionNumber(leaderCar);
        }
        if (playerCar) {
            drawPositionNumber(playerCar);
        }

        // Reset transform
        ctx.setTransform(1, 0, 0, 1, 0, 0);
    }

    function onTrackMap(value: TrackMap) {
        if (trackPathElement == null || trackPathLength === 0) return;
        pendingTrackMapUpdate = value;

        if (animationFrameId === null) {
            animationFrameId = requestAnimationFrame(updateTrackMap);
        }
    }

    let unsubscribe_track_id: () => void = () => {};
    let unsubscribe_track_map: () => void = () => {};

    onMount(async () => {
        // Create temporary SVG element for path calculations
        const tempSvg = document.createElementNS(
            "http://www.w3.org/2000/svg",
            "svg",
        );
        trackPathElement = document.createElementNS(
            "http://www.w3.org/2000/svg",
            "path",
        );
        tempSvg.appendChild(trackPathElement);
        document.body.appendChild(tempSvg);

        // Set up canvas
        ctx = canvas.getContext("2d")!;
        canvas.width = settings.width;
        canvas.height = settings.width; // Square aspect ratio

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
        if (trackPathElement?.parentElement) {
            trackPathElement.parentElement.remove();
        }
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
        <canvas
            bind:this={canvas}
            width={settings.width}
            height={settings.width}
        ></canvas>
    </div>
</div>

<style>
</style>
