# dokka-shell

**Hace:** es el ejecutable; arranca Dokka y conecta las piezas. Pasa al compositor
la fuente del sistema y los bytes de la fuente de iconos Lucide (en memoria,
sin instalar). Muestra la barra superior (`menubar.rs`: icono, app al frente
con `foreground_app` + `NameCache` y hora con `local_time` (y el hoy del calendario);
fila derecha a la izquierda de la fecha (de derecha a izquierda: centro de control,
buscar, idioma como texto con el código real, música, red y volumen con sus
dibujos según los datos) con su panel real por icono (medido con su `<x>_size`,
colocado bajo su icono sin salirse salvo el buscador, separado de la barra y
centrado a lo ancho de la pantalla);
su menú reutiliza `context_menu.rs` para Preferencias, escritorio, administrador, reiniciar y
salir, con `menubar_menu_dismissed` antes de Preferencias, administrador, ajustes
y apps nuevas y `menubar_hide` al salir o reiniciar; su calendario comparte `calendar_panel`
con el dock (misma tarjeta y píldora, a la derecha bajo el reloj) y se cierra
al repetir la hora, al pulsar fuera, al perder el foco o al abrir el menú, la fila o un
panel del dock (y al revés; solo uno abierto a la vez, también entre iconos de la fila:
repetir el mismo lo cierra); mientras la barra está desplegada no se abren
previas) en su ventana (`menubar_show` con las medidas del tema, ventana de 480 para
que quepa el idioma con una docena de filas y que crece con el monitor para el
buscador separado, sin tocar la reserva, y `draw_menubar`), redibujándola solo si cambia
(`MenubarPainted`: mes, hover, idioma, apertura y datos de cada panel incluidos).
Los paneles viven un archivo por panel en el shell (`menubar_volume.rs`,
`menubar_brightness.rs`, `menubar_network.rs`, `menubar_language.rs`, `menubar_music.rs`,
`menubar_search.rs`: el shell les pasa `VolumeState`/brillo/`NetworkState`/
`Vec<LanguageEntry>`/`NowPlayingInfo`/`Vec<AudioOutput>` y ejecuta su `MenubarAction` en
`run_menubar_action`; el centro de control delega en esas mismas variantes). El segundo plano (`background_panel`:
cristal único Tahoe con título, una fila por app con círculo suave, icono real y nombre
recortado, hasta diez filas según la ventana, "Sin apps en segundo plano" si no hay,
capas constantes y golpe por fila) lo rellena `menubar_background.rs` cada `Tick` con la
revisión del `TrayHost` (`apply_background_apps` al arrancar según Preferencias, error
una vez y parte apagada; `TrayActivate`/`TrayMenu` cierran con `menubar_menu_dismissed`
para no devolver el foco; clic derecho con `MenubarRightClicked`).
Volumen cada `Tick` y salidas al abrir el panel y tras elegir (`ChooseOutput`
las cambia, las relee con el volumen y deja el panel abierto; un error se
avisa una vez y la sección desaparece); brillo DDC sin esperar (`SystemBrightness`
en su hilo, último valor cada `Tick`, relectura solo al abrir el centro y
`BrightnessSetLevel` con valor adelantado; sin DDC no hay tarjeta y el panel
es el de antes); red e idioma cada pocos `Tick` y al abrir (el idioma solo
con la fila cerrada, antes de tomar el foco); música cada `Tick` con la pausa
adelantada al instante por el monitor. El `Tick` además cierra solo el centro
de control tras 6 s sin uso con el cursor fuera (`tick_control_idle`, por el
mismo camino que `MenubarDismissed`: empieza la salida en bloque de
`menubar_dismiss.rs` (Composition vuela cada pieza a su icono una sola vez
y cuenta el tiempo hasta el cierre real, que solo entonces recoge la
ventana; el foco vuelve al instante y los fotogramas se piden solo mientras
dura para ese temporizado, sin repintar la capa ni hilos nuevos);
los demás paneles no cambian. El buscador filtra el catálogo del
Escritorio más las fijadas (como el Launchpad) con `filter_search_order` e
iconos de `IconCache`; flechas, Enter, clic y Esc. Con la galería del modo
editar la ventana pasa a todo el alto del monitor (como el buscador) y vuelve
a su alto al cerrar; el teclado de la barra escribe en su casilla y la rueda
(`MenubarWheel`) desplaza su columna o sus secciones, con la plataforma
limitada a entregar ese evento. Pide al dock abrir el menú
(con si la barra está oculta o en automático) y ejecuta su `MenuAction`; mientras el menú
está abierto, el overlay del menú y su submenú tienen prioridad. «Añadir app…» fija la elegida
en el diálogo; «Preferencias» abre la ventana de configuración (o la trae al frente);
atiende su foco (semáforos grises sin foco), sus botones abajo y arrastres
(`MenubarPressed` fija el volumen o el brillo ya, `MenubarMoved` lo sigue, `MenubarClicked`
al soltar; Esc llega como `MenubarDismissed`), su búsqueda (teclear, Retroceso, Enter,
flechas y Esc) y su historial atrás/adelante, y solo la repinta si cambió lo dibujado;
«Reiniciar Dokka» relanza el ejecutable actual. Las acciones de apps e iconos del sistema
viven en `app_menu_actions.rs` (activar, abrir, administrador, carpeta, cerrar la del frente,
salir sin matar, carpetas del explorador, papelera y Launchpad por el mismo camino que su icono);
guarda `SystemAppActions` y `SystemRecycleBin` en la sesión. Un error de una acción se registra
sin cerrar Dokka. Es dueño de si la barra está
oculta y del modo manual/automático (lo guarda en disco; en automático la oculta ya y en cada
arranque, con `resize`) y siempre la devuelve al salir, al reiniciar y al terminar `run`,
incluso con error. Lo ajustado en Preferencias se guarda con `PreferencesFile`
(`PreferencesStore`) y se aplica en el acto (`apply_preferences.rs`: sesión, dock,
previas y barra, con `resize`; `set_taskbar_auto` guarda el modo y oculta o muestra
la barra de Windows ya reutilizando `taskbar_mode.rs`; `load_preferences` lo recupera
al arrancar antes de dibujar; sin previas no se abre ninguna y se cierra la abierta;
un error al leer o guardar se registra y se sigue). Lo ocultado desde el modo
editar del centro (`HiddenControlsChanged`) se guarda por ese mismo camino
(`apply_hidden_controls`: mismo archivo, `set_shown` en la barra y copia de
Preferencias sincronizada aunque esté abierta) y el panel sigue en modo editar. `preview_safety` recupera la salida del panel con el
cursor real (cada fotograma, cada segundo y cada 200 ms solo con panel) y
cierra el panel si otro primer plano llega por `Refresh` sin el cursor
encima, sin quitarlo de debajo del cursor. `dock_fit.rs` pega la ventana
al borde (margen 0) mientras una app ocupa la pantalla con auto-ocultado,
para que el píxel 0 siga dentro de la ventana al volver; sin oclusión usa
el margen habitual.

**No hace:** no contiene lógica. Se cierra con Ctrl+C.

Mientras la barra de Windows deba estar oculta, se vuelve a ocultar en cada aviso y cada segundo.

Traer una ventana al frente pasa siempre por `window_activation.rs`.
