# dokka-win-tracking

**Hace:** lista las ventanas abiertas de otras apps (`WindowSource`) y avisa cuando cambian.
Da además la app de la ventana activa (`foreground_app`: `None` si es de Dokka o no se
sabe; el escritorio cuenta como el Explorador y el nombre puede ser el ejecutable).
Dice si una app ajena ocupa el monitor principal (`foreground_occludes_primary`:
maximizada con `IsZoomed` o cubriéndolo con `GetWindowRect`/`DwmGetWindowAttribute`
frente a `MonitorFromWindow`; ignora el escritorio y a Dokka; si no se sabe,
`false` y el dock queda visible).
Observa también el cambio de primer plano (`EVENT_SYSTEM_FOREGROUND`) y el movimiento
solo de la ventana al frente (`EVENT_OBJECT_LOCATIONCHANGE`, avisando solo cuando
cambia lo ocupado para no inundar), así que activar, maximizar o restaurar
otra ventana produce el aviso `WINDOWS_CHANGED_MESSAGE`.

**No hace:** no obtiene iconos, no activa ni minimiza ventanas y no avisa de cambios de título.
La identidad de una app es la ruta de su ejecutable.
