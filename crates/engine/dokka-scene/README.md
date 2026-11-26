# dokka-scene

**Hace:** describe qué se debe dibujar (formas, medidas y colores).

- `Text` admite alineación (`TextAlign::Center` o `Left`) respecto a `x`.
- `Label` es la etiqueta flotante: texto de una línea con la posición de su
  punta; la plataforma mide el texto y la muestra como píldora de cristal
  (radio = mitad del alto) con punta del mismo tinte y borde.
- `Overlay` añade `highlight`, `dividers` y `pointer` (la punta del menú:
  centro x y altura; base = 2 × altura; nace en el borde inferior del panel
  y usa su tinte y su borde). Orden: panel, punta, resaltado, líneas, puntos, textos.
- `Overlay` lleva `opacity` y escala con origen (`scale` y `origin`): los paneles
  animados crecen desde el icono; el menú siempre vale 1.
- `Scene` añade `texts` sueltos, `highlight` opcional para el Launchpad y
  `submenu` opcional para el segundo panel del menú (mismo cristal, sin punta).

**No hace:** no dibuja ni habla con Windows. Las medidas son lógicas, sin escalar por DPI.
