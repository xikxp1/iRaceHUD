<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let session_state = "";

    listen("session_state", (event) => {
        session_state = event.payload as string;
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div
        class="flex flex-col bg-primary-content items-center justify-center rounded-md w-[7%]"
    >
        <div class="text-primary text-xl">{session_state}</div>
    </div>
</div>

<style>
</style>
