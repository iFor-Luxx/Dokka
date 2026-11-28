# dokka-installer

**Hace:** es el ejecutable del instalador propio de Dokka. Solo arranca y
conecta: crea la ventana del instalador (`dokka-win-surface`), le dibuja la
escena de la isla (`dokka-setup`) con la foto de fondo (`InstallerRenderer`
de `dokka-win-composition`) y ejecuta el trabajo real (`dokka-win-install`)
en un hilo aparte. El avance llega a la ventana por mensajes; la interfaz
nunca se bloquea. Con `--uninstall` abre la pantalla de desinstalar.

**No hace:** no contiene lógica de instalación ni dibuja por sí misma:
delega en la isla, la plataforma y el contrato.
