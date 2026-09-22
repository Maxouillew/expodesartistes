<script lang="ts">
  import { onMount } from "svelte";
  import {
    listArtists,
    addArtist,
    updateArtist,
    deleteArtist,
    type ArtistWithVotes,
  } from "$lib/services/artists";
  import Input from "$lib/components/Input.svelte";
  import Button from "$lib/components/Button.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let artists = $state<ArtistWithVotes[]>([]);
  let loading = $state(true);
  let loadError = $state<string | undefined>(undefined);

  let newName = $state("");
  let newId = $state("1");
  let addError = $state<string | undefined>(undefined);
  let adding = $state(false);

  let editingId = $state<number | null>(null);
  let editName = $state("");
  let editNumber = $state("");
  let editError = $state<string | undefined>(undefined);
  let saving = $state(false);

  let deleteModalOpen = $state(false);
  let confirmDeleteArtist = $state<ArtistWithVotes | null>(null);
  let deleteError = $state<string | undefined>(undefined);
  let deleting = $state(false);

  function suggestedNextNumber(list: ArtistWithVotes[]): number {
    return list.length === 0 ? 1 : Math.max(...list.map((a) => a.id)) + 1;
  }

  async function load() {
    loading = true;
    loadError = undefined;
    try {
      artists = await listArtists();
      newId = String(suggestedNextNumber(artists));
    } catch (err) {
      loadError = String(err);
    } finally {
      loading = false;
    }
  }

  onMount(load);

  function parsePositiveInt(raw: string): number | null {
    const value = Number(raw);
    return Number.isInteger(value) && value > 0 ? value : null;
  }

  async function handleAdd(event: SubmitEvent) {
    event.preventDefault();
    addError = undefined;

    const idNum = parsePositiveInt(newId);
    if (idNum === null) {
      addError = "Le numéro doit être un entier positif.";
      return;
    }
    if (newName.trim().length === 0) {
      addError = "Le nom de l'artiste est requis.";
      return;
    }

    adding = true;
    try {
      await addArtist(idNum, newName.trim());
      newName = "";
      await load();
    } catch (err) {
      addError = String(err);
    } finally {
      adding = false;
    }
  }

  function startEdit(artist: ArtistWithVotes) {
    editingId = artist.id;
    editName = artist.name;
    editNumber = String(artist.id);
    editError = undefined;
  }

  function cancelEdit() {
    editingId = null;
    editError = undefined;
  }

  async function saveEdit(currentId: number) {
    editError = undefined;

    const idNum = parsePositiveInt(editNumber);
    if (idNum === null) {
      editError = "Le numéro doit être un entier positif.";
      return;
    }
    if (editName.trim().length === 0) {
      editError = "Le nom de l'artiste est requis.";
      return;
    }

    saving = true;
    try {
      await updateArtist(currentId, idNum, editName.trim());
      editingId = null;
      await load();
    } catch (err) {
      editError = String(err);
    } finally {
      saving = false;
    }
  }

  function requestDelete(artist: ArtistWithVotes) {
    confirmDeleteArtist = artist;
    deleteError = undefined;
    deleteModalOpen = true;
  }

  async function confirmDelete() {
    if (!confirmDeleteArtist) return;
    deleting = true;
    deleteError = undefined;
    try {
      await deleteArtist(confirmDeleteArtist.id);
      deleteModalOpen = false;
      confirmDeleteArtist = null;
      await load();
    } catch (err) {
      deleteError = String(err);
    } finally {
      deleting = false;
    }
  }
</script>

<section>
  <h2>Artistes</h2>

  <form class="add-form" onsubmit={handleAdd}>
    <h3>Ajouter un artiste</h3>
    <div class="fields">
      <Input id="new-artist-number" label="Numéro" type="number" placeholder="Ex. 1" bind:value={newId} />
      <Input
        id="new-artist-name"
        label="Nom"
        placeholder="Ex. Vincent van Gogh"
        bind:value={newName}
      />
    </div>
    {#if addError}
      <p class="error" role="alert"><Icon name="alert-circle" size={18} />{addError}</p>
    {/if}
    <Button type="submit" icon="plus" disabled={adding}>Ajouter</Button>
  </form>

  {#if loading}
    <p>Chargement...</p>
  {:else if loadError}
    <p class="error" role="alert"><Icon name="alert-circle" size={18} />{loadError}</p>
  {:else if artists.length === 0}
    <p>Aucun artiste pour le moment. Il faut au moins 3 artistes pour ouvrir le vote.</p>
  {:else}
    <ul class="artist-list animate-in">
      {#each artists as artist (artist.id)}
        <li class="artist-row">
          {#if editingId === artist.id}
            <div class="fields">
              <Input id="edit-number-{artist.id}" label="Numéro" type="number" bind:value={editNumber} />
              <Input id="edit-name-{artist.id}" label="Nom" bind:value={editName} />
            </div>
            {#if editError}
              <p class="error" role="alert"><Icon name="alert-circle" size={18} />{editError}</p>
            {/if}
            <div class="row-actions">
              <Button variant="secondary" onclick={cancelEdit}>Annuler</Button>
              <Button onclick={() => saveEdit(artist.id)} disabled={saving}>Enregistrer</Button>
            </div>
          {:else}
            <div class="artist-info">
              <span class="artist-number">#{artist.id}</span>
              <span class="artist-name">{artist.name}</span>
              <span class="artist-votes">{artist.voteCount} vote{artist.voteCount === 1 ? "" : "s"}</span>
            </div>
            <div class="row-actions">
              <Button variant="secondary" icon="pencil" onclick={() => startEdit(artist)}>Modifier</Button>
              <Button variant="danger" icon="trash" onclick={() => requestDelete(artist)}>Supprimer</Button>
            </div>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</section>

<Modal bind:open={deleteModalOpen} title="Supprimer l'artiste">
  {#snippet children()}
    {#if confirmDeleteArtist}
      <p>Supprimer l'artiste #{confirmDeleteArtist.id} {confirmDeleteArtist.name} ?</p>
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

  .add-form {
    background-color: $color-surface;
    border: 1px solid $color-border;
    border-radius: $radius;
    box-shadow: $shadow-card;
    padding: $space-5;
    margin-bottom: $space-5;

    h3 {
      margin-top: 0;
    }
  }

  .fields {
    display: flex;
    gap: $space-4;
    flex-wrap: wrap;
    margin-bottom: $space-4;

    :global(.field) {
      flex: 1;
      min-width: 10rem;
    }
  }

  .artist-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: $space-3;
  }

  .artist-row {
    background-color: $color-surface;
    border: 1px solid $color-border;
    border-radius: $radius;
    box-shadow: $shadow-card;
    padding: $space-4;
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: $space-3;
    transition: border-color $duration-fast ease-out;

    &:hover {
      border-color: $color-primary;
    }
  }

  .artist-info {
    display: flex;
    align-items: baseline;
    gap: $space-3;
    flex-wrap: wrap;
  }

  .artist-number {
    font-weight: 700;
    color: $color-primary;
  }

  .artist-name {
    font-size: $font-size-lg;
  }

  .artist-votes {
    color: $color-text-muted;
    font-size: $font-size-sm;
  }

  .row-actions {
    display: flex;
    gap: $space-3;
  }

  .error {
    display: flex;
    align-items: center;
    gap: $space-2;
    color: $color-danger;
  }
</style>
