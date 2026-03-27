# AstroTool-NpfCalc

NPF exposure time calculator for astrophotography.

## Features
- NPF rule calculation to avoid star trailing.
- Dynamic 3D visualization of the NPF surface.
- Celestial objects database (Messier, Constellations, etc.).
- Filtering by latitude and season.
- Equipment management (Sensors and Lenses).

## Installation

### Windows
Download and run the `.exe` installer file (recommended).

### Linux
Extract the `.tar.gz` archive and run the executable.

## Development

### Build and Distribute (Windows)
**Prerequisites:**
- [NSIS](https://nsis.sourceforge.io/) (Recommended): To generate the `.exe` installer.

To generate the installer, run the following PowerShell script:
```powershell
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process; .\build_dist.ps1
```
or simply:
```powershell
powershell -ExecutionPolicy Bypass -File .\build_dist.ps1
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Credits

### Software Libraries
This project uses the following libraries:
- **[egui](https://github.com/emilk/egui)**: Immediate-mode graphical user interface library.
- **[plotters](https://github.com/plotters-rs/plotters)**: Rust plotting library.
- **[fluent](https://projectfluent.org/)**: Localization system for natural-sounding translations.
- **[serde](https://serde.rs/)**: Serialization/deserialization framework.

### Astronomical Data
Celestial object data (Messier, Constellations, Nebulae) are based on public astronomical catalogs.
- Messier Catalog
- New General Catalogue (NGC)
- Index Catalogue (IC)
