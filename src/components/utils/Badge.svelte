<script lang="ts">
    export let color: string = "";
    export let text: string = "";
</script>

{#if text}
    <div class="rounded-md text-center {color}">
        <span class="text text-primary text-sm">{text}</span>
    </div>
{/if}

<style>
</style>
