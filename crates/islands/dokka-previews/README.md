# dokka-previews

**Hace:** decide qué panel flotante mostrar (calendario o vistas de ventanas), cuándo abrirlo y cerrarlo,
qué hace un clic en él (activar, cerrar o fijar una ventana) y con qué estilo entra y sale (crece desde el icono con un rebote leve;
si el cursor vuelve a mitad de salida, retoma desde donde está). Los chevrones de la píldora
del calendario se crean una vez con los colores del tema y se reutilizan al cambiar de mes.
El panel de una app espera 0,45 s desde cero (0,12 al cambiar entre paneles
o para el calendario); la vista grande espera 1 s sobre la mini (al instante
si ya está visible) y se quita al salir de ella; el resaltado y los botones
siguen al instante y el cierre al salir en 0,25 s. Expone
`large_preview_window()` (distinta de `hovered_window()`), `has_panel()` y
`still_inside()` para la red de seguridad del shell.

**No hace:** no conoce al dock ni habla con Windows: recibe lo que hay bajo el cursor.
Todavía no tiene controles de música.
