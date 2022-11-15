<script>
  import {onMount} from "svelte";

  let resp = "N/A";
  onMount(async () => {
    const res = await fetch(`/api/hello`);
    resp = JSON.stringify(await res.json());
  });
</script>

<main>
  <div class="card">
    <span>From server: {resp}</span>
  </div>
</main>

<style>
</style>
