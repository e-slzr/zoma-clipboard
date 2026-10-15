<script lang="ts">
  import { ICONS } from '../icons';
  import { TRANSLATIONS } from '../i18n';

  let {
    searchTerm = $bindable(''),
    t = TRANSLATIONS.es,
    onClear,
  }: {
    searchTerm: string;
    t?: typeof TRANSLATIONS.es;
    onClear?: () => void;
  } = $props();

  let inputEl: HTMLInputElement;

  export function focus() {
    inputEl?.focus();
  }
</script>

<div class="search-container">
  <div class="search-icon">
    <svg width="13" height="13" viewBox={ICONS.search.viewBox} fill="currentColor">
      <path d={ICONS.search.path} />
    </svg>
    <span class="search-label">{t.searchLabel}</span>
  </div>
  <input
    bind:this={inputEl}
    type="text"
    class="search-input"
    placeholder={t.searchPlaceholder}
    bind:value={searchTerm}
    spellcheck="false"
    autocomplete="off"
  />
  {#if searchTerm.length > 0}
    <button
      class="clear-pill"
      onclick={() => {
        searchTerm = '';
        onClear?.();
      }}
      title="Borrar búsqueda"
      type="button"
    >
      <svg width="10" height="10" viewBox={ICONS.close.viewBox} fill="currentColor">
        <path d={ICONS.close.path} />
      </svg>
    </button>
  {:else}
    <div class="esc-pill" title="Presiona Esc para salir">
      <span>Esc</span>
    </div>
  {/if}
</div>

<style>
  .search-container {
    display: flex;
    align-items: center;
    height: 36px;
    margin: 12px 16px 8px 16px;
    padding: 0 8px 0 12px;
    background-color: var(--search-bg);
    border: 1px solid var(--border-color);
    border-radius: var(--control-radius);
    transition: border-color 0.15s ease, box-shadow 0.15s ease;
  }

  .search-container:focus-within {
    border-color: var(--accent-color);
    box-shadow: 0 0 0 2px var(--accent-alpha);
  }

  .search-icon {
    display: flex;
    align-items: center;
    gap: 6px;
    color: var(--text-secondary);
    font-size: 11px;
    font-weight: 700;
    flex-shrink: 0;
  }

  .search-label {
    letter-spacing: -0.01em;
  }

  .search-input {
    flex: 1;
    height: 100%;
    margin-left: 8px;
    background: transparent;
    border: none;
    outline: none;
    font-family: inherit;
    font-size: 11.5px;
    color: var(--text-primary);
  }

  .search-input::placeholder {
    color: var(--text-placeholder);
    font-weight: 400;
  }

  .esc-pill,
  .clear-pill {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 20px;
    min-width: 32px;
    padding: 0 6px;
    border-radius: 4px;
    background-color: var(--esc-pill-bg);
    color: var(--esc-pill-text);
    font-size: 10.5px;
    font-weight: 600;
    user-select: none;
    border: none;
  }

  .clear-pill {
    cursor: pointer;
    min-width: 22px;
    padding: 0 4px;
  }

  .clear-pill:hover {
    background-color: var(--border-color);
    color: var(--text-primary);
  }
</style>
