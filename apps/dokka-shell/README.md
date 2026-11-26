# dokka-shell

**Hace:** es el ejecutable; arranca Dokka y conecta las piezas. Pasa al compositor
la fuente del sistema y los bytes de la fuente de iconos Lucide (en memoria,
sin instalar). Pide al dock abrir el menú
(con si la barra está oculta o en automático) y ejecuta su `MenuAction`; mientras el menú
está abierto, el overlay del menú y su submenú tienen prioridad. «Añadir app…» fija la elegida
en el diálogo; «Preferencias» abre la ventana de configuración (o la trae al frente);
«Reiniciar Dokka» relanza el ejecutable actual. Es dueño de si la barra está
oculta y del modo manual/automático (lo guarda en disco; en automático la oculta ya y en cada
arranque, con `resize`) y siempre la devuelve al salir, al reiniciar y al terminar `run`,
incluso con error.

**No hace:** no contiene lógica. Se cierra con Ctrl+C.
