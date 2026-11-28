# dokka-contracts

**Hace:** define los traits que las islas usan para pedir cosas a Windows o a los servicios.
`AppCatalog` lista las apps instaladas para el Launchpad; `AppLauncher` las abre.
`WindowControl` trae al frente, minimiza, cierra (`WM_CLOSE`), fija por encima y dice cuál de varias está más al frente.
`AppActions` abre como administrador y muestra el archivo en su carpeta; `RecycleBin` vacía la
papelera con la confirmación de Windows.

`VolumeControl`, `NetworkStatus` y `KeyboardLanguages` sirven a los iconos de la derecha de la barra.
`AudioOutputs` lista las salidas activas y cambia la predeterminada.
`BrightnessControl` da el brillo del monitor principal por DDC/CI
(`brightness` de 0 a 1, `None` sin DDC) y lo cambia sin esperar.
`SystemInfo` da el resumen del equipo para "Acerca de este PC" sin bloquear
(último valor + pedir refresco, como el brillo) y abre sus páginas
(`SystemPage`: pantalla, almacenamiento e informe).

**No hace:** no implementa ninguno. Las implementaciones irán en `platform/` y `services/`.

Cambiar un contrato requiere aprobación.

`SystemTray` da los iconos de las apps en segundo plano y reenvía sus clics; `PreferencesStore`
guarda lo ajustado en Preferencias.
