<script lang="ts">
    import NavBar from "./NavBar.svelte";
    import '../assets/dlf.sass';
    import ErrorAlert from "./components/ErrorAlert.svelte";
    import TinderCard from "./components/TinderCard.svelte";
    import Loading from "./components/Loading.svelte";

    let articlePromise: Promise<any> | null = null;
    const done = 'DONE';

    let newArticle = () => {
        articlePromise = new Promise((resolve, reject) => {
            fetch("/article")
                .then((res) => {
                    if (!res.ok) {
                        if (res.status === 404) {
                            console.log("No tinders left!");
                            resolve(done);
                        }
                        console.error("API response not ok!", res);
                        reject("API response not ok!");
                    }
                    return res.json();
                })
                .then((json) => {
                    resolve(json);
                })
                .catch((err) => {
                    console.error("Error processing '/article/!", err)
                });
        });
    }

    newArticle();
</script>

<NavBar title="Nachrichten Tinder" />

<main>
    <section class="content">
        {#await articlePromise}
            <Loading />
        {:then article}
            {#if article === done}
                <div class="alert alert-success">
                    <h3 class="alert-heading">Keine Artikel zum Tindern in der Datenbank!</h3>
                </div>
            {:else}
                <TinderCard article={article} newArticle={newArticle} />
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
