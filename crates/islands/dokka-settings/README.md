# dokka-settings

**Hace:** ventana de Preferencias estilo Ajustes del Sistema de macOS Tahoe, solo el
diseño y sin ajustes reales: semáforos rojo (cierra solo la ventana), amarillo
(minimiza) y verde (sin acción); barra lateral translúcida con casilla "Buscar" y
siete filas (General, Dock, Launchpad, Apariencia, Barra de tareas, Atajos y Acerca
de) con icono blanco de Lucide sobre cuadrado de color; contenido con píldora de
navegación atenuada, título de la sección y tarjeta gris muy suave "Próximamente". Guarda si está
abierta, la sección elegida, la fila bajo el cursor y los chevrones de la píldora
creados una vez; decide qué hace cada clic
(`SettingsAction`) y qué puntos arrastran la ventana (`drag_zone`). Da su `Scene`
de 760 × 720 con forma fija: fondo, barra flotante y píldora como
paneles; tarjeta, buscador, selección y hover como planos (la tarjeta va
plana sobre el fondo claro); semáforos como puntos; insignias, glifos, lupa y
chevrones como iconos; y nombres, búsqueda, título y tarjeta como textos.

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco. Sin
búsqueda real, sin navegación atrás/adelante y sin ajustes: el contenido es solo
la tarjeta. No mide la fuente.
