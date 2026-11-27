# dokka-config

**Hace:** guarda y lee las apps fijadas en `%APPDATA%\Dokka\pinned_apps.txt` (`PinnedAppsStore`),
una por línea: nombre, tabulador y ruta. Guarda si la barra se oculta sola en
`%APPDATA%\Dokka\taskbar_mode.txt` (`TaskbarModeFile`): `auto` o `manual`. Guarda
las preferencias en `%APPDATA%\Dokka\preferences.txt` (`PreferencesFile`,
`PreferencesStore`): una por línea como `clave=valor` (`dock.icon_scale`,
`dock.magnification`, `dock.magnification_strength`, `dock.previews`,
`menubar.volume`, `menubar.network`, `menubar.music`, `menubar.language`,
`menubar.search`, `menubar.control_center`, `menubar.background_apps`); sin
archivo devuelve los valores de fábrica, las claves desconocidas o los valores
mal escritos se ignoran, los números se limitan a su rango y la escritura es
atómica (borrador y renombre).

**No hace:** no decide qué se fija.
