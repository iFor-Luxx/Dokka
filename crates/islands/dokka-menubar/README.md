# dokka-menubar

**Hace:** barra superior estilo macOS de lado a lado (28-32 de alto): icono
Lucide de Dokka a 20, nombre de la app activa en negrita a la izquierda y
fecha con hora a la derecha. El clic en el icono abre debajo el mismo menú
de cristal que el dock (`menu_panel` sin punta); el clic en la hora abre
debajo el mismo calendario del dock (`calendar_panel` con su píldora de
flechas), alineado a la derecha bajo el reloj. Navega meses con las flechas,
con hover, sin encogerse al navegar; la hora se resalta suave (plano, como
el icono) al pasar o con el calendario abierto. Solo uno de los dos a la vez:
abrir uno cierra el otro. Guarda el nombre, la hora, el hoy, si el menú o el
calendario están abiertos, el desfase de mes, las semanas vistas y los
resaltados; decide el clic y devuelve `MenubarAction`. Da su `Scene` (con el
menú o el calendario como `overlay`) y sus zonas activas (la franja y, abierto,
lo desplegado con su hueco). Expone `MenubarPainted` para repintar solo si
cambia (mes, hover y apertura incluidos).

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco.
No mide texto: el reloj va centrado en su hueco derecho.
