<script lang="ts">
    import { onMount } from "svelte";
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
    import {
        isOpenKneeboard,
        setPreferredPixelSize,
        enablePageBasedContent,
        setPages,
        onPageChanged,
        generateOverlayGuid,
        type PageInfo,
    } from "$lib/backend/openkneeboard";

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

    // Settings stores - use $derived to get the store reference for auto-subscription
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

    // Current page for OpenKneeBoard navigation
    let currentPageGuid = $state<string | null>(null);
    let isOKB = $state(false);
    let pageBasedEnabled = $state(false);

    // Overlay definitions with getters for settings
    type OverlayDef = {
        name: string;
        label: string;
        component: typeof Main | typeof Timer | typeof SubTimer | typeof Standings | typeof LapTimes | typeof Proximity | typeof Relative | typeof Telemetry | typeof TelemetryReference | typeof TrackMapCanvas;
        getSettings: () => any;
    };

    const overlays: OverlayDef[] = [
        { name: 'main', label: 'Main', component: Main, getSettings: () => $mainSettings },
        { name: 'timer', label: 'Timer', component: Timer, getSettings: () => $timerSettings },
        { name: 'subtimer', label: 'Subtimer', component: SubTimer, getSettings: () => $subtimerSettings },
        { name: 'standings', label: 'Standings', component: Standings, getSettings: () => $standingsSettings },
        { name: 'lap_times', label: 'Lap Times', component: LapTimes, getSettings: () => $lapTimesSettings },
        { name: 'proximity', label: 'Proximity', component: Proximity, getSettings: () => $proximitySettings },
        { name: 'relative', label: 'Relative', component: Relative, getSettings: () => $relativeSettings },
        { name: 'telemetry', label: 'Telemetry', component: Telemetry, getSettings: () => $telemetrySettings },
        { name: 'telemetry_reference', label: 'Telemetry Ref', component: TelemetryReference, getSettings: () => $telemetryReferenceSettings },
        { name: 'track_map', label: 'Track Map', component: TrackMapCanvas, getSettings: () => $trackMapSettings },
    ];

    // Get enabled overlays - use the settings getter functions
    let enabledOverlays = $derived(overlays.filter((overlay) => {
        const settings = overlay.getSettings();
        return settings?.common_settings?.enabled ?? false;
    }));

    // Helper to get dimensions
    function getDimensions(settings: any) {
        return {
            width: settings?.common_settings?.width ?? 0,
            height: settings?.common_settings?.height ?? 0,
            scale: (settings?.common_settings?.scale ?? 100) / 100.0,
        };
    }

    // Helper to get opacity
    function getOpacity(settings: any) {
        return settings?.common_settings?.opacity ?? 100;
    }

    // Get current overlay based on page GUID
    let currentOverlay = $derived(
        currentPageGuid
            ? overlays.find((o) => generateOverlayGuid(o.name) === currentPageGuid)
            : enabledOverlays[0]
    );

    // Get current overlay settings
    let currentSettings = $derived(
        currentOverlay ? currentOverlay.getSettings() : null
    );

    let currentDims = $derived(getDimensions(currentSettings));

    // Initialize OpenKneeBoard
    onMount(() => {
        isOKB = isOpenKneeboard();

        if (isOKB) {
            // Try to enable page-based content
            enablePageBasedContent().then((enabled) => {
                pageBasedEnabled = enabled;
            });

            // Set up page change listener
            const unsubscribe = onPageChanged((page: PageInfo) => {
                currentPageGuid = page.guid;
            });

            return unsubscribe;
        }
    });

    // Update OpenKneeBoard pages when enabled overlays change
    $effect(() => {
        if (isOKB && pageBasedEnabled && enabledOverlays.length > 0) {
            const pages: PageInfo[] = enabledOverlays.map((overlay) => {
                const settings = overlay.getSettings();
                const dims = getDimensions(settings);
                return {
                    guid: generateOverlayGuid(overlay.name),
                    pixelSize: {
                        width: Math.round(dims.width * dims.scale),
                        height: Math.round(dims.height * dims.scale),
                    },
                };
            });
            setPages(pages);

            // Set current page to first if not set
            if (!currentPageGuid && pages.length > 0) {
                currentPageGuid = pages[0].guid;
            }
        }
    });

    // Update preferred pixel size when current overlay dimensions change
    $effect(() => {
        if (isOKB && currentDims.width > 0 && currentDims.height > 0) {
            setPreferredPixelSize(
                currentDims.width * currentDims.scale,
                currentDims.height * currentDims.scale
            );
        }
    });
</script>

