<script lang="ts">
  export let error: Error;
  export let heading: string = "Fehler!";
  export let body: string;
</script>

<div class="alert alert-danger">
  <h2 class="alert-heading">{heading}</h2>
  <div>{body}</div>
  <div><code>{error}</code></div>
</div>
