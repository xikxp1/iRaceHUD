<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let gear = "N";
    let speed = 0;
    let rpm = 0;
    let lap = 0;
    let position = 1;
    let incidents = 0;

    listen("gear", (event) => {
        gear = event.payload as string;
    });

    listen("speed", (event) => {
        speed = event.payload as number;
    });

    listen("rpm", (event) => {
        rpm = event.payload as number;
    });

    listen("lap", (event) => {
        lap = event.payload as number;
    });

    listen("position", (event) => {
        position = event.payload as number;
    });

    listen("incidents", (event) => {
        incidents = event.payload as number;
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
            <progress
                class="progress progress-accent w-5/6 outline outline-2 outline-offset-2 outline-primary"
                value={rpm}
                max=10000
            ></progress>
            <div class="flex flex-row items-center justify-evenly">
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary text-xl">
                        {lap}/7
                    </div>
                    <div class="text-accent text-xs">lap</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary text-xl">
                        {position}/15
                    </div>
                    <div class="text-accent text-xs">pos</div>
                </div>
                <div class="divider divider-horizontal divider-primary"></div>
                <div class="flex flex-col items-center justify-evenly">
                    <div class="text-primary text-xl">
                        {incidents}/17
                    </div>
                    <div class="text-accent text-xs">inc</div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
