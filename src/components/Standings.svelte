<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import {
        createGrid,
        type GetRowIdParams,
        type GridApi,
        type GridOptions,
    } from "ag-grid-community";
    import "ag-grid-community/styles/ag-grid.css";
    import "ag-grid-community/styles/ag-theme-quartz.css";
    import { onMount } from "svelte";

    let standings: HTMLDivElement;

    interface Standings {
        car_id: number;
        position: number;
        user_name: string;
        car_number: string;
        leader_gap: string;
        player_gap: string;
        is_player: boolean;
    }

    let gridApi: GridApi<Standings>;

    const gridOptions: GridOptions<Standings> = {
        rowData: [],
        defaultColDef: {
            sortable: false,
        },
        columnDefs: [
            {
                field: "position",
                headerName: "Pos",
                resizable: false,
                width: 70,
                pinned: "left",
                sortable: true,
                sort: "asc",
                cellClass: "ag-right-aligned-cell",
                headerClass: "ag-right-aligned-header",
                enableCellChangeFlash: true,
            },
            {
                field: "car_number",
                headerName: "#",
                resizable: false,
                width: 60,
                cellClass: "ag-right-aligned-cell",
                headerClass: "ag-right-aligned-header",
            },
            {
                field: "user_name",
                headerName: "Name",
                resizable: false,
                width: 200,
            },
            {
                field: "leader_gap",
                headerName: "Int",
                resizable: false,
                width: 70,
            },
            {
                field: "player_gap",
                headerName: "Gap",
                resizable: false,
                width: 70,
            },
        ],
        getRowId: (params: GetRowIdParams) => String(params.data.car_id),
        icons: {
            sortAscending: " ",
            sortDescending: " ",
        },
        rowClassRules: {
            player: "data.is_player",
        },
    };

    onMount(() => {
        gridApi = createGrid(standings, gridOptions);
    });

    listen("standings", (event) => {
        gridApi.setGridOption("rowData", event.payload as Standings[]);
    });
</script>

<div class="align-right" />
<div class="flex flex-row items-center justify-center opacity-75">
    <div
        bind:this={standings}
        id="standings"
        class="ag-theme-quartz ag-theme-iracing w-[480px] h-[210px]"
    ></div>
</div>

<style>
</style>
