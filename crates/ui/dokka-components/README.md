# dokka-components

**Hace:** componentes reutilizables de interfaz: etiqueta flotante de cristal (`tooltip`),
fondo de paneles (`popup_panel`), calendario con su píldora de navegación (`calendar_panel`:
mes ya calculado, semanas, tamaño, píldora y paso entre meses compartidos por
el dock y en la barra, con el mismo cristal claro;
la tarjeta nunca se encoge al navegar), píldora
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
y cristal único de música Tahoe (`music_panel`: carátula cuadrada con la foto
real en píxeles cuando hay (marcador si no), título y artista recortados
con "…", controles grandes de anterior, pausa/reproducir y siguiente con los glifos Lucide
en blanco, "Nada en reproducción" con controles atenuados sin responder al golpe; con y sin
canción entrega los mismos textos e iconos),
panel de volumen (`volume_panel`: título con porcentaje a la derecha en texto
secundario, barra fina con tirador redondo entre dos altavoces —atenuados
con silencio—, botón de silencio, separador y fila de ajustes), panel de red (`network_panel`: menú Tahoe con título, cápsula de estado
con círculo, nombre, tipo e internet y ajustes en su zona propia), lista de
idiomas (`language_panel`: menú Tahoe con título, cápsulas con círculo y
"Activa", resaltado suave y ajustes aparte, sin marca),
centro de control (`control_center_panel`: rejilla estricta de cuatro
columnas de 66 con hueco de 12 (círculo 66, cápsula 144 × 66, música
144 × 144 con la carátula real en píxeles cuando hay —marcador si no—,
igual que `music_panel`, Sonido a todo lo ancho de 68): Red con dos círculos blancos,
música de dos por dos, Preferencias con dos círculos, Sonido y cuatro
círculos; fondo invisible sin cristal y piezas en cristal translúcido
por zonas: `capsules`, `music`, `circles`, `slider`) y buscador Spotlight
de Tahoe (`search_panel`: cápsula grande con lupa y texto más tarjeta de
resultados con icono, hasta siete nombres, resaltado redondeado concéntrico,
"Sin resultados" y `filter_search_order` con tildes ignoradas) y segundo plano
(`background_panel`: cristal único Tahoe con título, hasta diez filas con círculo
suave, icono real en píxeles y nombre recortado, "Sin apps en segundo plano",
capas constantes y golpe por fila),
interruptor (`switch`: pista redondeada de 40 × 22 con tirador alargado en
cápsula blanca de 24 × 18, encendido o apagado según el tema, sin saber qué
ajusta) y grupo de filas de ajustes (`settings_group`: tarjeta clara con
filas de alto variable separadas por líneas finas con aire a ambos lados,
etiqueta a la izquierda y control a la derecha, sin saber qué ajusta cada
fila; da la tarjeta, las líneas, las etiquetas y la geometría de cada fila
para que la isla coloque sus interruptores y deslizadores) y deslizador
(`slider`: pista fina de 4 con relleno y tirador blanco en cápsula alargada
de 22 × 15 con borde fino, que solo usa Preferencias; el volumen y el centro
de control tienen su propio dibujo).

**No hace:** no conoce ninguna isla ni contiene lógica de negocio. No dibuja: describe qué dibujar.
