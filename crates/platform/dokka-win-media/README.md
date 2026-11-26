# dokka-win-media

**Hace:** consulta qué está sonando en cada app (controles multimedia de Windows) y envía
anterior, reproducir/pausar y siguiente. Trabaja en su propio hilo para no frenar la interfaz.

**No hace:** no dibuja. Solo ve apps que publican sus controles a Windows. La relación entre sesión y app
se deduce del nombre del ejecutable y puede fallar en algunas.
