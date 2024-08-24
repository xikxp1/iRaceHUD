<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import Main from "../components/Main.svelte";
  import Telemetery from "../components/Telemetry.svelte";
  import Timer from "../components/Timer.svelte";
  import Session from "../components/Session.svelte";
  import Subtimer from "../components/SubTimer.svelte";

  let active: boolean = false;

  listen("active", (event) => {
    active = event.payload as boolean;
  });
</script>

{#if active}
  <div class="outer">
    <div class="middle">
      <div class="main">
        <Main />
      </div>
    </div>
  </div>
  <div class="outer">
    <div class="middle">
      <div class="telemetry">
        <Telemetery />
      </div>
    </div>
  </div>
  <div class="outer">
    <div class="middle">
      <div class="timer">
        <Timer />
      </div>
    </div>
  </div>
  <div class="outer">
    <div class="middle">
      <div class="session">
        <Session />
      </div>
    </div>
  </div>
  <div class="outer">
    <div class="middle">
      <div class="subtimer">
        <Subtimer />
      </div>
    </div>
  </div>
{/if}

<style>
  .outer {
    display: table;
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
  }

  .middle {
    display: table-cell;
    vertical-align: middle;
  }

  .main {
    margin-left: auto;
    margin-right: auto;
    margin-top: 280px;
    margin-bottom: auto;
    align-items: center;
  }

  .telemetry {
    margin-left: auto;
    margin-right: auto;
    margin-top: 454px;
    margin-bottom: auto;
    align-items: center;
  }

  .timer {
    margin-left: auto;
    margin-right: auto;
    margin-top: -335px;
    margin-bottom: auto;
    align-items: center;
  }

  .session {
    margin-left: -1400px;
    margin-right: auto;
    margin-top: -520px;
    margin-bottom: auto;
    align-items: center;
  }

  .subtimer {
    margin-left: auto;
    margin-right: auto;
    margin-top: -295px;
    margin-bottom: auto;
    align-items: center;
  }
</style>
