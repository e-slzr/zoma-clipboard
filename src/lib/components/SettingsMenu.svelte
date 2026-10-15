<script lang="ts">
  import { ICONS } from '../icons';
  import { TRANSLATIONS, type Language } from '../i18n';
  import type { AppSettings } from '../types';

  let {
    settings,
    t = TRANSLATIONS.es,
    isOpen = false,
    onClose,
    onSelectTheme,
    onSelectLanguage,
    onOpenAbout,
  }: {
    settings: AppSettings;
    t?: typeof TRANSLATIONS.es;
    isOpen: boolean;
    onClose: () => void;
    onSelectTheme: (theme: AppSettings['theme']) => void;
    onSelectLanguage: (lang: Language) => void;
    onOpenAbout: () => void;
  } = $props();

  let themes = $derived([
    { id: 'pantheon-light' as const, label: t.themePantheonLight },
    { id: 'pantheon-dark' as const, label: t.themePantheonDark },
    { id: 'modern-flat-light' as const, label: t.themeModernLight },
    { id: 'modern-flat-dark' as const, label: t.themeModernDark },
  ]);

  let languages: { id: Language; label: string }[] = [
    { id: 'es', label: 'Español' },
    { id: 'en', label: 'English' },
  ];
</script>

{#if isOpen}
  <!-- Backdrop to close on outside click -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="menu-backdrop" onclick={onClose}></div>

  <div class="settings-popover">
    <!-- Visual Theme Section -->
    <div class="menu-header">{t.visualTheme}</div>

    <div class="options-group">
      {#each themes as item}
        <button
          class="menu-item"
          class:active={settings.theme === item.id}
          onclick={() => onSelectTheme(item.id)}
          type="button"
        >
          <span class="check-icon">
            {#if settings.theme === item.id}
              <svg width="12" height="12" viewBox={ICONS.check.viewBox} fill="currentColor">
                <path d={ICONS.check.path} />
              </svg>
            {/if}
          </span>
          <span class="item-label">{item.label}</span>
        </button>
      {/each}
    </div>

    <div class="menu-divider"></div>

    <!-- Language Section -->
    <div class="menu-header">{t.languageSection}</div>

    <div class="options-group">
      {#each languages as lang}
        <button
          class="menu-item"
          class:active={settings.language === lang.id}
          onclick={() => onSelectLanguage(lang.id)}
          type="button"
        >
          <span class="check-icon">
            {#if settings.language === lang.id}
              <svg width="12" height="12" viewBox={ICONS.check.viewBox} fill="currentColor">
                <path d={ICONS.check.path} />
              </svg>
            {/if}
          </span>
          <span class="item-label">{lang.label}</span>
        </button>
      {/each}
    </div>

    <div class="menu-divider"></div>

    <!-- About App -->
    <button
      class="menu-item"
      onclick={() => {
        onClose();
        onOpenAbout();
      }}
      type="button"
    >
      <span class="check-icon"></span>
      <span class="item-label">{t.aboutApp}</span>
    </button>
  </div>
{/if}

<style>
  .menu-backdrop {
    position: absolute;
    inset: 0;
    z-index: 99;
  }

  .settings-popover {
    position: absolute;
    top: 48px;
    right: 16px;
    width: 250px;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.2), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
    padding: 8px 0;
    z-index: 100;
    user-select: none;
    animation: popoverFadeIn 0.12s ease-out;
  }

  @keyframes popoverFadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px) scale(0.98);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .menu-header {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.05em;
    color: var(--text-secondary);
    padding: 6px 14px 4px 14px;
  }

  .options-group {
    display: flex;
    flex-direction: column;
  }

  .menu-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 7px 14px;
    border: none;
    background: transparent;
    color: var(--text-primary);
    font-size: 12px;
    font-family: inherit;
    text-align: left;
    cursor: pointer;
    transition: background-color 0.12s ease;
  }

  .menu-item:hover {
    background-color: var(--hover-bg);
  }

  .menu-item.active {
    background-color: var(--menu-active-bg);
    color: var(--text-primary);
  }

  .check-icon {
    width: 16px;
    display: flex;
    align-items: center;
    color: var(--accent-color);
    margin-right: 6px;
    flex-shrink: 0;
  }

  .item-label {
    flex: 1;
    font-weight: 500;
  }

  .menu-divider {
    height: 1px;
    background-color: var(--border-color);
    margin: 6px 0;
  }
</style>
