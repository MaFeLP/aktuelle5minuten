<script lang="ts">
  import Home from "./lib/Home.svelte";
  import Tinder from "./lib/Tinder.svelte";
  import PDFList from "./lib/PDFList.svelte";
  import PDFCreate from "./lib/PDFCreate.svelte";
  import DateSelect from "./lib/DateSelect.svelte";

  let path = window.location.pathname;
</script>

<div>
  {#if path.startsWith("/tinder")}
    <Tinder />
  {:else if path === "/dates"}
    <DateSelect />
  {:else if path === "/pdflist"}
    <PDFList />
  {:else if path === "/pdfcreate"}
    <PDFCreate />
  {:else}
    <Home />
  {/if}
</div>
