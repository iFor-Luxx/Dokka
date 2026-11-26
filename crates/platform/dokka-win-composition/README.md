# dokka-win-composition

**Hace:** dibuja una `Scene` (cristal, iconos, puntos y separador) y coloca las miniaturas en vivo
de Windows (DWM). Solo repinta un icono cuando cambia su imagen. En el overlay pinta en orden
panel, resaltado, líneas, puntos, iconos (punta del menú) y textos; el texto se repinta si cambia
su contenido, estilo, color o alineación (izquierda o centro).

**No hace:** no decide qué dibujar ni crea ventanas. Las miniaturas siempre quedan por encima del resto.
Todavía no se recupera si falla la tarjeta gráfica.
