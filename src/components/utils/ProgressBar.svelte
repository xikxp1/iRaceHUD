<script lang="ts">
    interface Props {
        value?: number;
        min?: number;
        max?: number;
        cls?: string;
        elem_cls?: string;
    }

    let {
        value = 0,
        min = 0,
        max = 100,
        cls = "flex flex-col flex-nowrap justify-end w-2 h-20 rounded-full bg-neutral-content overflow-hidden outline outline-2 outline-offset-2 outline-primary",
        elem_cls = "rounded-full overflow-hidden bg-accent",
    }: Props = $props();
</script>

<div
    class={cls}
    role="progressbar"
    aria-valuenow={value}
    aria-valuemin={min}
    aria-valuemax={max}
>
    <div class={elem_cls} style="height: {value}%"></div>
</div>

<style>
</style>
