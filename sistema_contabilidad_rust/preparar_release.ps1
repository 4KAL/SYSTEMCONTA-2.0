$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

Write-Host "Compilando release..." -ForegroundColor Cyan
cargo build --release --bin sistema-contabilidad
if ($LASTEXITCODE -ne 0) { Write-Host "Compilacion fallo." -ForegroundColor Red; exit 1 }

$r = Join-Path $PSScriptRoot "target\release"
$objetivos = @("contabilidad_rust.db", "contabilidad_rust.db-wal", "contabilidad_rust.db-shm", "migracion_reporte.txt")
foreach ($o in $objetivos) {
    $p = Join-Path $r $o
    if (Test-Path $p) { Remove-Item -Force $p; Write-Host "Eliminado: $o" -ForegroundColor Yellow }
}
$bk = Join-Path $r "backups"
if (Test-Path $bk) { Remove-Item -Recurse -Force $bk; Write-Host "Eliminado: backups\" -ForegroundColor Yellow }

Write-Host ""
Write-Host "Release listo en: $r" -ForegroundColor Green
Write-Host "Sin contabilidad_rust.db: la migracion automatica se ejecutara en el primer arranque del cliente." -ForegroundColor Green
