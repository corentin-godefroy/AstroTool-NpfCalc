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
Téléchargez et lancez le fichier d'installation `.exe` (recommandé).

### Linux
Extrayez l'archive `.tar.gz` et lancez l'exécutable.

## Développement

### Compiler et distribuer (Windows)
**Prérequis :**
- [NSIS](https://nsis.sourceforge.io/) (Recommandé) : Pour générer l'installeur `.exe`.

Pour générer l'installeur, exécutez le script PowerShell suivant :
```powershell
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process; .\build_dist.ps1
```
ou simplement :
```powershell
powershell -ExecutionPolicy Bypass -File .\build_dist.ps1
```

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

## Crédits

### Bibliothèques logicielles
Ce projet utilise les bibliothèques suivantes :
- **[egui](https://github.com/emilk/egui)** : Bibliothèque d'interface utilisateur graphique immédiate.
- **[plotters](https://github.com/plotters-rs/plotters)** : Bibliothèque de traçage de graphiques Rust.
- **[fluent](https://projectfluent.org/)** : Système de localisation pour des traductions naturelles.
- **[serde](https://serde.rs/)** : Framework de sérialisation/désérialisation.

### Données astronomiques
Les données d'objets célestes (Messier, Constellations, Nébuleuses) sont basées sur des catalogues astronomiques publics.
- Catalogue de Messier
- New General Catalogue (NGC)
- Index Catalogue (IC)
