<script lang="ts">
    import {
        mainOverlaySettings,
        timerOverlaySettings,
        subtimerOverlaySettings,
        standingsOverlaySettings,
        lapTimesOverlaySettings,
        proximityOverlaySettings,
        relativeOverlaySettings,
        telemetryOverlaySettings,
        telemetryReferenceOverlaySettings,
        trackMapOverlaySettings,
    } from "$lib/backend/settings.svelte";
    import { active } from "$lib/backend/telemetry.svelte";

    import Main from "../../components/overlays/Main.svelte";
    import Timer from "../../components/overlays/Timer.svelte";
    import SubTimer from "../../components/overlays/SubTimer.svelte";
    import Standings from "../../components/overlays/Standings.svelte";
    import LapTimes from "../../components/overlays/LapTimes.svelte";
    import Proximity from "../../components/overlays/Proximity.svelte";
    import Relative from "../../components/overlays/Relative.svelte";
    import Telemetry from "../../components/overlays/Telemetry.svelte";
    import TelemetryReference from "../../components/overlays/TelemetryReference.svelte";
    import TrackMapCanvas from "../../components/overlays/TrackMapCanvas.svelte";

    // Settings stores
    let mainSettings = $derived(mainOverlaySettings);
    let timerSettings = $derived(timerOverlaySettings);
    let subtimerSettings = $derived(subtimerOverlaySettings);
    let standingsSettings = $derived(standingsOverlaySettings);
    let lapTimesSettings = $derived(lapTimesOverlaySettings);
    let proximitySettings = $derived(proximityOverlaySettings);
    let relativeSettings = $derived(relativeOverlaySettings);
    let telemetrySettings = $derived(telemetryOverlaySettings);
    let telemetryReferenceSettings = $derived(telemetryReferenceOverlaySettings);
    let trackMapSettings = $derived(trackMapOverlaySettings);

    // Derived enabled states
    let mainEnabled = $derived($mainSettings?.common_settings?.enabled ?? false);
    let timerEnabled = $derived($timerSettings?.common_settings?.enabled ?? false);
    let subtimerEnabled = $derived($subtimerSettings?.common_settings?.enabled ?? false);
    let standingsEnabled = $derived($standingsSettings?.common_settings?.enabled ?? false);
    let lapTimesEnabled = $derived($lapTimesSettings?.common_settings?.enabled ?? false);
    let proximityEnabled = $derived($proximitySettings?.common_settings?.enabled ?? false);
    let relativeEnabled = $derived($relativeSettings?.common_settings?.enabled ?? false);
    let telemetryEnabled = $derived($telemetrySettings?.common_settings?.enabled ?? false);
    let telemetryReferenceEnabled = $derived($telemetryReferenceSettings?.common_settings?.enabled ?? false);
    let trackMapEnabled = $derived($trackMapSettings?.common_settings?.enabled ?? false);

    // Helper to get opacity
    function getOpacity(settings: any) {
        return settings?.common_settings?.opacity ?? 100;
    }

    // Helper to get dimensions
    function getDimensions(settings: any) {
        return {
            width: settings?.common_settings?.width ?? 0,
            height: settings?.common_settings?.height ?? 0,
            scale: (settings?.common_settings?.scale ?? 100) / 100.0,
        };
    }

    let mainDims = $derived(getDimensions($mainSettings));
    let timerDims = $derived(getDimensions($timerSettings));
    let subtimerDims = $derived(getDimensions($subtimerSettings));
    let standingsDims = $derived(getDimensions($standingsSettings));
    let lapTimesDims = $derived(getDimensions($lapTimesSettings));
    let proximityDims = $derived(getDimensions($proximitySettings));
    let relativeDims = $derived(getDimensions($relativeSettings));
    let telemetryDims = $derived(getDimensions($telemetrySettings));
    let telemetryReferenceDims = $derived(getDimensions($telemetryReferenceSettings));
    let trackMapDims = $derived(getDimensions($trackMapSettings));
</script>

