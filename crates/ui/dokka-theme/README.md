# dokka-theme

**Hace:** reúne todos los estilos. Cambiar un valor aquí lo cambia en todo Dokka.

- `palette.rs`: colores base. Único lugar con valores de color.
- `theme.rs`: para qué se usa cada color. `MenuColors` (`theme.menu`), `TooltipColors`
  (`theme.tooltip`: tinte, borde y texto) y `LaunchpadColors` (`theme.launchpad`: fondo
  apenas oscurecido, texto claro, resaltado suave, casilla de búsqueda y puntos de página).
- `themes/`: un archivo por tema (hoy solo `light`).
- `typography.rs`: fuente y tamaños.
- `measures/`: medidas visuales (`menu.rs` para el menú del clic derecho; `tooltip.rs` para la
  etiqueta en forma de píldora; `launchpad.rs` para la cuadrícula a pantalla completa
  (proporcional a la pantalla: 7 columnas, casilla arriba y puntos al 70 % del alto);
  `popup::SPACE` reserva 420 sobre el dock para que quepa el menú completo).

**No hace:** no dibuja ni contiene lógica. Todavía no se puede cambiar de tema con Dokka abierto.
