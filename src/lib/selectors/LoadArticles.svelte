<script lang="ts">
    async function loadNew() {
        let res = await fetch('/load');
        console.debug(res);
        if (res.ok) {
            let self = document.getElementById('btn-load-new')!;
            self.classList.replace('btn-outline-primary', 'btn-outline-success');
            self.classList.add('disabled');
            self.innerText = self.innerText + ' ✅';
            console.info("New articles imported into database");
        } else {
            alert("Fehler! Neue Artikel konnten nicht importiert werden!");
        }
    }
</script>

<button id="btn-load-new" class="btn btn-outline-primary" on:click={loadNew}>
    Neue Artikel laden
</button>
