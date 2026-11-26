# dokka-dock

**Hace:** ordena el dock (explorador, launchpad, calendario, reloj, apps, ventanas minimizadas y papelera),
calcula la lupa y el salto, y decide qué hace cada clic. Guarda el menú del clic derecho
(`MenuAction`, con el menú completo en la separadora y sus submenús estilo Tahoe:
«Elementos del dock», «Barra de tareas» y «Sistema») y da sus `Overlay` y sus zonas activas.
El contenido vive en `logic/dock_menu.rs` y el paso entre madre e hijo en `logic/submenu.rs`;
recibe si la barra está oculta o en automático como dato.

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco. Todavía no tiene miniaturas ni arrastre.

**Pendiente:** mover `view/dock_style.rs` a `dokka-theme` cuando exista.
