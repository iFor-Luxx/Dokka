# dokka-components

**Hace:** componentes reutilizables de interfaz: etiqueta flotante de cristal (`tooltip`),
fondo de paneles (`popup_panel`), calendario, vistas previas, menú del clic derecho
(`menu_panel`: tamaño, `Overlay` y fila bajo un punto, sin significado) y cuadrícula de apps
(`app_grid`: iconos, textos recortados con "…", resaltado y app bajo un punto, sin saber qué es el
Launchpad). Los `Overlay` de calendario y vistas previas salen con escala 1 y origen neutro;
la isla les pone la animación. El menú siempre vale 1.

**No hace:** no conoce ninguna isla ni contiene lógica de negocio. No dibuja: describe qué dibujar.
