# Compila el instalador propio y deja DokkaSetup.exe en installer/salida/.
# Se detiene con mensaje si un paso falla.
$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

function Fail($message) {
    Write-Error $message
    exit 1
}

try {
    cargo build --release -p dokka-shell
} catch {
    Fail "No se pudo compilar dokka-shell: $_"
}
if ($LASTEXITCODE -ne 0) {
    Fail "No se pudo compilar dokka-shell."
}

$targetDir = $env:CARGO_TARGET_DIR
if ([string]::IsNullOrWhiteSpace($targetDir)) {
    $targetDir = Join-Path $root 'target'
}
$shellExe = Join-Path $targetDir 'release\dokka-shell.exe'
if (-not (Test-Path $shellExe)) {
    Fail "Falta el instalable en $shellExe."
}

$env:DOKKA_PAYLOAD = $shellExe
try {
    cargo build --release -p dokka-installer
} catch {
    Fail "No se pudo compilar dokka-installer: $_"
}
if ($LASTEXITCODE -ne 0) {
    Fail "No se pudo compilar dokka-installer."
}

$outDir = Join-Path $root 'installer\salida'
New-Item -ItemType Directory -Force -Path $outDir | Out-Null
$setupExe = Join-Path $targetDir 'release\dokka-installer.exe'
Copy-Item -Force $setupExe (Join-Path $outDir 'DokkaSetup.exe')
Write-Host 'Listo: installer\salida\DokkaSetup.exe'
