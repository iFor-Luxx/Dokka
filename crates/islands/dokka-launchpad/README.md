# dokka-launchpad

**Hace:** une el catálogo del Escritorio con las fijadas (ordenadas por nombre, sin repetidas),
guarda si está abierto, la búsqueda, la página y qué app resalta, y decide qué app se abre, qué
página se muestra o si se cierra. Da su `Scene` a pantalla completa: cuadrícula de 7 columnas que
empieza arriba, casilla de búsqueda, puntos de página, iconos y nombres. Sin resultados muestra
"Sin resultados" centrado. Trata cada icono como
baldosa uniforme una sola vez por apertura y la reutiliza en cada redibujado. El cambio de página
desliza la cuadrícula (~0,3 s, frenando) y en los extremos cede y vuelve; la rueda y las flechas no
encadenan cambios mientras se mueve. La transición vive en `motion/` y la escena dibuja la página
destino desplazada, así las capas solo se actualizan.

**No hace:** no crea ventanas, no habla con Windows, no guarda en disco ni usa otras islas.
Sin animación de apertura, arrastre ni carpetas.
