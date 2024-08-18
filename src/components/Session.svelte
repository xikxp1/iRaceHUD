<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let stength_of_field = 0;
    let race_laps = 0;
    let laps_total = 0;
    let session_time = "––:––:––";
    let session_time_total = "";
    let current_time = "––:––";

    listen("strength_of_field", (event) => {
        stength_of_field = event.payload as number;
    });

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
        class="flex flex-row w-[400px] items-center justify-center rounded-md bg-primary-content"
    >
        <div class="flex flex-col w-[40px] items-center justify-center">
            <div class="text-primary">
                {stength_of_field}
            </div>
            <div class="text-secondary text-xs">SoF</div>
        </div>
        <div class="divider divider-horizontal divider-primary w-[2px]"></div>
        <div class="flex flex-col w-[60px] items-center justify-center">
            <div class="text-primary">
                {race_laps}{#if laps_total > 0}/{laps_total}{/if}
            </div>
            <div class="text-secondary text-xs">race lap</div>
        </div>
        <div class="divider divider-horizontal divider-primary w-[2px]"></div>
        <div class="flex flex-col w-[120px] items-center justify-center">
            <div class="text-primary">
                {session_time}
            </div>
            {#if laps_total == 0 && session_time_total != ""}
                <div class="text-secondary text-xs">
                    of {session_time_total} session time
                </div>
            {:else}
                <div class="text-secondary text-xs">session time</div>
            {/if}
        </div>
        <div class="divider divider-horizontal divider-primary w-[2px]"></div>
        <div class="flex flex-col w-[40px] items-center justify-center">
            <div class="text-primary">
                {current_time}
            </div>
            <div class="text-secondary text-xs">time</div>
        </div>
    </div>
</div>

<style>
</style>
