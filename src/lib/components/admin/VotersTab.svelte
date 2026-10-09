<script lang="ts">
  import { onMount } from "svelte";
  import { listVoters, deleteVoter, type Voter } from "$lib/services/voters";
  import Button from "$lib/components/Button.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let voters = $state<Voter[] | null>(null);
  let error = $state<string | undefined>(undefined);
  let loading = $state(true);

  async function load() {
    loading = true;
    error = undefined;
    try {
      voters = await listVoters();
    } catch (err) {
      error = String(err);
    } finally {
      loading = false;
    }
  }

  onMount(load);

  let deleteModalOpen = $state(false);
  let voterToDelete = $state<Voter | null>(null);
  let deleteError = $state<string | undefined>(undefined);
  let deleting = $state(false);

  function requestDelete(voter: Voter) {
    voterToDelete = voter;
    deleteError = undefined;
    deleteModalOpen = true;
  }

  async function confirmDelete() {
    if (!voterToDelete) return;
    deleting = true;
    deleteError = undefined;
    try {
      await deleteVoter(voterToDelete.id);
      deleteModalOpen = false;
      voterToDelete = null;
      await load();
    } catch (err) {
      deleteError = String(err);
    } finally {
      deleting = false;
    }
  }

  const dateFormatter = new Intl.DateTimeFormat("fr-BE", {
    dateStyle: "short",
    timeStyle: "short",
  });

  // SQLite's datetime('now') stores naive UTC ("YYYY-MM-DD HH:MM:SS") with
  // no timezone marker — append "Z" so the Date parses it as UTC instead of
  // (incorrectly) as local time.
  function formatDate(createdAt: string): string {
    return dateFormatter.format(new Date(`${createdAt.replace(" ", "T")}Z`));
  }
</script>

<section>
  <div class="header">
    <h2>Votants</h2>
    <div class="header-actions">
      {#if voters}
        <p class="total">{voters.length} votant{voters.length === 1 ? "" : "s"} au total</p>
      {/if}
      <Button variant="secondary" icon="refresh" onclick={load} disabled={loading}>Actualiser</Button>
    </div>
  </div>

  {#if loading}
    <p>Chargement...</p>
  {:else if error}
    <p class="error" role="alert">
      <Icon name="alert-circle" size={20} />
      {error}
    </p>
  {:else if voters && voters.length === 0}
    <p>Aucun votant pour le moment.</p>
  {:else if voters}
    <div class="table-wrapper animate-in">
      <table>
        <thead>
          <tr>
            <th scope="col">Prénom</th>
            <th scope="col">Nom</th>
            <th scope="col">Téléphone</th>
            <th scope="col">Date</th>
            <th scope="col"><span class="sr-only">Actions</span></th>
          </tr>
        </thead>
        <tbody>
          {#each voters as voter (voter.id)}
            <tr>
              <td>{voter.firstName}</td>
              <td>{voter.lastName}</td>
              <td>{voter.phone}</td>
              <td class="date">{formatDate(voter.createdAt)}</td>
              <td class="actions">
                <Button variant="danger" icon="trash" onclick={() => requestDelete(voter)}>
                  Supprimer
                </Button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</section>

<Modal bind:open={deleteModalOpen} title="Supprimer le vote">
  {#snippet children()}
    {#if voterToDelete}
      <p>
        Supprimer le vote de <strong>{voterToDelete.firstName} {voterToDelete.lastName}</strong>
        ({voterToDelete.phone}) ?
      </p>
      <p class="hint">
        Ses points disparaissent du classement et cette personne pourra voter à nouveau.
      </p>
    {/if}
    {#if deleteError}
      <p class="error" role="alert"><Icon name="alert-circle" size={18} />{deleteError}</p>
    {/if}
  {/snippet}
  {#snippet footer()}
    <Button variant="secondary" onclick={() => (deleteModalOpen = false)}>Annuler</Button>
    <Button variant="danger" icon="trash" onclick={confirmDelete} disabled={deleting}>Supprimer</Button>
  {/snippet}
</Modal>

<style lang="scss">
  @use "../../styles/variables" as *;

  .header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: $space-3;
    margin-bottom: $space-4;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: $space-4;
  }

  .total {
    margin: 0;
    color: $color-text-muted;
  }

  .table-wrapper {
    overflow-x: auto;
    border-radius: $radius;
    border: 1px solid $color-border;
    box-shadow: $shadow-card;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    background-color: $color-surface;
  }

  th,
  td {
    text-align: left;
    padding: $space-3 $space-4;
    border-bottom: 1px solid $color-border;
    white-space: nowrap;
  }

  th {
    background-color: $color-background;
    font-weight: 600;
  }

  tbody tr:last-child td {
    border-bottom: none;
  }

  tbody tr:hover td {
    background-color: $color-background;
  }

  .date {
    color: $color-text-muted;
  }

  .actions {
    text-align: right;
  }

  .hint {
    color: $color-text-muted;
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip: rect(0 0 0 0);
    white-space: nowrap;
  }

  .error {
    display: flex;
    align-items: center;
    gap: $space-2;
    color: $color-danger;
  }
</style>
