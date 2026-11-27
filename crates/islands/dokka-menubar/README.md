# dokka-menubar

**Hace:** barra superior estilo macOS de lado a lado (28-32 de alto): icono
Lucide de Dokka a 20, nombre de la app activa en negrita a la izquierda y
fecha con hora a la derecha. El clic en el icono abre debajo el mismo menú
de cristal que el dock (`menu_panel` sin punta); resalta la fila bajo el
cursor y el icono al pasar o con el menú abierto. Guarda el nombre, la hora,
si el menú está abierto y la fila resaltada; decide el clic y devuelve
`MenubarAction`. Da su `Scene` (con el menú como `overlay`) y sus zonas
activas (la franja y, abierto, el menú con su hueco).

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco.
No mide texto: el reloj va centrado en su hueco derecho.
