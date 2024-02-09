<script lang="ts">
  import Home from "./lib/Home.svelte";
  import Tinder from "./lib/Tinder.svelte";
  import PDFList from "./lib/PDFList.svelte";
  import PDFCreate from "./lib/PDFCreate.svelte";

  let path = window.location.pathname;
</script>

<div>
  {#if path === "/tinder"}
    <Tinder />
  {:else if path === "/pdflist"}
    <PDFList />
  {:else if path === "/pdfcreate"}
    <PDFCreate />
  {:else}
    <Home />
  {/if}
</div>
