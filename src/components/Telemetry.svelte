<script lang="ts">
    import ProgressBar from "./ProgressBar.svelte";
    import { Line } from "svelte-chartjs";
    import "chart.js/auto";

    const throttleData = [
        0, 1, 2, 3, 4, 5, 7, 9, 11, 13, 15, 17, 18, 18, 20, 21, 23, 23, 23, 25,
        23, 24, 26, 25, 27, 29, 30, 31, 33, 32, 34, 36, 38, 40, 41, 42, 42, 44,
        42, 46, 48, 50, 53, 52, 54, 55, 57, 58, 60, 63, 67, 65, 68, 69, 70, 72,
        76, 75, 77, 79, 75, 75, 74, 77, 79, 82, 80, 80, 83, 84, 84, 84, 84, 84,
        83, 82, 81, 80, 84, 85, 87, 88, 89, 91, 95, 96, 95, 98, 100, 100, 100,
        100, 100, 100, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 97, 95, 94, 91, 90, 87, 91, 95, 98, 100, 100, 96, 95, 93, 90,
        91, 85, 82, 77, 73, 71, 70, 65, 50, 44, 42, 36, 28, 28, 17, 16, 5, 4, 3,
        2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 7, 5, 10, 12, 14, 10, 10, 10, 10,
        12, 18, 23, 22, 26, 28, 30, 24, 20, 14, 18, 17, 16, 15, 15, 13, 18, 25,
        29, 34, 40, 45, 46, 44, 48, 56, 58, 59, 60, 62, 66, 75, 72, 78, 75, 84,
        80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70, 69, 68, 67, 66, 65, 64, 63,
        62, 61, 60, 59, 64, 65, 62, 64, 62, 55, 52, 47, 44, 48, 52, 53, 46, 45,
        44, 43, 42, 41, 40, 42, 45, 45, 48, 48, 52, 51, 54, 57, 59, 60, 62, 66,
        69, 72, 75, 78, 80, 82, 83, 85, 87, 86,
    ];

    const css = window.getComputedStyle(document.documentElement);

    const throttleColor: string = `oklch(${css.getPropertyValue("--su")})`;

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
        ],
    };
</script>

<div class="flex flex-row items-center justify-center opacity-80">
    <div class="join w-[14%] outline-1 bg-accent-content outline-dotted outline-accent">
        <div
            class="join-item flex flex-row items-center justify-center rounded-md w-[82%] h-20"
        >
            <div class="flex flex-col items-end justify-evenly w-[96%] h-[90%]">
                <div class="flex flex-row w-[98%] h-[70%]">
                    <Line options={telemetryOptions} data={telemetryData}
                    ></Line>
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
                    value={42}
                    cls="flex flex-col flex-nowrap justify-end w-1.5 h-[70%] rounded-full overflow-hidden outline outline-2 outline-offset-2 outline-primary"
                    elem_cls="rounded-full overflow-hidden bg-error"
                ></ProgressBar>
                <div class="text-primary text-sm h-[15%]">42</div>
            </div>
            <div
                class="flex flex-col items-center justify-evenly w-2/5 h-[90%]"
            >
                <ProgressBar
                    value={86}
                    cls="flex flex-col flex-nowrap justify-end w-1.5 h-[70%] rounded-full overflow-hidden outline outline-2 outline-offset-2 outline-primary"
                    elem_cls="rounded-full overflow-hidden bg-success"
                ></ProgressBar>
                <div class="text-primary text-sm h-[15%]">86</div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
