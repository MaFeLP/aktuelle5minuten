<script lang="ts">
    import ErrorAlert from "./ErrorAlert.svelte";

    export let promote: (category: string) => Promise<void>;

    export let closeDialog: () => void;

    async function onSubmit() {
        if ((document.getElementById('promote-dialog')! as HTMLDialogElement).returnValue === "cancel") {
            console.log("Aborting submission");
        } else {
            await promote((document.getElementById('category-input')! as HTMLInputElement).value);
            console.log("Promoting article...");
        }
    }

    let categoriesRequest = new Promise<string[]>((resolve, reject) => {
        fetch("/categories")
            .then((res) => {
                if (!res.ok) {
                    console.error("Error loading categories!", res);
                    reject("Error loading categories!");
                }
                return res.json();
            })
            .then((categories: string[]) => {
                console.debug("Received categories:", categories);
                resolve(categories);
            })
            .catch((err) => {
                console.error("Could not load categories!", err);
                reject(err);
            });
    });
</script>

<dialog id="promote-dialog" on:submit={onSubmit}>
    <form method="dialog">
        <div class="modal-header">
            <h5 class="modal-title">Post aufnehmen!</h5>
        </div>

        <hr/>

        <div class="modal-body">
            <label for="category-input" class="form-label">Kategorie auswählen</label>
            <input
                    class="form-control"
                    list="categories-options"
                    id="category-input"
                    placeholder="Innenpolitik"
                    minlength="1"
                    maxlength="63"
            >
            {#await categoriesRequest}
                <small class="text-muted hidden">Kategorien werden geladen...</small>
            {:then categories}
                <datalist id="categories-options">
                    {#each categories as category}
                        <option value="{category}">
                    {/each}
                </datalist>
            {:catch err}
                <ErrorAlert body="Kategories konnten nicht geladen werden!" error={err} />
            {/await}
        </div>

        <hr/>

        <div class="modal-footer">
            <button id="dialog-submit-btn" class="btn btn-success" type="submit">Abschicken</button>
            <button id="dialog-cancel-btn" class="btn btn-outline-danger" value="cancel" on:click={closeDialog} type="reset">Abbrechen</button>
        </div>
    </form>
</dialog>

<style lang="sass">
  dialog
    border: 1px solid rgba(0,0,0,.2)
    border-radius: .3rem
    outline: 0

    form
      position: relative
      display: flex
      flex-direction: column
      width: 100%
      max-width: 500px

  .modal-footer
    gap: 1rem

  #category-input
    width: 64em
    max-width: 100%

  /* Set a nice fade-in and out animation */
  dialog
    animation: fade-out 0.2s ease-out

  dialog[open]
    animation: fade-in 0.2s ease-out

  dialog[open]::backdrop
    animation: backdrop-fade-in 0.2s ease-out forwards

  /* Animation keyframes */
  @keyframes fade-in
    0%
      opacity: 0
      display: none

    100%
      opacity: 1
      display: block

  @keyframes fade-out
    0%
      opacity: 1
      display: block

    100%
      opacity: 0
      display: none

  @keyframes backdrop-fade-in
    0%
      background-color: rgb(0 0 0 / 0%)

    100%
      background-color: rgb(0 0 0 / 50%)
</style>
