<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { open } from "@tauri-apps/plugin-dialog";
    import { LazyStore } from '@tauri-apps/plugin-store';

    const settings = new LazyStore('settings.json');

    let is_autostart_enabled = $state(false);
    let file_path = $state('');

    onMount(() => {
        invoke("get_autostart", {}).then((value) => {
            is_autostart_enabled = value as boolean;
        });
        getIBTFilePath().then((value) => {
            file_path = value;
        });
    });

    function handleAutostartChange(event: Event) {
        const isChecked = (event.target as HTMLInputElement).checked;
        invoke("set_autostart", { enabled: isChecked });
    }

    async function toggleDemoMode() {
        await invoke("toggle_demo_mode", {});
    }

    async function openIBTFileDialog() {
        const file = await open({
            multiple: false,
            directory: false,
        });
        if (file !== null) {
            settings.set("demo_mode_ibt_file_path", file);
            file_path = file;
        }
    }

    async function getIBTFilePath() {
        const file = await settings.get("demo_mode_ibt_file_path");
        console.log(file);
        if (file === null) {
            return "No IBT file selected";
        }
        return file as string;
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
    <div class="flex flex-row space-x-2">
        <!-- Limit span length-->
        <span class="label-text text-sm max-w-[300px] truncate" title={file_path}>{file_path}</span>
        <button class="btn btn-sm btn-outline" onclick={openIBTFileDialog}>Open IBT File</button>
        <button class="btn btn-sm btn-outline" onclick={toggleDemoMode}>Toggle Demo Mode</button>
    </div>
</div>

<style>
</style>
