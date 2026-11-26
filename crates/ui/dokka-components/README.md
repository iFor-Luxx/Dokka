# dokka-components

**Hace:** componentes reutilizables de interfaz: etiqueta flotante de cristal (`tooltip`),
fondo de paneles (`popup_panel`), calendario con sus flechas Lucide, vistas previas, menú del clic derecho
(`menu_panel`: tamaño, `Overlay` y fila bajo un punto, con iconos Lucide de marca elegida y del chevrón
y submenú sin punta; sin significado), cuadrícula de apps (`app_grid`: 7 columnas que empiezan arriba, iconos,
textos recortados con "…", mensaje de vacío centrado, resaltado y app bajo un punto), casilla de búsqueda (`search_field`:
panel nítido, lupa Lucide reutilizable, texto escrito a la izquierda y marcador centrado, y si un punto cae dentro) y puntos de página
(`page_dots`: punto actual en blanco con aro claro algo mayor). Los `Overlay` salen con escala 1; la isla les pone la animación.

**No hace:** no conoce ninguna isla ni contiene lógica de negocio. No dibuja: describe qué dibujar.
