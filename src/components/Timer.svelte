<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let lap_time = "–:––.–––";
    let delta_last_time = "–";
    let delta_optimal_time = "–";

    listen("lap_time", (event) => {
        lap_time = event.payload as string;
    });

    listen("delta_last_time", (event) => {
        delta_last_time = event.payload as string;
    });

    listen("delta_optimal_time", (event) => {
        delta_optimal_time = event.payload as string;
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="join flex flex-row bg-primary-content rounded-md">
        <div class="join-item flex flex-col items-end justify-center w-[75px]">
            <div class="text-primary text-xl font-square">
                {delta_optimal_time}
            </div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-center justify-center w-[160px]"
        >
            <div class="text-primary text-3xl font-square">{lap_time}</div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-start justify-center w-[75px]"
        >
            <div class="text-primary text-xl font-square">
                {delta_last_time}
            </div>
        </div>
    </div>
</div>

<style>
</style>
