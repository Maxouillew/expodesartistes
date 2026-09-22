<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import Icon from "./Icon.svelte";

  let {
    id,
    label,
    type = "text",
    value = $bindable(""),
    error,
    autocomplete,
    placeholder,
    onblur,
  }: {
    id: string;
    label: string;
    type?: "text" | "password" | "number" | "tel";
    value?: string;
    error?: string;
    autocomplete?: HTMLInputAttributes["autocomplete"];
    placeholder?: string;
    onblur?: (event: FocusEvent) => void;
  } = $props();

  let errorId = $derived(`${id}-error`);
</script>

<div class="field">
  <label for={id}>{label}</label>
  <input
    {id}
    {type}
    {autocomplete}
    {placeholder}
    bind:value
    {onblur}
    aria-invalid={error ? "true" : undefined}
    aria-describedby={error ? errorId : undefined}
  />
  {#if error}
    <p class="error" id={errorId}>
      <Icon name="alert-circle" size={16} />
      {error}
    </p>
  {/if}
</div>

<style lang="scss">
  @use "../styles/variables" as *;

  .field {
    display: flex;
    flex-direction: column;
    gap: $space-1;
    text-align: left;
  }

  label {
    font-weight: 600;
  }

  input {
    min-height: $min-tap-target;
    padding: $space-3;
    border: 1px solid $color-border;
    border-radius: $radius;
    background-color: $color-surface;
    color: $color-text;
    transition:
      border-color $duration-fast ease-out,
      box-shadow $duration-fast ease-out;

    &[aria-invalid="true"] {
      border-color: $color-danger;
    }

    &::placeholder {
      color: $color-text-muted;
      opacity: 0.75;
    }
  }

  .error {
    margin: 0;
    display: flex;
    align-items: center;
    gap: $space-1;
    color: $color-danger;
    font-size: $font-size-sm;
  }
</style>
