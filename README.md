# iRaceHUD

[iRacing](https://www.iracing.com/) HUD overlay built in [Tauri](https://tauri.app/)

## Development

### Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Fetching new track SVG paths

Script downloads track maps from iracing API and saves them into a static file. Generally it should be used when new tracks are released.

Set environment variables:

```
$env:IRACING_LOGIN = '<iracing login>'
$env:IRACING_PWD = '<iracing password>'
```

Run script via pnpm:

```
pnpm run fetch-track-paths
```

Script will report its progress and final state upon finish.

Track paths will be saved to [track_paths.json](./static/track_paths/track_paths.json).
