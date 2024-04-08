<script lang="ts">
  import { ActionsApi } from "../../api-client";

  let show: "normal" | "loading" | "done" = "normal";

  const actionsApi = new ActionsApi();

  function showConfirmationDialog() {
    if (
      window.confirm("Artikel älter als einen Monat unwiderruflich löschen?")
    ) {
      show = "loading";

      actionsApi
        .clean()
        .then(() => {
          console.info("New articles imported into database");
        })
        .catch((err) => {
          show = "normal";
          alert("Fehler! Neue Artikel konnten nicht importiert werden!");
          console.error("Could not load new articles into the database!", err);
        });
    }
  }
</script>

{#if show === "loading"}
  <button id="btn-load-new" class="btn btn-outline-success disabled">
    <span role="status">Gelöscht!</span>
    <i class="bi bi-check2-square" />
  </button>
{:else if show === "done"}
  <button id="btn-load-new" class="btn btn-outline-danger disabled">
    <span role="status">Laden... </span>
    <span class="spinner-border spinner-border-sm" aria-hidden="true"></span>
  </button>
{:else}
  <button
    id="btn-reset-db"
    class="btn btn-danger"
    on:click={showConfirmationDialog}
  >
    Artikel älter als 1mo löschen
  </button>
{/if}
