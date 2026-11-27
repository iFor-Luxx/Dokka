# dokka-win-control

**Hace:** trae al frente y minimiza ventanas de otras apps (`WindowControl`), las cierra
con `WM_CLOSE`, las deja por encima (`topmost`) y dice cuál está al frente, cuál de
una lista sigue más al frente (`front_window` por orden Z) y si está fijada;
da el rectángulo real de una ventana (para una minimizada, su posición restaurada sin tocarla). Muestra u oculta la barra de tareas (`set_taskbar_visible`, sin ajustes persistentes).
Bloquea la sesión (`lock_screen`) y abre el cuadro de apagar, reiniciar o suspender
(`show_shutdown_dialog` con `IShellDispatch::ShutdownWindows`): solo muestra el diálogo,
nunca apaga sin preguntar.
Consulta la red en uso (`SystemNetwork` con `NetworkInformation::GetInternetConnectionProfile`):
tipo (cable, wifi, otra o ninguna), nombre y si hay internet; sin conexión devuelve
apagado y nombre vacío.
Lista los idiomas de teclado (`SystemKeyboard` con `GetKeyboardLayoutList`): código corto,
nombre completo y cuál usa la ventana al frente; activar uno envía
`WM_INPUTLANGCHANGEREQUEST` a esa ventana. Si la barra tiene el foco (menú abierto), el
idioma activo es el de Dokka: conviene preguntar antes de abrir el menú.
Da el brillo DDC/CI del monitor principal (`SystemBrightness` con `dxva2`):
hilo propio porque tarda decenas o cientos de milisegundos, última orden al
arrastrar y valor adelantado; `None` sin DDC.

**No hace:** no las busca ni recuerda la última ventana ajena. Windows puede negarse con ventanas abiertas como administrador.
