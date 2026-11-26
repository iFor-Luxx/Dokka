# dokka-win-composition

**Hace:** dibuja una `Scene` (cristal, paneles extra, iconos, puntos, separador, textos sueltos, resaltado y
etiquetas) y coloca las miniaturas en vivo de Windows (DWM). Solo repinta un icono cuando cambia
su imagen y solo cambia el pincel de un punto cuando cambia su color. La etiqueta es una píldora de cristal con punta redondeada de la misma pieza: reutiliza
sus capas al moverse entre iconos y solo repinta el texto si cambia. En el overlay pinta en orden
panel, punta, resaltado, líneas, puntos y textos; el texto se repinta si cambia su contenido,
estilo, color o alineación (izquierda o centro). El Launchpad usa el mismo compositor en un tercer
árbol visual (`draw_launchpad`).

**No hace:** no decide qué dibujar ni crea ventanas. Las miniaturas siempre quedan por encima del resto.
Todavía no se recupera si falla la tarjeta gráfica.
