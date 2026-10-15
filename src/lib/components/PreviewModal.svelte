<script lang="ts">
  import { ICONS } from '../icons';
  import { TRANSLATIONS } from '../i18n';
  import type { Clip } from '../types';

  let {
    clip,
    t = TRANSLATIONS.es,
    onClose,
    onCopy,
    onPaste,
  }: {
    clip: Clip | null;
    t?: typeof TRANSLATIONS.es;
    onClose: () => void;
    onCopy: () => void;
    onPaste: () => void;
  } = $props();
</script>

{#if clip}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={onClose}>
    <div class="modal-card" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div class="title-wrap">
          <span class="modal-type-tag">{clip.content_type.toUpperCase()}</span>
          <span class="modal-title truncate">{clip.title || t.previewDetail}</span>
        </div>
        <button class="close-btn" onclick={onClose} title="Cerrar (Esc)" type="button">
          <svg width="14" height="14" viewBox={ICONS.close.viewBox} fill="currentColor">
            <path d={ICONS.close.path} />
          </svg>
        </button>
      </div>

      <div class="modal-body">
        {#if clip.content_type === 'color'}
          <div class="color-preview-box">
            <div class="big-swatch" style="background-color: {clip.color_hex};"></div>
            <div class="color-details">
              <span class="color-hex mono">{clip.color_hex}</span>
              <span class="color-sub">{clip.metadata || t.colorHex}</span>
            </div>
          </div>

        {:else if clip.content_type === 'image'}
          <div class="image-preview-box">
            {#if clip.thumbnail_data}
              <img src={clip.thumbnail_data} alt="Vista previa" class="full-img" />
            {/if}
            <div class="img-meta">
              <span>{clip.image_width || 1920} × {clip.image_height || 1080} {t.pixels}</span>
              <span>·</span>
              <span>{Math.round((clip.image_size_bytes || 400000) / 1024)} KB</span>
            </div>
          </div>

        {:else}
          <div class="code-preview-scroll mono">
            <pre>{clip.text_content}</pre>
          </div>
        {/if}

        {#if clip.metadata}
          <div class="modal-meta-bar">
            <span>{clip.metadata}</span>
          </div>
        {/if}
      </div>

      <div class="modal-footer">
        <button class="secondary-btn" onclick={onCopy} type="button">
          <svg width="13" height="13" viewBox={ICONS.copy.viewBox} fill="currentColor">
            <path d={ICONS.copy.path} />
          </svg>
          <span>{t.copyToClipboard}</span>
        </button>
        <button class="primary-btn" onclick={onPaste} type="button">
          <span>{t.pasteAndExit}</span>
        </button>
      </div>
    </div>
  </div>
{/if}

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
    max-width: 390px;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 10px;
    box-shadow: 0 16px 32px rgba(0, 0, 0, 0.25);
    display: flex;
    flex-direction: column;
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

  .title-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .modal-type-tag {
    font-size: 10px;
    font-weight: 700;
    background-color: var(--tag-bg);
    color: var(--text-secondary);
    padding: 2px 6px;
    border-radius: 3px;
  }

  .modal-title {
    font-size: 12.5px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
  }

  .close-btn:hover {
    background-color: var(--hover-bg);
    color: var(--text-primary);
  }

  .modal-body {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .color-preview-box {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 12px;
    background-color: var(--code-box-bg);
    border-radius: 8px;
  }

  .big-swatch {
    width: 56px;
    height: 56px;
    border-radius: 8px;
    box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.2);
  }

  .color-details {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .color-hex {
    font-size: 18px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .color-sub {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .image-preview-box {
    display: flex;
    flex-direction: column;
    gap: 8px;
    align-items: center;
  }

  .full-img {
    max-width: 100%;
    max-height: 220px;
    border-radius: 6px;
    border: 1px solid var(--border-color);
    object-fit: contain;
  }

  .img-meta {
    font-size: 11px;
    color: var(--text-secondary);
    display: flex;
    gap: 6px;
  }

  .code-preview-scroll {
    max-height: 260px;
    overflow-y: auto;
    padding: 12px;
    background-color: var(--code-box-bg);
    border-radius: 6px;
    font-size: 12px;
    line-height: 1.45;
    color: var(--text-primary);
    white-space: pre-wrap;
    word-break: break-all;
  }

  .modal-meta-bar {
    font-size: 11px;
    color: var(--text-secondary);
  }

  .modal-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    padding: 12px 16px;
    border-top: 1px solid var(--border-color);
    background-color: var(--footer-bg);
  }

  .primary-btn,
  .secondary-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 7px 12px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.15s ease;
  }

  .primary-btn {
    background-color: var(--accent-color);
    color: #FFFFFF;
  }

  .primary-btn:hover {
    filter: brightness(1.1);
  }

  .secondary-btn {
    background-color: transparent;
    border-color: var(--border-color);
    color: var(--text-primary);
  }

  .secondary-btn:hover {
    background-color: var(--hover-bg);
  }

  .truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .mono {
    font-family: 'JetBrains Mono', monospace;
  }
</style>
