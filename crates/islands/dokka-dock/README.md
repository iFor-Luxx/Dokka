# dokka-dock

**Hace:** ordena el dock (explorador, launchpad, calendario, reloj, apps, ventanas minimizadas y papelera),
calcula la lupa y el salto, y decide qué hace cada clic. Guarda el menú del clic derecho
(`MenuAction`, con «Añadir app…» y «Reiniciar Dokka» en la separadora) y da su `Overlay` y su zona activa.

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco. Todavía no tiene miniaturas ni arrastre.

**Pendiente:** mover `view/dock_style.rs` a `dokka-theme` cuando exista.
