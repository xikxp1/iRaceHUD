<script lang="ts">
    import { onMount } from "svelte";
    import Application from "../../components/settings/Application.svelte";
    import LapTimes from "../../components/settings/overlays/LapTimes.svelte";
    import Main from "../../components/settings/overlays/Main.svelte";
    import Proximity from "../../components/settings/overlays/Proximity.svelte";
    import Relative from "../../components/settings/overlays/Relative.svelte";
    import Standings from "../../components/settings/overlays/Standings.svelte";
    import SubTimer from "../../components/settings/overlays/SubTimer.svelte";
    import Telemetry from "../../components/settings/overlays/Telemetry.svelte";
    import TelemetryReference from "../../components/settings/overlays/TelemetryReference.svelte";
    import Timer from "../../components/settings/overlays/Timer.svelte";
    import TrackMap from "../../components/settings/overlays/TrackMap.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";

    let chosenPage = $state("application");
    let appVersion = $state("");

    function setChosenPage(page: string) {
        chosenPage = page;
    }

    onMount(async () => {
        appVersion = await invoke("get_app_version");
    });
</script>

<div class="drawer drawer-open">
    <input id="my-drawer" type="checkbox" class="drawer-toggle" />
    <div class="drawer-side">
        <div class="bg-secondary min-h-full w-64">
            <button
                type="button"
                class="btn btn-ghost p-0 w-full h-auto"
                onclick={() => openUrl("https://github.com/xikxp1/iRaceHUD")}
            >
                <img src="/icons/logo.svg" alt="" class="w-full" />
            </button>
            <p class="text-sm text-primary-content text-right mr-2">
                v{appVersion}
            </p>
            <ul class="menu text-primary-content">
                <li>
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage ===
                        'application'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("application")}
                        >Application</button
                    >
                </li>
                <div class="divider h-1"></div>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage === 'main'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("main")}
                        >Main Overlay</button
                    >
                </li>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage ===
                        'telemetry'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("telemetry")}
                        >Telemetry Overlay</button
                    >
                </li>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage ===
                        'telemetry_reference'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("telemetry_reference")}
                        >Telemetry Reference Overlay</button
                    >
                </li>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage ===
                        'standings'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("standings")}
                        >Standings Overlay</button
                    >
                </li>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage === 'relative'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("relative")}
                        >Relative Overlay</button
                    >
                </li>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage ===
                        'track_map'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("track_map")}
                        >Track Map Overlay</button
                    >
                </li>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage ===
                        'proximity'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("proximity")}
                        >Proximity Overlay</button
                    >
                </li>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage === 'timer'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("timer")}
                        >Timer Overlay</button
                    >
                </li>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage === 'subtimer'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("subtimer")}
                        >Subtimer Overlay</button
                    >
                </li>
                <li class="mb-2">
                    <button
                        type="button"
                        class="btn btn-sm btn-outline {chosenPage ===
                        'lap_times'
                            ? 'btn-active'
                            : ''}"
                        onclick={() => setChosenPage("lap_times")}
                        >Lap Times Overlay</button
                    >
                </li>
            </ul>
        </div>
    </div>
    <div class="drawer-content">
        {#if chosenPage === "application"}
            <Application />
        {:else if chosenPage === "main"}
            <Main />
        {:else if chosenPage === "telemetry"}
            <Telemetry />
        {:else if chosenPage === "telemetry_reference"}
            <TelemetryReference />
        {:else if chosenPage === "standings"}
            <Standings />
        {:else if chosenPage === "relative"}
            <Relative />
        {:else if chosenPage === "track_map"}
            <TrackMap />
        {:else if chosenPage === "proximity"}
            <Proximity />
        {:else if chosenPage === "timer"}
            <Timer />
        {:else if chosenPage === "subtimer"}
            <SubTimer />
        {:else if chosenPage === "lap_times"}
            <LapTimes />
        {/if}
    </div>
</div>
