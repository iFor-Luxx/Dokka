# dokka-win-apps

**Hace:** obtiene el icono de un ejecutable, lo guarda en memoria y abre apps (`AppLauncher`).
También pide una app con el diálogo de Windows (`AppPicker`, en su propio hilo)
y lista el Escritorio (`DesktopCatalog` como `AppCatalog`: `.lnk` filtrados por
destino pero abiertos por el `.lnk`, `.url` solo de juegos y `.exe` sueltos).
Para un `.url` usa su `IconFile`/`IconIndex` en grande cuando existe.
También abre como administrador y muestra el archivo en su carpeta (`AppActions`:
un `.lnk` muestra su destino si se resuelve) y vacía la papelera con la
confirmación de Windows (`RecycleBin`, en su propio hilo con COM).
También convierte el `HICON` que una app deja en la bandeja (`tray_icon_bitmap`).

**No hace:** no controla ventanas. No elimina nada sin preguntar: vaciar la
papelera nunca quita la confirmación. Algunas apps de la tienda pueden mostrar un icono genérico o no abrirse por su ruta.
