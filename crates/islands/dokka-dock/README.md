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
Preferencias en el dock (`logic/settings_app.rs`): solo su miniatura minimizada a la
derecha del separador, con el engranaje blanco de ajustes como insignia y nombre
"Preferencias"; sin hueco de app a la izquierda, sin fijar, sin lanzar otra instancia
y con menú propio (ventana al frente + "Cerrar ventana" que solo la oculta).
La ventana reserva a cada lado sitio para el menú con su submenú a la derecha
(`menu::WIDTH + menu::SUBMENU_GAP`, o la lupa si pide más); la zona activa sigue
cubriendo solo la barra más la lupa para no tapar clics a otras apps.
El contenido vive en `logic/dock_menu.rs` y el paso entre madre e hijo en `logic/submenu.rs`;
recibe si la barra está oculta o en automático como dato. Los dos chevrones y las dos marcas
de opción elegida del menú (normal y resaltado) se crean una vez con los colores del tema
y se reutilizan. `set_preferences(DockPreferences)` aplica la escala de iconos
(`icon_scale` multiplica iconos, huecos, separaciones y alto de forma proporcional
desde `view/dock_style.rs`, sin medidas nuevas fuera del tema), apaga la lupa
(`magnification`) y cuánto amplía (`magnification_strength`: efectiva
1 + (MAX-1) * fuerza); con valores de fábrica el resultado es el de hoy.
Respeta `position` (`Bottom` idéntico al de hoy; `Left`/`Right` verticales,
centrados, con el mismo margen, que calculan en «a lo largo / hacia dentro»
(`logic/orientation.rs` + `logic/place_dock.rs`) y convierten a x/y en un solo
sitio: lupa, rebote hacia dentro, puntos en el lado del borde, separador
horizontal, etiquetas al lado hacia dentro, menús y submenús al lado
(`logic/menu_place.rs`, sin tocar sus filas), zonas activas y cursor.
Respeta `auto_hide` (`motion/hide.rs` + `logic/home_line.rs`: solo con una
app ocupando la pantalla principal se esconde con espera corta de 0,4 s y
deslizamiento suave; escondido dibuja solo la línea al borde —cápsula de
140 × 5 a 6 del borde— y vuelve al tocar su zona (180 × 28 pegada al borde),
sin sondeo en reposo; con menú, vista previa o Launchpad no se esconde).
En vertical la línea es vertical y encoge (`fit`) para caber.
Si algo no puede calcularse, abajo y visible.
Sin rebote (`animate_launch`) no se guardan saltos nuevos y se olvidan los
en curso para que el icono vuelva a su sitio sin pedir fotogramas
(el hueco sobre la barra se mantiene). Sin indicadores (`show_indicators`)
la lista de puntos queda vacía.

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco. Todavía no tiene miniaturas ni arrastre.

**Pendiente:** mover `view/dock_style.rs` a `dokka-theme` cuando exista.
