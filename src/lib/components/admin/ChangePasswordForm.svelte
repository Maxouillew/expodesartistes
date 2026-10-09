<script lang="ts">
  import { adminChangePassword } from "$lib/services/auth";
  import Input from "$lib/components/Input.svelte";
  import Button from "$lib/components/Button.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let currentPassword = $state("");
  let newPassword = $state("");
  let confirmPassword = $state("");
  let error = $state<string | undefined>(undefined);
  let success = $state(false);
  let submitting = $state(false);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    error = undefined;
    success = false;

    if (newPassword.length < 8) {
      error = "Le nouveau mot de passe doit contenir au moins 8 caractères.";
      return;
    }
    if (newPassword !== confirmPassword) {
      error = "Les deux mots de passe ne correspondent pas.";
      return;
    }

    submitting = true;
    try {
      await adminChangePassword(currentPassword, newPassword);
      currentPassword = newPassword = confirmPassword = "";
      success = true;
    } catch (err) {
      error = String(err);
    } finally {
      submitting = false;
    }
  }
</script>

<form class="password-form" onsubmit={handleSubmit}>
  <Input
    id="current-password"
    label="Mot de passe actuel"
    type="password"
    bind:value={currentPassword}
    autocomplete="current-password"
  />
  <Input
    id="change-password"
    label="Nouveau mot de passe"
    type="password"
    placeholder="8 caractères minimum"
    bind:value={newPassword}
    autocomplete="new-password"
  />
  <Input
    id="change-password-confirm"
    label="Confirmer le nouveau mot de passe"
    type="password"
    bind:value={confirmPassword}
    {error}
    autocomplete="new-password"
  />
  {#if success}
    <p class="success" role="status"><Icon name="check-circle" size={18} />Mot de passe modifié.</p>
  {/if}
  <div>
    <Button type="submit" icon="lock" disabled={submitting}>Modifier le mot de passe</Button>
  </div>
</form>

<style lang="scss">
  @use "../../styles/variables" as *;

  .password-form {
    display: flex;
    flex-direction: column;
    gap: $space-4;
    max-width: 26rem;
  }

  .success {
    display: flex;
    align-items: center;
    gap: $space-2;
    margin: 0;
    color: $color-success;
    font-weight: 600;
  }
</style>
