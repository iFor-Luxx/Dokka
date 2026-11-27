# dokka-contracts

**Hace:** define los traits que las islas usan para pedir cosas a Windows o a los servicios.
`AppCatalog` lista las apps instaladas para el Launchpad; `AppLauncher` las abre.
`WindowControl` trae al frente, minimiza, cierra (`WM_CLOSE`), fija por encima y dice cuál de varias está más al frente.
`AppActions` abre como administrador y muestra el archivo en su carpeta; `RecycleBin` vacía la
papelera con la confirmación de Windows.

`VolumeControl`, `NetworkStatus` y `KeyboardLanguages` sirven a los iconos de la derecha de la barra.
`AudioOutputs` lista las salidas activas y cambia la predeterminada.

**No hace:** no implementa ninguno. Las implementaciones irán en `platform/` y `services/`.

Cambiar un contrato requiere aprobación.

`SystemTray` da los iconos de las apps en segundo plano y reenvía sus clics; `PreferencesStore`
guarda lo ajustado en Preferencias.
