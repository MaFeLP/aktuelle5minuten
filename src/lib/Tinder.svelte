<script lang="ts">
    import NavBar from "./NavBar.svelte";
    import '../assets/dlf.sass';
    import ErrorAlert from "./components/ErrorAlert.svelte";
    import TinderCard from "./components/TinderCard.svelte";
    import Loading from "./components/Loading.svelte";
    import TinderProgress from "./components/TinderProgress.svelte";
    import {type Article, ArticleApi, StatusApi} from "../api-client";

    const articleApi = new ArticleApi();
    const statusApi = new StatusApi();

    let articlePromise: Promise<Article | 'DONE'> | null = null;
    let counter = 0;
    let maxCounterPromise = statusApi.count();

    let newArticle = () => {
        articlePromise = new Promise((resolve, reject) => {
            articleApi.getFirst()
                .then((article) => { resolve(article )})
                .catch((res) => {
                    console.error("API response not ok!", res);
                    reject("API response not ok!");
                })
        });
    }

    newArticle();
</script>

<NavBar title="Nachrichten Tinder" />

<main>
    <section class="content">
        {#if articlePromise !== null}
            {#await articlePromise}
                <Loading />
            {:then article}
                {#if article === 'DONE'}
                    <div class="alert alert-success">
                        <h3 class="alert-heading">Keine Artikel zum Tindern in der Datenbank!</h3>
                    </div>
                {:else}
                    <TinderProgress counter={counter} progressPromise={maxCounterPromise} />
                    <TinderCard article={article} newArticle={newArticle} />
                {/if}
            {:catch error}
                <ErrorAlert error={error} body="Es ist ein Fehler beim Laden der Daten aufgetreten! Dies kann daran liegen, dass keine Artikel mehr zum tindern in der Datenbank sind oder ein interner Fehler aufgetreten ist." />
            {/await}
        {:else}
            <Loading />
        {/if}
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
