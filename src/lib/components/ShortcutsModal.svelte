<script lang="ts">
  import { ICONS } from '../icons';

  let { onClose }: { onClose: () => void } = $props();

  const SHORTCUTS = [
    { key: 'Super + V', desc: 'Mostrar u ocultar Zoma Clipboard desde cualquier app' },
    { key: '↑ / ↓', desc: 'Navegar entre los elementos del historial' },
    { key: '↵ (Enter)', desc: 'Pegar elemento seleccionado en la ventana activa' },
    { key: 'Espacio', desc: 'Abrir vista previa ampliada del elemento' },
    { key: 'Ctrl + C', desc: 'Copiar elemento al portapapeles sin salir' },
    { key: 'Ctrl + F', desc: 'Enfocar la barra de búsqueda' },
    { key: '1 - 9', desc: 'Acceso directo rápido a los primeros 9 clips' },
    { key: 'Supr / Del', desc: 'Eliminar el elemento seleccionado del historial' },
    { key: 'Esc', desc: 'Cerrar modales o minimizar la ventana' },
  ];
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={onClose}>
  <div class="modal-card" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <span class="modal-title">Atajos de Teclado</span>
      <button class="close-btn" onclick={onClose} title="Cerrar" type="button">
        <svg width="14" height="14" viewBox={ICONS.close.viewBox} fill="currentColor">
          <path d={ICONS.close.path} />
        </svg>
      </button>
    </div>

    <div class="modal-body">
      <div class="shortcuts-list">
        {#each SHORTCUTS as s}
          <div class="shortcut-item">
            <span class="shortcut-key mono">{s.key}</span>
            <span class="shortcut-desc">{s.desc}</span>
          </div>
        {/each}
      </div>
    </div>

    <div class="modal-footer">
      <button class="primary-btn" onclick={onClose} type="button">
        Cerrar
      </button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.45);
    backdrop-filter: blur(2px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
    padding: 20px;
  }

  .modal-card {
    width: 100%;
    max-width: 410px;
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
    padding: 14px 16px;
    max-height: 380px;
    overflow-y: auto;
  }

  .shortcuts-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .shortcut-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 6px 0;
    border-bottom: 1px solid var(--border-color);
  }

  .shortcut-item:last-child {
    border-bottom: none;
  }

  .shortcut-key {
    background-color: var(--code-box-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 3px 7px;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-primary);
    white-space: nowrap;
    box-shadow: 0 1px 1px rgba(0,0,0,0.05);
  }

  .shortcut-desc {
    font-size: 11.5px;
    color: var(--text-secondary);
    text-align: right;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    padding: 10px 16px;
    border-top: 1px solid var(--border-color);
    background-color: var(--footer-bg);
  }

  .primary-btn {
    padding: 6px 14px;
    background-color: var(--accent-color);
    color: #FFFFFF;
    border: none;
    border-radius: 5px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
  }

  .mono {
    font-family: 'JetBrains Mono', monospace;
  }
</style>
