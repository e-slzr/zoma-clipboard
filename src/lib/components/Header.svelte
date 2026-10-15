<script lang="ts">
  import { ICONS } from '../icons';

  let { onToggleSettings, isSettingsOpen }: { onToggleSettings: () => void; isSettingsOpen: boolean } = $props();
</script>

<header class="header-bar" data-tauri-drag-region>
  <div class="header-title" data-tauri-drag-region>
    Zoma Clipboard
  </div>
  <button
    class="gear-btn"
    class:active={isSettingsOpen}
    onclick={onToggleSettings}
    title="Ajustes (⚙)"
    type="button"
  >
    <svg width="14" height="14" viewBox={ICONS.gear.viewBox} fill="currentColor">
      <path d={ICONS.gear.path} />
    </svg>
  </button>
</header>

<style>
  .header-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 44px;
    padding: 0 16px;
    background-color: var(--header-bg);
    border-bottom: 1px solid var(--border-color);
    border-top-left-radius: var(--window-radius);
    border-top-right-radius: var(--window-radius);
    user-select: none;
    -webkit-app-region: drag;
  }

  .header-title {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .gear-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 5px;
    border: 1px solid transparent;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
    -webkit-app-region: no-drag;
  }

  .gear-btn:hover {
    background-color: var(--hover-bg);
    color: var(--text-primary);
  }

  .gear-btn.active {
    background-color: var(--hover-bg);
    color: var(--accent-color);
  }
</style>
