<script lang="ts">
    import Application from "../../components/settings/Application.svelte";
    import LapTimes from "../../components/settings/widgets/LapTimes.svelte";
    import Main from "../../components/settings/widgets/Main.svelte";
    import Proximity from "../../components/settings/widgets/Proximity.svelte";
    import Relative from "../../components/settings/widgets/Relative.svelte";
    import Standings from "../../components/settings/widgets/Standings.svelte";
    import SubTimer from "../../components/settings/widgets/SubTimer.svelte";
    import Telemetry from "../../components/settings/widgets/Telemetry.svelte";
    import Timer from "../../components/settings/widgets/Timer.svelte";
    import TrackMap from "../../components/settings/widgets/TrackMap.svelte";

    let chosenPage = $state("application");

    function setChosenPage(page: string) {
        chosenPage = page;
    }
</script>

<div class="drawer drawer-open">
    <input id="my-drawer" type="checkbox" class="drawer-toggle" />
    <div class="drawer-side">
        <ul class="menu bg-accent text-base-content min-h-full w-64 p-4">
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'application'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("application")}
                    >Application</button
                >
            </li>
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'main'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("main")}
                    >Main Widget</button
                >
            </li>
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'telemetry'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("telemetry")}
                    >Telemetry Widget</button
                >
            </li>
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'standings'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("standings")}
                    >Standings Widget</button
                >
            </li>
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'relative'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("relative")}
                    >Relative Widget</button
                >
            </li>
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'track_map'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("track_map")}
                    >Track Map Widget</button
                >
            </li>
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'proximity'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("proximity")}
                    >Proximity Widget</button
                >
            </li>
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'timer'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("timer")}>Timer Widget</button
                >
            </li>
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'subtimer'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("subtimer")}
                    >Subtimer Widget</button
                >
            </li>
            <li class="mb-2">
                <button
                    type="button"
                    class="btn btn-sm btn-outline {chosenPage === 'lap_times'
                        ? 'btn-active'
                        : ''}"
                    onclick={() => setChosenPage("lap_times")}
                    >Lap Times Widget</button
                >
            </li>
        </ul>
    </div>
    <div class="drawer-content">
        {#if chosenPage === "application"}
            <Application />
        {:else if chosenPage === "main"}
            <Main />
        {:else if chosenPage === "telemetry"}
            <Telemetry />
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
