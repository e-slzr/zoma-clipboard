# Zoma Clipboard (v1.0.0)

[![elementary OS](https://img.shields.io/badge/elementary%20OS-Pantheon-3584E4?logo=elementary)](https://elementary.io)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.99+-DEA584?logo=rust)](https://www.rust-lang.org)
[![Svelte 5](https://img.shields.io/badge/Svelte-5.56-FF3E00?logo=svelte)](https://svelte.dev)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Gestor de portapapeles nativo, ultra-rápido y elegante para Linux (Wayland & X11), concebido y diseñado específicamente siguiendo la guía de diseño de **elementary OS 8 (Pantheon Desktop)**.

Construido sobre una arquitectura moderna con **Tauri v2**, **Rust** y **Svelte 5 (Runes)**. Basado en el prototipo interactivo diseñado en **FigCraft**:  
📁 `Prototypes/zoma-clipboard-prototype-v1.figcraft`

---

## 🎨 Características & Temas Visuales

- **Pantheon Claro (elementary OS Oficial):**
  Bordes redondeados de 12px, sutiles sombras Gala, cabecera gris neutra (`#F6F7F9`) y acento oficial elementary OS (`#3584E4`).
- **Pantheon Oscuro (Granite Palette):**
  Paleta oscura oficial con gris granito Pantheon (`#3A3A3A` fondo, `#313131` tarjetas, `#484848` bordes y azul acento `#3689E6`).
- **Modern Flat Claro & Oscuro:**
  Diseño minimalista con bordes nítidos para entornos contemporáneos.
- **Soporte Bilingüe (Español / Inglés):**
  Cambio instantáneo de idioma desde el menú de opciones (Español e Inglés incluidos de fábrica).
- **Diseño sin bordes ni icono en el Dock:**
  Comportamiento idéntico a una utilidad flotante del sistema: no satura la barra de tareas ni el dock Plank.

---

## ⚡ Capacidades del Portapapeles

- **Monitor en Tiempo Real:** Detección automática en segundo plano mediante `arboard` de cualquier copia en el sistema (Wayland & X11).
- **Pegado Directo en la Aplicación Activa:** Al presionar `Enter` o hacer doble clic en cualquier elemento, Zoma oculta la ventana e inyecta la pulsación de pegado (`Ctrl+V`) directamente en la aplicación que estabas usando (soporta `ydotool`, `wtype` y `xdotool`).
- **Reordenamiento Inteligente (MRU):** Al volver a copiar o pegar un clip existente, este se mueve inmediatamente a la primera posición como el más reciente, sin generar duplicados.
- **Categorización Inteligente:**
  - 🎨 **Colores:** Reconocimiento de códigos HEX (`#3584E4`), RGB y visualización con swatch interactivo.
  - 💻 **Código:** Detección de comandos de terminal (`ssh`, `docker`, `meson`, `git`), objetos JSON y sintaxis de desarrollo.
  - 🔗 **Enlaces:** URLs web con botón de apertura en el navegador predeterminado.
  - 🖼 **Imágenes:** Capturas de pantalla e imágenes con vista previa en miniatura, resolución y peso.
  - 📝 **Texto:** Fragmentos de texto plano con conteo de caracteres/bytes y tiempo transcurrido.
- **Persistencia en SQLite Local:** Base de datos ultraligera y segura (`~/.local/share/com.zoma.clipboard/zoma_clipboard.db`), 100% offline y sin telemetría.
- **Instancia Única (Socket Daemon):** Se ejecuta un único proceso en segundo plano coordinado a través de un socket Unix (`/run/user/1000/zoma-clipboard.sock`).

---

## ⌨️ Atajos de Teclado

| Atajo | Acción |
|---|---|
| **Super + V** | Mostrar / alternar la ventana flotante de Zoma Clipboard |
| **↑ / ↓** | Navegar entre los elementos del portapapeles |
| **↵ (Enter)** | Pegar el elemento seleccionado directamente en el input activo |
| **Espacio** | Abrir vista previa expandida y detallada |
| **Ctrl + C** | Copiar el elemento al portapapeles |
| **Ctrl + F** | Enfocar inmediatamente la barra de búsqueda |
| **1 - 9** | Selección rápida numérica de los primeros 9 clips |
| **Supr / Del** | Eliminar el clip seleccionado del historial |
| **Esc** | Minimizar / ocultar la ventana o cerrar modales |

---

## 🚀 Instalación y Ejecución

### 1. Requisitos Previos

En elementary OS / Ubuntu / Debian:
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev ydotool
```

### 2. Compilar el Proyecto

```bash
git clone https://github.com/tu-usuario/zoma-clipboard.git
cd zoma-clipboard

# Instalar dependencias de frontend
npm install

# Compilar binario nativo optimizado
npm run tauri build -- --no-bundle
```

El binario compilado se generará en:
`src-tauri/target/release/zoma-clipboard`

### 3. Configurar Atajo Global en elementary OS 8

1. Abre **Ajustes del Sistema** > **Teclado** > **Atajos**.
2. Ve a la sección **Personalizados** y presiona **+**.
3. En el comando introduce:
   ```bash
   /ruta/a/zoma-clipboard/start.sh --toggle
   ```
4. Asigna la combinación de teclas: **Super + V** (o el atajo de tu preferencia).

---

## 📁 Estructura del Código

```
zoma-clipboard/
├── src/                          # Frontend en Svelte 5 (Runes)
│   ├── lib/
│   │   ├── components/
│   │   │   ├── Header.svelte     # Barra superior arrastrable con menú de ajustes
│   │   │   ├── SearchBar.svelte  # Búsqueda instantánea con badge de escape
│   │   │   ├── CategoryToolbar.svelte # Filtro de categorías (Todo, Texto, Código, Enlaces, Imágenes, Favoritos)
│   │   │   ├── ClipCard.svelte   # Tarjetas dinámicas con preview
│   │   │   ├── FooterBar.svelte  # Barra de pie con atajos rápidos
│   │   │   ├── SettingsMenu.svelte # Menú desplegable con temas e idioma (ES / EN)
│   │   │   ├── PreviewModal.svelte # Vista previa ampliada (Espacio)
│   │   │   └── AboutModal.svelte # Ventana de información Acerca de
│   │   ├── i18n.ts               # Sistema de internacionalización bilingüe (Español / Inglés)
│   │   ├── icons.ts              # Vectores SVG optimizados desde FigCraft
│   │   └── types.ts              # Modelos TypeScript de Clips y Preferencias
│   └── routes/+page.svelte       # Vista principal y control de navegación
├── src-tauri/                    # Backend en Rust (Tauri v2)
│   ├── src/
│   │   ├── clipboard.rs          # Monitor de portapapeles y deduplicación en tiempo real
│   │   ├── db.rs                 # Capa de almacenamiento SQLite y migraciones
│   │   ├── models.rs             # Tipos y esquemas de datos serializables
│   │   ├── commands.rs           # Comandos IPC para el frontend
│   │   ├── main.rs               # Punto de entrada, daemon e instancia única (Unix Domain Socket)
│   │   └── lib.rs                # Inicialización de la ventana y configuración
│   ├── Cargo.toml                # Crates y dependencias de Rust
│   └── tauri.conf.json           # Configuración de ventana flotante sin decoraciones
├── start.sh                      # Script orquestador (detecta daemon o invoca toggle)
└── package.json                  # Dependencias de Vite y Svelte
```

---

## 📄 Licencia

Este proyecto está bajo la licencia [MIT](LICENSE).
