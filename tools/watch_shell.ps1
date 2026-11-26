# Recompila y reinicia dokka-shell cada vez que se guarda un cambio.
# Detener con Ctrl+C.

$root = Split-Path $PSScriptRoot -Parent
Set-Location $root

# Resume los fuentes: cambia al editar, crear o borrar un archivo.
function Get-SourceStamp {
    $files = @(Get-ChildItem apps, crates -Recurse -File -Include *.rs, *.toml)
    $files += Get-Item Cargo.toml
    $newest = ($files | Measure-Object -Property LastWriteTimeUtc -Maximum).Maximum
    "$($files.Count)|$($newest.Ticks)"
}

function Stop-Shell($process) {
    if ($process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force
        $process.WaitForExit()
    }
}

$exe = Join-Path $root 'target\debug\dokka-shell.exe'

# Detiene los procesos sueltos de esta carpeta (tras "Reiniciar Dokka" el proceso
# nuevo ya no es el que lanzó este script y bloquearía la siguiente compilación).
# Solo toca el ejecutable de aquí: compara la ruta completa.
function Stop-StaleShells {
    $stopped = @()
    foreach ($found in Get-Process -Name 'dokka-shell' -ErrorAction SilentlyContinue) {
        try {
            if ($found.Path -and ($found.Path -eq $exe) -and -not $found.HasExited) {
                Stop-Process -Id $found.Id -Force -ErrorAction SilentlyContinue
                $stopped += $found
            }
        } catch {}
    }
    foreach ($process in $stopped) {
        try {
            $null = $process.WaitForExit(2000)
        } catch {}
    }
}

$app = $null
$lastStamp = ''

try {
    while ($true) {
        $stamp = Get-SourceStamp
        if ($stamp -ne $lastStamp) {
            $lastStamp = $stamp
            # En Windows no se puede reemplazar un ejecutable en uso.
            Stop-Shell $app
            Stop-StaleShells
            Write-Host "`n--- Compilando Dokka ---" -ForegroundColor Cyan
            cargo build -p dokka-shell
            if ($LASTEXITCODE -eq 0) {
                $app = Start-Process 'target\debug\dokka-shell.exe' -NoNewWindow -PassThru
            }
            else {
                Write-Host 'Hay un error; esperando el siguiente cambio.' -ForegroundColor Red
            }
        }
        Start-Sleep -Seconds 1
    }
}
finally {
    Stop-Shell $app
    Stop-StaleShells
}
