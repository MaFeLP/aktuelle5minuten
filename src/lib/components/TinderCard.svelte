<script lang="ts">
    import PromoteDialog from "./PromoteDialog.svelte";

    export let newArticle: () => void;

    export let article: any;

    async function promote(category: string) {
        console.debug("Promoting article", article)
        let response = await fetch(`/promote/${category}/${article["key"]}`);
        console.debug("Response from promoting the article", response)
        if (response.ok) {
            newArticle();
        } else {
            console.error("Could not demote article:", article, response);
            alert("Something went wrong, while demoting the article!");
        }
    }

    async function demote() {
        console.debug("Demoting article", article)
        let response = await fetch(`/demote/${article["key"]}`);
        console.debug("Response from demoting the article", response)
        if (response.ok) {
            newArticle();
        } else {
            console.error("Could not demote article:", article, response);
            alert("Something went wrong, while demoting the article!");
        }
    }
</script>

<div id="tinder-card" class="card">
    <img
            class="card-img-top"
            loading="lazy"
            src="{article['figures'][0]['image']['src']}"
            alt="{article['figures'][0]['image']['alt']}"
            srcset="{article['figures'][0]['image']['srcset']}"
            title="{article['figures'][0]['image']['title']}"
    />
    <div class="card-body">
        <h5 class="card-title">{article["kicker"]} - {article["title"]}</h5>
        <small class="card-subtitle">
            {article["description"]}
            ({new Date(article["date"]).toLocaleDateString()} um {new Date(article["date"]).toLocaleTimeString()})
        </small>

        <hr />

        <p class="card-text">{@html article["content"]["html"]}</p>
        <div id="button-row" class="row align-items-center">
            <button class="select-btn col btn btn-outline-success" on:click={() => {
                document.getElementById('promote-dialog').showModal();
            }}>
                <i class="bi bi-check2"></i>
            </button>
            <button class="select-btn col btn btn-outline-danger" on:click={demote}>
                <i class="bi bi-x-lg"></i>
            </button>
        </div>
    </div>
</div>

<PromoteDialog promote={promote} closeDialog={() => {
  document.getElementById('promote-dialog').close();
}}/>

<!--
<code>
    {JSON.stringify(article)}
</code>
-->

<style lang="sass">
  #tinder-card
    margin: 2em 0

  #button-row
    display: flex
    align-content: center
    justify-content: start

    .select-btn
      margin: 0 25px
</style>
