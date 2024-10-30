<script lang="ts">
    import type { Active } from "$lib/types/telemetry";
    import Main from "../components/Main.svelte";
    import Telemetry from "../components/Telemetry.svelte";
    import Timer from "../components/Timer.svelte";
    import Subtimer from "../components/SubTimer.svelte";
    import Proximity from "../components/Proximity.svelte";
    import Standings from "../components/Standings.svelte";
    import TrackMap from "../components/TrackMap.svelte";
    import LapTimes from "../components/LapTimes.svelte";
    import Relative from "../components/Relative.svelte";
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";

    let active: boolean = false;

    let channel = new Channel<Active>();

    onMount(() => {
        channel.onmessage = (message) => {
            active = message;
        };

        invoke("register_event_emitter", {
            event: "active",
            onEvent: channel,
        });
    });

    onDestroy(() => {
        channel.onmessage = () => {};

        invoke("unregister_event_emitter", {
            event: "active",
        });
    });
</script>

{#if active}
    <div class="outer">
        <div class="middle">
            <div class="main">
                <Main />
            </div>
        </div>
    </div>
    <div class="outer">
        <div class="middle">
            <div class="telemetry">
                <Telemetry />
            </div>
        </div>
    </div>
    <div class="outer">
        <div class="middle">
            <div class="timer">
                <Timer />
            </div>
        </div>
    </div>
    <div class="outer">
        <div class="middle">
            <div class="subtimer">
                <Subtimer />
            </div>
        </div>
    </div>
    <div class="outer">
        <div class="middle">
            <div class="proximity">
                <Proximity />
            </div>
        </div>
    </div>
    <div class="outer">
        <div class="middle">
            <div class="standings">
                <Standings />
            </div>
        </div>
    </div>
    <div class="outer">
        <div class="middle">
            <div class="trackmap">
                <TrackMap />
            </div>
        </div>
    </div>
    <div class="outer">
        <div class="middle">
            <div class="laptimes">
                <LapTimes />
            </div>
        </div>
    </div>
    <div class="outer">
        <div class="middle">
            <div class="relative">
                <Relative />
            </div>
        </div>
    </div>
{/if}

<style>
    .outer {
        display: table;
        position: absolute;
        top: 0;
        left: 0;
        height: 100%;
        width: 100%;
    }

    .middle {
        display: table-cell;
        vertical-align: middle;
    }

    .main {
        margin-left: auto;
        margin-right: auto;
        margin-top: 280px;
        margin-bottom: auto;
        align-items: center;
    }

    .telemetry {
        margin-left: auto;
        margin-right: auto;
        margin-top: 445px;
        margin-bottom: auto;
        align-items: center;
    }

    .timer {
        margin-left: auto;
        margin-right: auto;
        margin-top: -335px;
        margin-bottom: auto;
        align-items: center;
    }

    .subtimer {
        margin-left: auto;
        margin-right: auto;
        margin-top: -295px;
        margin-bottom: auto;
        align-items: center;
    }

    .proximity {
        margin-left: auto;
        margin-right: auto;
        margin-top: -135px;
        margin-bottom: auto;
        align-items: center;
    }

    .standings {
        margin-left: -1300px;
        margin-right: auto;
        margin-top: -520px;
        margin-bottom: auto;
        align-items: center;
    }

    .trackmap {
        margin-left: 1300px;
        margin-right: auto;
        margin-top: -550px;
        margin-bottom: auto;
        align-items: center;
    }

    .laptimes {
        margin-left: 1800px;
        margin-right: auto;
        margin-top: auto;
        margin-bottom: auto;
        align-items: center;
    }

    .relative {
        margin-left: 1500px;
        margin-right: auto;
        margin-top: 700px;
        margin-bottom: auto;
        align-items: center;
    }
</style>
