<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy } from "svelte";

    let gear = "N";
    let speed = 0;
    let rpm = 0;
    let gear_shift_rpm = 0;
    let gear_blink_rpm = 0;
    let lap = 0;
    let laps_total = 0;
    let position = 0;
    let positions_total = 0;
    let incidents = 0;
    let incident_limit = 0;

    let gear_indicator: HTMLDivElement;
    let rpm_incidator: HTMLProgressElement;

    let unlistens = [];

    unlistens.push(
        listen("gear", (event) => {
            gear = event.payload as string;
        }),
    );

    unlistens.push(
        listen("gear_shift_rpm", (event) => {
            gear_shift_rpm = event.payload as number;
        }),
    );

    unlistens.push(
        listen("gear_blink_rpm", (event) => {
            gear_blink_rpm = event.payload as number;
        }),
    );

    unlistens.push(
        listen("speed", (event) => {
            speed = event.payload as number;
        }),
    );

    unlistens.push(
        listen("rpm", (event) => {
            rpm = event.payload as number;

            if (rpm >= gear_blink_rpm) {
                gear_indicator.classList.remove("text-secondary");
                gear_indicator.classList.remove("text-info");
                gear_indicator.classList.add("text-error");

                rpm_incidator.classList.remove("progress-secondary");
                rpm_incidator.classList.remove("progress-info");
                rpm_incidator.classList.add("progress-error");
            } else if (rpm >= gear_shift_rpm) {
                gear_indicator.classList.remove("text-secondary");
                gear_indicator.classList.add("text-info");
                gear_indicator.classList.remove("text-error");

                rpm_incidator.classList.remove("progress-secondary");
                rpm_incidator.classList.add("progress-info");
                rpm_incidator.classList.remove("progress-error");
            } else {
                gear_indicator.classList.add("text-secondary");
                gear_indicator.classList.remove("text-info");
                gear_indicator.classList.remove("text-error");

                rpm_incidator.classList.add("progress-secondary");
                rpm_incidator.classList.remove("progress-info");
                rpm_incidator.classList.remove("progress-error");
            }
        }),
    );

    unlistens.push(
        listen("lap", (event) => {
            lap = event.payload as number;
        }),
    );

    unlistens.push(
        listen("laps_total", (event) => {
            laps_total = event.payload as number;
        }),
    );

    unlistens.push(
        listen("position", (event) => {
            position = event.payload as number;
        }),
    );

    unlistens.push(
        listen("positions_total", (event) => {
            positions_total = event.payload as number;
        }),
    );

    unlistens.push(
        listen("incidents", (event) => {
            incidents = event.payload as number;
        }),
    );

    unlistens.push(
        listen("incident_limit", (event) => {
            incident_limit = event.payload as number;
        }),
    );

    onDestroy(() => {
        unlistens.forEach(async (unlisten) => (await unlisten)());
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="join w-[17%] bg-primary-content">
        <div
            class="join-item flex flex-col items-center justify-center rounded-md w-1/4"
        >
            <div
                bind:this={gear_indicator}
                class="text-secondary text-5xl font-square"
            >
                {gear}
            </div>
            <div class="text-primary text-2xl">{speed}</div>
        </div>
        <div
            class="join-item flex flex-col items-center justify-evenly rounded-md w-3/4"
        >
            <progress
                bind:this={rpm_incidator}
                class="progress progress-secondary w-5/6 outline outline-2 outline-offset-2 outline-primary"
                value={rpm}
                max={gear_blink_rpm}
            ></progress>
            <div class="flex flex-row items-center justify-evenly">
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary">
                        {lap}{#if laps_total > 0}/{laps_total}{/if}
                    </div>
                    <div class="text-secondary text-xs">lap</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary">
                        {#if position > 0}
                            {position}{#if positions_total > 0}/{positions_total}{/if}
                        {:else}
                            -
                        {/if}
                    </div>
                    <div class="text-secondary text-xs">pos</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary">
                        {incidents}{#if incident_limit > 0}/{incident_limit}{/if}
                    </div>
                    <div class="text-secondary text-xs">inc</div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
