<script lang="ts">
    import Proximity from "$lib/components/widgets/Proximity.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { ProximityWidgetSettings } from "$lib/types/telemetry";

    let settings = $state<ProximityWidgetSettings | undefined>(undefined);

    onMount(() => {
        invoke<ProximityWidgetSettings>("get_proximity_widget_settings").then(
            (x) => {
                settings = x;
            },
        );
    });
</script>

{#if settings?.enabled}
    <div class="w-full h-full">
        <Proximity {settings} />
    </div>
{/if}

<style>
</style>
