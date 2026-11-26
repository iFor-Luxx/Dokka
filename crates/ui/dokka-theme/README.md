# dokka-theme

**Hace:** reúne todos los estilos. Cambiar un valor aquí lo cambia en todo Dokka.

- `palette.rs`: colores base. Único lugar con valores de color.
- `theme.rs`: para qué se usa cada color. `MenuColors` (`theme.menu`), `TooltipColors`
  (`theme.tooltip`), `LaunchpadColors` (`theme.launchpad`: fondo, texto, resaltado, casilla de
  búsqueda, puntos de página y baldosa de los iconos), `NavPillColors`
  (`theme.nav_pill`: cápsula, borde, chevrones, divisor y hover de la píldora de
  navegación) y `SettingsColors`
  (`theme.settings`: ventana, barra, casilla, selección y hover de filas, título, tarjeta,
  cuadrados de color y semáforos).
- `themes/`: un archivo por tema (hoy solo `light`).
- `measures/settings.rs`: ventana de Preferencias (760 × 720, barra de 260, semáforos,
  filas, cabecera con su píldora de 72 × 34 y tarjeta), con funciones para filas, título y tarjeta.
- `typography.rs`: fuente y tamaños.
- `measures/`: medidas visuales (`menu.rs` para el menú del clic derecho, con sus glifos Lucide
  a 20; `tooltip.rs` para la
  etiqueta en forma de píldora; `launchpad.rs` para la cuadrícula proporcional a la pantalla, la
  baldosa de los iconos, la casilla de búsqueda (lupa a la izquierda, cursor y marcador
  tenue), el mensaje de vacío, el aro del punto actual, los tiempos de las páginas y del zoom y su
  desenfoque máximo (`ZOOM_BLUR_MAX`); `popup.rs`: reserva 420 sobre el dock, animación de paneles,
  píldora del calendario de 64 × 28 y glifos de la música y los botones de cerrar/fijar, a 20;
  `nav_pill.rs`: chevrones a 20, radio igual a la mitad del alto y línea central de medio alto).

**No hace:** no dibuja ni contiene lógica. Todavía no se puede cambiar de tema con Dokka abierto.
