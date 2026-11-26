# dokka-win-apps

**Hace:** obtiene el icono de un ejecutable, lo guarda en memoria y abre apps (`AppLauncher`).
También pide una app con el diálogo de Windows (`AppPicker`, en su propio hilo)
y lista el Escritorio (`DesktopCatalog` como `AppCatalog`: `.lnk` filtrados por
destino pero abiertos por el `.lnk`, `.url` solo de juegos y `.exe` sueltos).

**No hace:** no controla ventanas. Algunas apps de la tienda pueden mostrar un icono genérico o no abrirse por su ruta.
