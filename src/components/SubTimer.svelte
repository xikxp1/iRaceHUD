<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let session_state = "";
    let gap_next = "";
    let gap_prev = "";

    listen("session_state", (event) => {
        session_state = event.payload as string;
    });

    listen("gap_next", (event) => {
        gap_next = event.payload as string;
    });

    listen("gap_prev", (event) => {
        gap_prev = event.payload as string;
    });
</script>

<div class="flex flex-row items-center justify-center opacity-75">
    <div class="join flex flex-row bg-primary-content rounded-md">
        <div class="join-item flex flex-col items-end justify-center w-[50px]">
            <div class="text-primary text-xl">{gap_next}</div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-center justify-center w-[100px]"
        >
            <div class="text-primary text-xl">{session_state}</div>
        </div>
        <div
            class="join-item divider divider-horizontal divider-primary w-[2px]"
        ></div>
        <div
            class="join-item flex flex-col items-start justify-center w-[50px]"
        >
            <div class="text-primary text-xl">{gap_prev}</div>
        </div>
    </div>
</div>

<style>
</style>
