# dokka-shell

**Hace:** es el ejecutable; arranca Dokka y conecta las piezas. Pide al dock abrir el menú
y ejecuta su `MenuAction`; mientras el menú está abierto, el overlay del menú tiene prioridad.
«Añadir app…» fija la elegida en el diálogo; «Reiniciar Dokka» relanza el ejecutable actual.
Es dueño de si la barra está oculta («Mostrar/Ocultar barra de tareas» recoloca con `resize`)
y siempre la devuelve al salir, al reiniciar y al terminar `run`, incluso con error.

**No hace:** no contiene lógica. Se cierra con Ctrl+C.
