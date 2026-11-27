# dokka-scene

**Hace:** describe qué se debe dibujar (formas, medidas y colores).

- `Text` admite alineación (`TextAlign::Center` o `Left`) respecto a `x`.
- `Shape::Glyph` es un glifo de la fuente de iconos, centrado en (`x`, `y`).
- `Label` es la etiqueta flotante: texto de una línea con la posición de su
  punta; la plataforma mide el texto y la muestra como píldora de cristal
  (radio = mitad del alto) con punta del mismo tinte y borde. `side` dice
  dónde está el icono (`Below` abajo como hoy, `After` a la izquierda con la
  etiqueta a la derecha, `Before` al revés); `center_y` y `bottom` (punta)
  colocan las laterales.
- `Overlay` añade `highlight`, `dividers` y `pointer` (la punta del menú:
  centro y altura; base = 2 × altura; nace en el borde hacia el dock
  —abajo, izquierda o derecha según `side`— y usa su tinte y su borde). Orden: panel, punta, resaltado, líneas, puntos, textos, iconos.
- `Overlay` lleva `icons` sobre el resaltado, junto a los textos (el chevrón del menú).
- `Overlay` lleva `flat_panels`: rectángulos planos (sin cristal) para barras y tarjetas.
- `Overlay` lleva `tiles`: piezas de cristal separadas, como en el Centro de control de Tahoe.
- `Overlay` lleva `opacity` y escala con origen (`scale` y `origin`): los paneles
  animados crecen desde el icono; el menú siempre vale 1.
- `Scene` añade `texts` sueltos, `highlight` opcional para el Launchpad y
  `submenu` opcional para el segundo panel del menú (mismo cristal, sin punta).
- `Scene` añade `flat_panels`: planos (tinte y borde, sin cristal) sobre el
  cristal de la misma ventana, como el buscador, la selección y el hover de
  Preferencias; el dock y el Launchpad no los usan.
- `Scene` añade `opacity` y `scale`: cuánto se ve todo (1 en reposo) y cuánto
  escala el contenido sobre el fondo, con origen en su centro. El dock siempre vale 1.
- `Scene` añade `blur`: desenfoque gaussiano del contenido en unidades lógicas
  (0 en reposo, máximo del tema durante el zoom del Launchpad).

**No hace:** no dibuja ni habla con Windows. Las medidas son lógicas, sin escalar por DPI.