<svelte:head>
    <title>iRaceHUD - VR Overlay</title>
    <style>
        /* Transparent background for OpenKneeBoard */
        html, body {
            background-color: transparent !important;
        }
    </style>
</svelte:head>

{#if isOKB}
    <!-- OpenKneeBoard mode: Show single overlay at a time with page-based navigation -->
    <div class="okb-container">
        {#if currentOverlay && currentSettings && $active}
            {@const dims = getDimensions(currentSettings)}
            {@const opacity = getOpacity(currentSettings)}
            <div
                class="overlay-wrapper"
                style="opacity: {opacity}%; width: {dims.width * dims.scale}px; height: {dims.height * dims.scale}px;"
            >
                <div style="transform: scale({dims.scale}); transform-origin: top left;">
                    <div style="width: {dims.width}px; height: {dims.height}px;">
                        {#if currentOverlay.name === 'main'}
                            <Main settings={currentSettings} />
                        {:else if currentOverlay.name === 'timer'}
                            <Timer settings={currentSettings} />
                        {:else if currentOverlay.name === 'subtimer'}
                            <SubTimer settings={currentSettings} />
                        {:else if currentOverlay.name === 'standings'}
                            <Standings settings={currentSettings} />
                        {:else if currentOverlay.name === 'lap_times'}
                            <LapTimes settings={currentSettings} />
                        {:else if currentOverlay.name === 'proximity'}
                            <Proximity settings={currentSettings} />
                        {:else if currentOverlay.name === 'relative'}
                            <Relative settings={currentSettings} />
                        {:else if currentOverlay.name === 'telemetry'}
                            <Telemetry settings={currentSettings} />
                        {:else if currentOverlay.name === 'telemetry_reference'}
                            <TelemetryReference settings={currentSettings} />
                        {:else if currentOverlay.name === 'track_map'}
                            <TrackMapCanvas settings={currentSettings} />
                        {/if}
                    </div>
                </div>
            </div>
        {:else if !$active}
            <div class="waiting-indicator">
                <div class="pulse"></div>
            </div>
        {/if}
    </div>
{:else}
    <!-- Standard VR mode: Dashboard showing all overlays -->
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
            {#each enabledOverlays as overlay}
                {@const settings = overlay.getSettings()}
                {@const dims = getDimensions(settings)}
                {@const opacity = getOpacity(settings)}
                {#if $active}
                    <div
                        class="vr-overlay-container"
                        style="opacity: {opacity}%; width: {dims.width * dims.scale}px; height: {dims.height * dims.scale}px"
                    >
                        <div class="overlay-label">{overlay.label}</div>
                        <div style="transform: scale({dims.scale}); transform-origin: top left;">
                            <div style="width: {dims.width}px; height: {dims.height}px;">
                                {#if overlay.name === 'main'}
                                    <Main {settings} />
                                {:else if overlay.name === 'timer'}
                                    <Timer {settings} />
                                {:else if overlay.name === 'subtimer'}
                                    <SubTimer {settings} />
                                {:else if overlay.name === 'standings'}
                                    <Standings {settings} />
                                {:else if overlay.name === 'lap_times'}
                                    <LapTimes {settings} />
                                {:else if overlay.name === 'proximity'}
                                    <Proximity {settings} />
                                {:else if overlay.name === 'relative'}
                                    <Relative {settings} />
                                {:else if overlay.name === 'telemetry'}
                                    <Telemetry {settings} />
                                {:else if overlay.name === 'telemetry_reference'}
                                    <TelemetryReference {settings} />
                                {:else if overlay.name === 'track_map'}
                                    <TrackMapCanvas {settings} />
                                {/if}
                            </div>
                        </div>
                    </div>
                {/if}
            {/each}
        </div>

        {#if !$active}
            <div class="waiting-message">
                <p>Waiting for an active iRacing session...</p>
                <p class="hint">Start iRacing and join a session to see the overlays</p>
            </div>
        {/if}
    </div>
{/if}

<style>
    /* OpenKneeBoard mode styles */
    .okb-container {
        background-color: transparent;
    }

    .overlay-wrapper {
        background-color: transparent;
    }

    .waiting-indicator {
        width: 50px;
        height: 50px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .pulse {
        width: 20px;
        height: 20px;
        background-color: rgba(255, 170, 0, 0.8);
        border-radius: 50%;
        animation: pulse 2s ease-in-out infinite;
    }

    @keyframes pulse {
        0%, 100% { transform: scale(1); opacity: 0.8; }
        50% { transform: scale(1.2); opacity: 0.4; }
    }

    /* Standard VR dashboard styles */
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
