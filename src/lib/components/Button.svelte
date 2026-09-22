<script lang="ts">
  import type { Snippet } from "svelte";
  import Icon, { type IconName } from "./Icon.svelte";

  type Variant = "primary" | "secondary" | "danger";

  let {
    variant = "primary",
    type = "button",
    disabled = false,
    icon,
    fullWidth = false,
    onclick,
    children,
  }: {
    variant?: Variant;
    type?: "button" | "submit";
    disabled?: boolean;
    icon?: IconName;
    fullWidth?: boolean;
    onclick?: (event: MouseEvent) => void;
    children: Snippet;
  } = $props();
</script>

<button class="btn {variant}" class:full-width={fullWidth} {type} {disabled} {onclick}>
  {#if icon}
    <Icon name={icon} size={18} />
  {/if}
  {@render children()}
</button>

<style lang="scss">
  @use "../styles/variables" as *;

  .btn {
    min-height: $min-tap-target;
    min-width: $min-tap-target;
    padding: $space-3 $space-5;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: $space-2;
    border-radius: $radius;
    border: 1px solid transparent;
    font-size: $font-size-base;
    font-weight: 600;
    cursor: pointer;
    transition:
      background-color $duration-fast ease-out,
      border-color $duration-fast ease-out,
      transform $duration-fast ease-out,
      box-shadow $duration-fast ease-out;

    &:hover:not(:disabled) {
      transform: translateY(-1px);
    }

    &:active:not(:disabled) {
      transform: translateY(0);
    }

    &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }
  }

  .full-width {
    width: 100%;
  }

  .primary {
    background-color: $color-primary;
    color: $color-primary-contrast;

    &:hover:not(:disabled) {
      background-color: $color-primary-hover;
      box-shadow: 0 4px 12px rgba($color-primary, 0.35);
    }
  }

  .secondary {
    background-color: $color-surface;
    color: $color-text;
    border-color: $color-border;

    &:hover:not(:disabled) {
      background-color: $color-background;
      border-color: $color-primary;
    }
  }

  .danger {
    background-color: $color-danger;
    color: $color-danger-contrast;

    &:hover:not(:disabled) {
      background-color: $color-danger-hover;
      box-shadow: 0 4px 12px rgba($color-danger, 0.35);
    }
  }
</style>
