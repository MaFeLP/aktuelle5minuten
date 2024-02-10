<script lang="ts">
    import NavBar from "./NavBar.svelte";

    let filesPromise = fetch("/files");
</script>

<NavBar title="PDF Dateien" />

<main>
    {#await filesPromise}
        <small class="text-muted">Laden...</small>
    {:then filesResponse}
        {#await filesResponse.json()}
            <small class="text-muted">Verarbeiten...</small>
        {:then filesList}
            <div id="fileList" class="btn-group-vertical">
                {#each filesList as file}
                    <a class="btn btn-outline-secondary" href="/files/{file}" target="_blank">{file}</a>
                {/each}
            </div>
        {:catch error}
            <div class="alert alert-danger">
                <h3 class="alert-heading">Dateien konnten nicht verarbeitet werden!</h3>
                <code>{error}</code>
            </div>
        {/await}
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
