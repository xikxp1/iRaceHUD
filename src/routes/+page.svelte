<script lang="ts">
    import LapTimes from "../components/widgets/LapTimes.svelte";
    import Main from "../components/widgets/Main.svelte";
    import Proximity from "../components/widgets/Proximity.svelte";
    import Relative from "../components/widgets/Relative.svelte";
    import Standings from "../components/widgets/Standings.svelte";
    import Subtimer from "../components/widgets/SubTimer.svelte";
    import Telemetry from "../components/widgets/Telemetry.svelte";
    import Timer from "../components/widgets/Timer.svelte";
    import TrackMap from "../components/widgets/TrackMap.svelte";
    import { active } from "$lib/backend/telemetry.svelte";
    import { lapTimesWidgetSettings } from "$lib/backend/settings.svelte";
    import { mainWidgetSettings } from "$lib/backend/settings.svelte";
    import { proximityWidgetSettings } from "$lib/backend/settings.svelte";
    import { relativeWidgetSettings } from "$lib/backend/settings.svelte";
    import { standingsWidgetSettings } from "$lib/backend/settings.svelte";
    import { subtimerWidgetSettings } from "$lib/backend/settings.svelte";
    import { telemetryWidgetSettings } from "$lib/backend/settings.svelte";
    import { timerWidgetSettings } from "$lib/backend/settings.svelte";
    import { trackMapWidgetSettings } from "$lib/backend/settings.svelte";
    import type {
        MainWidgetSettings,
        ProximityWidgetSettings,
        RelativeWidgetSettings,
        StandingsWidgetSettings,
        SubTimerWidgetSettings,
        TelemetryWidgetSettings,
        TimerWidgetSettings,
        TrackMapWidgetSettings,
    } from "$lib/types/telemetry";
    import type { LapTimesWidgetSettings } from "$lib/types/telemetry";
    import { onDestroy } from "svelte";

    let lapTimesSettings: LapTimesWidgetSettings | undefined =
        $state(undefined);
    let lapTimesSettingsUpdate = $state(0);
    let unsubscribe_lap_times_settings = lapTimesWidgetSettings.subscribe(
        (settings) => {
            lapTimesSettings = settings;
            lapTimesSettingsUpdate++;
        },
    );

    let mainSettings: MainWidgetSettings | undefined = $state(undefined);
    let mainSettingsUpdate = $state(0);
    let unsubscribe_main_settings = mainWidgetSettings.subscribe((settings) => {
        mainSettings = settings;
        mainSettingsUpdate++;
    });

    let proximitySettings: ProximityWidgetSettings | undefined =
        $state(undefined);
    let proximitySettingsUpdate = $state(0);
    let unsubscribe_proximity_settings = proximityWidgetSettings.subscribe(
        (settings) => {
            proximitySettings = settings;
            proximitySettingsUpdate++;
        },
    );

    let standingsSettings: StandingsWidgetSettings | undefined =
        $state(undefined);
    let standingsSettingsUpdate = $state(0);
    let unsubscribe_standings_settings = standingsWidgetSettings.subscribe(
        (settings) => {
            standingsSettings = settings;
            standingsSettingsUpdate++;
        },
    );

    let trackMapSettings: TrackMapWidgetSettings | undefined =
        $state(undefined);
    let trackMapSettingsUpdate = $state(0);
    let unsubscribe_track_map_settings = trackMapWidgetSettings.subscribe(
        (settings) => {
            trackMapSettings = settings;
            trackMapSettingsUpdate++;
        },
    );

    let relativeSettings: RelativeWidgetSettings | undefined =
        $state(undefined);
    let relativeSettingsUpdate = $state(0);
    let unsubscribe_relative_settings = relativeWidgetSettings.subscribe(
        (settings) => {
            relativeSettings = settings;
            relativeSettingsUpdate++;
        },
    );

    let subtimerSettings: SubTimerWidgetSettings | undefined =
        $state(undefined);
    let subtimerSettingsUpdate = $state(0);
    let unsubscribe_subtimer_settings = subtimerWidgetSettings.subscribe(
        (settings) => {
            subtimerSettings = settings;
            subtimerSettingsUpdate++;
        },
    );

    let telemetrySettings: TelemetryWidgetSettings | undefined =
        $state(undefined);
    let telemetrySettingsUpdate = $state(0);
    let unsubscribe_telemetry_settings = telemetryWidgetSettings.subscribe(
        (settings) => {
            telemetrySettings = settings;
            telemetrySettingsUpdate++;
        },
    );

    let timerSettings: TimerWidgetSettings | undefined = $state(undefined);
    let timerSettingsUpdate = $state(0);
    let unsubscribe_timer_settings = timerWidgetSettings.subscribe(
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
                        class="widget"
                        style="margin-left: {mainSettings.x}px; margin-top: {mainSettings.y}px;"
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
                    class="widget"
                    style="margin-left: {telemetrySettings.x}px; margin-top: {telemetrySettings.y}px;"
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
                    class="widget"
                    style="margin-left: {timerSettings.x}px; margin-top: {timerSettings.y}px;"
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
                    class="widget"
                    style="margin-left: {subtimerSettings.x}px; margin-top: {subtimerSettings.y}px;"
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
                    class="widget"
                    style="margin-left: {proximitySettings.x}px; margin-top: {proximitySettings.y}px;"
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
                    class="widget"
                    style="margin-left: {standingsSettings.x}px; margin-top: {standingsSettings.y}px;"
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
                    class="widget"
                    style="margin-left: {trackMapSettings.x}px; margin-top: {trackMapSettings.y}px;"
                >
                    <TrackMap settings={trackMapSettings} />
                </div>
            </div>
        </div>
    {/if}
    {#if lapTimesSettings !== undefined && lapTimesSettings.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {lapTimesSettings.x}px; margin-top: {lapTimesSettings.y}px;"
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
                    class="widget"
                    style="margin-left: {relativeSettings.x}px; margin-top: {relativeSettings.y}px;"
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
    }

    .middle {
        display: table-cell;
        vertical-align: middle;
    }

    .widget {
        margin-right: auto;
        margin-bottom: auto;
        align-items: center;
    }
</style>
