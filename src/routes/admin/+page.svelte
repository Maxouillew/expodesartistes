<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { adminAuth } from "$lib/stores/adminAuth.svelte";
  import { adminNeedsSetup } from "$lib/services/auth";
  import LoginForm from "$lib/components/admin/LoginForm.svelte";
  import SetPasswordForm from "$lib/components/admin/SetPasswordForm.svelte";
  import RankingTab from "$lib/components/admin/RankingTab.svelte";
  import ArtistsTab from "$lib/components/admin/ArtistsTab.svelte";
  import VotersTab from "$lib/components/admin/VotersTab.svelte";
  import DrawTab from "$lib/components/admin/DrawTab.svelte";
  import VersionTab from "$lib/components/admin/VersionTab.svelte";
  import Button from "$lib/components/Button.svelte";
  import Icon, { type IconName } from "$lib/components/Icon.svelte";

  type TabId = "classement" | "artistes" | "votants" | "tirage" | "version";
  const TABS: { id: TabId; label: string; icon: IconName }[] = [
    { id: "classement", label: "Classement", icon: "trophy" },
    { id: "artistes", label: "Artistes", icon: "users" },
    { id: "votants", label: "Votants", icon: "list" },
    { id: "tirage", label: "Tirage au sort", icon: "shuffle" },
    { id: "version", label: "Version", icon: "info" },
  ];

  let needsSetup = $state<boolean | null>(null);
  let loadError = $state<string | undefined>(undefined);
  let activeTab = $state<TabId>("classement");

  async function checkSetup() {
    loadError = undefined;
    try {
      needsSetup = await adminNeedsSetup();
    } catch (err) {
      loadError = String(err);
    }
  }

  onMount(checkSetup);

  // Leaving the admin route always requires the password again next time —
  // "Retour au vote" is the only way out, and doubles as the sign-out
  // action. Important on a public kiosk: someone else could otherwise
  // reopen /admin after the admin merely navigates back to the vote screen.
  onDestroy(() => adminAuth.logout());

  function onTabKeydown(event: KeyboardEvent, index: number) {
    if (event.key !== "ArrowRight" && event.key !== "ArrowLeft") return;
    event.preventDefault();
    const direction = event.key === "ArrowRight" ? 1 : -1;
    const nextIndex = (index + direction + TABS.length) % TABS.length;
    const nextTab = TABS[nextIndex];
    activeTab = nextTab.id;
    document.getElementById(`tab-${nextTab.id}`)?.focus();
  }
</script>

<div class="top-bar">
  <div class="brand">
    <img src="/logo.png" alt="" class="brand-logo" />
    <span class="brand-title">Exposition des artistes.</span>
  </div>
  <a href="/" class="home-link">
    <Icon name="arrow-left" size={18} />
    Retour au vote
  </a>
</div>

<main class="admin">
  {#if needsSetup === null}
    {#if loadError}
      <div class="loading">
        <p class="error" role="alert">
          <Icon name="alert-circle" size={20} />
          {loadError}
        </p>
        <Button variant="secondary" icon="refresh" onclick={checkSetup}>Réessayer</Button>
      </div>
    {:else}
      <p class="loading">Chargement...</p>
    {/if}
  {:else if !adminAuth.isAuthenticated}
    {#if needsSetup}
      <SetPasswordForm
        onDone={() => {
          needsSetup = false;
          adminAuth.login();
        }}
      />
    {:else}
      <LoginForm onSuccess={() => adminAuth.login()} />
    {/if}
  {:else}
    <header class="admin-header">
      <h1>Administration</h1>
    </header>

    <div role="tablist" aria-label="Sections admin" class="tablist">
      {#each TABS as tab, index (tab.id)}
        <button
          id="tab-{tab.id}"
          role="tab"
          aria-selected={activeTab === tab.id}
          aria-controls="panel-{tab.id}"
          tabindex={activeTab === tab.id ? 0 : -1}
          class="tab"
          class:active={activeTab === tab.id}
          onclick={() => (activeTab = tab.id)}
          onkeydown={(event) => onTabKeydown(event, index)}
        >
          <Icon name={tab.icon} size={18} />
          {tab.label}
        </button>
      {/each}
    </div>

    {#if activeTab === "classement"}
      <div role="tabpanel" id="panel-classement" aria-labelledby="tab-classement" class="animate-in">
        <RankingTab />
      </div>
    {:else if activeTab === "artistes"}
      <div role="tabpanel" id="panel-artistes" aria-labelledby="tab-artistes" class="animate-in">
        <ArtistsTab />
      </div>
    {:else if activeTab === "votants"}
      <div role="tabpanel" id="panel-votants" aria-labelledby="tab-votants" class="animate-in">
        <VotersTab />
      </div>
    {:else if activeTab === "tirage"}
      <div role="tabpanel" id="panel-tirage" aria-labelledby="tab-tirage" class="animate-in">
        <DrawTab />
      </div>
    {:else}
      <div role="tabpanel" id="panel-version" aria-labelledby="tab-version" class="animate-in">
        <VersionTab />
      </div>
    {/if}
  {/if}
</main>

<style lang="scss">
  @use "../../lib/styles/variables" as *;

  .top-bar {
    max-width: $max-content-width;
    margin: 0 auto;
    padding: $space-4 $space-5 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: $space-3;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: $space-2;
  }

  .brand-logo {
    height: 1.75rem;
    width: auto;
  }

  .brand-title {
    font-weight: 700;
    color: $color-text-muted;
  }

  .home-link {
    display: inline-flex;
    align-items: center;
    gap: $space-2;
    min-height: $min-tap-target;
    padding: $space-2 $space-3;
    color: $color-text-muted;
    text-decoration: none;
    font-weight: 600;
    transition:
      color $duration-fast ease-out,
      transform $duration-fast ease-out;

    &:hover {
      color: $color-primary;
      text-decoration: underline;
      transform: translateX(-2px);
    }
  }

  .admin {
    max-width: $max-content-width;
    margin: 0 auto;
    padding: $space-5;
  }

  .loading {
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

  .admin-header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: $space-3;
    margin-bottom: $space-5;

    h1 {
      margin: 0;
    }
  }

  .tablist {
    display: flex;
    gap: $space-2;
    border-bottom: 2px solid $color-border;
    margin-bottom: $space-5;
  }

  .tab {
    min-height: $min-tap-target;
    padding: $space-3 $space-4;
    display: inline-flex;
    align-items: center;
    gap: $space-2;
    background: none;
    border: none;
    border-bottom: 3px solid transparent;
    font-size: $font-size-base;
    font-weight: 600;
    color: $color-text-muted;
    cursor: pointer;
    transition:
      color $duration-fast ease-out,
      border-color $duration-fast ease-out;

    &:hover {
      color: $color-primary;
    }

    &.active {
      color: $color-primary;
      border-bottom-color: $color-primary;
    }
  }
</style>
