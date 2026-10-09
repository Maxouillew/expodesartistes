<script lang="ts">
  import { onMount } from "svelte";
  import { isFullscreen, setFullscreen, quitApp } from "$lib/services/window";
  import { adminPasswordIsFixed } from "$lib/services/auth";
  import { resetDraw } from "$lib/services/draw";
  import { deleteAllVoters } from "$lib/services/voters";
  import ChangePasswordForm from "$lib/components/admin/ChangePasswordForm.svelte";
  import Input from "$lib/components/Input.svelte";
  import Button from "$lib/components/Button.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let fullscreen = $state(false);
  let fullscreenError = $state<string | undefined>(undefined);
  let passwordFixed = $state(false);

  onMount(async () => {
    try {
      fullscreen = await isFullscreen();
      passwordFixed = await adminPasswordIsFixed();
    } catch (err) {
      fullscreenError = String(err);
    }
  });

  async function toggleFullscreen() {
    fullscreenError = undefined;
    try {
      await setFullscreen(!fullscreen);
      fullscreen = !fullscreen;
    } catch (err) {
      fullscreenError = String(err);
    }
  }

  // Draw reset
  let drawModalOpen = $state(false);
  let drawResetDone = $state(false);
  let drawError = $state<string | undefined>(undefined);

  async function confirmResetDraw() {
    drawError = undefined;
    try {
      await resetDraw();
      drawModalOpen = false;
      drawResetDone = true;
    } catch (err) {
      drawError = String(err);
    }
  }

  // Delete every vote
  let votesModalOpen = $state(false);
  let votesPassword = $state("");
  let votesError = $state<string | undefined>(undefined);
  let votesDeleting = $state(false);
  let deletedCount = $state<number | null>(null);

  function requestDeleteVotes() {
    votesPassword = "";
    votesError = undefined;
    votesModalOpen = true;
  }

  async function confirmDeleteVotes(event?: SubmitEvent) {
    event?.preventDefault();
    votesError = undefined;
    votesDeleting = true;
    try {
      deletedCount = await deleteAllVoters(votesPassword);
      drawResetDone = false;
      votesModalOpen = false;
    } catch (err) {
      votesError = String(err);
    } finally {
      votesDeleting = false;
    }
  }

  // Quit
  let quitModalOpen = $state(false);
  let quitError = $state<string | undefined>(undefined);

  async function confirmQuit() {
    quitError = undefined;
    try {
      await quitApp();
    } catch (err) {
      quitError = String(err);
    }
  }
</script>

