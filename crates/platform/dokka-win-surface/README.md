# dokka-win-surface

**Hace:** crea las ventanas de Dokka (dock sin foco, Launchpad a pantalla completa
que sí lo toma, Preferencias normal y activable con sitio en la barra de tareas y con
sombra del compositor (una sola ventana exterior: el contenido más un aire
transparente donde cabe la sombra dibujada por `dokka-win-composition`; sin
ella se ve igual; la región no la recorta),
y superpuesta solo visual), cambia su tamaño y zona activa y coloca el dock
según su borde (`place_window.rs`: abajo centrado como hoy; en laterales
centrado en vertical pegado a su borde con el mismo margen, bajo la reserva
superior; con auto-ocultado se desplaza hacia fuera dejando la zona de toque
de la línea (24, como el tema) y vuelve al tocarla, también junto a la barra de tareas visible;
cada deslizamiento la deja delante (TOPMOST) para que la app maximizada
o a pantalla completa no la entierre),
que sí lo toma, vista grande a pantalla completa transparente al mouse, y superpuesta solo visual), cambia su tamaño y zona activa,
entrega los eventos del mouse, las teclas escritas (`WM_CHAR` con `TranslateMessage`),
el Retroceso, el Enter, las flechas izquierda/derecha y la rueda sobre el Launchpad, y sobre
Preferencias el movimiento, la salida, el botón abajo (para arrastrar, con
captura del ratón como la barra para que el movimiento y la suelta lleguen
aunque el cursor salga y arrastrar un deslizador funcione fuera), el clic
(solo si hubo botón abajo antes, con la captura liberada al soltar), el foco
ganado y perdido (reenvía `WM_ACTIVATE` a la cola, como `DISMISS_MESSAGE`), lo escrito,
el Retroceso, el Enter y el Esc con eventos propios sin mezclarse con el Launchpad,
y un aviso por segundo, marca
el ritmo de las animaciones, toma y devuelve el foco al abrir el menú o el Launchpad
(con `AttachThreadInput` para que Windows conceda el primer plano y `SetFocus` para el
teclado), avisa con `Dismissed` para el menú y `LaunchpadDismissed` al perder el foco, y con
`LaunchpadEscape` al pulsar Esc (para limpiar la búsqueda antes de cerrar), centra
Preferencias una vez con las medidas de contenido y el aire que pasa el shell,
la muestra al frente sin dejarla
siempre encima, la oculta sin destruirla (destruir una ventana sin dueña terminaría la
app; `WM_CLOSE` solo la oculta y avisa con `SettingsDismissed`), la mueve con el arrastre
nativo de Windows, permite minimizarla desde la barra de tareas, aplica un tamaño
nuevo de contenido con `settings_apply_size` (zoom verde o vuelta a normal:
limitado al área de trabajo contando el contenido, no el margen, y recentrado
con el aire alrededor) y da la
hora local. Los avisos de cambios se agrupan como `Refresh`. El Launchpad queda siempre justo debajo del dock y los
clics del dock no lo cierran. `cursor_over_dock` devuelve la posición
lógica del cursor real si está sobre el dock (con DPI y zonas activas) y
`set_preview_watch` arma un aviso cada 200 ms solo con panel, para que el
shell recupere la salida si se perdió; sin panel no hay sondeo.
`SurfaceOptions.over_taskbar` usa el monitor
completo para bajar hasta el borde cuando la barra está oculta; si no, usa el área
de trabajo.
Crea además la barra superior (`menubar_window.rs`, como el dock: sin activación,
`tool window`, topmost, cristal) a todo el ancho del principal; `menubar_show`
reserva arriba con AppBar (`ABM_NEW`, `ABM_QUERYPOS`, `ABM_SETPOS` borde superior,
`ABM_REMOVE` en `menubar_hide` y en `Drop`) y coloca una ventana más alta para el
menú desplegado, recortada por región (`menubar_area.rs`); `menubar_width`,
`menubar_monitor_height` y `menubar_set_active_areas` usan esa escala, y
`menubar_set_window_height` agranda la ventana para el buscador separado sin
tocar la reserva de la barra, y
`menubar_menu_opened/closed/dismissed` guardan el foco igual que el menú del dock,
pero `menubar_menu_opened` une la entrada al hilo activo (`AttachThreadInput`)
y fija el foco del teclado (`SetFocus`), como el Launchpad, para recibir teclas.
`run_loop.rs` traduce su ratón a `MenubarMoved`, `MenubarLeft`, `MenubarPressed`
(botón abajo) y `MenubarClicked`, y sus teclas con foco a `MenubarTyped`
(`WM_CHAR` con `TranslateMessage`), `MenubarBackspace`, `MenubarEnter`,
`MenubarUp` y `MenubarDown`; su pérdida de foco o Esc a `MenubarDismissed`
(`menubar_loop.rs`). Mientras el botón está abajo usa `SetCapture` para que
el movimiento y la suelta lleguen aunque el cursor salga, con coordenadas
negativas correctas, y libera la captura al soltar.
Ante pantalla completa (`ABN_FULLSCREENAPP`) la oculta y la recupera al salir; el
Launchpad y la vista grande (pantalla completa) sí la cubren, y el dock sigue
anclado al área de trabajo, así que la reserva de arriba no lo descoloca.
Si el proceso se mata a la fuerza no pasa por `Drop` y la reserva puede quedarse
hasta que el área se reevalúe; en uso normal nunca se registra dos veces y
`menubar_show` solo recoloca.

**No hace:** no dibuja ni decide qué hacer con los eventos. Usa solo el monitor principal y no reacciona aún a cambios de resolución o DPI.
