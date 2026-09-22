<script lang="ts">
  import { adminSetPassword } from "$lib/services/auth";
  import Input from "$lib/components/Input.svelte";
  import Button from "$lib/components/Button.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let { onDone }: { onDone: () => void } = $props();

  let password = $state("");
  let confirmPassword = $state("");
  let error = $state<string | undefined>(undefined);
  let submitting = $state(false);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    error = undefined;

    if (password.length < 8) {
      error = "Le mot de passe doit contenir au moins 8 caractères.";
      return;
    }
    if (password !== confirmPassword) {
      error = "Les deux mots de passe ne correspondent pas.";
      return;
    }

    submitting = true;
    try {
      await adminSetPassword(password);
      onDone();
    } catch (err) {
      error = String(err);
    } finally {
      submitting = false;
    }
  }
</script>

<form class="setup-form animate-in" onsubmit={handleSubmit}>
  <div class="icon-badge"><Icon name="lock" size={28} /></div>
  <h1>Configurer le mot de passe admin</h1>
  <p>
    Aucun mot de passe n'est encore configuré. Choisissez-en un pour protéger l'accès à
    l'administration.
  </p>
  <Input
    id="new-password"
    label="Nouveau mot de passe"
    type="password"
    placeholder="8 caractères minimum"
    bind:value={password}
    autocomplete="new-password"
  />
  <Input
    id="confirm-password"
    label="Confirmer le mot de passe"
    type="password"
    placeholder="Ressaisissez le mot de passe"
    bind:value={confirmPassword}
    {error}
    autocomplete="new-password"
  />
  <Button type="submit" fullWidth disabled={submitting}>Définir le mot de passe</Button>
</form>

<style lang="scss">
  @use "../../styles/variables" as *;

  .setup-form {
    max-width: 26rem;
    margin: $space-7 auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: $space-4;
    padding: $space-6;
    background-color: $color-surface;
    border-radius: $radius;
    border: 1px solid $color-border;
    box-shadow: $shadow-card;
    text-align: left;

    :global(.field) {
      width: 100%;
    }
  }

  .icon-badge {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 3.5rem;
    height: 3.5rem;
    border-radius: 50%;
    background-color: $color-background;
    color: $color-primary;
  }

  h1 {
    text-align: center;
    margin: 0;
  }

  p {
    text-align: center;
    color: $color-text-muted;
  }
</style>
