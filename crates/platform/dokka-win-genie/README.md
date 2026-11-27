# dokka-win-genie

**Hace:** efecto genio al minimizar y restaurar hacia el dock: foto de la
ventana en un hilo aparte, miniatura DWM fija, tiras de composición sobre una
ventana superpuesta que no recibe clics y apagado temporal de las
transiciones de las demás ventanas. Falla sola: si algo falla, la ventana se
minimiza o restaura como siempre.

**No hace:** no decide cuándo minimizar ni dónde está el hueco del dock (lo
pide el shell con rectángulos en píxeles de pantalla); no guarda preferencias
ni dibuja el dock. Solo vive mientras dura un vuelo.
