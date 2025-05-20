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
    import { active } from "$lib/telemetry/telemetry.svelte";
    import { lapTimesWidgetSettings } from "$lib/settings/settings.svelte";
    import { mainWidgetSettings } from "$lib/settings/settings.svelte";
    import { proximityWidgetSettings } from "$lib/settings/settings.svelte";
    import { relativeWidgetSettings } from "$lib/settings/settings.svelte";
    import { standingsWidgetSettings } from "$lib/settings/settings.svelte";
    import { subtimerWidgetSettings } from "$lib/settings/settings.svelte";
    import { telemetryWidgetSettings } from "$lib/settings/settings.svelte";
    import { timerWidgetSettings } from "$lib/settings/settings.svelte";
    import { trackMapWidgetSettings } from "$lib/settings/settings.svelte";
</script>

{#if $active}
    {#if $mainWidgetSettings?.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {$mainWidgetSettings?.x ??
                        0}px; margin-top: {$mainWidgetSettings?.y ?? 0}px;"
                >
                    <Main />
                </div>
            </div>
        </div>
    {/if}
    {#if $telemetryWidgetSettings?.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {$telemetryWidgetSettings?.x ??
                        0}px; margin-top: {$telemetryWidgetSettings?.y ?? 0}px;"
                >
                    <Telemetry />
                </div>
            </div>
        </div>
    {/if}
    {#if $timerWidgetSettings?.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {$timerWidgetSettings?.x ??
                        0}px; margin-top: {$timerWidgetSettings?.y ?? 0}px;"
                >
                    <Timer />
                </div>
            </div>
        </div>
    {/if}
    {#if $subtimerWidgetSettings?.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {$subtimerWidgetSettings?.x ??
                        0}px; margin-top: {$subtimerWidgetSettings?.y ?? 0}px;"
                >
                    <Subtimer />
                </div>
            </div>
        </div>
    {/if}
    {#if $proximityWidgetSettings?.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {$proximityWidgetSettings?.x ??
                        0}px; margin-top: {$proximityWidgetSettings?.y ?? 0}px;"
                >
                    <Proximity />
                </div>
            </div>
        </div>
    {/if}
    {#if $standingsWidgetSettings?.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {$standingsWidgetSettings?.x ??
                        0}px; margin-top: {$standingsWidgetSettings?.y ?? 0}px;"
                >
                    <Standings />
                </div>
            </div>
        </div>
    {/if}
    {#if $trackMapWidgetSettings?.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {$trackMapWidgetSettings?.x ??
                        0}px; margin-top: {$trackMapWidgetSettings?.y ?? 0}px;"
                >
                    <TrackMap />
                </div>
            </div>
        </div>
    {/if}
    {#if $lapTimesWidgetSettings?.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {$lapTimesWidgetSettings?.x ??
                        0}px; margin-top: {$lapTimesWidgetSettings?.y ?? 0}px;"
                >
                    <LapTimes />
                </div>
            </div>
        </div>
    {/if}
    {#if $relativeWidgetSettings?.enabled}
        <div class="outer">
            <div class="middle">
                <div
                    class="widget"
                    style="margin-left: {$relativeWidgetSettings?.x ??
                        0}px; margin-top: {$relativeWidgetSettings?.y ?? 0}px;"
                >
                    <Relative />
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
