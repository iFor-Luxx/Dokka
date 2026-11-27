# dokka-menubar

**Hace:** barra superior estilo macOS de lado a lado (30 de alto): icono
Lucide de Dokka a 20, nombre de la app activa en negrita a la izquierda, fila
de iconos pulsables a la izquierda de la fecha (de derecha a izquierda:
centro de control, buscar, idioma como texto de código, música, red y volumen)
y la hora a la derecha. El clic en el icono abre debajo el mismo menú
de cristal del dock (`menu_panel` sin punta); el clic en la hora abre
debajo el mismo calendario del dock (`calendar_panel` con su píldora de
flechas), alineado a la derecha bajo el reloj; cada icono de la fila abre
debajo su panel provisional (tarjeta de cristal de 340 × 420 con su título),
alineado bajo su icono sin salirse del ancho. Navega meses con las flechas,
con hover, sin encogerse al navegar; la hora y la fila se resaltan suave
(plano, como el icono) al pasar o con su panel abierto. Solo uno de los ocho
a la vez: abrir uno cierra los demás. Guarda el nombre, la hora, el hoy, el
código de idioma, si el menú, el calendario o la fila están abiertos, el
desfase de mes, las semanas vistas y los resaltados; decide el clic y devuelve
`MenubarAction`. Da su `Scene` (con el menú, el calendario o el provisional
como `overlay`) y sus zonas activas (la franja y, abierto, lo desplegado con
su hueco). Expone `MenubarPainted` para repintar solo si cambia (mes, hover,
idioma y apertura incluidos) y `TrayKind` para saber qué panel está abierto.

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco.
No mide texto: el reloj va centrado en su hueco derecho y el idioma en el suyo.

## Dónde y cómo se enchufa cada panel real (un archivo por panel)

Los provisionales los hacen otros agentes como piezas sueltas en
`crates/ui/dokka-components` (una carpeta por panel: `mod.rs`, `props.rs`,
`view.rs`...), con la forma común:

- `pub fn <x>_size(props...) -> dokka_scene::Size`
- `pub fn <x>_panel(theme, iconos, bounds: Rect, props...) -> dokka_scene::Overlay`
- `pub fn <x>_hit(bounds, x, y, props...) -> Option<<X>Hit>`
- `pub struct <X>Icons { .. }` con `new(theme)` (dibujos creados una vez)

El enganche en esta isla vive en `src/logic/`, un archivo por panel
(`control_center.rs`, `buscar.rs`, `idioma.rs`, `musica.rs`, `red.rs`,
`volumen.rs`): cada uno expone `open_<panel>(&mut self, datos..., bar_width)`
que mide con `<x>_size`, coloca con `tray_panel_for` (el mismo centrado bajo
su icono con tope al borde que usa el provisional), guarda su estado abierto
y traduce el clic con `<x>_hit` a una variante nueva de `MenubarAction`.
`view/tray.rs::tray_overlay` llamará a su `<x>_panel` con el mismo `bounds`
en vez de al provisional, y `MenubarPainted` ya incluye qué panel está abierto
para repintar al cambiar sus datos. Medidas nuevas van en
`dokka-theme/src/measures/menubar_tray.rs`; iconos nuevos, en un archivo propio
de `dokka-icons` (no en `lucide.rs`, casi lleno).

| Panel | Datos que le pasa el shell | Cómo vuelve su acción |
|---|---|---|
| Volumen | `VolumeState { level, is_muted }` (cada `Tick`; el shell lo lee de la plataforma) | `MenubarAction` nueva (p. ej. fijar nivel o silenciar) → el shell la ejecuta en `run_menubar_action` |
| Red | `NetworkState { kind, name, has_internet }` | igual: variante nueva → el shell abre el panel de red o cambia de red |
| Música | `NowPlayingInfo { title, artist, is_playing }` | igual: variante nueva (reproducir/pausar/siguiente) → el shell llama a la plataforma |
| Idioma | `KeyboardLanguage { id, code, name, is_active }` (el shell ya pasa `code` con `set_language_code`) | igual: variante nueva (elegir idioma) → el shell cambia la distribución |
| Buscar | texto escrito (lo guarda esta isla como el calendario guarda su mes) | igual: variante nueva (abrir búsqueda) → el shell abre el lanzador |
| Centro de control | combinados de arriba (volumen + red + música) | igual: delega en las variantes de cada panel |
