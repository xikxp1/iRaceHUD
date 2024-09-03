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
                onCellValueChanged: (params: NewValueParams<Standings, Standings>) => {
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
    };

    onMount(() => {
        gridApi = createGrid(standings, gridOptions);
    });

    listen("standings", (event) => {
        let newRowData = event.payload as Standings[];
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
    <div
        bind:this={standings}
        id="standings"
        class="ag-theme-quartz ag-theme-iracing w-[400px] h-[140px]"
    ></div>
</div>

<style>
</style>
