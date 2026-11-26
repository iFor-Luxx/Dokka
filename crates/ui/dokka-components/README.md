# dokka-components

**Hace:** componentes reutilizables de interfaz: etiqueta flotante de cristal (`tooltip`),
fondo de paneles (`popup_panel`), calendario, vistas previas, menú del clic derecho
(`menu_panel`: tamaño, `Overlay` y fila bajo un punto, sin significado), cuadrícula de apps
(`app_grid`: 7 columnas que empiezan arriba, iconos, textos recortados con "…", resaltado y app
bajo un punto), casilla de búsqueda (`search_field`: panel nítido, lupa de aro reutilizable, texto
y si un punto cae dentro) y puntos de página (`page_dots`). Los `Overlay` salen con escala 1; la
isla les pone la animación.

**No hace:** no conoce ninguna isla ni contiene lógica de negocio. No dibuja: describe qué dibujar.
