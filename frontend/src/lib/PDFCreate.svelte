<script lang="ts">
  import NavBar from "./NavBar.svelte";
  import CreateCard from "./components/CreateCard.svelte";
  import Loading from "./components/Loading.svelte";
  import CreateProgress from "./components/CreateProgress.svelte";

  import { CategoryApi } from "../api-client";

  const categoryApi = new CategoryApi();
  let categoryPromise = categoryApi
    .getAll({ print: true })
    .then((categories) => {
      if (categories.length < 1) {
        console.info("Not enough articles in category!");
        window.location.href = "/";
      }
      return categoryApi.summary({ category: categories[0] });
    });
</script>

<NavBar title="PDF Erstellen" />

<main>
  <section>
    {#await categoryPromise}
      <Loading />
    {:then category}
      <CreateProgress />
      <CreateCard title={category.category} content={category.text} />
    {:catch err}
      <div>Error!</div>
      <code>{err}</code>
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
