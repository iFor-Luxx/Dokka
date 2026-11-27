# dokka-win-tracking

**Hace:** lista las ventanas abiertas de otras apps (`WindowSource`) y avisa cuando cambian.
Da además la app de la ventana activa (`foreground_app`: `None` si es de Dokka o no se
sabe; el escritorio cuenta como el Explorador y el nombre puede ser el ejecutable).
Observa también el cambio de primer plano (`EVENT_SYSTEM_FOREGROUND`), así que activar
otra ventana produce el aviso `WINDOWS_CHANGED_MESSAGE`. Incluye SOLO Preferencias
del proceso propio (`settings_entry.rs`: identidad `dokka-preferencias`, nombre
"Preferencias", icono de engranaje del dock): solo minimizada, a la derecha del
separador como cualquier minimizada; nunca dock, barra, paneles, vista grande,
Launchpad ni bandeja. No cuenta como app que ocupa la pantalla.

**No hace:** no obtiene iconos, no activa ni minimiza ventanas y no avisa de cambios de título.
La identidad de una app es la ruta de su ejecutable.
