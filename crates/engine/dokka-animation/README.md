# dokka-animation

**Hace:** calcula movimientos en el tiempo: resorte sin rebote, salto de lanzamiento
y curvas de frenada (`ease_out_cubic`) y rebote leve (`ease_out_back`).
El embudo del efecto genio (`genie.rs`): curvas `bend`/`slide`, tiempos del
vuelo y reparto en tiras, todo puro a partir del tiempo transcurrido.

**No hace:** no dibuja, no mide pantallas ni sabe qué es un dock. No usa reloj propio: recibe el tiempo transcurrido.
