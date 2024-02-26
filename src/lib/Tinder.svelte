<script lang="ts">
    import NavBar from "./NavBar.svelte";
    import '../assets/dlf.sass';
    import ErrorAlert from "./components/ErrorAlert.svelte";
    import TinderCard from "./components/TinderCard.svelte";
    import Loading from "./components/Loading.svelte";
    import TinderProgress from "./components/TinderProgress.svelte";

    let articlePromise: Promise<DlfArticle | 'DONE'> | null = null;
    let counter = 0;
    let maxCounterPromise: Promise<number> = new Promise<number>((resolve, reject) => {
        fetch("/count")
            .then((res) => {
                if (!res.ok) {
                    console.error("Could not load progress count!", res);
                    reject("Could not load progress count!");
                }
                return res.json()
            })
            .then((progress: Progress) => {
                console.debug("Received progress status:", progress);
                resolve(progress.articles);
            })
            .catch((err) => {
                console.error("Something went wrong processing the progress.", err);
                reject(err);
            });

    });

    let newArticle = () => {
        articlePromise = new Promise((resolve, reject) => {
            fetch("/article")
                .then((res) => {
                    if (!res.ok) {
                        if (res.status === 404) {
                            console.log("No tinders left!");
                            resolve('DONE');
                        }
                        console.error("API response not ok!", res);
                        reject("API response not ok!");
                    }
                    return res.json();
                })
                .then((json) => {
                    // Increment counter
                    counter += 1;
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
                <ErrorAlert error={error} body="Es ist ein Fehler beim Laden der Daten aufgetreten!" />
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
