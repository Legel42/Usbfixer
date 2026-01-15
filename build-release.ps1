# Script de compilation Release - USB Fixer
# Par Angel Virion

Write-Host "=== USB Fixer - Build Release ===" -ForegroundColor Cyan
Write-Host ""

# Vérifie les prérequis
Write-Host "[1/4] Vérification des prérequis..." -ForegroundColor Yellow
if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    Write-Host "✗ npm n'est pas installé!" -ForegroundColor Red
    exit 1
}
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "✗ Rust/Cargo n'est pas installé!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Prérequis OK" -ForegroundColor Green
Write-Host ""

# Build
Write-Host "[2/4] Compilation de l'application..." -ForegroundColor Yellow
npm run tauri build
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Échec de la compilation!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Compilation réussie" -ForegroundColor Green
Write-Host ""

# Copie l'exécutable
Write-Host "[3/4] Copie de l'exécutable..." -ForegroundColor Yellow
$version = "1.0.0"
$exeName = "USB-Fixer-v$version.exe"
$sourcePath = "src-tauri\target\release\USB Fixer.exe"
$destPath = $exeName

if (Test-Path $sourcePath) {
    Copy-Item $sourcePath $destPath -Force
    $size = [math]::Round((Get-Item $destPath).Length / 1MB, 2)
    Write-Host "✓ Exécutable copié: $exeName ($size MB)" -ForegroundColor Green
} else {
    Write-Host "✗ Exécutable source introuvable!" -ForegroundColor Red
    exit 1
}
Write-Host ""

# Résumé
Write-Host "[4/4] Résumé" -ForegroundColor Yellow
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host "✓ Build terminé avec succès!" -ForegroundColor Green
Write-Host ""
Write-Host "Fichier release:" -ForegroundColor Cyan
Write-Host "  → $exeName" -ForegroundColor White
Write-Host ""
Write-Host "Vous pouvez maintenant distribuer cet exécutable." -ForegroundColor Yellow
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host ""
Write-Host "Appuyez sur Entrée pour quitter..."
Read-Host
