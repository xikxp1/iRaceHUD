<script lang="ts">
    import Main from "../components/widgets/Main.svelte";
    import Telemetry from "../components/widgets/Telemetry.svelte";
    import Timer from "../components/widgets/Timer.svelte";
    import Subtimer from "../components/widgets/SubTimer.svelte";
    import Proximity from "../components/widgets/Proximity.svelte";
    import Standings from "../components/widgets/Standings.svelte";
    import TrackMap from "../components/widgets/TrackMap.svelte";
    import LapTimes from "../components/widgets/LapTimes.svelte";
    import Relative from "../components/widgets/Relative.svelte";
    import { active } from "$lib/telemetry/telemetry.svelte";
</script>

{#if $active}
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
        margin-left: -1350px;
        margin-right: auto;
        margin-top: -520px;
        margin-bottom: auto;
        align-items: center;
    }

    .trackmap {
        margin-left: 1250px;
        margin-right: auto;
        margin-top: -520px;
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
        margin-left: 1480px;
        margin-right: auto;
        margin-top: 700px;
        margin-bottom: auto;
        align-items: center;
    }
</style>
