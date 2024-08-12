<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    let flag: string = "";

    listen("flag", (event) => {
        flag = event.payload as string;
    });
</script>

<div class="flex flex-row items-center justify-center opacity-80">
    {#if flag !== ""}
        <div
            class="flex flex-col bg-accent-content items-center justify-center rounded-md w-[8%] outline-1 outline-dotted outline-accent stripes stripes-primary stripes-size-lg stripes-reverse stripes-opacity-30"
        >
            <div class="text-primary text-2xl">{flag}</div>
        </div>
    {/if}
</div>

<style>
</style>
