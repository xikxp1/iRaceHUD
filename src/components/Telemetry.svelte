<script lang="ts">
    import { Chart } from "chart.js/auto";
    import { onMount } from "svelte";
    import ProgressBar from "./utils/ProgressBar.svelte";
    import { telemetry } from "$lib/telemetry/telemetry.svelte";
    let telemetryCanvas: HTMLCanvasElement | undefined = $state();
    let chart: Chart | undefined = $state();

    $effect(() => {
        if (telemetryCanvas != undefined) {
            let ctx = telemetryCanvas.getContext("2d")!;
            chart = new Chart(ctx, {
                type: "line",
                data: telemetryData,
                options: telemetryOptions,
            });
        }
    });

    let throttle = $state(0);
    let brake = $state(0);
    let abs = $state(false);

    const maxPoints = 300;

    let throttleData = new Array(maxPoints).fill(0);
    let brakeData = new Array(maxPoints).fill(0);

    const css = window.getComputedStyle(document.documentElement);

    const throttleColor: string = `oklch(${css.getPropertyValue("--su")})`;
    const brakeColor: string = `oklch(${css.getPropertyValue("--er")})`;
    const gridColor: string = `oklch(${css.getPropertyValue("--p")} / 0.8)`;

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
                    color: gridColor,
                    display: true,
                    drawOnChartArea: true,
                    drawTicks: false,
                },
            },
        },
        maintainAspectRatio: false,
    };

    const telemetryData = {
        labels: throttleData,
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

    onMount(async () => {
        telemetry.subscribe((data) => {
            throttle = data.throttle;
            brake = data.brake;
            abs = data.abs_active;
            throttleData.push(throttle);
            brakeData.push(brake);
            let currentThrottlePoints = throttleData.length;
            let currentBrakePoints = brakeData.length;
            if (currentThrottlePoints > maxPoints) {
                throttleData.splice(0, currentThrottlePoints - maxPoints);
            }
            if (currentBrakePoints > maxPoints) {
                brakeData.splice(0, currentBrakePoints - maxPoints);
            }
            if (chart != undefined) {
                chart.update("none");
            }
        });
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div
        class="join w-[14%] bg-primary-content {abs
            ? 'outline outline-2 outline-secondary'
            : ''}"
    >
        <div
            class="join-item flex flex-row items-center justify-center rounded-md w-[82%] h-20"
        >
            <div class="flex flex-col items-end justify-evenly w-[96%] h-[90%]">
                <div class="flex flex-row w-[98%] h-[70%]">
                    <canvas bind:this={telemetryCanvas} id="telemetry_chart"
                    ></canvas>
                </div>
                <div
                    class="{abs
                        ? 'text-secondary'
                        : 'text-neutral-600'} text-xs text-right w-[98%] h-[15%]"
                >
                    ABS
                </div>
            </div>
        </div>
        <div
            class="join-item flex flex-rowitems-center justify-evenly rounded-md w-[18%] h-20"
        >
            <div
                class="flex flex-col items-center justify-evenly w-2/5 h-[90%]"
            >
                <ProgressBar
                    value={brake}
                    cls="flex flex-col flex-nowrap justify-end w-1.5 h-[70%] rounded-full overflow-hidden outline outline-2 outline-offset-2 outline-primary mt-[8px] mb-[4px]"
                    elem_cls="rounded-full overflow-hidden bg-error"
                ></ProgressBar>
                <div
                    class="{abs
                        ? 'text-secondary'
                        : 'text-primary'} text-xs h-[15%]"
                >
                    {brake}
                </div>
            </div>
            <div
                class="flex flex-col items-center justify-evenly w-2/5 h-[90%]"
            >
                <ProgressBar
                    value={throttle}
                    cls="flex flex-col flex-nowrap justify-end w-1.5 h-[70%] rounded-full overflow-hidden outline outline-2 outline-offset-2 outline-primary mt-[8px] mb-[4px]"
                    elem_cls="rounded-full overflow-hidden bg-success"
                ></ProgressBar>
                <div class="text-primary text-xs h-[15%]">{throttle}</div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
