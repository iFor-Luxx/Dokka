# dokka-model

**Hace:** define qué es una app, una ventana y una app dentro del dock.

**No hace:** no cambia el estado, no habla con Windows, no dibuja. No depende de ningún otro crate.

También define las preferencias del usuario (`Preferences`, con `general` para
el formato de 24 horas y el primer día de la semana, propios de Dokka) y un icono de la bandeja (`TrayItem`).
El resumen del equipo (`SystemSummary`, todo opcional: lo que no se lee es
`None` y se muestra "No disponible") sirve a "Acerca de este PC" y trae además
fecha y hora (`DateTimeSummary`: automáticas, servidor NTP, zona automática y
su nombre) e idioma y región (`LocaleSummary`: idiomas preferidos, región,
calendario, temperatura, sistema de medida, patrones y ejemplos).
El volumen trae además la salida activa (`AudioOutput`). Los controles del centro
(`ControlItem` en su orden de fábrica, `HiddenControls` como máscara de bits vacía
por defecto) viven en `control_item.rs`; `MenubarPreferences.hidden_controls` dice
cuáles no se ven.
