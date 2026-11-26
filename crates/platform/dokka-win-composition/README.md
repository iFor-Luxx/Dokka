# dokka-win-composition

**Hace:** dibuja una `Scene` (cristal, paneles extra, iconos, puntos, separador, textos sueltos, resaltado y
etiquetas) y coloca las miniaturas en vivo de Windows (DWM). Solo repinta un icono cuando cambia
su imagen y solo cambia el pincel de un punto cuando cambia su color. La etiqueta es una píldora de cristal con punta redondeada de la misma pieza: reutiliza
sus capas al moverse entre iconos y solo repinta el texto si cambia. En el overlay pinta en orden
panel, punta, resaltado, líneas, puntos, textos e iconos; el texto se repinta si cambia su contenido,
estilo, color o alineación (izquierda o centro). El overlay se escala sobre el icono sin reconstruir
capas; las miniaturas del panel usan la misma transformación. El submenú reutiliza `OverlayLayers`
encima del menú y tampoco reconstruye al mover el resaltado. Cada escena cuelga de dos
contenedores: el fondo y el `holder` con el contenido dentro. La opacidad desvanece ambos y la escala agranda
solo el contenido desde el centro del fondo, sin reconstruir capas. El Launchpad usa el mismo compositor en un tercer
árbol visual (`draw_launchpad`). Durante su zoom, `blur_stage` captura el contenido con un `CompositionVisualSurface`
y lo muestra desenfocado con un pincel gaussiano (`blur_effect`, propiedad animable `Blur.BlurAmount`); con desenfoque
activo el `holder` se oculta y solo se ve la copia (sin doble imagen), y en reposo la
copia se oculta y no hay efectos activos. Si el efecto no se puede crear, se sigue con zoom y desvanecido. Los dibujos (incluidos los glifos Lucide, con la fuente en
memoria de `dokka-win-text`) se pintan al doble de su lado en pantalla, con un mínimo de 128
para que los iconos grandes del dock no pierdan al crecer con la lupa; solo se repintan si
cambia su imagen, nunca al moverse o escalarse.

**No hace:** no decide qué dibujar ni crea ventanas. Las miniaturas siempre quedan por encima del resto.
Todavía no se recupera si falla la tarjeta gráfica.
