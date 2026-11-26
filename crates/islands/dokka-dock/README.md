# dokka-dock

**Hace:** ordena el dock (explorador, launchpad, calendario, reloj, apps, ventanas minimizadas y papelera),
calcula la lupa y el salto, y decide qué hace cada clic. Guarda el menú del clic derecho
(`MenuAction`, con el menú completo en la separadora y sus submenús estilo Tahoe:
«Elementos del dock», «Barra de tareas» y «Sistema») y da sus `Overlay` y sus zonas activas.
Las apps tienen su propio menú (`logic/app_menu.rs`: hasta 6 ventanas con título acortado,
«Abrir»/«Abrir otra vez», «Ejecutar como administrador», submenú «Opciones»,
fijar/quitar, y si está abierta «Cerrar ventana»/«Salir»); los iconos del sistema
tienen el suyo (`logic/system_menus.rs`: explorador con carpetas, papelera con vaciado
solo con contenido, Launchpad con abrir y añadir). Reloj y calendario no tienen menú.
«Preferencias» abre la ventana de ajustes (`MenuAction::OpenSettings`, en `settings.rs`).
La ventana reserva a cada lado sitio para el menú con su submenú a la derecha
(`menu::WIDTH + menu::SUBMENU_GAP`, o la lupa si pide más); la zona activa sigue
cubriendo solo la barra más la lupa para no tapar clics a otras apps.
El contenido vive en `logic/dock_menu.rs` y el paso entre madre e hijo en `logic/submenu.rs`;
recibe si la barra está oculta o en automático como dato. Los dos chevrones y las dos marcas
de opción elegida del menú (normal y resaltado) se crean una vez con los colores del tema
y se reutilizan.

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco. Todavía no tiene miniaturas ni arrastre.

**Pendiente:** mover `view/dock_style.rs` a `dokka-theme` cuando exista.
