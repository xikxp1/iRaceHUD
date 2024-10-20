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
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";

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

    let gear_channel = new Channel<Gear>();
    let speed_channel = new Channel<Speed>();
    let rpm_channel = new Channel<RPM>();
    let gear_shift_rpm_channel = new Channel<GearRPM>();
    let gear_blink_rpm_channel = new Channel<GearRPM>();
    let lap_channel = new Channel<Laps>();
    let laps_total_channel = new Channel<Laps>();
    let position_channel = new Channel<Position>();
    let positions_total_channel = new Channel<Position>();
    let incidents_channel = new Channel<Incidents>();
    let incident_limit_channel = new Channel<Incidents>();

    onMount(() => {
        gear_channel.onmessage = (message) => {
            gear = message;
        };

        speed_channel.onmessage = (message) => {
            speed = message;
        };

        rpm_channel.onmessage = (message) => {
            rpm = message;

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
        };

        gear_shift_rpm_channel.onmessage = (message) => {
            gear_shift_rpm = message;
        };

        gear_blink_rpm_channel.onmessage = (message) => {
            gear_blink_rpm = message;
        };

        lap_channel.onmessage = (message) => {
            lap = message;
        };

        laps_total_channel.onmessage = (message) => {
            laps_total = message;
        };

        position_channel.onmessage = (message) => {
            position = message;
        };

        positions_total_channel.onmessage = (message) => {
            positions_total = message;
        };

        incidents_channel.onmessage = (message) => {
            incidents = message;
        };

        incident_limit_channel.onmessage = (message) => {
            incident_limit = message;
        };

        invoke("register_event_emitter", {
            event: "gear",
            onEvent: gear_channel,
        });

        invoke("register_event_emitter", {
            event: "speed",
            onEvent: speed_channel,
        });

        invoke("register_event_emitter", {
            event: "rpm",
            onEvent: rpm_channel,
        });

        invoke("register_event_emitter", {
            event: "gear_shift_rpm",
            onEvent: gear_shift_rpm_channel,
        });

        invoke("register_event_emitter", {
            event: "gear_blink_rpm",
            onEvent: gear_blink_rpm_channel,
        });

        invoke("register_event_emitter", {
            event: "lap",
            onEvent: lap_channel,
        });

        invoke("register_event_emitter", {
            event: "laps_total",
            onEvent: laps_total_channel,
        });

        invoke("register_event_emitter", {
            event: "position",
            onEvent: position_channel,
        });

        invoke("register_event_emitter", {
            event: "positions_total",
            onEvent: positions_total_channel,
        });

        invoke("register_event_emitter", {
            event: "incidents",
            onEvent: incidents_channel,
        });

        invoke("register_event_emitter", {
            event: "incident_limit",
            onEvent: incident_limit_channel,
        });
    });

    onDestroy(() => {
        gear_channel.onmessage = () => {};
        speed_channel.onmessage = () => {};
        rpm_channel.onmessage = () => {};
        gear_shift_rpm_channel.onmessage = () => {};
        gear_blink_rpm_channel.onmessage = () => {};
        lap_channel.onmessage = () => {};
        laps_total_channel.onmessage = () => {};
        position_channel.onmessage = () => {};
        positions_total_channel.onmessage = () => {};
        incidents_channel.onmessage = () => {};
        incident_limit_channel.onmessage = () => {};

        invoke("unregister_event_emitter", {
            event: "gear",
        });

        invoke("unregister_event_emitter", {
            event: "speed",
        });

        invoke("unregister_event_emitter", {
            event: "rpm",
        });

        invoke("unregister_event_emitter", {
            event: "gear_shift_rpm",
        });

        invoke("unregister_event_emitter", {
            event: "gear_blink_rpm",
        });

        invoke("unregister_event_emitter", {
            event: "lap",
        });

        invoke("unregister_event_emitter", {
            event: "laps_total",
        });

        invoke("unregister_event_emitter", {
            event: "position",
        });

        invoke("unregister_event_emitter", {
            event: "positions_total",
        });

        invoke("unregister_event_emitter", {
            event: "incidents",
        });

        invoke("unregister_event_emitter", {
            event: "incident_limit",
        });
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
