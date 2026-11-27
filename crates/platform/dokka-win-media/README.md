# dokka-win-media

**Hace:** volumen del dispositivo de salida predeterminado (`SystemVolume`,
contrato `VolumeControl`: lee nivel y silencio, los cambia con recorte a 0–1)
y qué está sonando en cada app (controles multimedia de Windows) con envío de
anterior, reproducir/pausar y siguiente. Cada sesión trae título y artista;
el monitor dice además la sesión actual del sistema (`current`) como
`NowPlayingInfo` para la barra. Al pedir reproducir/pausar adelanta el icono
al instante y el hilo lo confirma con una lectura real poco después; si la app
no obedece, el icono vuelve al estado real. Trabaja en su propio hilo para no
frenar la interfaz; el volumen resuelve el dispositivo en cada llamada sin
guardar nada, barato para consultar cada segundo.

**No hace:** no dibuja. No cambia de dispositivo ni usa `IPolicyConfig`. Solo
ve apps que publican sus controles a Windows. La relación entre sesión y app
se deduce del nombre del ejecutable y puede fallar en algunas.
