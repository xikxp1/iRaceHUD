<script lang="ts">
    import LapTimes from "../components/overlays/LapTimes.svelte";
    import Main from "../components/overlays/Main.svelte";
    import Proximity from "../components/overlays/Proximity.svelte";
    import Relative from "../components/overlays/Relative.svelte";
    import Standings from "../components/overlays/Standings.svelte";
    import Subtimer from "../components/overlays/SubTimer.svelte";
    import Telemetry from "../components/overlays/Telemetry.svelte";
    import Timer from "../components/overlays/Timer.svelte";
    import TrackMapCanvas from "../components/overlays/TrackMapCanvas.svelte";
    import { active } from "$lib/backend/telemetry.svelte";
    import { lapTimesOverlaySettings } from "$lib/backend/settings.svelte";
    import { mainOverlaySettings } from "$lib/backend/settings.svelte";
    import { proximityOverlaySettings } from "$lib/backend/settings.svelte";
    import { relativeOverlaySettings } from "$lib/backend/settings.svelte";
    import { standingsOverlaySettings } from "$lib/backend/settings.svelte";
    import { subtimerOverlaySettings } from "$lib/backend/settings.svelte";
    import { telemetryOverlaySettings } from "$lib/backend/settings.svelte";
    import { timerOverlaySettings } from "$lib/backend/settings.svelte";
    import { trackMapOverlaySettings } from "$lib/backend/settings.svelte";
    import type {
        MainOverlaySettings,
        ProximityOverlaySettings,
        RelativeOverlaySettings,
        StandingsOverlaySettings,
        SubTimerOverlaySettings,
        TelemetryOverlaySettings,
        TimerOverlaySettings,
        TrackMapOverlaySettings,
    } from "$lib/types/telemetry";
    import type { LapTimesOverlaySettings } from "$lib/types/telemetry";
    import { onDestroy } from "svelte";

    let lapTimesSettings: LapTimesOverlaySettings | undefined =
        $state(undefined);
    let lapTimesSettingsUpdate = $state(0);
    let unsubscribe_lap_times_settings = lapTimesOverlaySettings.subscribe(
        (settings) => {
            lapTimesSettings = settings;
            lapTimesSettingsUpdate++;
        },
    );

    let mainSettings: MainOverlaySettings | undefined = $state(undefined);
    let mainSettingsUpdate = $state(0);
    let unsubscribe_main_settings = mainOverlaySettings.subscribe(
        (settings) => {
            mainSettings = settings;
            mainSettingsUpdate++;
        },
    );

    let proximitySettings: ProximityOverlaySettings | undefined =
        $state(undefined);
    let proximitySettingsUpdate = $state(0);
    let unsubscribe_proximity_settings = proximityOverlaySettings.subscribe(
        (settings) => {
            proximitySettings = settings;
            proximitySettingsUpdate++;
        },
    );

    let standingsSettings: StandingsOverlaySettings | undefined =
        $state(undefined);
    let standingsSettingsUpdate = $state(0);
    let unsubscribe_standings_settings = standingsOverlaySettings.subscribe(
        (settings) => {
            standingsSettings = settings;
            standingsSettingsUpdate++;
        },
    );

    let trackMapSettings: TrackMapOverlaySettings | undefined =
        $state(undefined);
    let trackMapSettingsUpdate = $state(0);
    let unsubscribe_track_map_settings = trackMapOverlaySettings.subscribe(
        (settings) => {
            trackMapSettings = settings;
            trackMapSettingsUpdate++;
        },
    );

    let relativeSettings: RelativeOverlaySettings | undefined =
        $state(undefined);
    let relativeSettingsUpdate = $state(0);
    let unsubscribe_relative_settings = relativeOverlaySettings.subscribe(
        (settings) => {
            relativeSettings = settings;
            relativeSettingsUpdate++;
        },
    );

    let subtimerSettings: SubTimerOverlaySettings | undefined =
        $state(undefined);
    let subtimerSettingsUpdate = $state(0);
    let unsubscribe_subtimer_settings = subtimerOverlaySettings.subscribe(
        (settings) => {
            subtimerSettings = settings;
            subtimerSettingsUpdate++;
        },
    );

    let telemetrySettings: TelemetryOverlaySettings | undefined =
        $state(undefined);
    let telemetrySettingsUpdate = $state(0);
    let unsubscribe_telemetry_settings = telemetryOverlaySettings.subscribe(
        (settings) => {
            telemetrySettings = settings;
            telemetrySettingsUpdate++;
        },
    );

    let timerSettings: TimerOverlaySettings | undefined = $state(undefined);
    let timerSettingsUpdate = $state(0);
    let unsubscribe_timer_settings = timerOverlaySettings.subscribe(
        (settings) => {
            timerSettings = settings;
            timerSettingsUpdate++;
        },
    );

    onDestroy(() => {
        unsubscribe_lap_times_settings();
        unsubscribe_main_settings();
        unsubscribe_proximity_settings();
        unsubscribe_standings_settings();
        unsubscribe_track_map_settings();
        unsubscribe_relative_settings();
        unsubscribe_subtimer_settings();
        unsubscribe_telemetry_settings();
        unsubscribe_timer_settings();
    });
