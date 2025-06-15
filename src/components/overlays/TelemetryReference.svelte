<script lang="ts">
    import { Chart } from "chart.js/auto";
    import { onDestroy, onMount } from "svelte";
    import { telemetryReference } from "$lib/backend/telemetry.svelte";
    import { telemetryReferencePoints } from "$lib/backend/telemetry_reference.svelte";
    import type { TelemetryReferenceOverlaySettings } from "$lib/types/telemetry";

    let { settings }: { settings: TelemetryReferenceOverlaySettings } =
        $props();

    let telemetryCanvas: HTMLCanvasElement | undefined = $state();
    let chart: Chart | undefined = $state();

    const WINDOW_SIZE = 600 * 100; // 600m
    const HALF_WINDOW = WINDOW_SIZE / 2;

    const throttleData = [] as { x: number; y: number }[];
    const brakeData = [] as { x: number; y: number }[];
    const steeringAngleData = [] as { x: number; y: number }[];

    const referenceThrottleData = [] as { x: number; y: number }[];
    const referenceBrakeData = [] as { x: number; y: number }[];
    const referenceSteeringAngleData = [] as { x: number; y: number }[];

    const currentReferenceThrottleData = [] as { x: number; y: number }[];
    const currentReferenceBrakeData = [] as { x: number; y: number }[];
    const currentReferenceSteeringAngleData = [] as { x: number; y: number }[];

    let currentReferenceStartIndex = -1;
    let currentReferenceEndIndex = -1;

    const css = window.getComputedStyle(document.documentElement);
    const throttleColor: string = `oklch(${css.getPropertyValue("--su")})`;
    const brakeColor: string = `oklch(${css.getPropertyValue("--er")})`;
    const steeringAngleColor: string = `oklch(${css.getPropertyValue("--p")})`;
    const throttleReferenceColor: string = `oklch(${css.getPropertyValue("--in")} / 0.6)`;
    const brakeReferenceColor: string = `oklch(${css.getPropertyValue("--a")} / 0.6)`;
    const steeringAngleReferenceColor: string = `oklch(${css.getPropertyValue("--p")} / 0.5)`;

    const telemetryOptions = {
        plugins: {
            legend: {
                display: false,
            },
        },
        elements: {
            point: {
                radius: 0,
            },
            line: {
                tension: 0.1,
            },
        },
        scales: {
            x: {
                type: "linear" as const,
                min: -HALF_WINDOW,
                max: HALF_WINDOW,
                display: false,
            },
            y: {
                min: 0,
                max: 100,
                ticks: {
                    display: false,
                },
                border: {
                    display: false,
                    dash: [1, 6],
                },
                grid: {
                    display: false,
                    drawOnChartArea: false,
                    drawTicks: false,
                },
            },
        },
        maintainAspectRatio: false,
        animation: {
            duration: 0,
        },
        responsive: true,
        interaction: {
            intersect: false,
            mode: "index" as const,
        },
    };

    const telemetryData = {
        datasets: [
            {
                type: "line" as const,
                order: 2,
                borderColor: throttleColor,
                borderWidth: 2,
                data: throttleData,
            },
            {
                type: "line" as const,
                order: 1,
                borderColor: brakeColor,
                borderWidth: 2,
                data: brakeData,
            },
            {
                type: "line" as const,
                order: 5,
                borderColor: steeringAngleColor,
                borderWidth: 2,
                data: steeringAngleData,
            },
            {
                type: "line" as const,
                order: 4,
                borderColor: throttleReferenceColor,
                borderWidth: 2,
                data: currentReferenceThrottleData,
            },
            {
                type: "line" as const,
                order: 3,
                borderColor: brakeReferenceColor,
                borderWidth: 2,
                fill: true,
                backgroundColor: brakeReferenceColor,
                data: currentReferenceBrakeData,
            },
            {
                type: "line" as const,
                order: 6,
                borderColor: steeringAngleReferenceColor,
                borderWidth: 2,
                data: currentReferenceSteeringAngleData,
            },
        ],
    };

    // Initialize chart only once
    $effect(() => {
        if (telemetryCanvas && !chart) {
            const ctx = telemetryCanvas.getContext("2d")!;
            chart = new Chart(ctx, {
                data: telemetryData,
                options: telemetryOptions,
            });
        }
    });

    let throttle = $state(0);
    let brake = $state(0);
    let steering_angle = $state(50);
    let lap_dist = $state(0);

    let unsubscribe_telemetry: () => void = () => {};
    let unsubscribe_reference: () => void = () => {};

    onMount(async () => {
        unsubscribe_telemetry = telemetryReference.subscribe((data) => {
            lap_dist = data.lap_dist;
            throttle = data.throttle;
            brake = data.brake;
            // TODO: make configurable
            steering_angle = 50 + (50 * -data.steering_angle) / 170; // 0 is -1.7 radian, 50 is 0 radian, 100 is 1.7 radian; left is negative, right is positive
            if (steering_angle > 100) {
                steering_angle = 100;
            } else if (steering_angle < 0) {
                steering_angle = 0;
            }

            if (chart) {
                // Add new data point
                const newThrottlePoint = { x: lap_dist, y: throttle };
                const newBrakePoint = { x: lap_dist, y: brake };
                const newSteeringAnglePoint = {
                    x: lap_dist,
                    y: steering_angle,
                };

                // Keep only data points within the window
                let minDist = lap_dist - HALF_WINDOW;
                let maxDist = lap_dist + HALF_WINDOW;
                if (minDist < 0) {
                    minDist = 0;
                    maxDist = WINDOW_SIZE;
                }

                // Update datasets and filter in place
                throttleData.push(newThrottlePoint);
                brakeData.push(newBrakePoint);
                steeringAngleData.push(newSteeringAnglePoint);

                // Filter points outside the window in place
                let i = 0;
                while (i < throttleData.length) {
                    if (
                        throttleData[i].x < minDist ||
                        throttleData[i].x > maxDist
                    ) {
                        throttleData.splice(i, 1);
                        brakeData.splice(i, 1);
                        steeringAngleData.splice(i, 1);
                    } else {
                        i++;
                    }
                }

                let found_start = false;
                let found_end = false;
                for (let i = 0; i < referenceThrottleData.length; i++) {
                    if (!found_start && referenceThrottleData[i].x >= minDist) {
                        found_start = true;
                        currentReferenceStartIndex = i;
                    }
                    if (
                        found_start &&
                        !found_end &&
                        referenceThrottleData[i].x >= maxDist
                    ) {
                        found_end = true;
                        currentReferenceEndIndex = i - 1;
                        break;
                    }
                }

                currentReferenceThrottleData.splice(
                    0,
                    currentReferenceThrottleData.length,
                );
                currentReferenceBrakeData.splice(
                    0,
                    currentReferenceBrakeData.length,
                );
                currentReferenceSteeringAngleData.splice(
                    0,
                    currentReferenceSteeringAngleData.length,
                );

                for (
                    let i = currentReferenceStartIndex;
                    i <= currentReferenceEndIndex;
                    i++
                ) {
                    currentReferenceThrottleData.push(referenceThrottleData[i]);
                    currentReferenceBrakeData.push(referenceBrakeData[i]);
                    currentReferenceSteeringAngleData.push(
                        referenceSteeringAngleData[i],
                    );
                }

                // Update x-axis range to center on current position
                if (chart.options?.scales) {
                    chart.options.scales.x = {
                        ...chart.options.scales.x,
                        min: minDist,
                        max: maxDist,
                    };
                }

                // Force chart update
                chart.update("none");
            }
        });

        unsubscribe_reference = telemetryReferencePoints.subscribe((points) => {
            for (const point of points) {
                referenceThrottleData.push({
                    x: point.lap_dist,
                    y: point.throttle,
                });
                referenceBrakeData.push({
                    x: point.lap_dist,
                    y: point.brake,
                });
                referenceSteeringAngleData.push({
                    x: point.lap_dist,
                    y: 50 + (50 * -point.steering_angle) / 170,
                });
            }
        });
    });

    onDestroy(() => {
        unsubscribe_telemetry();
        unsubscribe_reference();
        if (chart) {
            chart.destroy();
        }
    });
</script>

<div class="join bg-primary-content w-full h-full">
    <div
        class="join-item flex flex-row items-center justify-center rounded-md w-full h-20"
    >
        <div class="flex flex-col items-end justify-evenly w-[96%] h-[90%]">
            <div class="flex flex-row w-[98%] h-[70%]">
                <canvas
                    bind:this={telemetryCanvas}
                    id="telemetry_reference_chart"
                ></canvas>
            </div>
            <div class="text-primary text-xs text-center w-full h-[15%]">
                {Math.round(lap_dist / 100)}m
            </div>
        </div>
    </div>
</div>

<style>
</style>
