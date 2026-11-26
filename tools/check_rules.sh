#!/usr/bin/env bash
# Filtro automático de las reglas de CLAUDE.md. No ejecuta tests ni lanza la app.
# Uso: bash tools/check_rules.sh [carpeta]   (por defecto, la carpeta actual)
set -u
cd "${1:-.}" || exit 2
failures=0
fail() { echo "  FALLA: $*"; failures=$((failures + 1)); }
warn() { echo "  AVISO: $*"; }

echo "== Tamaño de archivos (ideal 200, límite 300)"
while IFS= read -r file; do
    lines=$(wc -l <"$file")
    if ((lines > 300)); then
        fail "$file tiene $lines líneas"
    elif ((lines > 200)); then
        warn "$file tiene $lines líneas"
    fi
done < <(find apps crates -name '*.rs')

echo "== Nombres prohibidos"
while IFS= read -r file; do
    fail "$file: nombre demasiado genérico"
done < <(find apps crates -regextype posix-extended \
    -regex '.*/(utils|helpers|common|misc|manager)(\.rs)?')

echo "== Estilos solo en dokka-theme"
while IFS= read -r hit; do
    fail "color fuera de palette.rs: $hit"
done < <(grep -rn 'Color::rgba(' apps crates --include=*.rs |
    grep -v 'crates/ui/dokka-theme/src/palette.rs')
while IFS= read -r hit; do
    fail "fuente fuera de typography.rs: $hit"
done < <(grep -rn 'Segoe' apps crates --include=*.rs |
    grep -v 'crates/ui/dokka-theme/src/typography.rs')

echo "== Dependencias entre zonas"
zone_of() {
    local path
    path=$(find crates -mindepth 2 -maxdepth 2 -type d -name "$1")
    echo "$path" | cut -d/ -f2
}
for manifest in crates/*/*/Cargo.toml; do
    zone=$(echo "$manifest" | cut -d/ -f2)
    crate=$(echo "$manifest" | cut -d/ -f3)
    case "$zone" in
    core) allowed="core" ;;
    engine) allowed="core engine" ;;
    platform) allowed="core engine platform" ;;
    services) allowed="core services" ;;
    ui) allowed="core engine ui" ;;
    islands) allowed="core engine ui" ;;
    *) fail "zona desconocida: $zone"; continue ;;
    esac
    for dependency in $(grep -oE '^dokka-[a-z-]+' "$manifest"); do
        target=$(zone_of "$dependency")
        [[ " $allowed " == *" $target "* ]] ||
            fail "$crate ($zone) no puede usar $dependency ($target)"
    done
done

echo "== README por crate"
for crate in apps/* crates/*/*; do
    [[ -f "$crate/README.md" ]] || fail "$crate sin README.md"
done

echo "== Formato"
cargo fmt --all --check >/dev/null 2>&1 || fail "ejecutar: cargo fmt --all"

echo "== Compilación y advertencias"
if ! output=$(cargo clippy --workspace -- -D warnings 2>&1); then
    echo "$output" | tail -40
    fail "cargo clippy"
elif echo "$output" | grep -q '^warning:'; then
    echo "$output" | grep -A4 '^warning:'
    fail "advertencias de cargo"
fi

echo
if ((failures)); then
    echo "RESULTADO: $failures fallo(s)"
    exit 1
fi
echo "RESULTADO: todo en orden"
