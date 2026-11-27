# dokka-win-genie

**Hace:** efecto genio al minimizar y restaurar hacia el dock: foto de la
ventana en un hilo aparte, miniatura DWM fija, tiras de composición sobre una
ventana superpuesta que no recibe clics y apagado temporal de las
transiciones de las demás ventanas. Falla sola: si algo falla, la ventana se
minimiza o restaura como siempre.

**No hace:** no decide cuándo minimizar ni dónde está el hueco del dock (lo
pide el shell con rectángulos en píxeles de pantalla); no guarda preferencias
ni dibuja el dock. Solo vive mientras dura un vuelo.

**Vía inmediata:** el shell avisa la ventana concreta al empezar a minimizar
y la foto fija va primero (`from` entera con bordes para la miniatura DWM,
`frame` visible para copiar y volar; maximizadas desde su área de trabajo);
el hueco llega después con `aim`. El vuelo pasa tras el cristal solo con el
dock oculto. Cada salida temprana escribe una línea `genio: ...` con el
motivo; al empezar, apuntar, despegar y terminar también. Sin líneas por
fotograma.

A pantalla completa (vídeos, juegos) no hay genio y esas ventanas conservan la animación de
Windows. El registro anota lo que tarda cada paso (`genio: tiempos`).
