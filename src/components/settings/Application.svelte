<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { VrSettings } from "$lib/types/telemetry";

    let is_autostart_enabled = $state(false);
    let vr_enabled = $state(false);
    let vr_port = $state(8080);
    let vr_external_access = $state(false);
    let vr_server_port = $state<number | null>(null);
    let show_restart_notice = $state(false);

    onMount(() => {
        invoke("get_autostart", {}).then((value) => {
            is_autostart_enabled = value as boolean;
        });

        invoke<VrSettings>("get_vr_settings", {}).then((settings) => {
            vr_enabled = settings.enabled;
            vr_port = settings.http_port;
            vr_external_access = settings.external_access;
        });

        invoke<number | null>("get_vr_server_port_cmd", {}).then((port) => {
            vr_server_port = port;
        });
    });

    function handleAutostartChange(event: Event) {
        const isChecked = (event.target as HTMLInputElement).checked;
        invoke("set_autostart", { enabled: isChecked });
    }

    function handleVrEnabledChange(event: Event) {
        const isChecked = (event.target as HTMLInputElement).checked;
        vr_enabled = isChecked;
        saveVrSettings();
    }

    function handleVrPortChange(event: Event) {
        const value = parseInt((event.target as HTMLInputElement).value);
        if (!isNaN(value) && value > 0 && value <= 65535) {
            vr_port = value;
            saveVrSettings();
        }
    }

    function handleVrExternalAccessChange(event: Event) {
        const isChecked = (event.target as HTMLInputElement).checked;
        vr_external_access = isChecked;
        saveVrSettings();
    }

    function saveVrSettings() {
        const settings: VrSettings = {
            enabled: vr_enabled,
            http_port: vr_port,
            external_access: vr_external_access,
        };
        invoke("set_vr_settings", { settings });
        show_restart_notice = true;
    }
</script>

<div class="flex w-full h-full items-center justify-center">
    <div class="flex flex-col gap-8">
        <table>
            <tbody>
                <tr>
                    <td class="text-sm font-bold text-right"
                        >Launch on system startup</td
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

        <div class="divider">VR Web Overlay Mode</div>

        <table>
            <tbody>
                <tr>
                    <td class="text-sm font-bold text-right pr-4">Enable VR Mode</td>
                    <td>
                        <input
                            type="checkbox"
                            class="toggle toggle-sm"
                            bind:checked={vr_enabled}
                            onchange={handleVrEnabledChange}
                        />
                    </td>
                </tr>
                <tr>
                    <td class="text-sm font-bold text-right pr-4">HTTP Port</td>
                    <td>
                        <input
                            type="number"
                            class="input input-sm input-bordered w-24"
                            min="1"
                            max="65535"
                            bind:value={vr_port}
                            onchange={handleVrPortChange}
                            disabled={!vr_enabled}
                        />
                    </td>
                </tr>
                <tr>
                    <td class="text-sm font-bold text-right pr-4">Allow External Access</td>
                    <td>
                        <input
                            type="checkbox"
                            class="toggle toggle-sm"
                            bind:checked={vr_external_access}
                            onchange={handleVrExternalAccessChange}
                            disabled={!vr_enabled}
                        />
                    </td>
                </tr>
            </tbody>
        </table>

        {#if vr_enabled && vr_server_port}
            <div class="alert alert-info">
                <span>
                    VR overlay available at:
                    <code class="font-mono bg-base-300 px-1 rounded">
                        http://{vr_external_access ? 'YOUR_PC_IP' : 'localhost'}:{vr_server_port}/vr
                    </code>
                </span>
            </div>
        {/if}

        {#if show_restart_notice}
            <div class="alert alert-warning">
                <span>Restart the application for VR settings to take effect.</span>
            </div>
        {/if}

        {#if vr_enabled}
            <div class="text-sm opacity-70">
                <p>VR Mode allows you to access overlays through a web browser in your VR headset.</p>
                <p class="mt-2">Point your VR browser to the URL shown above to view all enabled overlays.</p>
                {#if vr_external_access}
                    <p class="mt-2 text-warning">External access is enabled - make sure your firewall allows connections on port {vr_port}.</p>
                {/if}
            </div>
        {/if}
    </div>
</div>

<style>
</style>
