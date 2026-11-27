# dokka-shell

**Hace:** es el ejecutable; arranca Dokka y conecta las piezas. Pasa al compositor
la fuente del sistema y los bytes de la fuente de iconos Lucide (en memoria,
sin instalar). Muestra la barra superior (`menubar.rs`: icono, app al frente
con `foreground_app` + `NameCache` y hora con `local_time` (y el hoy del calendario);
fila derecha a la izquierda de la fecha (de derecha a izquierda: centro de control,
buscar, idioma como texto con `set_language_code` —provisional "ES"—, música, red y
volumen, iconos Lucide provisionales) con un panel provisional por icono (tarjeta de
340 × 420 con su título, alineada bajo su icono sin salirse); su menú reutiliza
`context_menu.rs` para Preferencias, escritorio, administrador, reiniciar y
salir, con `menubar_menu_dismissed` antes de Preferencias y administrador y
`menubar_hide` al salir o reiniciar; su calendario comparte `calendar_panel`
con el dock (misma tarjeta y píldora, a la derecha bajo el reloj) y se cierra
al repetir la hora, al pulsar fuera, al perder el foco o al abrir el menú, la fila o un
panel del dock (y al revés; solo uno abierto a la vez, también entre iconos de la fila:
repetir el mismo lo cierra); mientras la barra está desplegada no se abren
previas) en su ventana (`menubar_show` con las medidas del tema, ventana de 460 para
que quepa el provisional, y `draw_menubar`), redibujándola solo si cambia
(`MenubarPainted`: mes, hover, idioma y apertura incluidos). Los paneles reales los
enchufan otros agentes: un archivo por panel en `dokka-menubar/src/logic/` que usa su
`<x>_size/<x>_panel/<x>_hit` de `dokka-components`, el shell les pasa
`VolumeState`/`NetworkState`/`KeyboardLanguage`/`NowPlayingInfo` cada `Tick` y ejecuta
su `MenubarAction` en `run_menubar_action` (ver el README de `dokka-menubar`). Pide al dock abrir el menú
(con si la barra está oculta o en automático) y ejecuta su `MenuAction`; mientras el menú
está abierto, el overlay del menú y su submenú tienen prioridad. «Añadir app…» fija la elegida
en el diálogo; «Preferencias» abre la ventana de configuración (o la trae al frente);
atiende su foco (semáforos grises sin foco), su búsqueda (filtrar, Retroceso, Enter y Esc)
y su historial atrás/adelante, y solo la repinta si cambió lo dibujado;
«Reiniciar Dokka» relanza el ejecutable actual. Las acciones de apps e iconos del sistema
viven en `app_menu_actions.rs` (activar, abrir, administrador, carpeta, cerrar la del frente,
salir sin matar, carpetas del explorador, papelera y Launchpad por el mismo camino que su icono);
guarda `SystemAppActions` y `SystemRecycleBin` en la sesión. Un error de una acción se registra
sin cerrar Dokka. Es dueño de si la barra está
oculta y del modo manual/automático (lo guarda en disco; en automático la oculta ya y en cada
arranque, con `resize`) y siempre la devuelve al salir, al reiniciar y al terminar `run`,
incluso con error. `preview_safety` recupera la salida del panel con el
cursor real (cada fotograma, cada segundo y cada 200 ms solo con panel) y
cierra el panel si otro primer plano llega por `Refresh` sin el cursor
encima, sin quitarlo de debajo del cursor.

**No hace:** no contiene lógica. Se cierra con Ctrl+C.
