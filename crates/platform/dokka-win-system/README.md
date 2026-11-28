# dokka-win-system

**Hace:** lee el resumen del equipo para "Acerca de este PC" (`SystemInfo`):
nombre del equipo, fabricante y modelo (registro `HARDWARE\DESCRIPTION\System\BIOS`),
procesador, gráfica (DXGI), memoria instalada, número de serie (tabla SMBIOS),
edición y versión de Windows (compilación >= 22000 es Windows 11 aunque el
registro diga 10), resolución de la pantalla principal y disco del sistema.
Todo en un hilo propio: la isla solo lee el último valor y pide refresco.
También abre páginas del sistema (`ms-settings:display`,
`ms-settings:storagesense`, `msinfo32`, `ms-settings:dateandtime`,
`ms-settings:regionformatting`).
Trae además fecha y hora (`datetime.rs`: automáticas por `Type`, servidor NTP,
zona automática por `tzautoupdate` y nombre de la zona) e idioma y región
(`locale.rs`: idiomas preferidos, región, calendario, temperatura y medida por
el sistema de medida, patrones y ejemplos con el formato del usuario).

**No hace:** no cambia nada del equipo, solo lee y abre páginas.
Los textos de relleno de los PC montados a mano ("System Product Name",
"To be filled by O.E.M.", "System Serial Number", "Default string"...)
se tratan como no disponibles; para el modelo se usa el de la placa base.
Nunca registra en el log el número de serie ni el nombre del equipo.
Si una lectura falla, ese campo queda en `None` y el resto sigue.
