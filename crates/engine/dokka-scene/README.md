# dokka-scene

**Hace:** describe qué se debe dibujar (formas, medidas y colores).

- `Text` admite alineación (`TextAlign::Center` o `Left`) respecto a `x`.
- `Overlay` añade `highlight`, `dividers` e `icons` (la punta del menú es un
  triángulo). Orden: panel, resaltado, líneas, puntos, iconos, textos.

**No hace:** no dibuja ni habla con Windows. Las medidas son lógicas, sin escalar por DPI.
