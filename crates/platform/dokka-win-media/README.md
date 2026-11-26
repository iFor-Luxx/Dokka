# dokka-win-media

**Hace:** consulta qué está sonando en cada app (controles multimedia de Windows) y envía
anterior, reproducir/pausar y siguiente. Al pedir reproducir/pausar adelanta el icono al instante
y el hilo lo confirma con una lectura real poco después; si la app no obedece, el icono vuelve al
estado real. Trabaja en su propio hilo para no frenar la interfaz.

**No hace:** no dibuja. Solo ve apps que publican sus controles a Windows. La relación entre sesión y app
se deduce del nombre del ejecutable y puede fallar en algunas.
