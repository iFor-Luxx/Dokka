# dokka-components

**Hace:** componentes reutilizables de interfaz: etiqueta flotante de cristal (`tooltip`),
fondo de paneles (`popup_panel`), calendario con su píldora de navegación (`calendar_panel`:
mes ya calculado, semanas, tamaño, píldora y paso entre meses compartidos por
el dock y la barra; la tarjeta nunca se encoge al navegar), píldora
segmentada (`nav_pill`: cápsula con dos chevrones Lucide y línea central, a partir
de su rectángulo, qué lados están disponibles y el hover; sin saber qué navega),
vistas previas con botones redondos de cerrar y fijar, menú del clic derecho
(`menu_panel`: tamaño, `Overlay` y fila bajo un punto, con iconos Lucide de marca elegida y del chevrón
y submenú sin punta; sin significado), cuadrícula de apps (`app_grid`: 7 columnas que empiezan arriba, iconos,
textos recortados con "…", mensaje de vacío centrado, resaltado y app bajo un punto), casilla de búsqueda (`search_field`:
panel nítido, lupa Lucide reutilizable siempre a la izquierda, texto escrito a la izquierda y
marcador "Buscar" también a la izquierda (tenue junto al cursor si hay foco), cursor vertical como
línea de ancho cero cuando está oculto para no reconstruir capas al parpadear, y si un punto cae dentro;
con variantes `_at` de sitio fijo para la barra de Preferencias y variantes `settings_*` con
búsqueda, foco y cursor para su filtrado, sin tocar el aspecto del Launchpad) y puntos de página
(`page_dots`: punto actual en blanco con aro claro algo mayor). Los `Overlay` salen con escala 1; la isla les pone la animación.
Semáforos de la ventana (`window_controls`: cada botón es un icono con su
círculo rojo, amarillo o verde —gris sin foco— y su símbolo Lucide oscuro al
pasar el cursor, y botón bajo un punto)
y filas de la barra de Preferencias (`settings_list`: fondo translúcido, casilla, cuadrados de color
con glifo blanco, nombres, selección azul, hover suave, fila bajo un punto y variantes
`filtered_*` que listan seguidas las coincidentes con "Sin resultados" si no hay ninguna),
panel de red (`network_panel`: tarjeta con nombre, tipo e internet y fila de ajustes
con `NetworkHit::Settings`; el icono blanco de la barra según el tipo sale de
`NetworkIcons::for_bar`) y panel de idioma (`language_panel`: filas de idiomas
con marca Lucide en el activo, resaltado y fila de ajustes con
`LanguageHit::Language(id)` o `Settings`; en la barra el idioma es su código en texto).

**No hace:** no conoce ninguna isla ni contiene lógica de negocio. No dibuja: describe qué dibujar.
