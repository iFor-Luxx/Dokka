# dokka-previews

**Hace:** decide qué panel flotante mostrar (calendario o vistas de ventanas), cuándo abrirlo y cerrarlo,
qué hace un clic en él (activar, cerrar o fijar una ventana) y con qué estilo entra y sale (crece desde el icono con un rebote leve, con el progreso `Reveal` que vive en `dokka-animation` y comparte con la barra;
si el cursor vuelve a mitad de salida, retoma desde donde está). El calendario usa el mes
(con los días del mes anterior), la píldora y el paso entre meses compartidos de `calendar_panel`,
siempre con 6 filas y los vecinos atenuados. Los chevrones de la píldora
del calendario se crean una vez con los colores del tema y se reutilizan al cambiar de mes.
El panel de una app espera 0,45 s desde cero (0,12 al cambiar entre paneles
o para el calendario); la vista grande espera 1 s sobre la mini (al instante
si ya está visible) y se quita al salir de ella; el resaltado y los botones
siguen al instante y el cierre al salir en 0,25 s. Recibe `DockPosition` en cada
petición (sin conocer al dock): abajo como hoy, en laterales al lado hacia
dentro (`view/place.rs`), con origen y rebote laterales. Expone
`large_preview_window()` (distinta de `hovered_window()`), `has_panel()` y
`still_inside()` para la red de seguridad del shell.

**No hace:** no conoce al dock ni habla con Windows: recibe lo que hay bajo el cursor.
Todavía no tiene controles de música.
