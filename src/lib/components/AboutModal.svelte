<script lang="ts">
  import { ICONS } from '../icons';
  import { TRANSLATIONS } from '../i18n';

  let {
    onClose,
    t = TRANSLATIONS.es,
  }: {
    onClose: () => void;
    t?: typeof TRANSLATIONS.es;
  } = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={onClose}>
  <div class="modal-card" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <span class="modal-title">{t.aboutTitle}</span>
      <button class="close-btn" onclick={onClose} title="Cerrar" type="button">
        <svg width="14" height="14" viewBox={ICONS.close.viewBox} fill="currentColor">
          <path d={ICONS.close.path} />
        </svg>
      </button>
    </div>

    <div class="modal-body">
      <div class="app-brand">
        <div class="app-icon-wrap">
          <img src="/app-icon.png" alt="Zoma Clipboard" class="app-icon-img" width="36" height="36" style="border-radius: 8px;" />
        </div>
        <div>
          <h2 class="app-name">{t.appName}</h2>
          <span class="app-ver">{t.versionLabel}</span>
        </div>
      </div>

      <p class="app-desc">
        {t.aboutDesc}
      </p>

      <div class="specs-grid">
        <div class="spec-row">
          <span class="spec-label">{t.archLabel}:</span>
          <span class="spec-val">Tauri v2 + Rust 1.99 + Svelte 5</span>
        </div>
        <div class="spec-row">
          <span class="spec-label">{t.storageLabel}:</span>
          <span class="spec-val">{t.storageVal}</span>
        </div>
        <div class="spec-row">
          <span class="spec-label">{t.aestheticLabel}:</span>
          <span class="spec-val">{t.aestheticVal}</span>
        </div>
        <div class="spec-row">
          <span class="spec-label">{t.shortcutLabel}:</span>
          <span class="spec-val mono">Super + V</span>
        </div>
      </div>
    </div>

    <div class="modal-footer">
      <button class="primary-btn" onclick={onClose} type="button">
        {t.gotIt}
      </button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: absolute;
    inset: 0;
    border-radius: var(--window-radius);
    background-color: rgba(0, 0, 0, 0.45);
    backdrop-filter: blur(2px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
    padding: 20px;
    animation: fadeIn 0.15s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .modal-card {
    width: 100%;
    max-width: 380px;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 10px;
    box-shadow: 0 16px 32px rgba(0, 0, 0, 0.25);
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
    background-color: var(--header-bg);
  }

  .modal-title {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px;
    display: flex;
    border-radius: 4px;
  }

  .close-btn:hover {
    background-color: var(--hover-bg);
    color: var(--text-primary);
  }

  .modal-body {
    padding: 18px 16px;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .app-brand {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .app-icon-wrap {
    width: 44px;
    height: 44px;
    border-radius: 10px;
    background-color: var(--hover-bg);
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--border-color);
  }

  .app-name {
    font-size: 15px;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.2;
  }

  .app-ver {
    font-size: 11.5px;
    color: var(--text-secondary);
  }

  .app-desc {
    font-size: 12px;
    line-height: 1.55;
    color: var(--text-secondary);
  }

  .specs-grid {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 10px 12px;
    background-color: var(--search-bg);
    border-radius: 6px;
    border: 1px solid var(--border-color);
  }

  .spec-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 11.5px;
  }

  .spec-label {
    color: var(--text-secondary);
  }

  .spec-val {
    font-weight: 600;
    color: var(--text-primary);
  }

  .mono {
    font-family: 'JetBrains Mono', monospace;
    font-size: 11px;
    background: var(--tag-bg);
    padding: 1px 5px;
    border-radius: 3px;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    padding: 12px 16px;
    background-color: var(--footer-bg);
    border-top: 1px solid var(--border-color);
  }

  .primary-btn {
    padding: 6px 16px;
    background-color: var(--accent-color);
    color: #FFFFFF;
    border: none;
    border-radius: var(--control-radius);
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.15s ease;
  }

  .primary-btn:hover {
    opacity: 0.9;
  }
</style>
