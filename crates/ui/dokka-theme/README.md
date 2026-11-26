# dokka-theme

**Hace:** reúne todos los estilos. Cambiar un valor aquí lo cambia en todo Dokka.

- `palette.rs`: colores base. Único lugar con valores de color.
- `theme.rs`: para qué se usa cada color.
- `themes/`: un archivo por tema (hoy solo `light`).
- `typography.rs`: fuente y tamaños.
- `measures/`: medidas visuales (`menu.rs` para el menú del clic derecho;
  `popup::SPACE` reserva 420 sobre el dock para que quepa el menú completo).
- `theme.rs`: `MenuColors` (`theme.menu`) para el menú.

**No hace:** no dibuja ni contiene lógica. Todavía no se puede cambiar de tema con Dokka abierto.
