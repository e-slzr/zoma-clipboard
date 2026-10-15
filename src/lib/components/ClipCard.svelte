<script lang="ts">
  import { ICONS } from '../icons';
  import { TRANSLATIONS } from '../i18n';
  import type { Clip } from '../types';

  let {
    clip,
    index,
    isSelected = false,
    isCompact = false,
    t = TRANSLATIONS.es,
    onSelect,
    onPaste,
    onCopy,
    onDelete,
    onTogglePin,
  }: {
    clip: Clip;
    index?: number;
    isSelected?: boolean;
    isCompact?: boolean;
    t?: typeof TRANSLATIONS.es;
    onSelect?: () => void;
    onPaste?: () => void;
    onCopy?: () => void;
    onDelete?: () => void;
    onTogglePin?: () => void;
  } = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="clip-card"
  class:selected={isSelected}
  class:compact={isCompact}
  onclick={() => {
    if (isSelected) {
      onPaste?.();
    } else {
      onSelect?.();
    }
  }}
  ondblclick={onPaste}
>
  <!-- Content based on type and compact state -->
  {#if isCompact && clip.content_type === 'color'}
    <!-- Compact Color Swatch -->
    <div class="compact-row">
      <div class="color-swatch" style="background-color: {clip.color_hex};"></div>
      <span class="val-text mono">{clip.color_hex}</span>
      {#if clip.metadata}
        <span class="desc-text truncate">{clip.metadata}</span>
      {/if}
      <button
        class="pin-btn active"
        onclick={(e) => {
          e.stopPropagation();
          onTogglePin?.();
        }}
        title={t.unpinAction}
        type="button"
      >
        <svg width="14" height="13" viewBox={ICONS.heart.viewBox} fill="currentColor">
          <path d={ICONS.heart.path} />
        </svg>
      </button>
    </div>

  {:else if isCompact}
    <!-- Compact Pinned Item (Code or Text) -->
    <div class="compact-row">
      <span class="tag-badge">SSH</span>
      <span class="code-line mono truncate">{clip.text_content}</span>
      <button
        class="pin-btn active"
        onclick={(e) => {
          e.stopPropagation();
          onTogglePin?.();
        }}
        title={t.unpinAction}
        type="button"
      >
        <svg width="14" height="13" viewBox={ICONS.heart.viewBox} fill="currentColor">
          <path d={ICONS.heart.path} />
        </svg>
      </button>
    </div>

  {:else if clip.content_type === 'code'}
    <!-- Code Row -->
    <div class="card-header">
      <span class="meta-text">{clip.metadata || 'Código · Hace un momento'}</span>
      <div class="card-actions">
        {#if isSelected}
          <span class="paste-hint" onclick={(e) => { e.stopPropagation(); onPaste?.(); }}>{t.pasteAction}</span>
        {:else if index !== undefined}
          <span class="key-badge">{index}</span>
        {/if}
        <button
          class="action-icon-btn pin"
          class:pinned={clip.is_pinned}
          onclick={(e) => {
            e.stopPropagation();
            onTogglePin?.();
          }}
          title={clip.is_pinned ? t.unpinAction : t.pinAction}
          type="button"
        >
          <svg width="12" height="11" viewBox={ICONS.heart.viewBox} fill="currentColor">
            <path d={ICONS.heart.path} />
          </svg>
        </button>
        <button
          class="action-icon-btn delete"
          onclick={(e) => {
            e.stopPropagation();
            onDelete?.();
          }}
          title={t.deleteAction}
          type="button"
        >
          <svg width="12" height="12" viewBox={ICONS.trash.viewBox} fill="currentColor">
            <path d={ICONS.trash.path} />
          </svg>
        </button>
      </div>
    </div>
    <div class="code-inner-box">
      <div class="code-content mono truncate">{clip.text_content}</div>
      {#if clip.title}
        <div class="code-subtext">{clip.title}</div>
      {/if}
    </div>

  {:else if clip.content_type === 'image'}
    <!-- Image Row -->
    <div class="image-row">
      <div class="thumbnail-wrapper">
        {#if clip.thumbnail_data}
          <img src={clip.thumbnail_data} alt="Miniatura" class="thumb-img" />
        {:else}
          <div class="thumb-placeholder">
            <svg width="24" height="24" viewBox={ICONS.photo.viewBox} fill="currentColor">
              <path d={ICONS.photo.path} />
            </svg>
          </div>
        {/if}
      </div>
      <div class="image-info">
        <div class="info-top">
          <span class="item-title truncate">{clip.title || 'Captura de pantalla.png'}</span>
          <div class="card-actions">
            {#if isSelected}
              <span class="paste-hint" onclick={(e) => { e.stopPropagation(); onPaste?.(); }}>{t.pasteAction}</span>
            {:else if index !== undefined}
              <span class="key-badge">{index}</span>
            {/if}
            <button
              class="action-icon-btn pin"
              class:pinned={clip.is_pinned}
              onclick={(e) => {
                e.stopPropagation();
                onTogglePin?.();
              }}
              title={clip.is_pinned ? t.unpinAction : t.pinAction}
              type="button"
            >
              <svg width="12" height="11" viewBox={ICONS.heart.viewBox} fill="currentColor">
                <path d={ICONS.heart.path} />
              </svg>
            </button>
            <button
              class="action-icon-btn delete"
              onclick={(e) => {
                e.stopPropagation();
                onDelete?.();
              }}
              title={t.deleteAction}
              type="button"
            >
              <svg width="12" height="12" viewBox={ICONS.trash.viewBox} fill="currentColor">
                <path d={ICONS.trash.path} />
              </svg>
            </button>
          </div>
        </div>
        <div class="item-specs">{clip.image_width || 1920} × {clip.image_height || 1080} px  ·  PNG  ·  {Math.round((clip.image_size_bytes || 400000) / 1024)} KB</div>
        <div class="item-source">{clip.metadata || 'Herramienta de Capturas  ·  Hace 8m'}</div>
      </div>
    </div>

  {:else if clip.content_type === 'url'}
    <!-- URL Row -->
    <div class="card-header">
      <span class="meta-text">{clip.metadata || 'Enlace · Hace un momento'}</span>
      <div class="card-actions">
        {#if isSelected}
          <span class="paste-hint" onclick={(e) => { e.stopPropagation(); onPaste?.(); }}>{t.pasteAction}</span>
        {:else if index !== undefined}
          <span class="key-badge">{index}</span>
        {/if}
        <button
          class="action-icon-btn pin"
          class:pinned={clip.is_pinned}
          onclick={(e) => {
            e.stopPropagation();
            onTogglePin?.();
          }}
          title={clip.is_pinned ? t.unpinAction : t.pinAction}
          type="button"
        >
          <svg width="12" height="11" viewBox={ICONS.heart.viewBox} fill="currentColor">
            <path d={ICONS.heart.path} />
          </svg>
        </button>
        <button
          class="action-icon-btn delete"
          onclick={(e) => {
            e.stopPropagation();
            onDelete?.();
          }}
          title={t.deleteAction}
          type="button"
        >
          <svg width="12" height="12" viewBox={ICONS.trash.viewBox} fill="currentColor">
            <path d={ICONS.trash.path} />
          </svg>
        </button>
      </div>
    </div>
    <div class="url-body">
      <div class="item-title truncate">{clip.title || clip.text_content}</div>
      <a
        href={clip.text_content || '#'}
        target="_blank"
        rel="noopener noreferrer"
        class="url-link truncate"
        onclick={(e) => e.stopPropagation()}
      >
        {clip.text_content}
      </a>
    </div>

  {:else}
    <!-- Plain Text or Terminal Row -->
    <div class="card-header">
      <span class="meta-text">{clip.metadata || 'Texto · Hace un momento'}</span>
      <div class="card-actions">
        {#if isSelected}
          <span class="paste-hint" onclick={(e) => { e.stopPropagation(); onPaste?.(); }}>{t.pasteAction}</span>
        {:else if index !== undefined}
          <span class="key-badge">{index}</span>
        {/if}
        <button
          class="action-icon-btn pin"
          class:pinned={clip.is_pinned}
          onclick={(e) => {
            e.stopPropagation();
            onTogglePin?.();
          }}
          title={clip.is_pinned ? t.unpinAction : t.pinAction}
          type="button"
        >
          <svg width="12" height="11" viewBox={ICONS.heart.viewBox} fill="currentColor">
            <path d={ICONS.heart.path} />
          </svg>
        </button>
        <button
          class="action-icon-btn delete"
          onclick={(e) => {
            e.stopPropagation();
            onDelete?.();
          }}
          title={t.deleteAction}
          type="button"
        >
          <svg width="12" height="12" viewBox={ICONS.trash.viewBox} fill="currentColor">
            <path d={ICONS.trash.path} />
          </svg>
        </button>
      </div>
    </div>
    <div class="text-body">
      <div class="main-text truncate mono">{clip.text_content}</div>
      {#if clip.title}
        <div class="item-subtext truncate">{clip.title}</div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .clip-card {
    position: relative;
    padding: 10px 12px;
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: var(--control-radius);
    cursor: pointer;
    transition: background-color 0.15s ease, border-color 0.15s ease, box-shadow 0.15s ease;
    user-select: none;
  }

  .clip-card:hover {
    background-color: var(--card-hover-bg);
    border-color: var(--border-hover-color);
  }

  .clip-card.selected {
    border: 2px solid var(--accent-color);
    background-color: var(--card-selected-bg);
    box-shadow: 0 0 0 1px var(--accent-color);
  }

  .clip-card.compact {
    padding: 6px 12px;
    height: 38px;
    display: flex;
    align-items: center;
  }

  .compact-row {
    display: flex;
    align-items: center;
    width: 100%;
    gap: 8px;
  }

  .color-swatch {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    flex-shrink: 0;
    box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.15);
  }

  .val-text {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .desc-text {
    font-size: 12px;
    color: var(--text-secondary);
    flex: 1;
  }

  .tag-badge {
    font-size: 11px;
    font-weight: 700;
    color: var(--text-secondary);
    background: var(--tag-bg);
    padding: 1px 6px;
    border-radius: 3px;
  }

  .code-line {
    font-size: 12px;
    color: var(--text-primary);
    flex: 1;
  }

  .pin-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    margin-left: auto;
    border-radius: 3px;
    transition: color 0.15s ease;
  }

  .pin-btn:hover {
    color: var(--accent-color);
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 6px;
  }

  .meta-text {
    font-size: 11.5px;
    color: var(--text-secondary);
  }

  .card-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .paste-hint {
    font-size: 11.5px;
    font-weight: 700;
    color: var(--accent-color);
  }

  .key-badge {
    font-size: 11.5px;
    font-weight: 600;
    color: var(--text-secondary);
    min-width: 14px;
    text-align: right;
  }

  .action-icon-btn {
    display: none;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    border-radius: 3px;
    cursor: pointer;
  }

  .clip-card:hover .action-icon-btn {
    display: flex;
  }

  .action-icon-btn:hover {
    background: var(--hover-bg);
    color: var(--text-primary);
  }

  .action-icon-btn.pin.pinned {
    display: flex;
    color: var(--accent-color);
  }

  .code-inner-box {
    background-color: var(--code-box-bg);
    border-radius: 5px;
    padding: 7px 10px;
    margin-top: 4px;
  }

  .code-content {
    font-size: 12px;
    color: var(--text-primary);
    line-height: 1.35;
  }

  .code-subtext {
    font-size: 11px;
    color: var(--text-secondary);
    margin-top: 4px;
  }

  .image-row {
    display: flex;
    gap: 12px;
    align-items: center;
  }

  .thumbnail-wrapper {
    width: 96px;
    height: 72px;
    border-radius: 6px;
    overflow: hidden;
    background-color: var(--code-box-bg);
    border: 1px solid var(--border-color);
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .thumb-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .thumb-placeholder {
    color: var(--text-placeholder);
  }

  .image-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .info-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .item-title {
    font-size: 12.5px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .item-specs {
    font-size: 11.5px;
    color: var(--text-secondary);
  }

  .item-source {
    font-size: 11.5px;
    color: var(--text-secondary);
  }

  .url-body {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .url-link {
    font-size: 11.5px;
    color: var(--accent-color);
    text-decoration: none;
  }

  .url-link:hover {
    text-decoration: underline;
  }

  .text-body {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .main-text {
    font-size: 12.5px;
    color: var(--text-primary);
  }

  .item-subtext {
    font-size: 11px;
    color: var(--text-secondary);
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
