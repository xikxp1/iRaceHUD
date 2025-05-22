<script lang="ts">
    import Timer from "$components/widgets/Timer.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { TimerWidgetSettings } from "$lib/types/telemetry";

    let settings = $state<TimerWidgetSettings | undefined>(undefined);

    onMount(() => {
        invoke<TimerWidgetSettings>("get_timer_widget_settings").then((x) => {
            settings = x;
        });
    });
</script>

{#if settings?.enabled}
    <div class="widget" style="transform: translate({settings.x}px, {settings.y}px);">
        <Timer settings={settings} />
    </div>
{/if}

<style>
    .widget {
        margin-right: auto;
        margin-bottom: auto;
        align-items: center;
    }
</style> 
