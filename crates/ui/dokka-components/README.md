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
`filtered_*` que listan seguidas las coincidentes con "Sin resultados" si no hay ninguna)
y tarjeta de música (`music_panel`: tamaño fijo, título y artista recortados con "…",
barra plana con anterior, pausa/reproducir y siguiente con los glifos de música
existentes, "Nada en reproducción" sin canción y golpe por tercios; con y sin
canción entrega los mismos textos, iconos y planos —ocultos con tamaño cero—),
panel de volumen (`volume_panel`: título, deslizador con altavoz según nivel,
botón de silencio, separador y fila de ajustes), panel de red (`network_panel`: menú Tahoe con título, cápsula de estado
con círculo, nombre, tipo e internet y ajustes en su zona propia), lista de
idiomas (`language_panel`: menú Tahoe con título, cápsulas con círculo y
"Activa", resaltado suave y ajustes aparte, sin marca),
centro de control (`control_center_panel`: tarjetas de red, volumen con
deslizador y música más botones redondos, sin resaltar) y buscador
(`search_panel`: casilla grande con lupa y texto, hasta siete resultados con
icono y resaltado, "Sin resultados" y `filter_search_order` con tildes
ignoradas; con `search_field` para la casilla).

**No hace:** no conoce ninguna isla ni contiene lógica de negocio. No dibuja: describe qué dibujar.
