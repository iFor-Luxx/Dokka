# dokka-menubar

**Hace:** barra superior estilo macOS de lado a lado (30 de alto): icono
Lucide de Dokka a 20, nombre de la app activa en negrita a la izquierda, fila
de iconos pulsables a la izquierda de la fecha en el orden guardado en
Preferencias (la fila de arriba de la lista queda a la izquierda; la hora no
se ordena y va siempre al final)
y la hora a la derecha. El clic en el icono abre debajo el mismo menú
de cristal del dock (`menu_panel` sin punta); el clic en la hora abre
debajo el calendario de la barra (`calendar_panel` con su píldora de
flechas), alineado a la derecha bajo el reloj; cada icono visible de la fila abre
debajo su panel real (`volume_panel`, `network_panel`, `language_panel`,
`music_panel`, `search_panel`, `control_center_panel`, `background_panel`), medido con su
`<x>_size` y colocado bajo su icono sin salirse del ancho (el buscador va
separado de la barra, centrado a lo ancho de la pantalla, con sombra suave
bajo la casilla y la tarjeta). `set_shown(MenubarPreferences)` elige qué iconos se ven
y en qué orden, y qué controles del centro se ocultan: los apagados no se dibujan
ni reciben clics, los demás se recolocan sin huecos según `order` y si estaba
abierto su panel se cierra (también si cambia el orden con un panel abierto);
si el centro está abierto y cambian sus ocultos, su tamaño se recalcula sin
cerrarlo; el filtrado y el orden
viven en `logic/tray.rs` (`is_shown` y `saved_order`: añadir un `TrayKind`
nuevo es una línea en cada uno) y la huella lo nota. La burbuja "Editar
controles" abre el modo editar (`control_editing` en el estado, con su hueco
en la huella; lógica en `logic/control_center_edit.rs`): cada pieza visible
lleva su "−" y no hace su acción; un "−" oculta ese control al momento y
avisa `HiddenControlsChanged` para guardarlo; cerrar, pulsar fuera o Esc
sale del modo (cerrando). Navega meses con las flechas, con hover, midiendo siempre
6 filas con los vecinos atenuados; la hora y la fila se resaltan suave (plano, como el icono) al
pasar o con su panel abierto. Solo uno de los nueve a la vez: abrir uno cierra
los demás. Guarda el nombre, la hora, el hoy, el código de idioma, si el menú,
el calendario o la fila están abiertos, el desfase de mes,
los resaltados y los datos de cada panel (volumen, brillo DDC, red, idiomas, segundo plano con iconos
por identidad, canción y
carátula, texto y resultados de búsqueda); decide el clic, el botón abajo y el arrastre
del deslizador y devuelve `MenubarAction`. El centro se cierra solo tras 6 s
sin uso con el cursor fuera de su panel y sin clics, arrastres ni teclas
(`logic/control_idle.rs`: cuenta un `Tick` de 1 s; con el cursor encima o
arrastrando no se cierra y cualquier uso reinicia; el shell lo cierra por el
camino normal). Al cerrarse (también por clic fuera o Esc) no desaparece
de golpe: se desvanece en 0,2 s encogiéndose un poco hacia su icono
(`logic/control_dismiss.rs` con el `Reveal` de `dokka-animation` y los
tiempos de `dokka-theme`); el foco vuelve al instante, dentro no acepta
clics mientras dura y la huella incluye el progreso. Da su `Scene` (con el menú, el
calendario o el panel real como `overlay`) y sus zonas activas (la franja y,
abierto, lo desplegado con su hueco; el buscador la casilla y la tarjeta con el
aire de la sombra para no recortarla, sin los huecos vacíos; en modo editar
el área cubre también lo que sobresalen los "−"; el clic en ese
aire no hace nada y no cierra; con la galería las zonas son la galería y el
centro). La galería (`logic/control_gallery.rs`) guarda categoría (0: todas),
búsqueda y los dos desplazamientos: elige categoría, filtra por nombre sin
tildes, añade el control quitado al momento con el mismo
`HiddenControlsChanged` (los del centro salen atenuados sin responder),
"Aceptar" cierra galería y modo editar, la rueda desplaza su columna o sus
secciones y el teclado de la barra escribe en su casilla. Expone `MenubarPainted` para repintar
solo si cambia (datos de los paneles y progreso del fundido incluidos) y `TrayKind` para saber qué
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
| `volume.rs` | `open_volume` (mide y coloca), `set_volume`, `set_audio_outputs` (recoloca el alto), gesto de botón abajo y arrastre del deslizador con `VolumeSetLevel` en vivo, `ChooseOutput` por índice |
| `network.rs` | `open_network`, `set_network`, clic en ajustes |
| `language.rs` | `open_language`, `set_languages`, elección de idioma |
| `music.rs` | `open_music`, `set_now_playing` (canción y carátula), botón anterior/pausa/siguiente; expone `artwork` para reutilizarla |
| `search.rs` | `open_search` (separado de la barra, centrado a lo ancho), texto, flechas, Enter y resaltado; `set_search_results` recoloca al cambiar las filas |
| `control_center.rs` | `open_control_center` con los datos combinados (incluida la carátula, los ocultos de `shown` y el brillo en sus props, con `set_brightness` que recoloca el alto y `set_shown` que lo recoloca si cambian los ocultos); los deslizadores delegan en volumen y brillo (`VolumeSetLevel` / `BrightnessSetLevel` en vivo, con recuerdo de cuál se arrastra), los botones en sus variantes (vista de tareas, proyectar, calculadora, recorte, reloj y tareas son variantes nuevas) y la tarjeta de red abre el panel de red |
| `control_idle.rs` | cierre solo del centro tras 6 s (`IDLE_CLOSE_SECONDS` del tema) con el cursor fuera y sin clics, arrastres ni teclas: cuenta un `Tick` de 1 s, decide en `control_should_auto_close` (el modo editar queda excluido ahí) y el shell cierra por el camino normal |
| `control_center.rs` | `open_control_center` con los datos combinados (incluida la carátula, los ocultos de `shown` y el brillo en sus props, con `set_brightness` que recoloca el alto y `set_shown` que lo recoloca si cambian los ocultos); los deslizadores delegan en volumen y brillo (`VolumeSetLevel` / `BrightnessSetLevel` en vivo, con recuerdo de cuál se arrastra), los botones en sus variantes (vista de tareas, proyectar, calculadora, recorte, reloj y tareas son variantes nuevas) y la tarjeta de red abre el panel de red; la burbuja y los "−" pasan `editing` a tamaño, panel y golpe; el centro aporta `preview_panels`/`preview_size` para dibujar la galería igual |
| `control_dismiss.rs` | fundido de salida del centro (progreso `Reveal`, avance por fotograma con cierre real al final, clics ignorados, cancelación al reabrir o cambiar de panel, galería al instante) |
| `control_center_edit.rs` | modo editar (`control_editing`, `enter_control_edit`, `hide_control` que oculta al momento y reacomoda; al entrar resetea la galería) |
| `control_gallery.rs` | galería del modo editar (`is_gallery_open`, `gallery_rect` centrada sin tapar el centro, `gallery_overlay` en el `submenu` sobre el centro, `gallery_click` con categoría/control/aceptar, `gallery_typed`/`gallery_erased` con el teclado de la barra, `gallery_scroll` con la rueda y `tick_gallery_cursor`; el control quitado se añade quitándolo de ocultos) |
| `background.rs` | `open_background`, `set_background_apps` (recoloca el alto si está abierto), clic izquierdo `TrayActivate`, clic derecho `TrayMenu` y hover |
| `open.rs` | cajas de la franja y `open_tray`, que reparte a cada `open_<panel>` |
| `tray.rs` | `TrayKind`, celdas de la fila y colocación (`tray_place`, `tray_place_centered` del buscador estilo Spotlight con el alto del monitor) |
| `click.rs` | `click` (suelta el arrastre), `pressed` (fija el nivel ya) y `pointer_moved` (lo sigue arrastrando) |
| `pointer.rs` | resaltados de la fila y de cada panel |

