---
name: delegar-en-opencode
description: Flujo de trabajo de Dokka. Úsala ante cualquier pedido que cambie código (función nueva, corrección, ajuste visual, investigación): Claude entiende y supervisa, un agente de OpenCode escribe en su propio worktree, y el resultado se revisa antes de llegar a main.
---

# Delegar en OpenCode

Claude piensa, encarga y revisa. El agente escribe. El usuario prueba en `main`.

## 1. Entender antes de encargar

- Repetir en una o dos frases lo que se va a hacer. Si hay dudas reales, preguntar sin tecnicismos.
- Buscar la **causa** y las piezas afectadas. Leer lo justo.
- Definir la forma antes de programar: qué crate, qué tipos y funciones nuevas, quién los usa.
- Pensar qué más puede romper el cambio.
- Pedir aprobación **antes** de encargar si hace falta: dependencia nueva, cambio de contrato, crate nuevo o mover carpetas.

No delegar: decisiones de arquitectura, `CLAUDE.md`, esta skill ni `tools/check_rules.sh`.

## 2. Preparar la copia

Una tarea = un agente = un worktree. Nombre corto en kebab-case.

```bash
git -C /c/Users/Lux/orca/Dokka status --short      # main debe estar limpio
git -C /c/Users/Lux/orca/Dokka worktree add /c/Users/Lux/orca/.worktrees/dokka/<tarea> -b agente/<tarea>
cp /c/Users/Lux/orca/Dokka/CLAUDE.md /c/Users/Lux/orca/.worktrees/dokka/<tarea>/
```

**La copia de `CLAUDE.md` es obligatoria:** está en `.gitignore`, así que el worktree no lo trae y el agente
trabajaría sin reglas. No forzar su subida a git. `Docs/` tampoco viaja: citar en el encargo lo que haga falta.

Si `main` tiene cambios sin guardar del usuario, el agente no los verá: avisar y preguntar.

**Cola:** si la tarea toca archivos que otro agente activo está tocando, esperar. Zonas de choque habituales:
`Cargo.toml`, `apps/dokka-shell`, `crates/engine/dokka-scene`, `crates/ui/dokka-theme`.
Empezar con 2 o 3 agentes a la vez como máximo.

## 3. El encargo

Guardarlo como archivo en la carpeta temporal de la sesión (no en el repo):

```text
Lee primero CLAUDE.md y el README.md de cada crate que toques.

OBJETIVO: <qué debe pasar para el usuario>
CAUSA / CONTEXTO: <lo que ya se averiguó, con rutas y líneas>
TOCA SOLO: <archivos o crates>
NO TOQUES: <lo que queda fuera>
FORMA: <tipos, funciones, eventos y quién llama a quién>
LISTO CUANDO: <criterios comprobables leyendo el código>

REGLAS:
- Haz lo mínimo necesario; arregla la causa, no el síntoma.
- No compiles, no ejecutes, no escribas ni corras tests.
- No hagas commits, ramas ni push. No salgas de esta carpeta.
- No añadas dependencias ni crates que no estén en este encargo.
- Al terminar, resume qué cambiaste, qué no hiciste y qué dudas quedan.
```

## 4. Lanzar (en segundo plano)

Desde la carpeta del worktree, con `run_in_background`:

```bash
cd /c/Users/Lux/orca/.worktrees/dokka/<tarea> && opencode run --standalone --auto \
  -m opencode-go/muse-spark-1.3-contributor --title "<tarea>" \
  --file "<ruta del encargo>" "Cumple el encargo del archivo adjunto."
```

No sondear: la sesión avisa al terminar. Para pedir correcciones al mismo agente, añadir `--continue`.

## 5. Revisar

```bash
cd /c/Users/Lux/orca/.worktrees/dokka/<tarea>
git status --short && git diff
CARGO_TARGET_DIR=/c/Users/Lux/orca/.worktrees/dokka-target bash tools/check_rules.sh
```

- Primero el filtro; después leer **solo el diff**.
- Comprobar: cumple el objetivo, no toca de más, respeta las capas, `unsafe` justificado, sin estado global,
  sin bloquear el hilo de la interfaz.
- El informe del agente es una afirmación, no una prueba: mandar el diff.
- Como máximo **dos devoluciones**. A la tercera, corregir Claude o cambiar el encargo, y decirlo.

## 6. Llevar a main

Solo si el filtro pasa y la revisión convence.

```bash
git -C /c/Users/Lux/orca/.worktrees/dokka/<tarea> add -A
git -C /c/Users/Lux/orca/.worktrees/dokka/<tarea> commit -m "<tipo>: <qué cambia>"
git -C /c/Users/Lux/orca/Dokka merge --no-ff agente/<tarea> -m "merge: <tarea>"
git -C /c/Users/Lux/orca/Dokka worktree remove /c/Users/Lux/orca/.worktrees/dokka/<tarea>
git -C /c/Users/Lux/orca/Dokka branch -d agente/<tarea>
```

- Revisar `git status` antes de `add`: nada de secretos ni archivos ajenos a la tarea.
- Los commits terminan con la línea de atribución que indique la sesión.
- **Nunca `push`** sin que el usuario lo pida.
- Si hay conflicto, lo resuelve Claude, vuelve a pasar el filtro y lo cuenta.
- La recarga en vivo (`tools/watch_shell.ps1`) detecta el cambio sola. No lanzar ni capturar la app.

## 7. Informar

Mensaje corto: qué se hizo, qué no se comprobó y **qué debe revisar el usuario** en su pantalla.
Con varias tareas, mostrar el tablero:

| Tarea | Estado |
|---|---|
| … | en curso / en cola / lista para probar / falló |

Si el usuario no queda conforme: nuevo encargo, o `git revert -m 1 <merge>` si lo pide.
