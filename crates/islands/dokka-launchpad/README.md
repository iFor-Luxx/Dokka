# dokka-launchpad

**Hace:** une el catálogo del Escritorio con las fijadas (ordenadas por nombre, sin repetidas),
guarda si está abierto, la búsqueda, la página y qué app resalta, y decide qué app se abre, qué
página se muestra o si se cierra. Da su `Scene` a pantalla completa: cuadrícula de 7 columnas que
empieza arriba, casilla de búsqueda, puntos de página, iconos y nombres. Sin resultados muestra
"Sin resultados" centrado. Trata cada icono como
baldosa uniforme una sola vez por app y la reutiliza en cada redibujado y entre
aperturas. El cambio de página
desliza la cuadrícula (~0,3 s, frenando) y en los extremos cede y vuelve; la rueda y las flechas no
encadenan cambios mientras se mueve. La transición vive en `motion/` y la escena dibuja la página
destino desplazada, así las capas solo se actualizan. La casilla (`search_box.rs`) guarda el foco:
sin foco muestra lupa y "Buscar" a la izquierda; con foco (clic o primera letra) muestra el cursor
vertical parpadeante tras el texto y el marcador tenue a su derecha, con el aviso de cada segundo
(`tick`) y sin fotogramas continuos; Esc limpia y quita el foco, y con la casilla vacía y sin foco
cierra. La apertura y el cierre usan el zoom clásico (`motion/zoom.rs`: entra encogiendo desde
~1,15 y sale creciendo hacia el usuario mientras todo se desenfoca y se desvanece); la escena lleva opacidad,
escala y desenfoque, y reabrir a mitad del cierre continúa desde donde está. La paginación vive en `paging.rs`
y el clic en `logic/`.

**No hace:** no crea ventanas, no habla con Windows, no guarda en disco ni usa otras islas.
Sin arrastre ni carpetas. No mide la fuente: el cursor usa un avance aproximado por letra.
