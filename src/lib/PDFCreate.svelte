<script lang="ts">
    import NavBar from "./NavBar.svelte";
    import CreateCard from "./components/CreateCard.svelte";

    function nextCategory() {
        return new Promise((resolve, reject) => {
            fetch('/print_categories')
                .then((res) => {
                    console.debug("Received print_categories response:", res);
                    if (!res.ok) {
                        console.error(res);
                        reject("API hat nicht 'OK' zurückgegeben!");
                    }

                    return res.json()
                })
                .then((body) => {
                    console.debug("print_categories JSON is", body);
                    if (body.length < 1) {
                        console.info("Not enough articles in category!")
                        reject("Nicht genug Artikel in dieser Kategorie!");
                    }
                    return fetch(`/category/${body[0]}`);
                })
                .then((res) => {
                    console.debug("Received Category response:", res);
                    if (!res.ok) {
                        console.error("API error (#2):", res);
                        reject("API hat nicht 'OK' zurückgegeben! (#2)");
                    }

                    return res.json()
                })
                .then((json) => {
                    console.debug("Category Content:", json);
                    resolve(json);
                })
                .catch((err) => {
                    console.error("Fehler beim Verarbeiten der Daten!", err);
                    reject("Fehler beim Verarbeiten der Daten!");
                });
        });
    }

    let categoryPromise = nextCategory();
</script>

<NavBar title="PDF Erstellen" />

<main>
    <section>
        {#await categoryPromise}
            <small class="text-muted">Laden...</small>
        {:then category}
            <CreateCard title="{category['category']}" content="{category['text']}" nextCategory="{nextCategory}" />

            <code>{JSON.stringify(category)}</code>
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