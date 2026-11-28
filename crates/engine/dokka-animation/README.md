# dokka-animation

**Hace:** calcula movimientos en el tiempo: resorte sin rebote, salto de lanzamiento,
curvas de frenada (`ease_out_cubic`) y rebote leve (`ease_out_back`), y el
progreso de aparición de un panel (`Reveal`: sube al entrar y baja al salir,
retomando desde donde está si vuelve).

**No hace:** no dibuja, no mide pantallas ni sabe qué es un dock. No usa reloj propio: recibe el tiempo transcurrido.