`view/tray.rs` llama a cada `<x>_panel` con el mismo `bounds` y dibuja los
iconos de la barra según los datos; `view/bar.rs` pone la galería en el
`submenu`, encima del centro. Los iconos de cada panel se crean una vez
en `Menubar::new`. Medidas nuevas van en `dokka-theme/src/measures/menubar_tray.rs`;
iconos nuevos, en un archivo propio de `dokka-icons` (no en `lucide.rs`, casi lleno).

| Panel | Datos que le pasa el shell | Cómo vuelve su acción |
|---|---|---|
| Volumen | `VolumeState` cada `Tick` y `Vec<AudioOutput>` al abrir y tras elegir | `VolumeSetLevel(f32)` / `VolumeToggleMute` / `ChooseOutput(usize)` / `OpenSoundSettings` |
| Red | `NetworkState` cada pocos `Tick` y al abrir | `OpenNetworkSettings` |
| Música | `NowPlayingInfo` y carátula (`artwork`) cada `Tick` | `MusicPrevious` / `MusicToggle` / `MusicNext` |
| Idioma | `Vec<LanguageEntry>` + código (leído antes de tomar el foco) | `ChooseLanguage(id)` / `OpenLanguageSettings` |
| Buscar | resultados filtrados (`set_search_results`) | `SearchOpen(index)` |
| Centro de control | combinados de arriba, con la carátula en sus props y brillo (`set_brightness`) al abrir | delega en las variantes de cada panel y en `TaskView` / `Project` / `Calculator` / `ScreenClip` / `Clock`, con `BrightnessSetLevel(f32)` para la Pantalla; en modo editar avisa `HiddenControlsChanged(HiddenControls)` |
| Segundo plano | `Vec<BackgroundApp>` (nombre limpio + icono) si cambia la revisión | `TrayActivate(id)` / `TrayMenu(id)` |
