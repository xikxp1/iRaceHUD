<script lang="ts">
    import SubTimer from "$lib/components/widgets/SubTimer.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { SubTimerWidgetSettings } from "$lib/types/telemetry";

    let settings = $state<SubTimerWidgetSettings | undefined>(undefined);

    onMount(() => {
        invoke<SubTimerWidgetSettings>("get_subtimer_widget_settings").then((x) => {
            settings = x;
        });
    });
</script>

{#if settings?.enabled}
    <div class="widget" style="transform: translate({settings.x}px, {settings.y}px);">
        <SubTimer settings={settings} />
    </div>
{/if}

<style>
    .widget {
        margin-right: auto;
        margin-bottom: auto;
        align-items: center;
    }
</style> 
