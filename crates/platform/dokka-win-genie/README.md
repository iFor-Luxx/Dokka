# dokka-win-genie

**Hace:** efecto genio al minimizar y restaurar hacia el dock: tiras de composición con la imagen
viva de la ventana (`dokka-win-thumbnail`) sobre una ventana superpuesta que no recibe clics, y
apagado temporal de las transiciones de las demás ventanas. Falla sola: si algo falla, la ventana se
minimiza o restaura sin efecto.

**No hace:** no decide cuándo minimizar ni dónde está el hueco del dock (lo pide el shell con
rectángulos en píxeles de pantalla); no copia la pantalla, no guarda imágenes ni preferencias. Solo
trabaja mientras dura un vuelo.

**Vuelo:** el shell avisa la ventana al empezar a minimizar y las tiras aparecen donde estaba
(`from`, entera con bordes invisibles; `frame`, lo visible, que es lo que vuela; maximizadas desde su
área de trabajo). El hueco llega después con `aim`. De cada ventana minimizada solo se recuerda de
dónde salió. El vuelo pasa tras el cristal solo con el dock oculto. Cada salida temprana escribe una
línea `genio: ...` con el motivo.

A pantalla completa (vídeos, juegos) no hay genio y esas ventanas conservan la animación de Windows.
