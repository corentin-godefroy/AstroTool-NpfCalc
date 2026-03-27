# AstroTool-NpfCalc

Calculateur de temps de pose NPF pour l'astrophotographie.

## Fonctionnalités
- Calcul de la règle NPF pour éviter le filé d'étoiles.
- Visualisation 3D dynamique de la nappe NPF.
- Base de données d'objets célestes (Messier, Constellations, etc.).
- Filtrage par latitude et saison.
- Gestion du matériel (Capteurs et Objectifs).

## Installation

### Windows
Téléchargez et lancez le fichier d'installation `.exe` (recommandé) ou `.msi`.

### Linux
Extrayez l'archive `.tar.gz` et lancez l'exécutable.

## Développement

### Compiler et distribuer (Windows)
**Prérequis :**
- [NSIS](https://nsis.sourceforge.io/) (Recommandé) : Pour générer l'installeur `.exe`.
- [WiX Toolset v4+](https://wixtoolset.org/) : Pour générer l'installeur `.msi`.

Pour générer l'installeur, exécutez le script PowerShell suivant :
```powershell
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process; .\build_dist.ps1
```
ou simplement :
```powershell
powershell -ExecutionPolicy Bypass -File .\build_dist.ps1
```
