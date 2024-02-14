<script lang="ts">
    import chatGPTIcon from '/icon-chatgpt.svg';

    const PROMPT = "Fasse folgende Artikel in kurzen Stichpunkten zusammen. Lasse unwichtige Informationen aus.\n\n";

    export let title: string;
    export let content: string;

    let chatGPTEnabledPromise = new Promise<boolean>((resolve, reject) => {
        fetch("/chatgpt")
            .then((res) => {
                console.debug("Received response for ChatGPT button", res);
                return res.json();
            })
            .then((json: ChatGPTEnabled) => {
                console.debug("ChatGPT button is enabled?", json);
                resolve(json.enabled);
            })
            .catch((err) => {
                console.error("Could not fetch the ChatGPT status", err);
                reject(err);
            })
    });

    let bulletsCopied = false;
    let bulletsPasted = false;

    function copy() {
        navigator.clipboard.writeText(`${PROMPT}${content}`)
            .then(() => {
                bulletsCopied = true;
            })
            .catch((err) => {
                console.error("Bullets could not be copied!", err);
            });
    }

    function paste() {
        navigator.clipboard.readText()
            .then((text) => {
                bulletsPasted = true;
                (document.querySelector('textarea#bullets-text')! as HTMLTextAreaElement).value = text;
            })
            .catch((err) => {
                console.error("Could not paste bullets!", err);
            });
    }
</script>

<div id="section-card" class="card">
    <h5 class="card-header">{title}</h5>
    <div class="card-body">
        <details class="card-text form-element">
            <summary>Text Anzeigen</summary>
            <label for="texts" class="form-label visually-hidden">Zusammengesammelte Texte</label>
            <textarea
                    class="form-control form-element"
                    id="texts"
                    readonly
                    rows="20"
            >{content}
            </textarea>
        </details>

        <div class="d-flex col">
            <button id="copy-btn" class="btn btn-outline-success" on:click={copy}>
                Texte kopieren
                {#if bulletsCopied}
                    <i class="bi bi-clipboard-check" />
                {:else}
                    <i class="bi bi-clipboard" />
                {/if}
            </button>
            <span class="flex-fill" />
            {#await chatGPTEnabledPromise}
                <span class="visually-hidden">ChatGPT Status lädt...</span>
            {:then chatGPTenabled}
                {#if chatGPTenabled}
                    <a id="chatgpt-btn" class="btn btn-outline-success" href="https://chat.openai.com/" target="_blank" title="Zu ChatGPT">
                        <img id="chatgpt-logo" src="{chatGPTIcon}" width="30px" height="30px" alt="ChatGPT Logo" />
                    </a>
                {/if}
            {:catch err}
                <span class="visually-hidden">ChatGPT Status hat einen Fehler!</span>
            {/await}
        </div>

        <hr />

        <form action="/bullets" method="post">
            <div class="visually-hidden">
                <input type="text" id="category-input" readonly value="{title}" name="category" />
            </div>
            <div class="form-element">
                <label for="bullets-text" class="form-label">Stichpunkte hier eingeben</label>
                <textarea
                    class="form-control"
                    id="bullets-text"
                    rows="20"
                    name="bullets"
                />
            </div>
            <div class="d-flex col">
                <button id="paste-btn" class="btn btn-outline-secondary" type="button" on:click={paste}>
                    Einfügen
                    {#if bulletsPasted}
                        <i class="bi bi-clipboard-plus" />
                    {:else}
                        <i class="bi bi-clipboard" />
                    {/if}
                </button>
                <span class="flex-fill" />
                <button id="submit-btn" class="btn btn-outline-success" type="submit">
                    Absenden <i class="bi bi-check2" />
                </button>
            </div>
        </form>
    </div>
</div>

<style lang="sass">
  #chatgpt-logo
    height: 2em
    width: auto

  .form-element
    margin-bottom: 1em

  details
    margin-bottom: 1em

  #submit-btn, #paste-btn
    width: 10em
</style>