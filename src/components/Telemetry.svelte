<script lang="ts">
    import ProgressBar from "./ProgressBar.svelte";
    import { Chart } from "chart.js/auto";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    let ctx;
    let telemeteryCanvas: HTMLCanvasElement;
    let chart: Chart;

    let throttle = 0;
    let brake = 0;

    let throttleData: number[] = [];
    let brakeData: number[] = [];

    const maxPoints = 300;

    let currentThrottlePoints = 0;
    let currentBrakePoints = 0;

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
                borderWidth: 1.5,
                data: throttleData,
            },
            {
                borderColor: brakeColor,
                borderWidth: 1.5,
                data: brakeData,
            },
        ],
    };

    onMount(async () => {
        ctx = telemeteryCanvas.getContext("2d")!;
        chart = new Chart(ctx, {
            type: "line",
            data: telemetryData,
            options: telemetryOptions,
        });
    });

    listen("throttle", (event) => {
        let payload = event.payload as { ts: number; value: number };
        throttle = payload.value;
        throttleData.push(throttle);
        currentThrottlePoints = throttleData.length;
        if (currentThrottlePoints > maxPoints) {
            throttleData.splice(0, currentThrottlePoints - maxPoints);
        }
        chart.update("none");
    });

    listen("brake", (event) => {
        let payload = event.payload as { ts: number; value: number };
        brake = payload.value;
        brakeData.push(brake);
        currentBrakePoints = brakeData.length;
        if (currentBrakePoints > maxPoints) {
            brakeData.splice(0, currentBrakePoints - maxPoints);
        }
        chart.update("none");
    });
</script>

<div class="flex flex-row items-center justify-center opacity-80">
    <div
        class="join w-[14%] outline-1 bg-accent-content outline-dotted outline-accent"
    >
        <div
            class="join-item flex flex-row items-center justify-center rounded-md w-[82%] h-20"
        >
            <div class="flex flex-col items-end justify-evenly w-[96%] h-[90%]">
                <div class="flex flex-row w-[98%] h-[70%]">
                    <canvas bind:this={telemeteryCanvas} id="telemetery_chart"
                    ></canvas>
                </div>
                <div class="flex flex-row w-[98%] h-[15%]"></div>
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
                    cls="flex flex-col flex-nowrap justify-end w-1.5 h-[70%] rounded-full overflow-hidden outline outline-2 outline-offset-2 outline-primary"
                    elem_cls="rounded-full overflow-hidden bg-error"
                ></ProgressBar>
                <div class="text-primary text-sm h-[15%]">{brake}</div>
            </div>
            <div
                class="flex flex-col items-center justify-evenly w-2/5 h-[90%]"
            >
                <ProgressBar
                    value={throttle}
                    cls="flex flex-col flex-nowrap justify-end w-1.5 h-[70%] rounded-full overflow-hidden outline outline-2 outline-offset-2 outline-primary"
                    elem_cls="rounded-full overflow-hidden bg-success"
                ></ProgressBar>
                <div class="text-primary text-sm h-[15%]">{throttle}</div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
