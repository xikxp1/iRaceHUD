<script lang="ts">
    interface Props {
        colorClasses?: string;
        outlineClasses?: string;
        textClasses?: string;
        text?: string;
    }

    let {
        colorClasses = "",
        outlineClasses = "",
        textClasses = "",
        text = ""
    }: Props = $props();
</script>

{#if text}
    <div class="rounded-md {colorClasses} {outlineClasses}">
        <span class="text {textClasses}">{text}</span>
    </div>
{/if}

<style>
</style>
