<script lang="ts">
  import { TRANSLATIONS } from '../i18n';

  let { t = TRANSLATIONS.es }: { t?: typeof TRANSLATIONS.es } = $props();
</script>

<footer class="footer-bar">
  <div class="footer-hints">
    <span class="hint-key">{t.pasteAction}</span>
    <span class="dot">·</span>
    <span class="hint-key">{t.spaceDetail}</span>
    <span class="dot">·</span>
    <span class="hint-key">{t.copyAction}</span>
    <span class="dot">·</span>
    <span class="hint-key">{t.delAction}</span>
    <span class="dot">·</span>
    <span class="hint-key">{t.escExit}</span>
  </div>
</footer>

<style>
  .footer-bar {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 48px;
    padding: 0 16px;
    background-color: var(--footer-bg);
    border-top: 1px solid var(--border-color);
    border-bottom-left-radius: var(--window-radius);
    border-bottom-right-radius: var(--window-radius);
    user-select: none;
    flex-shrink: 0;
  }

  .footer-hints {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11.5px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .hint-key {
    transition: color 0.15s ease;
  }

  .hint-key:hover {
    color: var(--text-primary);
  }

  .dot {
    opacity: 0.6;
  }
</style>
