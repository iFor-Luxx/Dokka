# dokka-theme

**Hace:** reúne todos los estilos. Cambiar un valor aquí lo cambia en todo Dokka.

- `palette.rs`: colores base. Único lugar con valores de color.
- `theme.rs`: para qué se usa cada color. `MenuColors` (`theme.menu`), `TooltipColors`
  (`theme.tooltip`) y `LaunchpadColors` (`theme.launchpad`: fondo, texto, resaltado, casilla de
  búsqueda, puntos de página y baldosa de los iconos).
- `themes/`: un archivo por tema (hoy solo `light`).
- `typography.rs`: fuente y tamaños.
- `measures/`: medidas visuales (`menu.rs` para el menú del clic derecho, con sus glifos Lucide
  a 20; `tooltip.rs` para la
  etiqueta en forma de píldora; `launchpad.rs` para la cuadrícula proporcional a la pantalla, la
  baldosa de los iconos, la casilla de búsqueda (lupa a la izquierda, cursor y marcador
  tenue), el mensaje de vacío, el aro del punto actual y los tiempos de las páginas y del zoom; `popup.rs`: reserva 420 sobre el dock, animación de paneles y glifos
  del calendario y la música, a 20).

**No hace:** no dibuja ni contiene lógica. Todavía no se puede cambiar de tema con Dokka abierto.
