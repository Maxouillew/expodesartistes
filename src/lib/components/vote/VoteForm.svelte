<script lang="ts">
  import { submitVote, type PublicArtist } from "$lib/services/vote";
  import { validateName } from "$lib/utils/validation";
  import { looksLikePhoneNumber } from "$lib/utils/phone";
  import Input from "$lib/components/Input.svelte";
  import Button from "$lib/components/Button.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let { artists, onVoted }: { artists: PublicArtist[]; onVoted: () => void } = $props();

  type Rank = 1 | 2 | 3;

  let firstName = $state("");
  let lastName = $state("");
  let phone = $state("");
  let top1 = $state<PublicArtist | null>(null);
  let top2 = $state<PublicArtist | null>(null);
  let top3 = $state<PublicArtist | null>(null);

  let firstNameTouched = $state(false);
  let lastNameTouched = $state(false);
  let phoneTouched = $state(false);

  let pickerOpen = $state(false);
  let pickerRank = $state<Rank | null>(null);

  let submitting = $state(false);
  let submitError = $state<string | undefined>(undefined);

  const firstNameError = $derived(validateName(firstName));
  const lastNameError = $derived(validateName(lastName));
  const phoneError = $derived(
    phone.trim().length === 0
      ? "Le numéro de téléphone est requis."
      : looksLikePhoneNumber(phone)
        ? undefined
        : "Numéro de téléphone invalide.",
  );

  const topsValid = $derived(
    top1 !== null &&
      top2 !== null &&
      top3 !== null &&
      top1.id !== top2.id &&
      top1.id !== top3.id &&
      top2.id !== top3.id,
  );

  const canSubmit = $derived(
    !submitting &&
      firstNameError === undefined &&
      lastNameError === undefined &&
      phoneError === undefined &&
      topsValid,
  );

  function excludedIdsFor(rank: Rank): number[] {
    const others = [
      rank !== 1 ? top1?.id : undefined,
      rank !== 2 ? top2?.id : undefined,
      rank !== 3 ? top3?.id : undefined,
    ];
    return others.filter((id): id is number => id !== undefined);
  }

  function openPicker(rank: Rank) {
    pickerRank = rank;
    pickerOpen = true;
  }

  function handleSelect(artist: PublicArtist) {
    if (pickerRank === 1) top1 = artist;
    else if (pickerRank === 2) top2 = artist;
    else if (pickerRank === 3) top3 = artist;
    pickerOpen = false;
  }

  function fieldLabel(artist: PublicArtist | null): string {
    return artist ? `#${artist.id} ${artist.name}` : "Aucun artiste choisi";
  }

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    firstNameTouched = true;
    lastNameTouched = true;
    phoneTouched = true;
    submitError = undefined;

    if (!canSubmit || !top1 || !top2 || !top3) return;

    submitting = true;
    try {
      await submitVote({
        firstName: firstName.trim(),
        lastName: lastName.trim(),
        phone: phone.trim(),
        top1Id: top1.id,
        top2Id: top2.id,
        top3Id: top3.id,
      });
      onVoted();
    } catch (err) {
      submitError = String(err);
    } finally {
      submitting = false;
    }
  }
</script>

