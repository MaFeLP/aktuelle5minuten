<script lang="ts">
    import NavBar from "./NavBar.svelte";
    import '../assets/dlf.sass';
    import ErrorAlert from "./components/ErrorAlert.svelte";
    import TinderCard from "./components/TinderCard.svelte";

    let articlePromise = fetch("/article");

    let newArticle = () => {
        articlePromise = fetch("/article");
    }
</script>

<NavBar title="Nachrichten Tinder" />

<main>
    <section class="content">
        <div class="alert alert-info">
            <h2 class="alert-heading">
                Info
            </h2>
            <div>
                Dieser Teil der Webseite ist leider noch nicht verfügbar!
            </div>
            <a class="btn btn-primary" href="/">
                Zurück zum Startbildschirm
            </a>
        </div>

        {#await articlePromise}
            <small class="text-muted">Laden...</small>
        {:then articleResponse}
            {#if articleResponse.ok}
                {#await articleResponse.json()}
                    <small class="text-muted">Verarbeiten...</small>
                {:then article}
                    <TinderCard article={article} newArticle={newArticle} />
                {:catch error}
                    <ErrorAlert error={error} body="Es ist ein Fehler beim Verarbeiten der Daten aufgetreten!" />
                {/await}
            {:else if articleResponse.status === 404}
                <div class="alert alert-success">
                    <h3 class="alert-heading">Keine Artikel zum Tindern in der Datenbank!</h3>
                </div>
            {/if}
        {:catch error}
            <ErrorAlert error={error} body="Es ist ein Fehler beim Laden der Daten aufgetreten!" />
        {/await}
    </section>
</main>

<style lang="sass">
  main
    display: flex
    flex-direction: column
    place-content: center
    gap: 1.5em
    padding: 0 5vw

  section
    max-width: 1024px
    width: 90vw
    margin: auto
</style>
