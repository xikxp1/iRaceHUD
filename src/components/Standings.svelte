<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import {
        createGrid,
        type GetRowIdParams,
        type GridApi,
        type GridOptions,
        type NewValueParams,
    } from "ag-grid-community";
    import "ag-grid-community/styles/ag-grid.css";
    import "ag-grid-community/styles/ag-theme-quartz.css";
    import { onMount } from "svelte";

    let stength_of_field = 0;
    let current_time = "––:––";
    let driver_count = 0;

    let standings: HTMLDivElement;

    interface Standings {
        car_id: number;
        position: number;
        user_name: string;
        car_number: string;
        irating: string;
        leader_gap: string;
        best_lap: string;
        last_lap: string;
        is_player: boolean;
    }

    let gridApi: GridApi<Standings>;

    const gridOptions: GridOptions<Standings> = {
        rowData: [],
        defaultColDef: {
            sortable: false,
        },
        suppressHorizontalScroll: true,
        columnDefs: [
            {
                field: "position",
                headerName: "P",
                resizable: false,
                width: 50,
                pinned: "left",
                cellClass: "ag-right-aligned-cell",
                headerClass: "ag-right-aligned-header",
                onCellValueChanged: (
                    params: NewValueParams<Standings, Standings>,
                ) => {
                    // TODO: properly highlight changes
                },
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
                flex: 1,
            },
            {
                field: "irating",
                headerName: "iR",
                resizable: false,
                width: 60,
                cellClass: "ag-right-aligned-cell",
                headerClass: "ag-right-aligned-header",
            },
            {
                field: "leader_gap",
                headerName: "Gap",
                resizable: false,
                width: 60,
                cellClass: "ag-right-aligned-cell",
                headerClass: "ag-right-aligned-header",
            },
        ],
        getRowId: (params: GetRowIdParams) => String(params.data.car_id),
        icons: {
            sortAscending: "\xa0",
            sortDescending: "\xa0",
        },
        rowClassRules: {
            player: "data.is_player",
            odd: "!data.is_player && data.position % 2 === 1",
        },
        overlayNoRowsTemplate: "Time to race 🏁",
    };

    onMount(() => {
        gridApi = createGrid(standings, gridOptions);
    });

    listen("strength_of_field", (event) => {
        stength_of_field = event.payload as number;
    });

    listen("current_time", (event) => {
        current_time = event.payload as string;
    });

    listen("standings", (event) => {
        let newRowData = event.payload as Standings[];
        driver_count = newRowData.length;
        newRowData.sort((a, b) => a.position - b.position);
        let playerRowDataIdx = newRowData.findIndex((row) => row.is_player);
        let newLeaderRowData = newRowData.splice(0, 1);
        gridApi.updateGridOptions({
            rowData: newRowData,
            pinnedTopRowData: newLeaderRowData,
        });
        gridApi.ensureIndexVisible(playerRowDataIdx, "middle");
    });
</script>

<div class="align-right" />
<div class="flex flex-row items-center justify-center opacity-75">
    <div class="flex flex-col bg-primary-content rounded-md w-[400px]">
        <div class="flex flex-row items-center justify-center">
            <div class="flex flex-row items-center justify-start w-1/6 pl-2">
                <span class="text text-secondary">SoF&nbsp</span>
                <span class="text text-primary">{stength_of_field}</span>
            </div>
            <div class="flex flex-row items-center justify-center w-4/6">
                <span class="text text-primary">{current_time}</span>
            </div>
            <div class="flex flex-row items-center justify-end w-1/6 pr-2">
                <span class="text text-primary">{driver_count}&nbsp;</span>
                <img src="/icons/helmet.svg" alt="" />
            </div>
        </div>
        <div
            bind:this={standings}
            id="standings"
            class="ag-theme-quartz ag-theme-iracing h-[140px]"
        ></div>
    </div>
</div>

<style>
</style>
