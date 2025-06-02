<script lang="ts">
    document.addEventListener("mousedown", (e) => {
        if (e.button !== 0 || e.detail !== 1) {
            return;
        }

        const target = e.target as HTMLElement;
        if (!target.classList.contains("drag-region")) {
            return;
        }

        e.preventDefault();
        e.stopPropagation();

        // @ts-ignore
        window.__TAURI_INTERNALS__.invoke("plugin:window|start_dragging");
    });
</script>

<slot />
