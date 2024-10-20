<script lang="ts">
    import type {
        Gear,
        GearRPM,
        Incidents,
        Laps,
        Position,
        RPM,
        Speed,
    } from "$lib/types/telemetry";
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy } from "svelte";

    let gear: Gear = "N";
    let speed: Speed = 0;
    let rpm: RPM = 0;
    let gear_shift_rpm: GearRPM = 0;
    let gear_blink_rpm: GearRPM = 0;
    let lap: Laps = 0;
    let laps_total: Laps = 0;
    let position: Position = 0;
    let positions_total: Position = 0;
    let incidents: Incidents = 0;
    let incident_limit: Incidents = 0;

    let gear_indicator: HTMLDivElement;
    let rpm_incidator: HTMLProgressElement;

    let unlistens = [];

    unlistens.push(
        listen("gear", (event) => {
            gear = event.payload as Gear;
        }),
    );

    unlistens.push(
        listen("gear_shift_rpm", (event) => {
            gear_shift_rpm = event.payload as GearRPM;
        }),
    );

    unlistens.push(
        listen("gear_blink_rpm", (event) => {
            gear_blink_rpm = event.payload as GearRPM;
        }),
    );

    unlistens.push(
        listen("speed", (event) => {
            speed = event.payload as Speed;
        }),
    );

    unlistens.push(
        listen("rpm", (event) => {
            rpm = event.payload as RPM;

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
            lap = event.payload as Laps;
        }),
    );

    unlistens.push(
        listen("laps_total", (event) => {
            laps_total = event.payload as Laps;
        }),
    );

    unlistens.push(
        listen("position", (event) => {
            position = event.payload as Position;
        }),
    );

    unlistens.push(
        listen("positions_total", (event) => {
            positions_total = event.payload as Position;
        }),
    );

    unlistens.push(
        listen("incidents", (event) => {
            incidents = event.payload as Incidents;
        }),
    );

    unlistens.push(
        listen("incident_limit", (event) => {
            incident_limit = event.payload as Incidents;
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
