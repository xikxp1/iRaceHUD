<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let is_autostart_enabled = $state(false);

    onMount(() => {
        invoke("get_autostart", {}).then((value) => {
            is_autostart_enabled = value as boolean;
        });
    });

    function handleAutostartChange(event: Event) {
        const isChecked = (event.target as HTMLInputElement).checked;
        invoke("set_autostart", { enabled: isChecked });
    }
</script>

<div class="flex flex-col flex-grow items-center justify-center">
    <label class="flex flex-row space-x-2 label cursor-pointer">
        <input
            type="checkbox"
            class="toggle toggle-sm"
            bind:checked={is_autostart_enabled}
            onchange={handleAutostartChange}
        />
        <span class="label-text text-sm">Start iRaceHUD on system startup</span>
    </label>
</div>

<style>
</style>
