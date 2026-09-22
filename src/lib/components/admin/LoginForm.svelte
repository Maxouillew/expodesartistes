<script lang="ts">
  import { adminLogin } from "$lib/services/auth";
  import Input from "$lib/components/Input.svelte";
  import Button from "$lib/components/Button.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let { onSuccess }: { onSuccess: () => void } = $props();

  let password = $state("");
  let error = $state<string | undefined>(undefined);
  let submitting = $state(false);

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    error = undefined;
    submitting = true;
    try {
      await adminLogin(password);
      password = "";
      onSuccess();
    } catch (err) {
      error = String(err);
    } finally {
      submitting = false;
    }
  }
</script>

<form class="login-form animate-in" onsubmit={handleSubmit}>
  <div class="icon-badge"><Icon name="lock" size={28} /></div>
  <h1>Administration</h1>
  <Input
    id="admin-password"
    label="Mot de passe"
    type="password"
    placeholder="Votre mot de passe"
    bind:value={password}
    {error}
    autocomplete="current-password"
  />
  <Button type="submit" fullWidth disabled={submitting || password.length === 0}>Se connecter</Button>
</form>

<style lang="scss">
  @use "../../styles/variables" as *;

  .login-form {
    max-width: 24rem;
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
    text-align: center;

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
</style>
