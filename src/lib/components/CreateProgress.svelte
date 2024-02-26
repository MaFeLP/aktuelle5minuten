<script lang="ts">
    import ErrorAlert from "./ErrorAlert.svelte";

    let promise = new Promise<number>((resolve, reject) => {
        fetch("/count")
            .then((res) => {
                if (!res.ok) {
                    console.error("Could not load Progress!", res);
                    reject("Could not load Progress!");
                }
                return res.json();
            })
            .then((progress: Progress) => {
                resolve(progress.categories);
            })
            .catch((err) => {
                console.error("Error getting progress!", err);
                reject(err);
            });
    });
</script>

<div>
    {#await promise}
        <div id="loading-text" class="text-muted">
            <span role="status">Laden...</span>
            <span class="spinner-border spinner-border-sm" aria-hidden="true"></span>
        </div>
    {:then progress}
        <p>Noch <code>{progress - 1}</code> weitere Kategorien...</p>
    {:catch err}
        <ErrorAlert heading="Fehler!" body="Fortschritt konnte nicht geladen werden!" error={err} />
    {/await}
</div>
