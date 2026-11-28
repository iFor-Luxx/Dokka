# dokka-setup

**Hace:** isla de la ventana del instalador propio: pantallas de bienvenida,
opciones, avance, listo, error y desinstalación. Produce su `Scene` de
720 × 540 (desplazada por el aire de la sombra en un solo sitio) y emite
`SetupAction`; el fundido al cambiar de pantalla lo anima la propia isla en
bloque con `advance`.

**No hace:** no crea ventanas, no habla con Windows, no conoce la
implementación del contrato del instalador (`dokka-contracts` y
`dokka-win-install` los usa la app). El avance real lo hace la app en un
hilo aparte; la isla solo muestra el paso y la fracción que le fijan con
`set_progress`. La foto de fondo la dibuja la plataforma bajo la escena.
