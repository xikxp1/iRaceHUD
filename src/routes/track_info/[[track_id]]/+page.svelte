<script lang="ts">
    import { setTrackInfoState } from "$lib/stores/TrackInfoState.svelte";
    import { setTrackSettingsState } from "$lib/stores/TrackSettingsState.svelte";
    import Menu from "../../../components/track_info/Menu.svelte";
    import TrackInfo from "../../../components/track_info/TrackInfo.svelte";

    setTrackInfoState();
    setTrackSettingsState();
</script>

<div class="flex flex-row">
    <div class="w-1/4 h-screen overflow-x-auto">
        <Menu />
    </div>
    <div class="w-3/4">
        <TrackInfo />
    </div>
</div>

<style>
</style>
