# dokka-win-surface

**Hace:** crea las ventanas de Dokka (dock sin foco, Launchpad a pantalla completa
que sí lo toma, y superpuesta solo visual), cambia su tamaño y zona activa,
entrega los eventos del mouse y un aviso por segundo, marca el ritmo de las animaciones,
toma y devuelve el foco al abrir el menú o el Launchpad, avisa con `Dismissed`
para el menú y `LaunchpadDismissed` para el Launchpad al perder el foco o pulsar
Esc, y da la hora local. El Launchpad queda siempre justo debajo del dock y los
clics del dock no lo cierran. `SurfaceOptions.over_taskbar` usa el monitor
completo para bajar hasta el borde cuando la barra está oculta; si no, usa el área
de trabajo.

**No hace:** no dibuja ni decide qué hacer con los eventos. Usa solo el monitor principal y no reacciona aún a cambios de resolución o DPI.
