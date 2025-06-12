<script lang="ts">
    import { Chart } from "chart.js/auto";
    import { onDestroy, onMount } from "svelte";
    import { telemetryReference } from "$lib/backend/telemetry.svelte";
    import type { TelemetryReferenceOverlaySettings } from "$lib/types/telemetry";

    let { settings }: { settings: TelemetryReferenceOverlaySettings } =
        $props();

    let telemetryCanvas: HTMLCanvasElement | undefined = $state();
    let chart: Chart | undefined = $state();

    const WINDOW_SIZE = 300 * 100; // 300m
    const STEP_SIZE = 20;
    const HALF_WINDOW = WINDOW_SIZE / 2;
    const throttleData = [] as { x: number; y: number }[];
    const brakeData = [] as { x: number; y: number }[];

    const css = window.getComputedStyle(document.documentElement);
    const throttleColor: string = `oklch(${css.getPropertyValue("--su")})`;
    const brakeColor: string = `oklch(${css.getPropertyValue("--er")})`;

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
                borderColor: throttleColor,
                borderWidth: 2,
                data: throttleData,
            },
            {
                borderColor: brakeColor,
                borderWidth: 2,
                data: brakeData,
            },
        ],
    };

    // Initialize chart only once
    $effect(() => {
        if (telemetryCanvas && !chart) {
            const ctx = telemetryCanvas.getContext("2d")!;
            chart = new Chart(ctx, {
                type: "line",
                data: telemetryData,
                options: telemetryOptions,
            });
        }
    });

    // Data storage for telemetry relative to lap distance
    const telemetryPoints: {
        lap_dist: number;
        throttle: number;
        brake: number;
    }[] = [];

    // Use regular variables for values that don't need reactivity
    let throttle = $state(0);
    let brake = $state(0);
    let lap_dist = $state(0);

    let unsubscribe_telemetry: () => void = () => {};

    onMount(async () => {
        unsubscribe_telemetry = telemetryReference.subscribe((data) => {
            lap_dist = data.lap_dist;
            throttle = data.throttle;
            brake = data.brake;

            if (chart) {
                // Add new data point
                const newThrottlePoint = { x: lap_dist, y: throttle };
                const newBrakePoint = { x: lap_dist, y: brake };

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

                // Filter points outside the window in place
                let i = 0;
                while (i < throttleData.length) {
                    if (throttleData[i].x < minDist || throttleData[i].x > maxDist) {
                        throttleData.splice(i, 1);
                    } else {
                        i++;
                    }
                }

                i = 0;
                while (i < brakeData.length) {
                    if (brakeData[i].x < minDist || brakeData[i].x > maxDist) {
                        brakeData.splice(i, 1);
                    } else {
                        i++;
                    }
                }

                // Limit the number of points to prevent performance issues
                const maxPoints = WINDOW_SIZE / STEP_SIZE;
                if (throttleData.length > maxPoints) {
                    throttleData.splice(0, throttleData.length - maxPoints);
                }
                if (brakeData.length > maxPoints) {
                    brakeData.splice(0, brakeData.length - maxPoints);
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
    });

    onDestroy(() => {
        unsubscribe_telemetry();
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
