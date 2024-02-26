<script lang="ts">
    import ErrorAlert from "./ErrorAlert.svelte";

    export let progressPromise: Promise<number>;

    export let counter: number;
</script>

{#await progressPromise}
    <div id="loading-text" class="text-muted">
        <span role="status">Laden...</span>
        <span class="spinner-border spinner-border-sm" aria-hidden="true"></span>
    </div>
{:then maxArticles}
    <div class="progress"
         role="progressbar"
         aria-label="Tinder Progress"
         aria-valuenow="{counter}"
         aria-valuemin="0"
         aria-valuemax="{maxArticles}"
    >
        <div class="progress-bar overflow-visible text-dark bg-info"
            style="width: {Math.round((counter / maxArticles) * 100)}%"
        >
            {counter}/{maxArticles}
        </div>
    </div>
{:catch err}
    <ErrorAlert heading="Fehler!" body="Fortschritt konnte nicht geladen werden!" error={err} />
{/await}
