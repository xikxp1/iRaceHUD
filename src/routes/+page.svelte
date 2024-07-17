<script lang="ts">
  import Main from "../components/Main.svelte";
</script>

<div class="outer">
  <div class="middle">
    <div class="main">
      <Main />
    </div>
  </div>
</div>

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
    margin-top: 200px;
    align-items: center;
    /* width: 50px; */
  }
</style>
