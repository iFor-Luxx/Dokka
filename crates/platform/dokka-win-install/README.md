# dokka-win-install

**Hace:** instala y desinstala Dokka en Windows cumpliendo `Installer`
(`dokka-contracts`), sin ninguna interfaz: cierra Dokka limpio con
`WM_QUIT` para que restaure Windows, escribe `Dokka.exe` (del `payload`) y
`Desinstalar.exe` en Archivos de programa, crea los accesos directos del
menú Inicio y del escritorio, apunta el arranque con Windows y la entrada de
desinstalación en el registro, y abre Dokka sin elevación con
`explorer.exe`. Si algo falla a mitad, deshace lo ya hecho; el disco lleno y
el acceso denegado se devuelven como tales.

**No hace:** no muestra ventanas ni pide elevación (el proceso ya va
elevado), no toca la ventana, la composición ni las islas, y no borra nada
fuera de la carpeta de Dokka salvo su valor de arranque, su entrada de
desinstalación y sus accesos directos (más `%APPDATA%\Dokka` si se pide).
