<script lang="ts">
    import { Card } from "flowbite-svelte";
</script>

<Card>
    <div class="container rounded-xl outline outline-1 outline-orange-500">
        <div class="gear">
            <b>6</b>
        </div>
        <div class="speed">
            <b>156</b>
        </div>
    </div>
</Card>

<style>
    .container {
        box-shadow: inset 0 0 100px rgb(0 0 0 / 40%);
    }
    .gear {
        text-align: center;
        font-size: 44px;
        color: aliceblue;
    }
    .speed {
        text-align: center;
        font-size: 22px;
        color: aliceblue;
        margin-top: -14px;
    }
</style>
