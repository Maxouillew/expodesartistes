<script lang="ts">
  import { onMount } from "svelte";
  import {
    getCurrentVersion,
    checkForUpdate,
    installUpdateAndRelaunch,
    type Update,
  } from "$lib/services/update";
  import Button from "$lib/components/Button.svelte";
  import Icon from "$lib/components/Icon.svelte";

  type Status = "idle" | "checking" | "up-to-date" | "available" | "downloading" | "error";

  let currentVersion = $state("");
  let status = $state<Status>("idle");
  let update = $state<Update | null>(null);
  let error = $state<string | undefined>(undefined);
  let downloadedBytes = $state(0);
  let totalBytes = $state<number | undefined>(undefined);

  onMount(async () => {
    try {
      currentVersion = await getCurrentVersion();
    } catch {
      // Non-critical: the version label just stays blank.
    }
  });

  const progressPercent = $derived(
    totalBytes ? Math.min(100, Math.round((downloadedBytes / totalBytes) * 100)) : undefined,
  );

  async function handleCheck() {
    status = "checking";
    error = undefined;
    try {
      const result = await checkForUpdate();
      update = result;
      status = result ? "available" : "up-to-date";
    } catch (err) {
      error = String(err);
      status = "error";
    }
  }

  async function handleInstall() {
    if (!update) return;
    status = "downloading";
    error = undefined;
    downloadedBytes = 0;
    totalBytes = undefined;
    try {
      await installUpdateAndRelaunch(update, (downloaded, total) => {
        downloadedBytes = downloaded;
        totalBytes = total;
      });
    } catch (err) {
      // Only reached if the relaunch itself fails — a successful install
      // replaces the running app before this line would execute otherwise.
      error = String(err);
      status = "error";
    }
  }
</script>

<div class="update-checker" role="status">
  {#if status === "available" && update}
    <span class="version-badge available">
      <Icon name="download" size={16} />
      Version {update.version} disponible
    </span>
    <Button icon="download" onclick={handleInstall}>Installer et redémarrer</Button>
  {:else if status === "downloading"}
    <span class="version-badge">
      <Icon name="download" size={16} />
      Téléchargement{progressPercent !== undefined ? ` (${progressPercent} %)` : "..."}
    </span>
  {:else if status === "error"}
    <span class="version-badge error">
      <Icon name="alert-circle" size={16} />
      {error}
    </span>
    <Button variant="secondary" icon="refresh" onclick={handleCheck}>Réessayer</Button>
  {:else}
    {#if status === "up-to-date"}
      <span class="version-badge up-to-date">
        <Icon name="check-circle" size={16} />
        Application à jour
      </span>
    {:else if currentVersion}
      <span class="version-badge">Version {currentVersion}</span>
    {/if}
    <Button
      variant="secondary"
      icon="refresh"
      onclick={handleCheck}
      disabled={status === "checking"}
    >
      {status === "checking" ? "Vérification..." : "Vérifier les mises à jour"}
    </Button>
  {/if}
</div>

<style lang="scss">
  @use "../../styles/variables" as *;

  .update-checker {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    justify-content: center;
    gap: $space-3;
  }

  .version-badge {
    display: inline-flex;
    align-items: center;
    gap: $space-1;
    color: $color-text-muted;
    font-size: $font-size-sm;
    font-weight: 600;

    &.available {
      color: $color-primary;
    }

    &.up-to-date {
      color: $color-success;
    }

    &.error {
      color: $color-danger;
    }
  }
</style>
