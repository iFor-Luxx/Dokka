# dokka-win-control

**Hace:** trae al frente y minimiza ventanas de otras apps (`WindowControl`), las cierra
con `WM_CLOSE`, las deja por encima (`topmost`) y dice cuál está al frente, cuál de
una lista sigue más al frente (`front_window` por orden Z) y si está fijada;
da el rectángulo real de una ventana (para una minimizada, su posición restaurada sin tocarla). Muestra u oculta la barra de tareas (`set_taskbar_visible`, sin ajustes persistentes).
Bloquea la sesión (`lock_screen`) y abre el cuadro de apagar, reiniciar o suspender
(`show_shutdown_dialog` con `IShellDispatch::ShutdownWindows`): solo muestra el diálogo,
nunca apaga sin preguntar.

**No hace:** no las busca. Windows puede negarse con ventanas abiertas como administrador.
