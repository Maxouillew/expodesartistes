<script lang="ts">
  import { drawVoter, type DrawnVoter } from "$lib/services/draw";
  import Button from "$lib/components/Button.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let winner = $state<DrawnVoter | null>(null);
  let error = $state<string | undefined>(undefined);
  let drawing = $state(false);
  let hasDrawnOnce = $state(false);

  async function handleDraw() {
    error = undefined;
    drawing = true;
    try {
      winner = await drawVoter();
      hasDrawnOnce = true;
    } catch (err) {
      error = String(err);
    } finally {
      drawing = false;
    }
  }
</script>

<section>
  <h2>Tirage au sort</h2>
  <p class="intro">Sélectionne un votant au hasard parmi tous les votants enregistrés.</p>

  {#if winner}
    {#key winner}
      <div class="winner animate-in" role="status">
        <div class="winner-icon"><Icon name="trophy" size={32} /></div>
        <p class="winner-name">{winner.firstName} {winner.lastName}</p>
        <p class="winner-phone">{winner.phone}</p>
      </div>
    {/key}
  {/if}

  {#if error}
    <p class="error" role="alert"><Icon name="alert-circle" size={18} />{error}</p>
  {/if}

  <Button icon="shuffle" onclick={handleDraw} disabled={drawing}>
    {hasDrawnOnce ? "Tirer à nouveau" : "Tirer au sort"}
  </Button>
</section>

<style lang="scss">
  @use "../../styles/variables" as *;

  .intro {
    color: $color-text-muted;
  }

  .winner {
    background-color: $color-surface;
    border: 1px solid $color-border;
    border-radius: $radius;
    box-shadow: $shadow-card;
    padding: $space-6;
    margin-bottom: $space-4;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: $space-2;
  }

  .winner-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 4rem;
    height: 4rem;
    margin-bottom: $space-2;
    border-radius: 50%;
    background-color: $color-background;
    color: $color-primary;
  }

  .winner-name {
    font-size: $font-size-xl;
    font-weight: 700;
    margin: 0;
  }

  .winner-phone {
    margin: 0;
    color: $color-text-muted;
    font-size: $font-size-lg;
  }

  .error {
    display: flex;
    align-items: center;
    gap: $space-2;
    color: $color-danger;
  }
</style>
