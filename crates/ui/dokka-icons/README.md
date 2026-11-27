# dokka-icons

**Hace:** describe los iconos de color propios de Dokka (explorador, launchpad, calendario, reloj y
papelera) como dibujos, y expone los de interfaz desde la librería `lucide-icons` (chevrón del
submenú, marca elegida, marca de la barra, chevrones de la píldora de navegación, lupa, botones de música y de cerrar/fijar, nota musical de la barra en `music.rs`; iconos blancos de
las siete secciones de Preferencias y símbolos de cerrar, minimizar y zoom de sus semáforos) como glifos,
más la lupa del buscador (`search`: `search_bar_icon` blanca para la barra superior y
`search_field_icon` oscura para la casilla clara, ambas Lucide `Search`),
más el círculo liso y los tres botones completos (círculo con símbolo) de los semáforos, más los bytes
de su fuente. Y de red e idioma (`network_language`: wifi, sin conexión, cable,
globo y marca del idioma activo). Y uniforma los iconos de apps para el Launchpad (`app_tile`: si llenan el cuadro
—aunque traigan margen transparente— ocupan toda la baldosa con las esquinas recortadas; si son
silueta se centran sobre un degradado claro sacado de los colores del propio icono (color medio
de la mitad superior y de la inferior, mezclados con la baldosa clara del tema); el marco blanco
o fino se quita). Más iconos del Centro de control (`control_center`: deslizadores blancos
para la barra y wifi, cable, globo, volumen, música, ajustes, escritorio, bloqueo, vista de tareas,
proyectar, calculadora, recorte, reloj y actividad oscuros para sus tarjetas claras, con las variantes
azules de red, escritorio y bloqueo sobre círculo blanco).

**No hace:** no pinta, no lee la hora ni la papelera: recibe esos datos. Los de color son diseños
propios, no copias de Apple; los de interfaz son Lucide.
