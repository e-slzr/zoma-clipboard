<script lang="ts">
  import { ICONS } from '../icons';
  import { TRANSLATIONS } from '../i18n';
  import type { CategoryId } from '../types';

  let {
    activeCategory = $bindable('all'),
    t = TRANSLATIONS.es,
    onSelect,
  }: {
    activeCategory: CategoryId;
    t?: typeof TRANSLATIONS.es;
    onSelect?: (cat: CategoryId) => void;
  } = $props();

  let tabs = $derived<{ id: CategoryId; label: string; icon: keyof typeof ICONS }[]>([
    { id: 'all', label: t.all, icon: 'apps' },
    { id: 'text', label: t.text, icon: 'text' },
    { id: 'code', label: t.code, icon: 'code' },
    { id: 'url', label: t.links, icon: 'link' },
    { id: 'image', label: t.images, icon: 'photo' },
    { id: 'favorites', label: t.favorites, icon: 'heart' },
  ]);

  function choose(id: CategoryId) {
    activeCategory = id;
    onSelect?.(id);
  }
</script>

<div class="toolbar-container" role="tablist">
  {#each tabs as tab}
    <button
      class="tab-btn"
      class:active={activeCategory === tab.id}
      onclick={() => choose(tab.id)}
      role="tab"
      aria-selected={activeCategory === tab.id}
      title={tab.label}
      type="button"
    >
      <svg
        width="15"
        height="15"
        viewBox={ICONS[tab.icon].viewBox}
        fill="currentColor"
        class="tab-icon"
      >
        <path d={ICONS[tab.icon].path} />
      </svg>
    </button>
  {/each}
</div>

<style>
  .toolbar-container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 32px;
    margin: 4px 16px 12px 16px;
    padding: 2px;
    background-color: var(--toolbar-bg);
    border-radius: var(--control-radius);
    user-select: none;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    height: 28px;
    border: none;
    border-radius: calc(var(--control-radius) - 2px);
    background-color: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease, transform 0.1s ease;
  }

  .tab-btn:hover:not(.active) {
    background-color: var(--tab-hover-bg);
    color: var(--text-primary);
  }

  .tab-btn.active {
    background-color: var(--tab-active-bg);
    color: var(--tab-active-text);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
  }

  .tab-icon {
    display: block;
  }
</style>
