<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let race_laps = 0;
    let laps_total = 0;
    let session_time = "00:00:00";
    let session_time_total = "00:00:00";
    let current_time = "00:00";

    listen("race_laps", (event) => {
        race_laps = event.payload as number;
    });

    listen("laps_total", (event) => {
        laps_total = event.payload as number;
    });

    listen("session_time", (event) => {
        session_time = event.payload as string;
    });

    listen("session_time_total", (event) => {
        session_time_total = event.payload as string;
    });

    listen("current_time", (event) => {
        current_time = event.payload as string;
    });
</script>

<div class="flex flex-row items-center justify-center opacity-80">
    <div
        class="flex flex-row w-[350px] items-center justify-center rounded-md bg-accent-content outline-1 outline-dotted outline-accent"
    >
        <div class="flex flex-col w-[40px] items-center justify-center">
            <div class="text-primary text-xl">
                {race_laps}{#if laps_total > 0}/{laps_total}{/if}
            </div>
            <div class="text-accent text-xs">race lap</div>
        </div>
        <div class="divider divider-horizontal divider-primary"></div>
        <div class="flex flex-col w-[140px] items-center justify-center">
            <div class="text-primary text-xl">
                {session_time}{#if session_time_total != "00:00"}/{session_time_total}{/if}
            </div>
            <div class="text-accent text-xs">session time</div>
        </div>
        <div class="divider divider-horizontal divider-primary"></div>
        <div class="flex flex-col w-[45px] items-center justify-center">
            <div class="text-primary text-xl">
                {current_time}
            </div>
            <div class="text-accent text-xs">time</div>
        </div>
    </div>
</div>

<style>
</style>
