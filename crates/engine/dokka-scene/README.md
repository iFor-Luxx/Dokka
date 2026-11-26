# dokka-scene

**Hace:** describe qué se debe dibujar (formas, medidas y colores).

- `Text` admite alineación (`TextAlign::Center` o `Left`) respecto a `x`.
- `Overlay` añade `highlight`, `dividers` y `pointer` (la punta del menú:
  centro x y altura; base = 2 × altura; nace en el borde inferior del panel
  y usa su tinte y su borde). Orden: panel, punta, resaltado, líneas, puntos, textos.

**No hace:** no dibuja ni habla con Windows. Las medidas son lógicas, sin escalar por DPI.
