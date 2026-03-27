# Script de préparation de la distribution pour AstroTool-NpfCalc

# 1. Build de la version Release
Write-Host "Compilation en mode Release..."
cargo build --release

# 2. Création de l'installeur Windows (EXE via NSIS)
Write-Host "Génération de l'installeur EXE avec NSIS (via cargo-packager)..."

if (!(Get-Command cargo-packager -ErrorAction SilentlyContinue)) {
    Write-Host "Installation de cargo-packager..."
    cargo install cargo-packager
}

# Recherche de makensis (NSIS)
$nsis_path = Get-Command makensis -ErrorAction SilentlyContinue
if (!$nsis_path) {
    # Tentative de recherche dans les chemins par défaut
    $common_paths = @(
        "C:\Program Files (x86)\NSIS\makensis.exe",
        "C:\Program Files\NSIS\makensis.exe"
    )
    foreach ($path in $common_paths) {
        if (Test-Path $path) {
            $nsis_dir = Split-Path $path
            Write-Host "NSIS trouvé dans : $nsis_dir (ajout au PATH temporaire)" -ForegroundColor Cyan
            $env:Path += ";$nsis_dir"
            $nsis_path = Get-Command makensis -ErrorAction SilentlyContinue
            break
        }
    }
}

if (!$nsis_path) {
    Write-Host "`n[!] ERREUR : NSIS (makensis.exe) n'a pas été trouvé dans votre PATH." -ForegroundColor Red
    Write-Host "Pour générer l'installeur .exe, veuillez installer NSIS :"
    Write-Host "1. Téléchargez-le ici : https://nsis.sourceforge.io/Download"
    Write-Host "2. Assurez-vous que le dossier d'installation est dans votre PATH (ex: C:\Program Files (x86)\NSIS)."
    exit 1
} else {
    Write-Host "Détection de NSIS : $($nsis_path.Source)"
    cargo packager --release --formats nsis
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Succès : Installeur EXE généré." -ForegroundColor Green
    }
}

# 3. Préparation de l'archive Linux (tgz)
# Note: Cela nécessite un environnement Linux ou une cross-compilation configurée.
# Sur Windows, nous recommandons d'utiliser le workflow GitHub Actions fourni dans .github/workflows/release.yml
Write-Host "`nPour générer le .tar.gz Linux sur Windows, utilisez WSL ou le workflow GitHub Actions."
Write-Host "L'installeur Windows (.exe) se trouve dans : target\release\AstroTool-NpfCalc_0.1.0_x64-setup.exe" -ForegroundColor Green
