<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let to_go = "";

    listen("to_go", (event) => {
        to_go = event.payload as string;
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div
        class="flex flex-col bg-primary-content items-center justify-center rounded-md w-[7%]"
    >
        <div class="text-primary text-xl">{to_go}</div>
    </div>
</div>

<style>
</style>
