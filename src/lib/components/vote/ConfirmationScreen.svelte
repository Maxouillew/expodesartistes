<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import Icon from "$lib/components/Icon.svelte";

  let { onContinue }: { onContinue: () => void } = $props();

  const COUNTDOWN_SECONDS = 5;
  let secondsLeft = $state(COUNTDOWN_SECONDS);
  let interval: ReturnType<typeof setInterval>;

  onMount(() => {
    interval = setInterval(() => {
      secondsLeft -= 1;
      if (secondsLeft <= 0) {
        clearInterval(interval);
        onContinue();
      }
    }, 1000);
  });

  onDestroy(() => clearInterval(interval));
</script>

<div class="confirmation animate-in" role="status">
  <div class="checkmark">
    <Icon name="check-circle" size={64} />
  </div>
  <h1>Merci pour votre vote !</h1>
  <p>Votre vote a bien été enregistré.</p>
  <p class="countdown">Retour à l'accueil dans {secondsLeft}s...</p>
</div>

<style lang="scss">
  @use "../../styles/variables" as *;

  .confirmation {
    max-width: 28rem;
    margin: $space-8 auto;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: $space-4;
  }

  @keyframes checkmark-pop {
    from {
      transform: scale(0.5);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }

  .checkmark {
    display: flex;
    color: $color-success;
    animation: checkmark-pop $duration-base $ease-out;
  }

  h1 {
    margin: 0;
    font-size: $font-size-xl;
  }

  .countdown {
    color: $color-text-muted;
  }
</style>
