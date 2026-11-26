# dokka-theme

**Hace:** reúne todos los estilos. Cambiar un valor aquí lo cambia en todo Dokka.

- `palette.rs`: colores base. Único lugar con valores de color.
- `theme.rs`: para qué se usa cada color. `MenuColors` (`theme.menu`), `TooltipColors`
  (`theme.tooltip`: tinte, borde y texto) y `LaunchpadColors` (`theme.launchpad`: fondo
  oscurecido, texto claro, resaltado suave y baldosa clara de los iconos).
- `themes/`: un archivo por tema (hoy solo `light`).
- `typography.rs`: fuente y tamaños.
- `measures/`: medidas visuales (`menu.rs` para el menú del clic derecho; `tooltip.rs` para la
  etiqueta en forma de píldora; `launchpad.rs` para la cuadrícula a pantalla completa y la baldosa
  uniforme de los iconos; `popup.rs`: reserva 420 sobre el dock y animación de paneles —
  duraciones, escalas, recorridos y tramo de opacidad).

**No hace:** no dibuja ni contiene lógica. Todavía no se puede cambiar de tema con Dokka abierto.
