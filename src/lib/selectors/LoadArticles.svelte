<script lang="ts">
    let show: 'normal' | 'loading' | 'done' = 'normal';

    function loadNew() {
        show = 'loading';

        fetch('/load')
            .then((res) => {
                if (res.ok) {
                    console.info("New articles imported into database");
                } else {
                    show = 'normal';
                    alert("Fehler! Neue Artikel konnten nicht importiert werden!");
                    console.error("New articles loaded into the database", res);
                }
            })
            .catch((err) => {
                show = 'normal';
                alert("Fehler! Neue Artikel konnten nicht importiert werden!");
                console.error("Could not load new articles into the database!", err);
            });
    }
</script>

{#if show === 'loading'}
    <button id="btn-load-new" class="btn btn-outline-success disabled">
        <span role="status">Artikel geladen!</span>
        <i class="bi bi-check2-square" />
    </button>
{:else if show === 'done'}
    <button id="btn-load-new" class="btn btn-outline-primary disabled">
        <span role="status">Laden... </span>
        <span class="spinner-border spinner-border-sm" aria-hidden="true"></span>
    </button>
{:else}
    <button id="btn-load-new" class="btn btn-outline-primary" on:click={loadNew}>
        Neue Artikel laden
    </button>
{/if}
