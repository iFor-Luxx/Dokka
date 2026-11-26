# dokka-win-text

**Hace:** escribe texto de una línea con DirectWrite: centrado o alineado a la izquierda,
con la fuente de Windows. Además carga una sola vez la fuente de iconos desde
memoria (`IDWriteFactory5`, cargador en memoria y colección propia) y dibuja sus
glifos centrados. Si la carga falla devuelve error.

**No hace:** no decide qué texto va ni dónde. No ajusta líneas largas. No instala la fuente
ni escribe en disco; los bytes los recibe de fuera y deben seguir vivos.