<section>
  <h2>Configuration</h2>

  <div class="cards">
    <div class="card">
      <h3>Affichage</h3>
      <p class="intro">
        Le plein écran masque la barre de titre et les boutons pour réduire ou fermer la fenêtre.
        Il est conservé au prochain lancement ; il se désactive ici.
      </p>
      {#if fullscreenError}
        <p class="error" role="alert"><Icon name="alert-circle" size={18} />{fullscreenError}</p>
      {/if}
      <Button icon="maximize" variant={fullscreen ? "secondary" : "primary"} onclick={toggleFullscreen}>
        {fullscreen ? "Quitter le plein écran" : "Passer en plein écran"}
      </Button>
    </div>

    <div class="card">
      <h3>Mot de passe admin</h3>
      {#if passwordFixed}
        <p class="intro">Le mot de passe est défini à la compilation de l'application et ne peut pas être modifié ici.</p>
      {:else}
        <ChangePasswordForm />
      {/if}
    </div>

    <div class="card">
      <h3>Tirage au sort</h3>
      <p class="intro">
        Remet la liste des gagnants déjà tirés à zéro : tous les votants redeviennent tirables.
      </p>
      {#if drawResetDone}
        <p class="success" role="status"><Icon name="check-circle" size={18} />Tirage réinitialisé.</p>
      {/if}
      <Button variant="secondary" icon="shuffle" onclick={() => (drawModalOpen = true)}>
        Réinitialiser le tirage
      </Button>
    </div>

    <div class="card danger-zone">
      <h3>Zone sensible</h3>
      <p class="intro">
        Utile pour repartir de zéro après des votes de test. Les artistes sont conservés. Cette
        action est définitive.
      </p>
      {#if deletedCount !== null}
        <p class="success" role="status">
          <Icon name="check-circle" size={18} />
          {deletedCount} vote{deletedCount === 1 ? "" : "s"} supprimé{deletedCount === 1 ? "" : "s"}.
        </p>
      {/if}
      <Button variant="danger" icon="trash" onclick={requestDeleteVotes}>Supprimer tous les votes</Button>
    </div>

    <div class="card">
      <h3>Application</h3>
      <p class="intro">En plein écran, la fermeture de la fenêtre n'est plus accessible : utilisez ce bouton.</p>
      <Button variant="secondary" icon="power" onclick={() => (quitModalOpen = true)}>
        Quitter l'application
      </Button>
    </div>
  </div>
</section>

<Modal bind:open={drawModalOpen} title="Réinitialiser le tirage">
  {#snippet children()}
    <p>Les gagnants déjà tirés pourront être tirés à nouveau. Continuer ?</p>
    {#if drawError}
      <p class="error" role="alert"><Icon name="alert-circle" size={18} />{drawError}</p>
    {/if}
  {/snippet}
  {#snippet footer()}
    <Button variant="secondary" onclick={() => (drawModalOpen = false)}>Annuler</Button>
    <Button onclick={confirmResetDraw}>Réinitialiser</Button>
  {/snippet}
</Modal>

<Modal bind:open={votesModalOpen} title="Supprimer tous les votes">
  {#snippet children()}
    <form class="modal-form" onsubmit={confirmDeleteVotes}>
      <p>
        Tous les votants et leurs votes seront définitivement supprimés. Saisissez le mot de passe
        admin pour confirmer.
      </p>
      <Input
        id="delete-votes-password"
        label="Mot de passe admin"
        type="password"
        bind:value={votesPassword}
        error={votesError}
        autocomplete="current-password"
      />
    </form>
  {/snippet}
  {#snippet footer()}
    <Button variant="secondary" onclick={() => (votesModalOpen = false)}>Annuler</Button>
    <Button
      variant="danger"
      icon="trash"
      onclick={() => confirmDeleteVotes()}
      disabled={votesDeleting || votesPassword.length === 0}
    >
      Tout supprimer
    </Button>
  {/snippet}
</Modal>

<Modal bind:open={quitModalOpen} title="Quitter l'application">
  {#snippet children()}
    <p>Fermer l'application ?</p>
    {#if quitError}
      <p class="error" role="alert"><Icon name="alert-circle" size={18} />{quitError}</p>
    {/if}
  {/snippet}
  {#snippet footer()}
    <Button variant="secondary" onclick={() => (quitModalOpen = false)}>Annuler</Button>
    <Button variant="danger" icon="power" onclick={confirmQuit}>Quitter</Button>
  {/snippet}
</Modal>

<style lang="scss">
  @use "../../styles/variables" as *;

  .cards {
    display: flex;
    flex-direction: column;
    gap: $space-5;
  }

  .card {
    background-color: $color-surface;
    border: 1px solid $color-border;
    border-radius: $radius;
    box-shadow: $shadow-card;
    padding: $space-5;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: $space-3;

    h3 {
      margin: 0;
    }

    .intro {
      margin: 0;
    }
  }

  .danger-zone {
    border-color: $color-danger;
  }

  .intro {
    color: $color-text-muted;
  }

  .success,
  .error {
    display: flex;
    align-items: center;
    gap: $space-2;
    margin: 0;
  }

  .success {
    color: $color-success;
    font-weight: 600;
  }

  .error {
    color: $color-danger;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: $space-4;
  }
</style>
