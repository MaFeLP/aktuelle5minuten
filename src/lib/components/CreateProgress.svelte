<script lang="ts">
    import ErrorAlert from "./ErrorAlert.svelte";
    import {type Count, StatusApi} from "../../api-client";

    const statusApi = new StatusApi();

    let promise = statusApi.count();
</script>

<div>
    {#await promise}
        <div id="loading-text" class="text-muted">
            <span role="status">Laden...</span>
            <span class="spinner-border spinner-border-sm" aria-hidden="true"></span>
        </div>
    {:then progress}
        <p>Noch <code>{progress.categories - 1}</code> weitere Kategorien...</p>
    {:catch err}
        <ErrorAlert heading="Fehler!" body="Fortschritt konnte nicht geladen werden!" error={err} />
    {/await}
</div>
