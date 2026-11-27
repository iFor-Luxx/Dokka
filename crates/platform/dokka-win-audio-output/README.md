# dokka-win-audio-output

**Hace:** lista las salidas de audio activas (render) con su nombre
descriptivo y marca la predeterminada multimedia, y cambia la
predeterminada de consola y multimedia. Cumple `AudioOutputs`.

**No hace:** no dibuja ni guarda estado. Pieza aislada y apagable: usa la
interfaz sin documentar `IPolicyConfig` solo para cambiar; si algo falla
devuelve error y la sección de Salida se oculta, sin cerrar Dokka.
