<script lang="ts">
    export let value: number = 0;
    export let min: number = 0;
    export let max: number = 100;
    export let cls: string =
        "flex flex-col flex-nowrap justify-end w-2 h-20 rounded-full bg-neutral-content overflow-hidden outline outline-2 outline-offset-2 outline-primary";
    export let elem_cls: string = "rounded-full overflow-hidden bg-accent";
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
