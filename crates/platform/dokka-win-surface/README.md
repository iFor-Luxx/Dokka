# dokka-win-surface

**Hace:** crea las ventanas de Dokka (dock sin foco, Launchpad a pantalla completa
que sí lo toma, Preferencias normal y activable con sitio en la barra de tareas,
y superpuesta solo visual), cambia su tamaño y zona activa,
que sí lo toma, vista grande a pantalla completa transparente al mouse, y superpuesta solo visual), cambia su tamaño y zona activa,
entrega los eventos del mouse, las teclas escritas (`WM_CHAR` con `TranslateMessage`),
el Retroceso, el Enter, las flechas izquierda/derecha y la rueda sobre el Launchpad, y sobre
Preferencias el movimiento, la salida, el botón abajo (para arrastrar) y el clic,
y un aviso por segundo, marca
el ritmo de las animaciones, toma y devuelve el foco al abrir el menú o el Launchpad
(con `AttachThreadInput` para que Windows conceda el primer plano y `SetFocus` para el
teclado), avisa con `Dismissed` para el menú y `LaunchpadDismissed` al perder el foco, y con
`LaunchpadEscape` al pulsar Esc (para limpiar la búsqueda antes de cerrar), centra
Preferencias una vez con las medidas que pasa el shell, la muestra al frente sin dejarla
siempre encima, la oculta sin destruirla (destruir una ventana sin dueña terminaría la
app; `WM_CLOSE` solo la oculta y avisa con `SettingsDismissed`), la mueve con el arrastre
nativo de Windows, permite minimizarla desde la barra de tareas, y da la
hora local. El Launchpad queda siempre justo debajo del dock y los
clics del dock no lo cierran. `SurfaceOptions.over_taskbar` usa el monitor
completo para bajar hasta el borde cuando la barra está oculta; si no, usa el área
de trabajo.

**No hace:** no dibuja ni decide qué hacer con los eventos. Usa solo el monitor principal y no reacciona aún a cambios de resolución o DPI.