<form class="vote-form animate-in" onsubmit={handleSubmit} novalidate>
  <h1>Votez pour vos artistes préférés</h1>

  <div class="fields">
    <Input
      id="first-name"
      label="Prénom"
      placeholder="Ex. Camille"
      bind:value={firstName}
      error={firstNameTouched ? firstNameError : undefined}
      autocomplete="given-name"
      onblur={() => (firstNameTouched = true)}
    />

    <Input
      id="last-name"
      label="Nom"
      placeholder="Ex. Dupont"
      bind:value={lastName}
      error={lastNameTouched ? lastNameError : undefined}
      autocomplete="family-name"
      onblur={() => (lastNameTouched = true)}
    />

    <Input
      id="phone"
      label="Téléphone"
      type="tel"
      placeholder="Ex. 0470 12 34 56"
      bind:value={phone}
      error={phoneTouched ? phoneError : undefined}
      autocomplete="tel"
      onblur={() => (phoneTouched = true)}
    />
  </div>

  <fieldset class="top-picks">
    <legend>Votre top 3</legend>

    <button type="button" class="artist-field" class:selected={top1 !== null} onclick={() => openPicker(1)}>
      <span class="rank-badge" class:selected={top1 !== null}>1</span>
      <span class="artist-field-text">
        <span class="artist-field-label">Top 1</span>
        <span class="artist-field-value">{fieldLabel(top1)}</span>
      </span>
      <span class="artist-field-hint">
        {top1 !== null ? "Changer" : "Choisir"}
        <Icon name="chevron-right" size={18} />
      </span>
    </button>

    <button type="button" class="artist-field" class:selected={top2 !== null} onclick={() => openPicker(2)}>
      <span class="rank-badge" class:selected={top2 !== null}>2</span>
      <span class="artist-field-text">
        <span class="artist-field-label">Top 2</span>
        <span class="artist-field-value">{fieldLabel(top2)}</span>
      </span>
      <span class="artist-field-hint">
        {top2 !== null ? "Changer" : "Choisir"}
        <Icon name="chevron-right" size={18} />
      </span>
    </button>

    <button type="button" class="artist-field" class:selected={top3 !== null} onclick={() => openPicker(3)}>
      <span class="rank-badge" class:selected={top3 !== null}>3</span>
      <span class="artist-field-text">
        <span class="artist-field-label">Top 3</span>
        <span class="artist-field-value">{fieldLabel(top3)}</span>
      </span>
      <span class="artist-field-hint">
        {top3 !== null ? "Changer" : "Choisir"}
        <Icon name="chevron-right" size={18} />
      </span>
    </button>
  </fieldset>

  {#if submitError}
    <p class="error" role="alert">
      <Icon name="alert-circle" size={18} />
      {submitError}
    </p>
  {/if}

  <Button type="submit" icon="check-circle" disabled={!canSubmit}>Confirmer mon vote</Button>
</form>

<Modal
  bind:open={pickerOpen}
  title={pickerRank ? `Top ${pickerRank} : choisir un artiste` : "Choisir un artiste"}
>
  {#snippet children()}
    <ul class="artist-options">
      {#each artists as artist (artist.id)}
        {@const disabled = pickerRank !== null && excludedIdsFor(pickerRank).includes(artist.id)}
        <li>
          <button type="button" class="artist-option" {disabled} onclick={() => handleSelect(artist)}>
            #{artist.id} {artist.name}
          </button>
        </li>
      {/each}
    </ul>
  {/snippet}
  {#snippet footer()}
    <Button variant="secondary" onclick={() => (pickerOpen = false)}>Fermer</Button>
  {/snippet}
</Modal>

<style lang="scss">
  @use "../../styles/variables" as *;

  .vote-form {
    max-width: 36rem;
    margin: $space-6 auto;
    display: flex;
    flex-direction: column;
    gap: $space-5;
    padding: $space-6;
    background-color: $color-surface;
    border-radius: $radius;
    border: 1px solid $color-border;
    box-shadow: $shadow-card;
  }

  h1 {
    margin: 0;
    text-align: center;
    font-size: $font-size-xl;
  }

  .fields {
    display: flex;
    flex-direction: column;
    gap: $space-4;
  }

  .top-picks {
    border: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: $space-3;

    legend {
      padding: 0;
      margin-bottom: $space-1;
      font-weight: 700;
      color: $color-text;
    }
  }

  .artist-field {
    min-height: 4rem;
    display: flex;
    align-items: center;
    gap: $space-3;
    padding: $space-3 $space-4;
    background-color: $color-surface;
    border: 1px solid $color-border;
    border-radius: $radius;
    color: $color-text;
    cursor: pointer;
    text-align: left;
    font-size: $font-size-base;
    transition:
      border-color $duration-fast ease-out,
      background-color $duration-fast ease-out,
      box-shadow $duration-fast ease-out,
      transform $duration-fast ease-out;

    &:hover {
      border-color: $color-primary;
      box-shadow: $shadow-card;
      transform: translateY(-1px);
    }

    &.selected {
      border-color: $color-primary;
      background-color: rgba($color-primary, 0.06);
    }
  }

  .rank-badge {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.25rem;
    height: 2.25rem;
    border-radius: 50%;
    border: 2px solid $color-border;
    background-color: $color-surface;
    color: $color-text-muted;
    font-weight: 700;
    font-size: $font-size-sm;

    &.selected {
      border-color: $color-primary;
      background-color: $color-primary;
      color: $color-primary-contrast;
    }
  }

  .artist-field-text {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .artist-field-label {
    font-weight: 600;
    color: $color-text-muted;
    font-size: $font-size-sm;
  }

  .artist-field-value {
    font-weight: 600;
    color: $color-text-muted;
    font-style: italic;

    .selected & {
      color: $color-text;
      font-style: normal;
    }
  }

  .artist-field-hint {
    display: flex;
    align-items: center;
    gap: $space-1;
    margin-left: auto;
    flex-shrink: 0;
    color: $color-text-muted;
    font-size: $font-size-sm;
    font-weight: 600;

    .selected & {
      color: $color-primary;
    }
  }

  .artist-options {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: $space-2;
    max-height: 50vh;
    overflow-y: auto;
  }

  .artist-option {
    width: 100%;
    min-height: $min-tap-target;
    padding: $space-3 $space-4;
    text-align: left;
    background-color: $color-surface;
    border: 1px solid $color-border;
    border-radius: $radius;
    cursor: pointer;
    font-size: $font-size-base;
    transition:
      border-color $duration-fast ease-out,
      box-shadow $duration-fast ease-out;

    &:hover:not(:disabled) {
      border-color: $color-primary;
      box-shadow: $shadow-card;
    }

    &:disabled {
      opacity: 0.4;
      cursor: not-allowed;
      text-decoration: line-through;
    }
  }

  .error {
    display: flex;
    align-items: center;
    gap: $space-2;
    color: $color-danger;
    margin: 0;
  }
</style>