</script>

{#if $active}
    {#key mainSettingsUpdate}
        {#if mainSettings !== undefined && mainSettings.enabled}
            <div class="outer">
                <div class="middle">
                    <div
                        class="overlay"
                        style="transform: translate({mainSettings.x}px, {mainSettings.y}px);"
                    >
                        <Main settings={mainSettings} />
                    </div>
                </div>
            </div>
        {/if}
    {/key}
    {#if telemetrySettings !== undefined && telemetrySettings.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="overlay"
                    style="transform: translate({telemetrySettings.x}px, {telemetrySettings.y}px);"
                >
                    <Telemetry settings={telemetrySettings} />
                </div>
            </div>
        </div>
    {/if}
    {#if timerSettings !== undefined && timerSettings.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="overlay"
                    style="transform: translate({timerSettings.x}px, {timerSettings.y}px);"
                >
                    <Timer settings={timerSettings} />
                </div>
            </div>
        </div>
    {/if}
    {#if subtimerSettings !== undefined && subtimerSettings.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="overlay"
                    style="transform: translate({subtimerSettings.x}px, {subtimerSettings.y}px);"
                >
                    <Subtimer settings={subtimerSettings} />
                </div>
            </div>
        </div>
    {/if}
    {#if proximitySettings !== undefined && proximitySettings.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="overlay"
                    style="transform: translate({proximitySettings.x}px, {proximitySettings.y}px);"
                >
                    <Proximity settings={proximitySettings} />
                </div>
            </div>
        </div>
    {/if}
    {#if standingsSettings !== undefined && standingsSettings.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="overlay"
                    style="transform: translate({standingsSettings.x}px, {standingsSettings.y}px);"
                >
                    <Standings settings={standingsSettings} />
                </div>
            </div>
        </div>
    {/if}
    {#if trackMapSettings !== undefined && trackMapSettings.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="overlay"
                    style="transform: translate({trackMapSettings.x}px, {trackMapSettings.y}px);"
                >
                    <TrackMapCanvas settings={trackMapSettings} />
                </div>
            </div>
        </div>
    {/if}
    {#if lapTimesSettings !== undefined && lapTimesSettings.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="overlay"
                    style="transform: translate({lapTimesSettings.x}px, {lapTimesSettings.y}px);"
                >
                    <LapTimes settings={lapTimesSettings} />
                </div>
            </div>
        </div>
    {/if}
    {#if relativeSettings !== undefined && relativeSettings.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="overlay"
                    style="transform: translate({relativeSettings.x}px, {relativeSettings.y}px);"
                >
                    <Relative settings={relativeSettings} />
                </div>
            </div>
        </div>
    {/if}
{/if}

<style>
    .outer {
        display: table;
        position: absolute;
        top: 0;
        left: 0;
        height: 100%;
        width: 100%;
        overflow: hidden;
    }

    .middle {
        display: table-cell;
        vertical-align: middle;
    }

    .overlay {
        margin-right: auto;
        margin-bottom: auto;
        align-items: center;
    }
</style>
