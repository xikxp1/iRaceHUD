<script lang="ts">
    import Main from "../../components/settings/Main.svelte";
    import Widgets from "../../components/settings/Widgets.svelte";

    const tabs = [
        { name: "Main", component: Main },
        { name: "Widgets", component: Widgets },
    ];

    let activeTab = $state("Main");

    const SvelteComponent = $derived(tabs.find((tab) => tab.name === activeTab)?.component);
</script>

<div class="flex flex-col bg-primary min-h-screen">
    <div role="tablist" class="flex-item tabs tabs-boxed p-2">
        {#each tabs as tab}
            <button
                role="tab"
                class="tab {tab.name === activeTab ? 'tab-active' : ''}"
                onclick={() => (activeTab = tab.name)}
            >
                {tab.name}
            </button>
        {/each}
    </div>

    <div class="relative flex flex-grow">
        <SvelteComponent
        ></SvelteComponent>
    </div>
</div>

<style>
</style>
