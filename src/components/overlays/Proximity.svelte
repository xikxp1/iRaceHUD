<script lang="ts">
    import { proximity } from "$lib/backend/telemetry.svelte";
    import type {
        Proximity,
        ProximityOverlaySettings,
    } from "$lib/types/telemetry";
    import { onDestroy, onMount } from "svelte";

    let { settings }: { settings: ProximityOverlaySettings } = $props();

    let left_icon: HTMLImageElement | undefined = $state();
    let right_icon: HTMLImageElement | undefined = $state();

    function on_proximity(value: Proximity) {
        const opacity = settings.common_settings.opacity;
        if (left_icon != null) {
            left_icon.style.opacity = value.is_left ? "1" : "0";
            left_icon.style.backgroundColor = value.is_left
                ? `rgba(0, 0, 0, ${opacity / 100})`
                : "transparent";
        }
        if (right_icon != null) {
            right_icon.style.opacity = value.is_right ? "1" : "0";
            right_icon.style.backgroundColor = value.is_right
                ? `rgba(0, 0, 0, ${opacity / 100})`
                : "transparent";
        }
    }

    let unsubscribe_proximity: () => void = () => {};

    onMount(() => {
        unsubscribe_proximity = proximity.subscribe((value) =>
            on_proximity(value),
        );

        on_proximity($proximity);
    });

    onDestroy(() => {
        unsubscribe_proximity();
    });
</script>

<div class="flex flex-row w-full h-full">
    <div class="ml-2">
        <img
            bind:this={left_icon}
            src="/icons/alert.svg"
            alt=""
            style="opacity: 0; background-color: transparent; border-radius: 4px; padding: 4px;"
        />
    </div>
    <div class="flex-grow"></div>
    <div class="mr-2">
        <img
            bind:this={right_icon}
            src="/icons/alert.svg"
            alt=""
            style="opacity: 0; background-color: transparent; border-radius: 4px; padding: 4px;"
        />
    </div>
</div>

<style>
</style>
