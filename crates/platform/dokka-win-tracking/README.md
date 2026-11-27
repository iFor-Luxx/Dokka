# dokka-win-tracking

**Hace:** lista las ventanas abiertas de otras apps (`WindowSource`) y avisa cuando cambian.
Da además la app de la ventana activa (`foreground_app`: `None` si es de Dokka o no se
sabe; el escritorio cuenta como el Explorador y el nombre puede ser el ejecutable).
Observa también el cambio de primer plano (`EVENT_SYSTEM_FOREGROUND`), así que activar
otra ventana produce el aviso `WINDOWS_CHANGED_MESSAGE`.

**No hace:** no obtiene iconos, no activa ni minimiza ventanas y no avisa de cambios de título.
La identidad de una app es la ruta de su ejecutable.
