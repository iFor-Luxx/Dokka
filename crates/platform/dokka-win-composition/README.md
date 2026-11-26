# dokka-win-composition

**Hace:** dibuja una `Scene` (cristal, iconos, puntos, separador, textos sueltos
y resaltado) y coloca las miniaturas en vivo de Windows (DWM). Solo repinta un
icono cuando cambia su imagen. En el overlay pinta en orden panel, punta,
resaltado, líneas, puntos y textos; el texto se repinta si cambia su contenido,
estilo, color o alineación (izquierda o centro). El Launchpad usa el mismo
compositor en un tercer árbol visual (`draw_launchpad`).

**No hace:** no decide qué dibujar ni crea ventanas. Las miniaturas siempre quedan por encima del resto.
Todavía no se recupera si falla la tarjeta gráfica.
