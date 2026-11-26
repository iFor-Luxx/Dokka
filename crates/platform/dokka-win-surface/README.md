# dokka-win-surface

**Hace:** crea las ventanas de Dokka (sin borde, encima, sin robar el foco), cambia su tamaño y zona activa,
entrega los eventos del mouse y un aviso por segundo, marca el ritmo de las animaciones, toma y devuelve
el foco al abrir y cerrar el menú, avisa con `SurfaceEvent::Dismissed` al perder el foco o pulsar Esc,
y da la hora local. `SurfaceOptions.over_taskbar` usa el monitor completo para bajar hasta el borde
cuando la barra está oculta; si no, usa el área de trabajo.

**No hace:** no dibuja ni decide qué hacer con los eventos. Usa solo el monitor principal y no reacciona aún a cambios de resolución o DPI.
