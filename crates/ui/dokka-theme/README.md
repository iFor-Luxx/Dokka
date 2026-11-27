# dokka-theme

**Hace:** reúne todos los estilos. Cambiar un valor aquí lo cambia en todo Dokka.

- `palette.rs`: colores base. Único lugar con valores de color. `TRAY_TILE` y
  `TRAY_TILE_TRANSLUCENT` comparten el tono neutro de `MIST` y `MIST_STRONG` con
  menos alfa para las piezas Liquid Glass, sin aclarar el resto de paneles.
- `theme.rs`: para qué se usa cada color. `MenuColors` (`theme.menu`), `MenubarColors`
  (`theme.menubar`: mismo cristal del dock, texto y hover suave del icono), `TooltipColors`
  (`theme.tooltip`), `LaunchpadColors` (`theme.launchpad`: fondo, texto, resaltado, casilla de
  búsqueda, puntos de página y baldosa de los iconos), `NavPillColors`
  (`theme.nav_pill`: cápsula, borde, chevrones, divisor y hover de la píldora de
  navegación) y `SettingsColors`
  (`theme.settings`: ventana, barra, casilla, selección y hover de filas, título, tarjeta,
  cuadrados de color, semáforos con sus símbolos oscuros de hover, su gris sin foco y
  `TrayColors` (`theme.tray`: cristal ahumado de la barra, texto blanco, pista,
  relleno, hover y tinte translúcido propio del Centro de control).
- `themes/`: un archivo por tema (hoy solo `light`).
- `measures/settings.rs`: ventana de Preferencias (760 × 720, barra de 260, semáforos con
  glifos Lucide de 10, filas, búsqueda con cursor y límite, cabecera con su píldora de
  72 × 34, tarjetas de contenido con filas de 44 (56 la del deslizador, con
  "Pequeño" y "Grande" en 11 bajo la barra), interruptor de 40 × 22 con
  tirador en cápsula de 24 × 18 y deslizador de 168 con pista fina de 4 y
  tirador en cápsula de 22 × 15 con borde fino, y tarjeta "Próximamente"), con funciones para filas,
  título y tarjetas. `SettingsColors` trae además interruptor encendido y
  apagado con su tirador y barra, relleno y atenuado del deslizador, con ocho
  cuadrados de color (el nuevo de la barra superior en rosa).
- `typography.rs`: fuente y tamaños.
- `measures/`: medidas visuales (`menu.rs` para el menú del clic derecho, con sus glifos Lucide
  a 20; `tray_glass.rs` para los paneles de la barra estilo Tahoe (piezas de
  cristal separadas de 320 de ancho con hueco de 10, aire de 12, resaltado con margen de 6
  e inset de 10 y radio concéntrico de 10, cápsula de 60
  con círculo de 38, barra de 6 y títulos de 13/12); `control_center.rs` para
  la rejilla estricta del Centro de control (celda de 66, hueco de 12,
  cápsula doble de 144 × 66 con círculo de 42, música de 144 × 144,
  Sonido de 300 × 68, alto total 404); `menubar.rs` para la barra superior (franja de 30, icono a 20, hueco del
  reloj, menú colgando con su separación y ventana de 300 para que quepa el
  calendario de 6 semanas; la reserva de la franja no cambia); `tooltip.rs` para la
  etiqueta en forma de píldora; `search.rs` para el buscador Spotlight de Tahoe (panel de 560,
  cápsula de 52 con lupa a 20 y cursor, tarjeta de resultados con hasta 7 filas de 44
  con icono de 28, resaltado redondeado concéntrico y "Sin resultados"); `launchpad.rs` para la cuadrícula proporcional a la pantalla, la
  baldosa de los iconos, la casilla de búsqueda (lupa a la izquierda, cursor y marcador
  tenue), el mensaje de vacío, el aro del punto actual, los tiempos de las páginas y del zoom y su
  desenfoque máximo (`ZOOM_BLUR_MAX`); `popup.rs`: reserva 420 sobre el dock, animación de paneles,
  píldora del calendario de 64 × 28 y glifos de la música y los botones de cerrar/fijar, a 20;
  `music_panel.rs`: cristal único de música Tahoe de 320 × 138, carátula cuadrada de 56
  con radio concéntrico (exterior menos separación), título y artista recortados y
  "Nada en reproducción" con controles atenuados, glifos Lucide blancos a 28;
  `nav_pill.rs`: chevrones a 20, radio igual a la mitad del alto y línea central de medio alto; `background.rs` para el segundo plano (320 de ancho, título, filas de 34 con círculo de 26 e icono de 16, hasta diez filas y mensaje de vacío)).

**No hace:** no dibuja ni contiene lógica. Todavía no se puede cambiar de tema con Dokka abierto.
