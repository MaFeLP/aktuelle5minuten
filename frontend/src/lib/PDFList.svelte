<script lang="ts">
  import NavBar from "./NavBar.svelte";
  import Loading from "./components/Loading.svelte";
  import { FilesApi } from "../api-client";

  const filesApi = new FilesApi();
  let filesPromise = filesApi.list();
</script>

<NavBar title="PDF Dateien" />

<main>
  {#await filesPromise}
    <Loading />
  {:then files}
    <div id="fileList" class="btn-group-vertical">
      {#each files as file}
        <a
          class="btn btn-outline-secondary"
          href="/files/{file}"
          target="_blank">{file}</a
        >
      {/each}
    </div>
  {:catch error}
    <div class="alert alert-danger">
      <h3 class="alert-heading">Dateien konnten nicht geladen werden!</h3>
      <code>{error}</code>
    </div>
  {/await}
</main>

<style lang="sass">
  #fileList
    width: 100%

    a
      text-align: start
</style>
