<script lang="ts">
  import NavBar from "./NavBar.svelte";
  import "../assets/dlf.sass";
  import ErrorAlert from "./components/ErrorAlert.svelte";
  import TinderCard from "./components/TinderCard.svelte";
  import Loading from "./components/Loading.svelte";
  import TinderProgress from "./components/TinderProgress.svelte";
  import { type Article, ArticleApi, StatusApi } from "../api-client";

  const articleApi = new ArticleApi();
  const statusApi = new StatusApi();

  const query = new URLSearchParams(window.location.search);
  const query_date =
    query.get("date") == null ? null : new Date(query.get("date")!);

  let articlePromise: Promise<Article | "DONE"> | null = null;
  let counter = 0;
  let maxCounterPromise =
    query_date == null || isNaN(query_date!.getDate())
      ? statusApi.count()
      : statusApi.count({
          articleDate: query_date!.toISOString().slice(0, 10),
        });

  let newArticle = () => {
    // No query date was provided or the date is invalid
    if (query_date == null || isNaN(query_date!.getDate()))
      // get the first general article
      articlePromise = new Promise((resolve, reject) => {
        articleApi
          .getFirst()
          .then((article) => {
            counter++;
            resolve(article);
          })
          .catch((res) => {
            console.error("API response not ok!", res);
            reject("API response not ok!");
          });
      });
    else
      articlePromise = new Promise((resolve, reject) => {
        articleApi
          .getFirst({ articleDate: query_date!.toISOString().slice(0, 10) })
          .then((article) => {
            counter++;
            resolve(article);
          })
          .catch((res) => {
            console.error("API response not ok!", res);
            reject("API response not ok!");
          });
      });
  };

  newArticle();
</script>

<NavBar title="Nachrichten Tinder" />

<main>
  <section class="content">
    {#if articlePromise !== null}
      {#await articlePromise}
        <Loading />
      {:then article}
        {#if article === "DONE"}
          <div class="alert alert-success">
            <h3 class="alert-heading">
              Keine Artikel zum Tindern in der Datenbank!
            </h3>
          </div>
        {:else}
          <TinderProgress {counter} progressPromise={maxCounterPromise} />
          <TinderCard {article} {newArticle} />
        {/if}
      {:catch error}
        <ErrorAlert
          {error}
          body="Es ist ein Fehler beim Laden der Daten aufgetreten! Dies kann daran liegen, dass keine Artikel mehr zum tindern in der Datenbank sind oder ein interner Fehler aufgetreten ist."
        />
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
