# Zoma Clipboard

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/badge/Release-v0.1.0-green.svg)](https://github.com/e-slzr/zoma-clipboard/releases)
[![Platform](https://img.shields.io/badge/Platform-Linux%20(Wayland%20%7C%20X11)-lightgrey.svg)](https://github.com/e-slzr/zoma-clipboard)
[![elementary OS](https://img.shields.io/badge/elementary%20OS-Pantheon-3584E4.svg?logo=elementary)](https://elementary.io)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-FFC131.svg?logo=tauri)](https://tauri.app)

Gestor de portapapeles nativo, ultra-rápido y elegante para Linux (Wayland y X11), concebido y diseñado específicamente siguiendo las guías de diseño de **elementary OS 8 (Pantheon Desktop)**.

<p align="center">
  <img src="app-icon.png" width="128" height="128" alt="Zoma Clipboard Icon" />
</p>

Construido sobre una arquitectura moderna con **Tauri v2**, **Rust** y **Svelte 5 (Runes)**. Basado en el prototipo interactivo diseñado en **FigCraft**:  
`Prototypes/zoma-clipboard-prototype-v1.figcraft`

---

## Características destacadas

- **Diseño flotante sin marco:** Ventana flotante sin barra de título ni bordes extraños, con sombra suave natural y esquinas redondeadas de 12px acordes con la estética Gala / Pantheon.
- **Sin presencia en el Dock:** Se comporta como una verdadera utilidad flotante del sistema sin mostrarse en el Dock Plank ni en la barra de tareas.
- **Monitor en tiempo real:** Captura automática en segundo plano de cualquier contenido copiado en el sistema (texto, enlaces, código, colores e imágenes) compatible con Wayland y X11.
- **Pegado directo en la aplicación activa:** Al seleccionar un clip y presionar `Enter`, la ventana se oculta al instante e inyecta la pulsación de pegado (`Ctrl+V`) directamente en el campo de texto o aplicación activa mediante `ydotool`, `wtype` o `xdotool`.
- **Reordenamiento inteligente (MRU):** Al volver a copiar o seleccionar un clip ya existente en el historial, este asciende automáticamente al primer lugar como el más reciente, evitando duplicados innecesarios.
- **Categorización inteligente:**
  - **Colores:** Reconocimiento de valores HEX y RGB con previsualización de muestra de color interactiva.
  - **Código:** Detección de comandos de terminal (`ssh`, `docker`, `meson`, `git`), snippets JSON y funciones de programación.
  - **Enlaces:** URLs web con botón de acceso directo en el navegador predeterminado.
  - **Imágenes:** Previsualización en miniatura, dimensiones en píxeles y peso en disco.
  - **Texto:** Contenido general con conteo de caracteres y marca de tiempo relativa.
- **Soporte bilingüe en caliente:** Selección de idioma en tiempo real (**Español** e **Inglés**) desde el menú de opciones con persistencia automática.
- **Persistencia local en SQLite:** Base de datos ligera y privada ubicada en `~/.local/share/com.zoma.clipboard/zoma_clipboard.db`, 100% offline y sin telemetría.
- **Instancia única coordinada:** Funcionamiento basado en un daemon con socket Unix local (`/run/user/1000/zoma-clipboard.sock`), asegurando un único proceso en memoria y apertura instantánea.

---

## Temas visuales

- **Pantheon Claro (elementary OS):** Fondo neutro `#F6F7F9`, tarjetas blancas con relieve sutil y color de acento oficial `#3584E4`.
- **Pantheon Oscuro (Granite Palette):** Paleta gris granito oficial de elementary OS (`#3A3A3A` fondo, `#313131` tarjetas, `#484848` separadores y `#3689E6` acento).
- **Modern Flat Claro:** Estilo minimalista con bordes rectos y contraste refinado.
- **Modern Flat Oscuro:** Paleta profunda `#0B0F19` orientada a entornos oscuros contemporáneos.

---

## Atajos de teclado

| Atajo | Acción |
|---|---|
| `Super + V` | Mostrar u ocultar la ventana de Zoma Clipboard |
| `↑` / `↓` | Navegar entre los elementos del historial |
| `Enter` | Pegar el elemento seleccionado directamente en la aplicación activa |
| `Espacio` | Abrir vista previa expandida y detallada |
| `Ctrl + C` | Copiar el elemento al portapapeles |
| `Ctrl + F` | Enfocar la barra de búsqueda instantánea |
| `1` - `9` | Acceso rápido numérico a los primeros 9 clips |
| `Supr` / `Del` | Eliminar el elemento seleccionado del historial |
| `Esc` | Minimizar la ventana o cerrar menús y diálogos |

---

## Configuración en elementary OS 8

Para asignar el atajo global del sistema y abrir Zoma Clipboard con **Super + V**:

1. Abre **Ajustes del Sistema** > **Teclado** > pestaña **Atajos**.
2. En la lista lateral izquierda, selecciona **Personalizados**.
3. Haz clic en el botón **+** en la parte inferior.
4. En el campo de comando, escribe:
   ```bash
   /ruta/a/zoma-clipboard/start.sh --toggle
   ```
5. Pulsa sobre el nuevo atajo añadido y presiona la combinación de teclas: `Super + V`.

---

## Desarrollo y Compilación

### Requisitos previos

En elementary OS, Ubuntu o Debian:

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev ydotool
```

Asegúrate de contar con:
- [Node.js](https://nodejs.org/) (versión 20 o superior)
- [Rust](https://www.rust-lang.org/) y Cargo (1.80 o superior)

### Pasos

```bash
# 1. Clonar el repositorio
git clone https://github.com/e-slzr/zoma-clipboard.git
cd zoma-clipboard

# 2. Instalar dependencias del frontend
npm install

# 3. Ejecutar en modo desarrollo con recarga en vivo
npm run tauri dev

# 4. Compilar binario de producción optimizado
npm run tauri build -- --no-bundle
```

El binario ejecutable final se genera en:
`src-tauri/target/release/zoma-clipboard`

---

## Estructura del proyecto

```
zoma-clipboard/
├── src/                          # Interfaz de usuario en Svelte 5 (Runes)
│   ├── lib/
│   │   ├── components/
│   │   │   ├── Header.svelte     # Barra superior con botón de menú
│   │   │   ├── SearchBar.svelte  # Búsqueda interactiva con badge Esc
│   │   │   ├── CategoryToolbar.svelte # Filtro de categorías
│   │   │   ├── ClipCard.svelte   # Renderizado de tarjetas por tipo de contenido
│   │   │   ├── FooterBar.svelte  # Barra de atajos de pie de ventana
│   │   │   ├── SettingsMenu.svelte # Menú desplegable de temas e idioma
│   │   │   ├── PreviewModal.svelte # Modal de vista detallada (Espacio)
│   │   │   └── AboutModal.svelte # Diálogo Acerca de
│   │   ├── i18n.ts               # Diccionarios bilingües (Español / Inglés)
│   │   ├── icons.ts              # Vectores SVG optimizados desde FigCraft
│   │   └── types.ts              # Definiciones TypeScript
│   └── routes/+page.svelte       # Controlador principal de vista y atajos
├── src-tauri/                    # Backend nativo en Rust (Tauri v2)
│   ├── src/
│   │   ├── clipboard.rs          # Monitor de portapapeles y deduplicación
│   │   ├── db.rs                 # Capa de almacenamiento SQLite
│   │   ├── models.rs             # Modelos de datos compartidos
│   │   ├── commands.rs           # Comandos IPC para el frontend
│   │   ├── main.rs               # Punto de entrada, daemon y socket Unix
│   │   └── lib.rs                # Inicialización de ventana y eventos
│   ├── Cargo.toml                # Dependencias de Rust
│   └── tauri.conf.json           # Configuración de ventana flotante
├── Prototypes/                   # Prototipos interactivos de FigCraft (.figcraft)
├── start.sh                      # Script orquestador para arranque y toggle
└── package.json                  # Dependencias de frontend
```

---

## Autor

**Eliu Salazar**  
- GitHub: [@e-slzr](https://github.com/e-slzr)  
- Contacto: [eliu.asalazar@gmail.com](mailto:eliu.asalazar@gmail.com)

---

## Licencia

Este proyecto está bajo la Licencia **MIT**. Consulta el archivo [LICENSE](LICENSE) para más información.
