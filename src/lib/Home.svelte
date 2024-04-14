<script lang="ts">
  import newsLogo from "/news.svg";
  import LoadArticles from "./selectors/LoadArticles.svelte";
  import ResetDB from "./selectors/ResetDB.svelte";
</script>

<main>
  <img id="logo" src={newsLogo} alt="News Logo" />
  <h1>Aktuelle 5 Minuten</h1>

  <div id="selectors">
    <a href="/pdflist" class="btn btn-outline-secondary"
      >PDF Dateien auflisten</a
    >
    <LoadArticles />
    <a href="/dates" class="btn btn-success">Nachrichten Tinder spielen</a>
    <a href="/pdfcreate" class="btn btn-outline-warning">PDF erstellen</a>
    <ResetDB />
  </div>
</main>

<style lang="sass">
  main
    display: flex
    flex-direction: column
    align-content: center
    text-align: center
    gap: 1.5em

  #logo
    height: 12em
    will-change: filter
    transition: filter 300ms

  #selectors
    width: fit-content
    display: flex
    flex-direction: column
    gap: 0.5em
    align-content: center
    margin: auto
</style>
