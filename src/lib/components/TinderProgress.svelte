<script lang="ts">
    import ErrorAlert from "./ErrorAlert.svelte";
    import type { Count } from "../../api-client";

    export let progressPromise: Promise<Count>;

    export let counter: number;
</script>

{#await progressPromise}
    <div id="loading-text" class="text-muted">
        <span role="status">Laden...</span>
        <span class="spinner-border spinner-border-sm" aria-hidden="true"></span>
    </div>
{:then maxCounter}
    <div class="progress"
         role="progressbar"
         aria-label="Tinder Progress"
         aria-valuenow="{counter}"
         aria-valuemin="0"
         aria-valuemax="{maxCounter.articles}"
    >
        <div class="progress-bar overflow-visible text-dark bg-info"
            style="width: {Math.round((counter / maxCounter.articles) * 100)}%"
        >
            {counter}/{maxCounter.articles}
        </div>
    </div>
{:catch err}
    <ErrorAlert heading="Fehler!" body="Fortschritt konnte nicht geladen werden!" error={err} />
{/await}
