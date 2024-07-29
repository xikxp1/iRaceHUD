<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

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

    let rpm_incidator: HTMLProgressElement;

    listen("gear", (event) => {
        gear = event.payload as string;
    });

    listen("gear_shift_rpm", (event) => {
        gear_shift_rpm = event.payload as number;
    });

    listen("gear_blink_rpm", (event) => {
        gear_blink_rpm = event.payload as number;
    });

    listen("speed", (event) => {
        speed = event.payload as number;
    });

    listen("rpm", (event) => {
        rpm = event.payload as number;

        if (rpm >= gear_blink_rpm) {
            rpm_incidator.classList.remove("progress-accent");
            rpm_incidator.classList.remove("progress-info");
            rpm_incidator.classList.add("progress-error");
        } else if (rpm >= gear_shift_rpm) {
            rpm_incidator.classList.remove("progress-accent");
            rpm_incidator.classList.add("progress-info");
            rpm_incidator.classList.remove("progress-error");
        } else {
            rpm_incidator.classList.add("progress-accent");
            rpm_incidator.classList.remove("progress-info");
            rpm_incidator.classList.remove("progress-error");
        }
    });

    listen("lap", (event) => {
        lap = event.payload as number;
    });

    listen("laps_total", (event) => {
        laps_total = event.payload as number;
    });

    listen("position", (event) => {
        position = event.payload as number;
    });

    listen("positions_total", (event) => {
        positions_total = event.payload as number;
    });

    listen("incidents", (event) => {
        incidents = event.payload as number;
    });

    listen("incident_limit", (event) => {
        incident_limit = event.payload as number;
    });
</script>

<div class="flex flex-row items-center justify-center opacity-80">
    <div
        class="join w-[17%] bg-accent-content outline-1 outline-dotted outline-accent"
    >
        <div
            class="join-item flex flex-col items-center justify-center rounded-md w-1/4"
        >
            <div class="text-primary text-6xl">{gear}</div>
            <div class="text-primary text-2xl">{speed}</div>
        </div>
        <div
            class="join-item flex flex-col items-center justify-evenly rounded-md w-3/4"
        >
            <progress bind:this={rpm_incidator}
                class="progress progress-accent w-5/6 outline outline-2 outline-offset-2 outline-primary"
                value={rpm}
                max={gear_blink_rpm}
            ></progress>
            <div class="flex flex-row items-center justify-evenly">
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary text-xl">
                        {lap}{#if laps_total > 0}/{laps_total}{/if}
                    </div>
                    <div class="text-accent text-xs">lap</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary text-xl">
                        {#if position > 0}
                            {position}{#if positions_total > 0}/{positions_total}{/if}
                        {:else}
                            -
                        {/if}
                    </div>
                    <div class="text-accent text-xs">pos</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary text-xl">
                        {incidents}{#if incident_limit > 0}/{incident_limit}{/if}
                    </div>
                    <div class="text-accent text-xs">inc</div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
