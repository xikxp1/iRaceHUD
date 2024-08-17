<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let lap_time: string = "0:00.000";

    listen("lap_time", (event) => {
        lap_time = event.payload as string;
    });
</script>

<div class="flex flex-row items-center justify-center opacity-80">
    <div
        class="flex flex-col bg-primary-content items-center justify-center rounded-md w-[10%]"
    >
        <div class="text-primary text-3xl font-square">{lap_time}</div>
    </div>
</div>

<style>
</style>
