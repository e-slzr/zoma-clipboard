<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import type { AppSettings, CategoryId, Clip } from '$lib/types';
  import { TRANSLATIONS, type Language } from '$lib/i18n';
  import Header from '$lib/components/Header.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import CategoryToolbar from '$lib/components/CategoryToolbar.svelte';
  import ClipCard from '$lib/components/ClipCard.svelte';
  import FooterBar from '$lib/components/FooterBar.svelte';
  import SettingsMenu from '$lib/components/SettingsMenu.svelte';
  import PreviewModal from '$lib/components/PreviewModal.svelte';
  import AboutModal from '$lib/components/AboutModal.svelte';

  let clips = $state<Clip[]>([]);
  let settings = $state<AppSettings>({
    theme: 'pantheon-light',
    language: 'es',
    clear_on_exit: false,
    global_shortcut: 'Super+V',
    max_history: 100,
  });

  let t = $derived(TRANSLATIONS[settings.language] || TRANSLATIONS.es);

  let searchTerm = $state('');
  let activeCategory = $state<CategoryId>('all');
  let selectedIndex = $state(0);
  let isSettingsOpen = $state(false);
  let previewClip = $state<Clip | null>(null);
  let showAboutModal = $state(false);
  let searchBarComponent: SearchBar;

  let pinnedClips = $derived(clips.filter((c) => c.is_pinned));
  let recentClips = $derived(clips.filter((c) => !c.is_pinned));
  let visibleClips = $derived(
    activeCategory === 'favorites' ? pinnedClips : clips
  );

  async function loadSettings() {
    try {
      const res = await invoke<AppSettings>('get_settings');
      if (res) {
        settings = res;
      }
    } catch (e) {
      console.error('Failed to load settings:', e);
    }
  }

  async function fetchClips() {
    try {
      const catParam = activeCategory === 'all' ? null : activeCategory;
      const searchParam = searchTerm.trim().length > 0 ? searchTerm : null;
      const res = await invoke<Clip[]>('get_clips', {
        search: searchParam,
        category: catParam,
      });
      clips = res || [];
      if (selectedIndex >= clips.length) {
        selectedIndex = Math.max(0, clips.length - 1);
      }
    } catch (e) {
      console.error('Failed to fetch clips:', e);
    }
  }

  async function handlePaste(clip: Clip) {
    try {
      await invoke('paste_clip', { id: clip.id });
    } catch (e) {
      console.error('Failed to paste clip:', e);
    }
  }

  async function handleCopy(clip: Clip) {
    try {
      await invoke('copy_clip', { id: clip.id });
    } catch (e) {
      console.error('Failed to copy clip:', e);
    }
  }

  async function handleDelete(clip: Clip) {
    try {
      await invoke('delete_clip', { id: clip.id });
      await fetchClips();
    } catch (e) {
      console.error('Failed to delete clip:', e);
    }
  }

  async function handleTogglePin(clip: Clip) {
    try {
      await invoke('toggle_pin', { id: clip.id });
      await fetchClips();
    } catch (e) {
      console.error('Failed to toggle pin:', e);
    }
  }

  async function handleClearHistory() {
    try {
      await invoke('clear_history');
      await fetchClips();
    } catch (e) {
      console.error('Failed to clear history:', e);
    }
  }

  async function handleThemeChange(theme: AppSettings['theme']) {
    settings.theme = theme;
    try {
      await invoke('save_settings', { settings });
    } catch (e) {
      console.error('Failed to save settings:', e);
    }
  }

  async function handleLanguageChange(lang: Language) {
    settings.language = lang;
    try {
      await invoke('save_settings', { settings });
    } catch (e) {
      console.error('Failed to save settings:', e);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    // If modal is open
    if (previewClip || showAboutModal || isSettingsOpen) {
      if (e.key === 'Escape') {
        e.preventDefault();
        previewClip = null;
        showAboutModal = false;
        isSettingsOpen = false;
      }
      return;
    }

    // Ctrl + F focuses search
    if (e.ctrlKey && e.key.toLowerCase() === 'f') {
      e.preventDefault();
      searchBarComponent?.focus();
      return;
    }

    const isInputFocused =
      document.activeElement?.tagName === 'INPUT' ||
      document.activeElement?.tagName === 'TEXTAREA';

    if (e.key === 'Escape') {
      e.preventDefault();
      if (searchTerm.length > 0) {
        searchTerm = '';
        fetchClips();
      } else {
        invoke('hide_window');
      }
      return;
    }

    if (isInputFocused) {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        document.activeElement && (document.activeElement as HTMLElement).blur();
        selectedIndex = Math.min(selectedIndex + 1, visibleClips.length - 1);
        return;
      }
      if (e.key === 'ArrowUp') {
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        return;
      }
      if (e.key === 'Enter') {
        e.preventDefault();
        const current = visibleClips[selectedIndex];
        if (current) {
          handlePaste(current);
        }
        return;
      }
      return;
    }

    // Key navigation
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, visibleClips.length - 1);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const current = visibleClips[selectedIndex];
      if (current) {
        handlePaste(current);
      }
    } else if (e.key === ' ') {
      e.preventDefault();
      const current = visibleClips[selectedIndex];
      if (current) {
        previewClip = current;
      }
    } else if (e.ctrlKey && e.key.toLowerCase() === 'c') {
      e.preventDefault();
      const current = visibleClips[selectedIndex];
      if (current) {
        handleCopy(current);
      }
    } else if (e.key === 'Delete') {
      e.preventDefault();
      const current = visibleClips[selectedIndex];
      if (current) {
        handleDelete(current);
      }
    } else if (!e.ctrlKey && !e.metaKey && !e.altKey && e.key >= '1' && e.key <= '9') {
      const num = parseInt(e.key, 10);
      if (num <= visibleClips.length) {
        e.preventDefault();
        handlePaste(visibleClips[num - 1]);
      }
    }
  }

  function handleWindowBlur() {
    if (!previewClip && !showAboutModal && !isSettingsOpen) {
      invoke('hide_window');
    }
  }

  $effect(() => {
    // Re-fetch when search or category changes
    searchTerm;
    activeCategory;
    fetchClips();
  });

  onMount(() => {
    loadSettings();
    fetchClips();

    // Listen to background clipboard additions
    const unlistenClip = listen('clipboard-updated', () => {
      fetchClips();
    });

    const unlistenSearch = listen('focus-search', () => {
      searchTerm = '';
      selectedIndex = 0;
      fetchClips();
      setTimeout(() => {
        window.focus();
        searchBarComponent?.focus();
      }, 30);
    });

    return () => {
      unlistenClip.then((unlisten) => unlisten());
      unlistenSearch.then((unlisten) => unlisten());
    };
  });
