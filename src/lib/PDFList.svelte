<script lang="ts">
    import NavBar from "./NavBar.svelte";
    import Loading from "./components/Loading.svelte";

    let filesPromise = new Promise<string[]>((resolve, reject) => {
        fetch("/files")
            .then((res) => {
                if (!res.ok) {
                    console.error("Could not load PDF files!", res);
                    reject("API is not ok!");
                }
                return res.json();
            })
            .then((json) => {
                console.debug("Received the file list:", json);
                resolve(json);
            })
            .catch((err) => {
                console.error("Retrieving files list failed!", err);
                reject(err);
            });
    });
</script>

<NavBar title="PDF Dateien" />

<main>
    {#await filesPromise}
        <Loading />
    {:then files}
        <div id="fileList" class="btn-group-vertical">
            {#each files as file}
                <a class="btn btn-outline-secondary" href="/files/{file}" target="_blank">{file}</a>
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
