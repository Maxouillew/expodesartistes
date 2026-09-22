<script lang="ts">
  import { onMount } from "svelte";
  import { listArtistsForVote, type PublicArtist } from "$lib/services/vote";
  import { MIN_ARTISTS_TO_VOTE } from "$lib/config/validation";
  import VoteForm from "$lib/components/vote/VoteForm.svelte";
  import ConfirmationScreen from "$lib/components/vote/ConfirmationScreen.svelte";
  import Button from "$lib/components/Button.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let artists = $state<PublicArtist[] | null>(null);
  let loadError = $state<string | undefined>(undefined);
  let view = $state<"form" | "confirmation">("form");

  async function loadArtists() {
    loadError = undefined;
    try {
      artists = await listArtistsForVote();
    } catch (err) {
      loadError = String(err);
    }
  }

  onMount(loadArtists);

  function handleVoted() {
    view = "confirmation";
  }

  async function handleContinue() {
    view = "form";
    await loadArtists();
  }
</script>

<header class="brand">
  <img src="/logo.png" alt="" class="brand-logo" />
  <p class="brand-title">Exposition des artistes</p>
</header>

<main class="vote-page">
  {#if loadError}
    <div class="status">
      <p class="error" role="alert">
        <Icon name="alert-circle" size={20} />
        {loadError}
      </p>
      <Button variant="secondary" icon="refresh" onclick={loadArtists}>Réessayer</Button>
    </div>
  {:else if artists === null}
    <p class="status">Chargement...</p>
  {:else if view === "confirmation"}
    <ConfirmationScreen onContinue={handleContinue} />
  {:else if artists.length < MIN_ARTISTS_TO_VOTE}
    <p class="status">
      Le vote n'est pas encore ouvert : il faut au moins {MIN_ARTISTS_TO_VOTE} artistes enregistrés.
    </p>
  {:else}
    <VoteForm {artists} onVoted={handleVoted} />
  {/if}
</main>

<a href="/admin" class="admin-link">
  <Icon name="shield" size={18} />
  Administration
</a>

<style lang="scss">
  @use "../lib/styles/variables" as *;

  .brand {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: $space-2;
    padding: $space-6 $space-5 0;
  }

  .brand-logo {
    height: 4.5rem;
    width: auto;
  }

  .brand-title {
    margin: 0;
    font-size: $font-size-lg;
    font-weight: 700;
    letter-spacing: 0.02em;
    color: $color-text;
  }

  .vote-page {
    max-width: $max-content-width;
    margin: 0 auto;
    padding: $space-5 $space-5 $space-6;
  }

  .status {
    text-align: center;
    margin-top: $space-7;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: $space-4;
  }

  .error {
    display: flex;
    align-items: center;
    gap: $space-2;
    color: $color-danger;
    margin: 0;
  }

  .admin-link {
    position: fixed;
    right: $space-4;
    bottom: $space-4;
    display: flex;
    align-items: center;
    gap: $space-2;
    padding: $space-2 $space-4;
    min-height: $min-tap-target;
    background-color: $color-surface;
    border: 1px solid $color-border;
    border-radius: $radius;
    color: $color-text-muted;
    font-size: $font-size-sm;
    font-weight: 600;
    text-decoration: none;
    transition:
      color $duration-fast ease-out,
      border-color $duration-fast ease-out,
      transform $duration-fast ease-out;

    &:hover {
      color: $color-primary;
      border-color: $color-primary;
      transform: translateY(-1px);
    }
  }
</style>
