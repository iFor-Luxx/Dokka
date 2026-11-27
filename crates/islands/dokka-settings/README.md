# dokka-settings

**Hace:** ventana de Preferencias estilo Ajustes del Sistema de macOS Tahoe, con
ajustes reales: semáforos rojo (cierra solo la ventana), amarillo (minimiza) y
verde (sin acción): al pasar el cursor muestran a la vez sus símbolos Lucide
oscuros (×, −, +) y sin foco van en gris; barra lateral translúcida con casilla
"Buscar" y ocho filas (General, Dock, Launchpad, Barra superior, Apariencia,
Barra de tareas, Atajos y Acerca de) con icono blanco de Lucide sobre cuadrado
de color; la casilla filtra las filas por nombre sin distinguir mayúsculas ni
tildes, con cursor, Retroceso, Esc (limpia y quita el foco) y Enter (elige la
primera coincidencia), y "Sin resultados" si no hay ninguna; contenido con
píldora de navegación con historial atrás/adelante (elegir una sección nueva
borra lo de adelante) y título de la sección y tarjetas claras con filas de 44
(56 la del deslizador, con "Pequeño" y "Grande" debajo) separadas por líneas
finas con aire a ambos lados, etiqueta a la izquierda y control a la derecha
con el mismo margen (cristal claro y neutro como hoy). Dock: tamaño,
ampliación y cantidad en una tarjeta (relleno desde la fábrica con su marca);
posición en pantalla (segmentado Izquierda | Abajo | Derecha) y ocultar
automáticamente en otra; animar al abrir, indicadores y vistas previas en la
última. Barra superior, tras el
Launchpad: siete interruptores (Volumen, Red, Música, Idioma, Buscar, Centro
de control y Apps en segundo plano) en una sola tarjeta. Barra de tareas: ocultar automáticamente la
barra de Windows (interruptor). El resto sigue con la tarjeta gris muy suave
"Próximamente". Guarda si está abierta, si tiene el foco, la sección elegida,
el historial, la búsqueda y su foco, la fila y la mitad de la píldora bajo el
cursor, el hover de los semáforos, los valores reales (`set_values` los fija al
abrir) y el deslizador a medias, con los dibujos creados una vez; decide qué
hace cada clic (`SettingsAction`: `Changed` o `TaskbarAuto` solo al soltar, para
no guardar ni redimensionar a cada píxel) y qué puntos arrastran la ventana
(`drag_zone`, que nunca incluye un control del contenido), y resume lo dibujado
(`PaintedSettings` con valores y hover del contenido) para que el shell solo
repinte si cambió algo. Da su `Scene` de 760 × 720 con forma fija: fondo, barra
flotante y píldora como paneles; tarjeta, buscador, selección, hover de filas,
resaltado de la píldora, tarjetas del contenido, deslizadores (barra fina,
relleno desde la fábrica, marca y tirador en cápsula con "Pequeño" y "Grande" debajo),
segmentado (fondo y pastilla) e interruptores
(pista y tirador en cápsula) como planos (las tarjetas van planas sobre el
fondo claro y el resaltado como en el calendario, sin cristal); sin
`highlight` ni puntos en la escena; semáforos como iconos; insignias (como imagen
para que al filtrar repinten su color), glifos, lupa y chevrones como iconos;
nombres, búsqueda, título, etiquetas y textos del deslizador y del segmentado como textos.
Pulsar y arrastrar usan la misma pista y la misma cápsula que el dibujo, con el
centro del tirador recorriéndola sin salirse y los textos del segmentado sin
salirse de su tercio. El contenido cambia de tamaño entre secciones:
`SceneLayers` reconstruye si cambia el estilo o crece la cantidad; los textos
e iconos sobrantes se ocultan. La lógica vive dividida (`content.rs` geometría,
`slider_map.rs` conversión 0-1, `dock_position.rs` índice de posición,
`content_hit.rs` golpe); la vista vive dividida (`content.rs` ayudantes,
`dock_content.rs` Dock, `other_content.rs` resto).

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco. Sin
animaciones: la ventana aparece, cambia de sección y se arrastra sin
transiciones. No mide la fuente.
