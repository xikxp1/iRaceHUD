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

<div class="flex w-full h-full items-center justify-center">
    <table>
        <tbody>
            <tr>
                <td class="text-sm font-bold text-right"
                    >Start on system startup</td
                >
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-sm w-24 ml-2"
                        bind:checked={is_autostart_enabled}
                        onchange={handleAutostartChange}
                    />
                </td>
            </tr>
        </tbody>
    </table>
</div>

<style>
</style>
