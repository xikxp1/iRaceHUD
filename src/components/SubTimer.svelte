<script lang="ts">
    import type { Gap, SessionState } from "$lib/types/telemetry";
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { onMount, onDestroy } from "svelte";

    let session_state: SessionState = "";
    let gap_next: Gap = "";
    let gap_prev: Gap = "";

    let session_state_channel = new Channel<SessionState>();
    let gap_next_channel = new Channel<Gap>();
    let gap_prev_channel = new Channel<Gap>();

    onMount(() => {
        session_state_channel.onmessage = (message) => {
            session_state = message;
        };

        gap_next_channel.onmessage = (message) => {
            gap_next = message;
        };

        gap_prev_channel.onmessage = (message) => {
            gap_prev = message;
        };

        invoke("register_event_emitter", {
            event: "session_state",
            onEvent: session_state_channel,
        });

        invoke("register_event_emitter", {
            event: "gap_next",
            onEvent: gap_next_channel,
        });

        invoke("register_event_emitter", {
            event: "gap_prev",
            onEvent: gap_prev_channel,
        });
    });

    onDestroy(() => {
        session_state_channel.onmessage = () => {};
        gap_next_channel.onmessage = () => {};
        gap_prev_channel.onmessage = () => {};

        invoke("unregister_event_emitter", {
            event: "session_state",
        });

        invoke("unregister_event_emitter", {
            event: "gap_next",
        });

        invoke("unregister_event_emitter", {
            event: "gap_prev",
        });
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
