<script lang="ts">
    export let license: string = "";
    export let irating: string = "";

    function getBadgeColor(license: string) {
        switch (license.charAt(0)) {
            case "R":
                return "bg-red-600";
            case "D":
                return "bg-orange-600";
            case "C":
                return "bg-yellow-600";
            case "B":
                return "bg-green-700";
            case "A":
                return "bg-blue-800";
            case "P":
                return "bg-black";
            default:
                return "bg-red-600";
        }
    }
</script>

{#if license && irating}
    <span class="badge {getBadgeColor(license)}">
        <span class="text text-primary text-sm"
            >{license.substring(0, 5)}&nbsp|&nbsp{irating}</span
        >
    </span>
{/if}

<style>
</style>
