<script lang="ts">
    interface Props {
        colorClasses?: string;
        outlineClasses?: string;
        textClasses?: string;
        text?: string;
        extraStyles?: string;
    }

    let {
        colorClasses = "",
        outlineClasses = "",
        textClasses = "",
        text = "",
        extraStyles = "",
    }: Props = $props();
</script>

{#if text}
    <div class="rounded-md {colorClasses} {outlineClasses}" style={extraStyles}>
        <span class="text {textClasses}" style={extraStyles}>{text}</span>
    </div>
{/if}

<style>
</style>
