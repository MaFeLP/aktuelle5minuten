<script lang="ts">
    async function showConfirmationDialog() {
        if (window.confirm("Artikel älter als einen Monat unwiderruflich löschen?")) {
            let res = await fetch("/clean");
            if (res.ok) {
                let self = document.getElementById('btn-reset-db')!;
                self.classList.replace('btn-danger', 'btn-outline-success');
                self.classList.add('disabled');
                self.innerText = self.innerText + ' ✅';
                console.info("Database has been cleaned successfully!");
            } else {
                console.error("Error cleaning the database!", res);
            }
            console.debug(res);
        }
    }
</script>

<button id="btn-reset-db" class="btn btn-danger" on:click={showConfirmationDialog}>
    Artikel älter als 1mo löschen
</button>
