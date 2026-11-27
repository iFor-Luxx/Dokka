# dokka-settings

**Hace:** ventana de Preferencias estilo Ajustes del Sistema de macOS Tahoe, solo el
diseño y sin ajustes reales: semáforos rojo (cierra solo la ventana), amarillo
(minimiza) y verde (sin acción): al pasar el cursor sobre ellos muestran a la
vez sus símbolos Lucide oscuros (×, −, +) y sin foco van en gris; barra lateral
translúcida con casilla "Buscar" y siete filas (General, Dock, Launchpad,
Apariencia, Barra de tareas, Atajos y Acerca de) con icono blanco de Lucide
sobre cuadrado de color; la casilla filtra las filas por nombre sin distinguir
mayúsculas ni tildes, con cursor, Retroceso, Esc (limpia y quita el foco) y
Enter (elige la primera coincidencia), y "Sin resultados" si no hay ninguna;
contenido con píldora de navegación con historial atrás/adelante (elegir una
sección nueva borra lo de adelante) y título de la sección y tarjeta gris muy
suave "Próximamente". Guarda si está abierta, si tiene el foco, la sección
elegida, el historial, la búsqueda y su foco, la fila y la mitad de la píldora
bajo el cursor, el hover de los semáforos y los dibujos creados una vez;
decide qué hace cada clic (`SettingsAction`) y qué puntos arrastran la
ventana (`drag_zone`), y resume lo dibujado (`PaintedSettings`) para que el
shell solo repinte si cambió algo. Da su `Scene` de 760 × 720 con forma fija:
fondo, barra flotante y píldora como paneles; tarjeta, buscador, selección y
hover como planos (la tarjeta va plana sobre el fondo claro); resaltado de la
píldora como `highlight`; semáforos como puntos; insignias (como imagen para
que al filtrar repinten su color), glifos, lupa, chevrones y símbolos de los
semáforos como iconos; y nombres, búsqueda, título, tarjeta y vacío como
textos. Nada cambia de forma al filtrar: las capas de sobra se ocultan sin
reconstruir.

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco. Sin
ajustes: el contenido es solo la tarjeta. No mide la fuente.
