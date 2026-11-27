# dokka-config

**Hace:** guarda y lee las apps fijadas en `%APPDATA%\Dokka\pinned_apps.txt` (`PinnedAppsStore`),
una por línea: nombre, tabulador y ruta. Guarda si la barra se oculta sola en
`%APPDATA%\Dokka\taskbar_mode.txt` (`TaskbarModeFile`): `auto` o `manual`. Guarda
las preferencias en `%APPDATA%\Dokka\preferences.txt` (`PreferencesFile`,
`PreferencesStore`): una por línea como `clave=valor` (`dock.icon_scale`,
`dock.magnification`, `dock.magnification_strength`, `dock.position`
(`left`/`bottom`/`right`), `dock.auto_hide`, `dock.animate_launch`,
`dock.show_indicators`, `dock.previews`, `dock.genie`, `menubar.volume`, `menubar.network`,
`menubar.music`, `menubar.language`, `menubar.search`, `menubar.control_center`,
`menubar.background_apps`, `menubar.order` (`volume,network,music,background_apps,language,search,control_center`
de izquierda a derecha); sin archivo devuelve los valores de fábrica, las
claves desconocidas o los valores mal escritos se ignoran (el orden incompleto,
repetido o mal escrito conserva los válidos en su orden y añade los que falten
en el orden de fábrica, sin fallar), los números se
limitan a su rango y la escritura es atómica (borrador y renombre). El detalle
vive dividido: `preferences_dock.rs` (dock) y `preferences_menubar.rs` (barra).

**No hace:** no decide qué se fija.
