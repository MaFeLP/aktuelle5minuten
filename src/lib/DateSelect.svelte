<script lang="ts">
  import { ArticleApi } from "../api-client";
  import NavBar from "./NavBar.svelte";
  import ErrorAlert from "./components/ErrorAlert.svelte";
  import Loading from "./components/Loading.svelte";

  const articleApi = new ArticleApi();

  let datesPromise = articleApi.getDates();
</script>

<NavBar title="Nachrichten Tinder" />

<main>
  <section class="content">
    {#await datesPromise}
      <Loading />
    {:then dates}
      <div id="selectors" class="btn-group-vertical" role="group">
        <a href="/tinder" class="btn btn-outline-secondary">Alle Tage</a>
        {#each dates as date}
          <a href="/tinder?date={date}" class="btn btn-outline-secondary"
            >{new Date(date).toLocaleDateString()}</a
          >
        {/each}
      </div>
    {:catch error}
      <ErrorAlert
        {error}
        body="Es ist ein Fehler beim Laden der Daten aufgetreten!"
      />
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

  #selectors
    width: fit-content
    display: flex
    flex-direction: column
    align-content: center
    margin: auto
</style>
