<script lang="ts">
  import { Dialog } from "bits-ui";
  import type { Snippet } from "svelte";
  import Icon from "./Icon.svelte";

  let {
    open = $bindable(false),
    title,
    children,
    footer,
  }: {
    open?: boolean;
    title: string;
    children: Snippet;
    footer?: Snippet;
  } = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-overlay" />
    <Dialog.Content class="modal-content">
      <div class="modal-header">
        <Dialog.Title class="modal-title">{title}</Dialog.Title>
        <Dialog.Close class="modal-close" aria-label="Fermer">
          <Icon name="x" size={20} />
        </Dialog.Close>
      </div>
      <div class="modal-body">
        {@render children()}
      </div>
      {#if footer}
        <div class="modal-footer">
          {@render footer()}
        </div>
      {/if}
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style lang="scss">
  @use "../styles/variables" as *;

  @keyframes overlay-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes content-in {
    from {
      opacity: 0;
      transform: translate(-50%, -48%) scale(0.97);
    }
    to {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
    }
  }

  :global(.modal-overlay) {
    position: fixed;
    inset: 0;
    background-color: rgba($color-text, 0.38);
    backdrop-filter: blur(3px);
    -webkit-backdrop-filter: blur(3px);
    z-index: 100;
    animation: overlay-in $duration-fast ease-out;
  }

  :global(.modal-content) {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: min(90vw, 30rem);
    max-height: 85vh;
    overflow-y: auto;
    background-color: $color-surface;
    border: 1px solid $color-border;
    border-radius: $radius;
    padding: $space-5;
    z-index: 101;
    box-shadow: $shadow-modal;
    animation: content-in $duration-base $ease-out;
  }

  .modal-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: $space-3;
    margin-bottom: $space-4;
  }

  :global(.modal-title) {
    margin: 0;
    font-size: $font-size-lg;
  }

  :global(.modal-close) {
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: $min-tap-target;
    height: $min-tap-target;
    margin: -#{$space-3} -#{$space-3} 0 0;
    background: none;
    border: none;
    border-radius: $radius;
    color: $color-text-muted;
    cursor: pointer;
    transition:
      background-color $duration-fast ease-out,
      color $duration-fast ease-out;

    &:hover {
      background-color: $color-background;
      color: $color-text;
    }
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: $space-3;
    margin-top: $space-5;
  }
</style>
