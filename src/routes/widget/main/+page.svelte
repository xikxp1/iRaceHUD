<script lang="ts">
    import Main from "$lib/components/widgets/Main.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { MainWidgetSettings } from "$lib/types/telemetry";

    let settings = $state<MainWidgetSettings | undefined>(undefined);
    let scale = $state(1);

    const baseWidth = 300;
    const baseHeight = 80;

    onMount(() => {
        invoke<MainWidgetSettings>("get_main_widget_settings").then((x) => {
            settings = x;
        });

        // Calculate scale based on viewport
        const updateScale = () => {
            const viewportWidth = window.innerWidth;
            const viewportHeight = window.innerHeight;

            // Calculate scale based on both width and height, using the smaller value
            // to ensure the widget fits within the viewport
            const widthScale = viewportWidth / baseWidth;
            const heightScale = viewportHeight / baseHeight;
            scale = Math.min(widthScale, heightScale);
        };

        // Initial scale calculation
        updateScale();
    });
</script>

{#if settings?.enabled}
    <div class="viewport-center">
        <div
            class="widget-content"
            style="opacity: {settings.opacity / 100}; transform: scale({scale}); width: {baseWidth}px; height: {baseHeight}px;"
        >
            <div data-tauri-drag-region class="drag-region"></div>
            <Main {settings} />
        </div>
    </div>
{/if}

<style>
    .viewport-center {
        position: fixed;
        top: 0; left: 0;
        width: 100vw; height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        background: transparent;
    }
    .widget-content {
        position: relative;
        /* width/height set inline */
    }
    .drag-region {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 1000;
    }
</style>
