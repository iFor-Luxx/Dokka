# dokka-menubar

**Hace:** barra superior estilo macOS de lado a lado (30 de alto): icono
Lucide de Dokka a 20, nombre de la app activa en negrita a la izquierda, fila
de iconos pulsables a la izquierda de la fecha (de derecha a izquierda:
centro de control, buscar, idioma como texto de código, música, red y volumen)
y la hora a la derecha. El clic en el icono abre debajo el mismo menú
de cristal del dock (`menu_panel` sin punta); el clic en la hora abre
debajo el calendario de la barra (`calendar_panel` con su píldora de
flechas), alineado a la derecha bajo el reloj; cada icono de la fila abre
debajo su panel real (`volume_panel`, `network_panel`, `language_panel`,
`music_panel`, `search_panel`, `control_center_panel`), medido con su
`<x>_size` y colocado bajo su icono sin salirse del ancho (el buscador va
centrado en lo alto). Navega meses con las flechas, con hover, sin encogerse
al navegar; la hora y la fila se resaltan suave (plano, como el icono) al
pasar o con su panel abierto. Solo uno de los ocho a la vez: abrir uno cierra
los demás. Guarda el nombre, la hora, el hoy, el código de idioma, si el menú,
el calendario o la fila están abiertos, el desfase de mes, las semanas vistas,
los resaltados y los datos de cada panel (volumen, red, idiomas, canción y
carátula, texto y resultados de búsqueda); decide el clic, el botón abajo y el arrastre
del deslizador y devuelve `MenubarAction`. Da su `Scene` (con el menú, el
calendario o el panel real como `overlay`) y sus zonas activas (la franja y,
abierto, lo desplegado con su hueco). Expone `MenubarPainted` para repintar
solo si cambia (datos de los paneles incluidos) y `TrayKind` para saber qué
panel está abierto.

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco.
No mide texto: el reloj va centrado en su hueco derecho y el idioma en el suyo.

## Dónde vive cada panel real (un archivo por panel en `src/logic/`)

Cada panel usa su pieza de `dokka-components` (una carpeta por panel con
`mod.rs`, `props.rs`, `view.rs`...) con la forma común:

- `pub fn <x>_size(...) -> dokka_scene::Size`
- `pub fn <x>_panel(theme, iconos, bounds: Rect, ...) -> dokka_scene::Overlay`
- `pub fn <x>_hit(bounds, x, y, ...) -> Option<<X>Hit>`
- `pub struct <X>Icons { .. }` con `new(theme)` (dibujos creados una vez)

| Archivo | Enganche |
|---|---|
| `volume.rs` | `open_volume` (mide y coloca), `set_volume`, gesto de botón abajo y arrastre del deslizador con `VolumeSetLevel` en vivo |
| `network.rs` | `open_network`, `set_network`, clic en ajustes |
| `language.rs` | `open_language`, `set_languages`, elección de idioma |
| `music.rs` | `open_music`, `set_now_playing` (canción y carátula), botón anterior/pausa/siguiente; expone `artwork` para reutilizarla |
| `search.rs` | `open_search` (centrado), texto, flechas, Enter y resaltado; `set_search_results` recoloca al cambiar las filas |
| `control_center.rs` | `open_control_center` con los datos combinados; el deslizador delega en volumen, los botones en sus variantes (vista de tareas, proyectar, calculadora, recorte, reloj y tareas son variantes nuevas) y la tarjeta de red abre el panel de red |
| `open.rs` | cajas de la franja y `open_tray`, que reparte a cada `open_<panel>` |
| `tray.rs` | `TrayKind`, celdas de la fila y colocación (`tray_place`, `tray_place_centered`) |
| `click.rs` | `click` (suelta el arrastre), `pressed` (fija el nivel ya) y `pointer_moved` (lo sigue arrastrando) |
| `pointer.rs` | resaltados de la fila y de cada panel |

`view/tray.rs` llama a cada `<x>_panel` con el mismo `bounds` y dibuja los
iconos de la barra según los datos. Los iconos de cada panel se crean una vez
en `Menubar::new`. Medidas nuevas van en `dokka-theme/src/measures/menubar_tray.rs`;
iconos nuevos, en un archivo propio de `dokka-icons` (no en `lucide.rs`, casi lleno).

| Panel | Datos que le pasa el shell | Cómo vuelve su acción |
|---|---|---|
| Volumen | `VolumeState { level, is_muted }` cada `Tick` | `VolumeSetLevel(f32)` / `VolumeToggleMute` / `OpenSoundSettings` |
| Red | `NetworkState` cada pocos `Tick` y al abrir | `OpenNetworkSettings` |
| Música | `NowPlayingInfo` y carátula (`artwork`) cada `Tick` | `MusicPrevious` / `MusicToggle` / `MusicNext` |
| Idioma | `Vec<LanguageEntry>` + código (leído antes de tomar el foco) | `ChooseLanguage(id)` / `OpenLanguageSettings` |
| Buscar | resultados filtrados (`set_search_results`) | `SearchOpen(index)` |
| Centro de control | combinados de arriba | delega en las variantes de cada panel y en `TaskView` / `Project` / `Calculator` / `ScreenClip` / `Clock` |
