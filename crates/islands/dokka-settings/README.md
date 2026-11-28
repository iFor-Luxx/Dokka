# dokka-settings

**Hace:** ventana de Preferencias estilo Ajustes del Sistema de macOS Tahoe, con
sombra genuina del compositor (una sola ventana exterior: el contenido más un
aire transparente donde cabe la sombra; el contenido se dibuja desplazado por
ese aire y el clic en él no hace nada) y ajustes reales: semáforos rojo (cierra y limpia la navegación: al volver a abrir
empieza en General, sin historial, búsqueda ni foco; los ajustes guardados no se
tocan), amarillo (minimiza a nuestro dock, a la derecha del separador, como miniatura
con el engranaje) y verde (alterna el tamaño normal 760 × 720 y uno mayor ≈ 920 × 820,
limitado al área de trabajo, recentrado sin salirse; la barra lateral conserva su ancho
y el contenido se ensancha solo; al cerrar vuelve al tamaño normal): al pasar el cursor muestran a la vez sus símbolos Lucide
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
posición en pantalla (segmentado Izquierda | Abajo | Derecha) y ocultar con
apps a pantalla completa en otra; animar al abrir, 
indicadores y vistas previas en la última. General: cabecera centrada con el
engranaje grande y dos tarjetas con filas (Acerca de este PC, Actualización de
Dokka, Almacenamiento; Fecha y hora, Idioma y región) con cuadrito, chevrón y
resaltado; la píldora va y viene entre la lista y sus apartados y el título
cambia. "Acerca de este PC" trae el dibujo del PC, el modelo, la tarjeta con
Nombre, Procesador, Gráfica, Memoria y Número de serie, las secciones Windows,
Pantalla y Almacenamiento con sus botones y el informe abajo; los datos los da
el shell (`set_system_info`) y los botones avisan (`OpenAbout`); si no cabe, la
rueda baja la ficha sin pintar fuera. "Fecha y hora" trae si las automáticas
están activas y el servidor NTP, la fecha en vivo (la pasa el shell con el
`Tick` solo con el apartado abierto), el interruptor propio de 24 horas (lo
usan el reloj de la barra y la hora del dock), si la zona es automática y su
nombre, con su botón a la página de Windows. "Idioma y región" trae los idiomas
preferidos (el primero es el principal), una línea de ejemplo con la fecha
larga, la corta y un número con el formato de Windows, las filas de solo
lectura (región, calendario, temperatura, medida y formatos) y el segmentado
propio del primer día de la semana (lo usa el calendario de la barra y del
dock), con su botón a la página de Windows. "Actualización de Dokka" trae su tarjeta
con la versión instalada (dato del shell) y el aviso de que aún no hay de
dónde descargar. "Almacenamiento" trae el disco con su barra por categorías,
la lista con tamaños ("Calculando…" mientras el hilo calcula) y los botones
de vaciar la papelera (`EmptyTrash`) y ajustes; los demás discos van aparte.
Barra superior: siete filas reordenables (Volumen, Red, Música,
Idioma, Buscar, Centro de control y Apps en segundo plano) en una sola
tarjeta, cada una con agarre a la izquierda (seis puntos verticales en color
secundario), nombre e interruptor; debajo, texto pequeño secundario que
explica el arrastre. Arrastrar desde cualquier punto de la fila salvo el
interruptor cambia el orden: tras unos 4 px la fila sigue al cursor en
vertical, resaltada en plano, sin salirse de la tarjeta y dejando el hueco
donde caerá; al soltar emite `Changed` una sola vez y solo si cambió. Barra de tareas: ocultar automáticamente la
barra de Windows (interruptor). Guarda si está abierta, si tiene el foco, la sección elegida,
el historial, la búsqueda y su foco, la fila y la mitad de la píldora bajo el
cursor, el hover de los semáforos, los valores reales (`set_values` los fija al
abrir), el deslizador a medias y la fila a medias, con los dibujos creados una vez
(agarre incluido); decide qué
hace cada clic (`SettingsAction`: `Changed` o `TaskbarAuto` solo al soltar, para
no guardar ni redimensionar a cada píxel) y qué puntos arrastran la ventana
(`drag_zone`, que nunca incluye un control del contenido), y resume lo dibujado
(`PaintedSettings` con valores, orden arrastrado y hover del contenido) para que el shell solo
repinte si cambió algo. Da su `Scene` de 760 × 720 de contenido en una ventana
exterior mayor (más el aire de la sombra por cada lado, desplazado en un único
sitio): fondo, barra
flotante y píldora como paneles; tarjeta, buscador, selección, hover de filas,
resaltado de la píldora, tarjetas del contenido, deslizadores (barra fina,
relleno desde la fábrica, marca y tirador en cápsula con "Pequeño" y "Grande" debajo),
segmentado (fondo y pastilla), interruptores
(pista y tirador en cápsula) y agarre con resaltado de la fila como planos (las tarjetas van planas sobre el
fondo claro y el resaltado como en el calendario, sin cristal); sin
`highlight` ni puntos en la escena; semáforos, agarres como iconos; insignias (como imagen
para que al filtrar repinten su color), glifos, lupa y chevrones como iconos;
nombres, búsqueda, título, etiquetas, ayuda del reorden y textos del deslizador y del segmentado como textos.
La fila arrastrada se pinta en plano antes que sus iconos y textos para no
taparlos; la píldora, su resaltado, sus líneas, sus iconos y el título van
nítidos sobre la franja difuminada de la zona de contenido (invisible arriba
del todo) y el contenido queda recortado a la ventana redondeada. Pulsar y arrastrar usan la misma pista y la misma cápsula que el dibujo, con el
centro del tirador recorriéndola sin salirse y los textos del segmentado sin
salirse de su tercio. El contenido cambia de tamaño entre secciones:
`SceneLayers` reconstruye si cambia el estilo o crece la cantidad; los textos
e iconos sobrantes se ocultan. Al reordenar la cantidad y el estilo no cambian:
solo se mueven, sin reconstruir. La lógica vive dividida (`content.rs` geometría,
`general.rs` lista y ficha, `datetime.rs` y `language.rs` fecha e idioma,
`storage.rs` y `update.rs` sus apartados, `list_counts.rs` cuentas variables,
`menubar_order.rs` orden y huecos, `slider_map.rs` conversión 0-1, `dock_position.rs` índice de posición,
`content_hit.rs` golpe, `menubar_drag.rs` arrastre, `reorder_pointer.rs` gesto y
`slider_pointer.rs` pista); la vista vive dividida (`content.rs` ayudantes,
`general_content.rs` y `general_rows.rs` lista, `about_content.rs`, `about_cards.rs`,
`about_buttons.rs` y `about_pc.rs` ficha, `datetime_content.rs` fecha y hora,
`language_content.rs` idioma y región, `storage_content.rs` con sus tarjetas
(`storage_cards.rs`, `storage_rows.rs`, `storage_disks.rs`) y su barra
(`storage_bar.rs`), y `update_content.rs`, `dock_content.rs` Dock, `menubar_content.rs` barra, `other_content.rs` resto);
el historial vive en `selection.rs` y el apartado en `general.rs`. El desglose
de almacenamiento y la versión de Dokka viven en `storage_state.rs`; la fecha
viva de "Fecha y hora" vive en `values.rs` (`now`).

**No hace:** no crea ventanas, no habla con Windows ni guarda en disco. Sin
animaciones: la ventana aparece, cambia de sección y se arrastra sin
transiciones. No mide la fuente. La navegación (sección, historial, búsqueda)
vive en `navigation.rs` para que `state.rs` no crezca; el zoom (`zoomed`) y el
ancho actual (`content_width`, desde `dokka-theme`) adaptan cabecera, tarjetas,
controles y golpes sin duplicar código: con el tamaño normal el resultado es
idéntico al de hoy.
