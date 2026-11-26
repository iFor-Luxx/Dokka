# dokka-contracts

**Hace:** define los traits que las islas usan para pedir cosas a Windows o a los servicios.
`AppCatalog` lista las apps instaladas para el Launchpad; `AppLauncher` las abre.
`WindowControl` trae al frente, minimiza, cierra (`WM_CLOSE`) y fija por encima.

**No hace:** no implementa ninguno. Las implementaciones irán en `platform/` y `services/`.

Cambiar un contrato requiere aprobación.