<svelte:head>
    <title>iRaceHUD - VR Overlay Dashboard</title>
</svelte:head>

<div class="vr-dashboard">
    <div class="vr-header">
        <h1>iRaceHUD VR Dashboard</h1>
        <div class="status">
            {#if $active}
                <span class="status-active">Session Active</span>
            {:else}
                <span class="status-inactive">Waiting for iRacing...</span>
            {/if}
        </div>
    </div>

    <div class="vr-overlays">
        {#if mainEnabled && $active}
            <div
                class="vr-overlay-container"
                style="opacity: {getOpacity($mainSettings)}%; width: {mainDims.width * mainDims.scale}px; height: {mainDims.height * mainDims.scale}px"
            >
                <div class="overlay-label">Main</div>
                <div style="transform: scale({mainDims.scale}); transform-origin: top left;">
                    <div style="width: {mainDims.width}px; height: {mainDims.height}px;">
                        <Main settings={$mainSettings} />
                    </div>
                </div>
            </div>
        {/if}

        {#if timerEnabled && $active}
            <div
                class="vr-overlay-container"
                style="opacity: {getOpacity($timerSettings)}%; width: {timerDims.width * timerDims.scale}px; height: {timerDims.height * timerDims.scale}px"
            >
                <div class="overlay-label">Timer</div>
                <div style="transform: scale({timerDims.scale}); transform-origin: top left;">
                    <div style="width: {timerDims.width}px; height: {timerDims.height}px;">
                        <Timer settings={$timerSettings} />
                    </div>
                </div>
            </div>
        {/if}

        {#if subtimerEnabled && $active}
            <div
                class="vr-overlay-container"
                style="opacity: {getOpacity($subtimerSettings)}%; width: {subtimerDims.width * subtimerDims.scale}px; height: {subtimerDims.height * subtimerDims.scale}px"
            >
                <div class="overlay-label">Subtimer</div>
                <div style="transform: scale({subtimerDims.scale}); transform-origin: top left;">
                    <div style="width: {subtimerDims.width}px; height: {subtimerDims.height}px;">
                        <SubTimer settings={$subtimerSettings} />
                    </div>
                </div>
            </div>
        {/if}

        {#if standingsEnabled && $active}
            <div
                class="vr-overlay-container"
                style="opacity: {getOpacity($standingsSettings)}%; width: {standingsDims.width * standingsDims.scale}px; height: {standingsDims.height * standingsDims.scale}px"
            >
                <div class="overlay-label">Standings</div>
                <div style="transform: scale({standingsDims.scale}); transform-origin: top left;">
                    <div style="width: {standingsDims.width}px; height: {standingsDims.height}px;">
                        <Standings settings={$standingsSettings} />
                    </div>
                </div>
            </div>
        {/if}

        {#if lapTimesEnabled && $active}
            <div
                class="vr-overlay-container"
                style="opacity: {getOpacity($lapTimesSettings)}%; width: {lapTimesDims.width * lapTimesDims.scale}px; height: {lapTimesDims.height * lapTimesDims.scale}px"
            >
                <div class="overlay-label">Lap Times</div>
                <div style="transform: scale({lapTimesDims.scale}); transform-origin: top left;">
                    <div style="width: {lapTimesDims.width}px; height: {lapTimesDims.height}px;">
                        <LapTimes settings={$lapTimesSettings} />
                    </div>
                </div>
            </div>
        {/if}

        {#if proximityEnabled && $active}
            <div
                class="vr-overlay-container"
                style="width: {proximityDims.width * proximityDims.scale}px; height: {proximityDims.height * proximityDims.scale}px"
            >
                <div class="overlay-label">Proximity</div>
                <div style="transform: scale({proximityDims.scale}); transform-origin: top left;">
                    <div style="width: {proximityDims.width}px; height: {proximityDims.height}px;">
                        <Proximity settings={$proximitySettings} />
                    </div>
                </div>
            </div>
        {/if}

        {#if relativeEnabled && $active}
            <div
                class="vr-overlay-container"
                style="opacity: {getOpacity($relativeSettings)}%; width: {relativeDims.width * relativeDims.scale}px; height: {relativeDims.height * relativeDims.scale}px"
            >
                <div class="overlay-label">Relative</div>
                <div style="transform: scale({relativeDims.scale}); transform-origin: top left;">
                    <div style="width: {relativeDims.width}px; height: {relativeDims.height}px;">
                        <Relative settings={$relativeSettings} />
                    </div>
                </div>
            </div>
        {/if}

        {#if telemetryEnabled && $active}
            <div
                class="vr-overlay-container"
                style="opacity: {getOpacity($telemetrySettings)}%; width: {telemetryDims.width * telemetryDims.scale}px; height: {telemetryDims.height * telemetryDims.scale}px"
            >
                <div class="overlay-label">Telemetry</div>
                <div style="transform: scale({telemetryDims.scale}); transform-origin: top left;">
                    <div style="width: {telemetryDims.width}px; height: {telemetryDims.height}px;">
                        <Telemetry settings={$telemetrySettings} />
                    </div>
                </div>
            </div>
        {/if}

        {#if telemetryReferenceEnabled && $active}
            <div
                class="vr-overlay-container"
                style="opacity: {getOpacity($telemetryReferenceSettings)}%; width: {telemetryReferenceDims.width * telemetryReferenceDims.scale}px; height: {telemetryReferenceDims.height * telemetryReferenceDims.scale}px"
            >
                <div class="overlay-label">Telemetry Reference</div>
                <div style="transform: scale({telemetryReferenceDims.scale}); transform-origin: top left;">
                    <div style="width: {telemetryReferenceDims.width}px; height: {telemetryReferenceDims.height}px;">
                        <TelemetryReference settings={$telemetryReferenceSettings} />
                    </div>
                </div>
            </div>
        {/if}

        {#if trackMapEnabled && $active}
            <div
                class="vr-overlay-container"
                style="opacity: {getOpacity($trackMapSettings)}%; width: {trackMapDims.width * trackMapDims.scale}px; height: {trackMapDims.height * trackMapDims.scale}px"
            >
                <div class="overlay-label">Track Map</div>
                <div style="transform: scale({trackMapDims.scale}); transform-origin: top left;">
                    <div style="width: {trackMapDims.width}px; height: {trackMapDims.height}px;">
                        <TrackMapCanvas settings={$trackMapSettings} />
                    </div>
                </div>
            </div>
        {/if}
    </div>

    {#if !$active}
        <div class="waiting-message">
            <p>Waiting for an active iRacing session...</p>
            <p class="hint">Start iRacing and join a session to see the overlays</p>
        </div>
    {/if}
</div>

<style>
    .vr-dashboard {
        min-height: 100vh;
        background-color: rgba(0, 0, 0, 0.9);
        padding: 20px;
    }

    .vr-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
        padding-bottom: 10px;
        border-bottom: 1px solid rgba(255, 255, 255, 0.2);
    }

    .vr-header h1 {
        color: white;
        font-size: 1.5rem;
        margin: 0;
    }

    .status {
        font-size: 0.9rem;
    }

    .status-active {
        color: #00ff00;
    }

    .status-inactive {
        color: #ffaa00;
    }

    .vr-overlays {
        display: flex;
        flex-wrap: wrap;
        gap: 20px;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .vr-overlay-container {
        position: relative;
        background-color: rgba(0, 0, 0, 0.5);
        border-radius: 8px;
        overflow: hidden;
    }

    .overlay-label {
        position: absolute;
        top: -20px;
        left: 0;
        color: rgba(255, 255, 255, 0.5);
        font-size: 0.7rem;
        text-transform: uppercase;
    }

    .waiting-message {
        text-align: center;
        color: white;
        margin-top: 100px;
    }

    .waiting-message p {
        margin: 10px 0;
    }

    .waiting-message .hint {
        color: rgba(255, 255, 255, 0.5);
        font-size: 0.9rem;
    }
</style>
