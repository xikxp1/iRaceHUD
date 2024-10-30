<script lang="ts">
    export let colorClasses: string = "";
    export let outlineClasses: string = "";
    export let textClasses: string = "";
    export let text: string = "";
</script>

{#if text}
    <div class="rounded-md {colorClasses} {outlineClasses}">
        <span class="text {textClasses}">{text}</span>
    </div>
{/if}

<style>
</style>
