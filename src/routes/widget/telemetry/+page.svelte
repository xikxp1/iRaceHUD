<script lang="ts">
    import Telemetry from "$lib/components/widgets/Telemetry.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { TelemetryWidgetSettings } from "$lib/types/telemetry";

    let settings = $state<TelemetryWidgetSettings | undefined>(undefined);

    onMount(() => {
        invoke<TelemetryWidgetSettings>("get_telemetry_widget_settings").then(
            (x) => {
                settings = x;
            },
        );
    });
</script>

{#if settings?.enabled}
    <div class="w-full h-full">
        <Telemetry {settings} />
    </div>
{/if}

<style>
</style>
