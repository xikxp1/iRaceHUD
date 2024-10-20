<script lang="ts">
    import type { Gap, SessionState } from "$lib/types/telemetry";
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy } from "svelte";

    let session_state: SessionState = "";
    let gap_next: Gap = "";
    let gap_prev: Gap = "";

    let unlistens = [];

    unlistens.push(
        listen("session_state", (event) => {
            session_state = event.payload as SessionState;
        }),
    );

    unlistens.push(
        listen("gap_next", (event) => {
            gap_next = event.payload as Gap;
        }),
    );

    unlistens.push(
        listen("gap_prev", (event) => {
            gap_prev = event.payload as Gap;
        }),
    );

    onDestroy(() => {
        unlistens.forEach(async (unlisten) => (await unlisten)());
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
