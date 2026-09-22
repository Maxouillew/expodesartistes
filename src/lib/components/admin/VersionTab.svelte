<script lang="ts">
  import UpdateChecker from "$lib/components/admin/UpdateChecker.svelte";
</script>

<section>
  <h2>Version et mises à jour</h2>
  <p class="intro">Vérifie si une nouvelle version de l'application est disponible et l'installe.</p>

  <div class="update-card">
    <UpdateChecker />
  </div>
</section>

<style lang="scss">
  @use "../../styles/variables" as *;

  .intro {
    color: $color-text-muted;
  }

  .update-card {
    background-color: $color-surface;
    border: 1px solid $color-border;
    border-radius: $radius;
    box-shadow: $shadow-card;
    padding: $space-6;
  }
</style>