</script>

<svelte:window onkeydown={handleKeydown} onblur={handleWindowBlur} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="app-root"
  data-theme={settings.theme}
  onclick={(e) => {
    if (e.target === e.currentTarget) {
      invoke('hide_window');
    }
  }}
>
  <div class="window-shell">
    <!-- Header -->
    <Header
      isSettingsOpen={isSettingsOpen}
      onToggleSettings={() => (isSettingsOpen = !isSettingsOpen)}
    />

    <!-- Settings Popover -->
    <SettingsMenu
      settings={settings}
      t={t}
      isOpen={isSettingsOpen}
      onClose={() => (isSettingsOpen = false)}
      onSelectTheme={handleThemeChange}
      onSelectLanguage={handleLanguageChange}
      onOpenAbout={() => (showAboutModal = true)}
    />

    <!-- Search Bar -->
    <SearchBar
      bind:this={searchBarComponent}
      bind:searchTerm={searchTerm}
      t={t}
      onClear={() => fetchClips()}
    />

    <!-- Category Toolbar -->
    <CategoryToolbar
      bind:activeCategory={activeCategory}
      t={t}
      onSelect={() => fetchClips()}
    />

    <!-- Clips List Area -->
    <main class="clips-scroll-area">
      <!-- Pinned / Favoritos Section (shown in 'all' or 'favorites' mode when there are pinned clips) -->
      {#if activeCategory !== 'favorites' && pinnedClips.length > 0 && searchTerm.length === 0}
        <section class="clips-section">
          <div class="section-header">
            <span class="section-title">{t.pinnedSection}</span>
          </div>
          <div class="section-list">
            {#each pinnedClips as clip, i}
              {@const globalIdx = clips.findIndex((c) => c.id === clip.id)}
              <ClipCard
                clip={clip}
                isCompact={true}
                isSelected={selectedIndex === globalIdx}
                t={t}
                onSelect={() => (selectedIndex = globalIdx)}
                onPaste={() => handlePaste(clip)}
                onCopy={() => handleCopy(clip)}
                onDelete={() => handleDelete(clip)}
                onTogglePin={() => handleTogglePin(clip)}
              />
            {/each}
          </div>
        </section>
      {/if}

      <!-- Recent History Section -->
      {#if activeCategory !== 'favorites'}
        <section class="clips-section">
          <div class="section-header">
            <span class="section-title">{t.recentSection}</span>
            {#if recentClips.length > 0}
              <button
                class="clear-action-btn"
                onclick={handleClearHistory}
                title={t.clear}
                type="button"
              >
                {t.clear}
              </button>
            {/if}
          </div>

          <div class="section-list">
            {#if recentClips.length === 0}
              <div class="empty-state">
                <span>{t.emptyRecent}</span>
              </div>
            {:else}
              {#each recentClips as clip, i}
                {@const globalIdx = clips.findIndex((c) => c.id === clip.id)}
                <ClipCard
                  clip={clip}
                  index={i + 1 <= 9 ? i + 1 : undefined}
                  isSelected={selectedIndex === globalIdx}
                  t={t}
                  onSelect={() => (selectedIndex = globalIdx)}
                  onPaste={() => handlePaste(clip)}
                  onCopy={() => handleCopy(clip)}
                  onDelete={() => handleDelete(clip)}
                  onTogglePin={() => handleTogglePin(clip)}
                />
              {/each}
            {/if}
          </div>
        </section>
      {:else}
        <!-- Favorites Tab Full List -->
        <section class="clips-section">
          <div class="section-header">
            <span class="section-title">{t.favorites} ({pinnedClips.length})</span>
          </div>
          <div class="section-list">
            {#if pinnedClips.length === 0}
              <div class="empty-state">
                <span>{t.emptyFavorites}</span>
              </div>
            {:else}
              {#each pinnedClips as clip, i}
                <ClipCard
                  clip={clip}
                  index={i + 1 <= 9 ? i + 1 : undefined}
                  isSelected={selectedIndex === i}
                  t={t}
                  onSelect={() => (selectedIndex = i)}
                  onPaste={() => handlePaste(clip)}
                  onCopy={() => handleCopy(clip)}
                  onDelete={() => handleDelete(clip)}
                  onTogglePin={() => handleTogglePin(clip)}
                />
              {/each}
            {/if}
          </div>
        </section>
      {/if}
    </main>

    <!-- Footer Bar -->
    <FooterBar t={t} />

    <!-- Spacebar Detail Preview Modal (contained inside window-shell) -->
    {#if previewClip}
      <PreviewModal
        clip={previewClip}
        t={t}
        onClose={() => (previewClip = null)}
        onCopy={() => {
          if (previewClip) handleCopy(previewClip);
        }}
        onPaste={() => {
          if (previewClip) handlePaste(previewClip);
          previewClip = null;
        }}
      />
    {/if}

    <!-- About Dialog (contained inside window-shell) -->
    {#if showAboutModal}
      <AboutModal t={t} onClose={() => (showAboutModal = false)} />
    {/if}
  </div>
</div>

<style>
  /* 1. Pantheon Claro (elementary OS light) */
  .app-root[data-theme='pantheon-light'] {
    --window-radius: 12px;
    --control-radius: 6px;
    --window-bg: #FFFFFF;
    --header-bg: #FFFFFF;
    --footer-bg: #F7F7F7;
    --search-bg: #F0F0F0;
    --toolbar-bg: #EAEAEA;
    --tab-active-bg: #FFFFFF;
    --tab-active-text: #1C1E21;
    --tab-hover-bg: rgba(0, 0, 0, 0.05);
    --card-bg: #FFFFFF;
    --card-hover-bg: #F7F7F7;
    --card-selected-bg: #EBF3FC;
    --code-box-bg: #F0F3F7;
    --hover-bg: rgba(0, 0, 0, 0.06);
    --border-color: rgba(0, 0, 0, 0.13);
    --border-hover-color: rgba(0, 0, 0, 0.22);
    --text-primary: #1C1E21;
    --text-secondary: #6B6F76;
    --text-placeholder: #9CA0A6;
    --accent-color: #3689E6;
    --accent-alpha: rgba(54, 137, 230, 0.20);
    --esc-pill-bg: #E4E6EB;
    --esc-pill-text: #6B6F76;
    --tag-bg: #EAEAEA;
    --menu-active-bg: #EBF3FC;
    --window-shadow: 0 8px 22px rgba(0, 0, 0, 0.14), 0 2px 6px rgba(0, 0, 0, 0.08);
  }

  /* 2. Pantheon Oscuro (elementary OS dark) */
  .app-root[data-theme='pantheon-dark'] {
    --window-radius: 12px;
    --control-radius: 6px;
    --window-bg: #3A3A3A;
    --header-bg: #3A3A3A;
    --footer-bg: #333333;
    --search-bg: #2B2B2B;
    --toolbar-bg: #2D2D2D;
    --tab-active-bg: #484848;
    --tab-active-text: #FFFFFF;
    --tab-hover-bg: rgba(255, 255, 255, 0.08);
    --card-bg: #313131;
    --card-hover-bg: #383838;
    --card-selected-bg: #373F4D;
    --code-box-bg: #262626;
    --hover-bg: rgba(255, 255, 255, 0.10);
    --border-color: rgba(255, 255, 255, 0.11);
    --border-hover-color: rgba(255, 255, 255, 0.20);
    --text-primary: #FFFFFF;
    --text-secondary: rgba(255, 255, 255, 0.65);
    --text-placeholder: rgba(255, 255, 255, 0.40);
    --accent-color: #3689E6;
    --accent-alpha: rgba(54, 137, 230, 0.25);
    --esc-pill-bg: #444444;
    --esc-pill-text: rgba(255, 255, 255, 0.70);
    --tag-bg: #444444;
    --menu-active-bg: #373F4D;
    --window-shadow: 0 10px 26px rgba(0, 0, 0, 0.36), 0 3px 8px rgba(0, 0, 0, 0.22);
  }

  /* 3. Modern Flat Claro */
  .app-root[data-theme='modern-flat-light'] {
    --window-radius: 6px;
    --control-radius: 4px;
    --window-bg: #FFFFFF;
    --header-bg: #F8FAFC;
    --footer-bg: #F8FAFC;
    --search-bg: #F1F5F9;
    --toolbar-bg: #E2E8F0;
    --tab-active-bg: #FFFFFF;
    --tab-active-text: #0F172A;
    --tab-hover-bg: rgba(255, 255, 255, 0.5);
    --card-bg: #FFFFFF;
    --card-hover-bg: #F8FAFC;
    --card-selected-bg: #EFF6FF;
    --code-box-bg: #F1F5F9;
    --hover-bg: #E2E8F0;
    --border-color: #CBD5E1;
    --border-hover-color: #94A3B8;
    --text-primary: #0F172A;
    --text-secondary: #64748B;
    --text-placeholder: #94A3B8;
    --accent-color: #2563EB;
    --accent-alpha: rgba(37, 99, 235, 0.2);
    --esc-pill-bg: #E2E8F0;
    --esc-pill-text: #64748B;
    --tag-bg: #E2E8F0;
    --menu-active-bg: #EFF6FF;
    --window-shadow: 0 8px 20px rgba(15, 23, 42, 0.12), 0 2px 6px rgba(15, 23, 42, 0.06);
  }

  /* 4. Modern Flat Oscuro */
  .app-root[data-theme='modern-flat-dark'] {
    --window-radius: 6px;
    --control-radius: 4px;
    --window-bg: #0B0F19;
    --header-bg: #0F172A;
    --footer-bg: #0F172A;
    --search-bg: #0F172A;
    --toolbar-bg: #0F172A;
    --tab-active-bg: #1E293B;
    --tab-active-text: #F8FAFC;
    --tab-hover-bg: rgba(255, 255, 255, 0.05);
    --card-bg: #111827;
    --card-hover-bg: #162032;
    --card-selected-bg: #131C31;
    --code-box-bg: #0A0F1D;
    --hover-bg: #1E293B;
    --border-color: #334155;
    --border-hover-color: #475569;
    --text-primary: #F8FAFC;
    --text-secondary: #94A3B8;
    --text-placeholder: #64748B;
    --accent-color: #3B82F6;
    --accent-alpha: rgba(59, 130, 246, 0.25);
    --esc-pill-bg: #1E293B;
    --esc-pill-text: #94A3B8;
    --tag-bg: #1E293B;
    --menu-active-bg: #1E293B;
    --window-shadow: 0 10px 26px rgba(0, 0, 0, 0.45), 0 3px 8px rgba(0, 0, 0, 0.25);
  }

  .app-root {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    padding: 44px;
    overflow: hidden;
    box-sizing: border-box;
  }

  .window-shell {
    width: 100%;
    height: 100%;
    max-width: 440px;
    max-height: 720px;
    display: flex;
    flex-direction: column;
    background-color: var(--window-bg);
    border: 1px solid var(--border-color);
    border-radius: var(--window-radius);
    box-shadow: var(--window-shadow);
    overflow: hidden;
    position: relative;
    transition: background-color 0.2s ease, border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .clips-scroll-area {
    flex: 1;
    overflow-y: auto;
    padding: 0 16px 12px 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  /* Custom subtle scrollbar */
  .clips-scroll-area::-webkit-scrollbar {
    width: 6px;
  }

  .clips-scroll-area::-webkit-scrollbar-track {
    background: transparent;
  }

  .clips-scroll-area::-webkit-scrollbar-thumb {
    background: var(--border-color);
    border-radius: 3px;
  }

  .clips-scroll-area::-webkit-scrollbar-thumb:hover {
    background: var(--border-hover-color);
  }

  .clips-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 2px;
  }

  .section-title {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.04em;
    color: var(--text-secondary);
  }

  .clear-action-btn {
    background: transparent;
    border: none;
    font-size: 11.5px;
    font-weight: 500;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 2px 4px;
    border-radius: 3px;
    transition: color 0.15s ease;
  }

  .clear-action-btn:hover {
    color: var(--text-primary);
  }

  .section-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 28px 16px;
    color: var(--text-secondary);
    font-size: 12px;
    text-align: center;
  }
</style>
