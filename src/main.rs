#![windows_subsystem = "windows"]

use eframe::egui;
use once_cell::sync::Lazy;
use plotters::prelude::*;
use serde::{Deserialize, Serialize};

// --- CONFIGURATION PAR DÉFAUT ---
const DEFAULT_K_FACTOR: f64 = 1.0;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SensorConfig {
    pub name: String,
    pub pixel_size: f64,
}

impl Default for SensorConfig {
    fn default() -> Self {
        Self {
            name: "Canon 6D".to_string(),
            pixel_size: 6.54,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LensConfig {
    pub name: String,
    pub f_min: f64,
    pub f_max: f64,
    pub n_min: f64,
    pub n_max: f64,
}

impl LensConfig {
    fn get_n(&self, f: f64) -> f64 {
        if self.f_max > self.f_min + 1e-6 {
            self.n_min + (self.n_max - self.n_min) * (f - self.f_min) / (self.f_max - self.f_min)
        } else {
            self.n_min
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppSettings {
    sensors: Vec<SensorConfig>,
    lenses: Vec<LensConfig>,
    selected_sensor_idx: usize,
    selected_lens_idx: usize,
    latitude: f64,
    selected_season: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            sensors: vec![SensorConfig::default()],
            lenses: vec![
                LensConfig {
                    name: "Samyang 14mm".to_string(),
                    f_min: 13.9,
                    f_max: 14.1,
                    n_min: 2.8,
                    n_max: 2.8,
                },
                LensConfig {
                    name: "Canon 70-300mm".to_string(),
                    f_min: 70.0,
                    f_max: 300.0,
                    n_min: 4.0,
                    n_max: 5.6,
                },
            ],
            selected_sensor_idx: 0,
            selected_lens_idx: 0,
            latitude: 45.0,
            selected_season: "Toutes".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetType {
    Constellation,
    Messier,
    Nebula,   // Pour les objets hors-Messier (California, Barnard...)
    Galaxy,   // Pour les cibles spécifiques (LMC, SMC)
    Cluster,  // Pour les amas hors-Messier (Double Amas, 47 Tuc)
}

#[derive(Debug, Clone)]
pub struct Target {
    pub target_type: TargetType,
    pub id: Option<&'static str>,
    pub nom: &'static str,
    pub latin: &'static str,
    pub abbr: Option<&'static str>,
    pub parent: Option<&'static str>,
    pub saison: &'static str,
    pub dec: f64,
}

pub static TARGETS: Lazy<Vec<Target>> = Lazy::new(|| {
    vec![
        Target { target_type: TargetType::Constellation, id: None, nom: "Andromède", latin: "Andromeda", abbr: Some("And"), parent: None, saison: "Automne", dec: 37.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Aigle", latin: "Aquila", abbr: Some("Aql"), parent: None, saison: "Été", dec: 3.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Verseau", latin: "Aquarius", abbr: Some("Aqr"), parent: None, saison: "Automne", dec: -10.0 }, // Sud
        Target { target_type: TargetType::Constellation, id: None, nom: "Bélier", latin: "Aries", abbr: Some("Ari"), parent: None, saison: "Automne", dec: 20.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Cocher", latin: "Auriga", abbr: Some("Aur"), parent: None, saison: "Hiver", dec: 42.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Bouvier", latin: "Bootes", abbr: Some("Boo"), parent: None, saison: "Printemps", dec: 28.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Girafe", latin: "Camelopardalis", abbr: Some("Cam"), parent: None, saison: "Circumpolaire N", dec: 70.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Cancer", latin: "Cancer", abbr: Some("Cnc"), parent: None, saison: "Printemps", dec: 20.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Chiens de Chasse", latin: "Canes Venatici", abbr: Some("CVn"), parent: None, saison: "Printemps", dec: 40.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Petit Chien", latin: "Canis Minor", abbr: Some("CMi"), parent: None, saison: "Hiver", dec: 5.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Cassiopée", latin: "Cassiopeia", abbr: Some("Cas"), parent: None, saison: "Circumpolaire N", dec: 60.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Céphée", latin: "Cepheus", abbr: Some("Cep"), parent: None, saison: "Circumpolaire N", dec: 70.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Chevelure de Bérénice", latin: "Coma Berenices", abbr: Some("Com"), parent: None, saison: "Printemps", dec: 23.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Couronne Boréale", latin: "Corona Borealis", abbr: Some("CrB"), parent: None, saison: "Printemps", dec: 30.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Cygne", latin: "Cygnus", abbr: Some("Cyg"), parent: None, saison: "Été", dec: 42.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Dauphin", latin: "Delphinus", abbr: Some("Del"), parent: None, saison: "Été", dec: 12.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Dragon", latin: "Draco", abbr: Some("Dra"), parent: None, saison: "Circumpolaire N", dec: 65.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Petit Cheval", latin: "Equuleus", abbr: Some("Equ"), parent: None, saison: "Automne", dec: 7.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Gémeaux", latin: "Gemini", abbr: Some("Gem"), parent: None, saison: "Hiver", dec: 22.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Hercule", latin: "Hercules", abbr: Some("Her"), parent: None, saison: "Été", dec: 27.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Lézard", latin: "Lacerta", abbr: Some("Lac"), parent: None, saison: "Automne", dec: 45.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Lion", latin: "Leo", abbr: Some("Leo"), parent: None, saison: "Printemps", dec: 15.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Petit Lion", latin: "Leo Minor", abbr: Some("LMi"), parent: None, saison: "Printemps", dec: 35.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Lynx", latin: "Lynx", abbr: Some("Lyn"), parent: None, saison: "Hiver", dec: 45.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Lyre", latin: "Lyra", abbr: Some("Lyr"), parent: None, saison: "Été", dec: 38.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Licorne", latin: "Monoceros", abbr: Some("Mon"), parent: None, saison: "Hiver", dec: -3.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Ophiuchus", latin: "Ophiuchus", abbr: Some("Oph"), parent: None, saison: "Été", dec: -7.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Orion", latin: "Orion", abbr: Some("Ori"), parent: None, saison: "Hiver", dec: 5.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Pégase", latin: "Pegasus", abbr: Some("Peg"), parent: None, saison: "Automne", dec: 20.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Persée", latin: "Perseus", abbr: Some("Per"), parent: None, saison: "Automne", dec: 45.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Poissons", latin: "Pisces", abbr: Some("Psc"), parent: None, saison: "Automne", dec: 15.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Flèche", latin: "Sagitta", abbr: Some("Sge"), parent: None, saison: "Été", dec: 18.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Serpent", latin: "Serpens", abbr: Some("Ser"), parent: None, saison: "Été", dec: 0.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Sextant", latin: "Sextans", abbr: Some("Sex"), parent: None, saison: "Printemps", dec: -2.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Taureau", latin: "Taurus", abbr: Some("Tau"), parent: None, saison: "Hiver", dec: 16.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Triangle", latin: "Triangulum", abbr: Some("Tri"), parent: None, saison: "Automne", dec: 32.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Grande Ourse", latin: "Ursa Major", abbr: Some("UMa"), parent: None, saison: "Circumpolaire N", dec: 50.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Petite Ourse", latin: "Ursa Minor", abbr: Some("UMi"), parent: None, saison: "Circumpolaire N", dec: 75.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Vierge", latin: "Virgo", abbr: Some("Vir"), parent: None, saison: "Printemps", dec: 0.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Petit Renard", latin: "Vulpecula", abbr: Some("Vul"), parent: None, saison: "Été", dec: 25.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Machine Pneumatique", latin: "Antlia", abbr: Some("Ant"), parent: None, saison: "Printemps", dec: -35.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Oiseau de Paradis", latin: "Apus", abbr: Some("Aps"), parent: None, saison: "Circumpolaire S", dec: -75.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Autel", latin: "Ara", abbr: Some("Ara"), parent: None, saison: "Été", dec: -53.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Burin", latin: "Caelum", abbr: Some("Cae"), parent: None, saison: "Hiver", dec: -38.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Grand Chien", latin: "Canis Major", abbr: Some("CMa"), parent: None, saison: "Hiver", dec: -22.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Capricorne", latin: "Capricornus", abbr: Some("Cap"), parent: None, saison: "Automne", dec: -20.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Carène", latin: "Carina", abbr: Some("Car"), parent: None, saison: "Circumpolaire S", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Centaure", latin: "Centaurus", abbr: Some("Cen"), parent: None, saison: "Printemps", dec: -45.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Baleine", latin: "Cetus", abbr: Some("Cet"), parent: None, saison: "Automne", dec: -8.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Caméléon", latin: "Chamaeleon", abbr: Some("Cha"), parent: None, saison: "Circumpolaire S", dec: -75.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Circin", latin: "Circinus", abbr: Some("Cir"), parent: None, saison: "Circumpolaire S", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Colombe", latin: "Columba", abbr: Some("Col"), parent: None, saison: "Hiver", dec: -35.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Couronne Australe", latin: "Corona Australis", abbr: Some("CrA"), parent: None, saison: "Été", dec: -40.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Corbeau", latin: "Corvus", abbr: Some("Crv"), parent: None, saison: "Printemps", dec: -18.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Coupe", latin: "Crater", abbr: Some("Crt"), parent: None, saison: "Printemps", dec: -15.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Croix du Sud", latin: "Crux", abbr: Some("Cru"), parent: None, saison: "Circumpolaire S", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Dorade", latin: "Dorado", abbr: Some("Dor"), parent: None, saison: "Circumpolaire S", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Éridan", latin: "Eridanus", abbr: Some("Eri"), parent: None, saison: "Hiver", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Fourneau", latin: "Fornax", abbr: Some("For"), parent: None, saison: "Automne", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Grue", latin: "Grus", abbr: Some("Gru"), parent: None, saison: "Automne", dec: -45.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Horloge", latin: "Horologium", abbr: Some("Hor"), parent: None, saison: "Hiver", dec: -50.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Hydre", latin: "Hydra", abbr: Some("Hya"), parent: None, saison: "Printemps", dec: -20.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Hydre Mâle", latin: "Hydrus", abbr: Some("Hyi"), parent: None, saison: "Circumpolaire S", dec: -70.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Indien", latin: "Indus", abbr: Some("Ind"), parent: None, saison: "Été", dec: -55.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Lièvre", latin: "Lepus", abbr: Some("Lep"), parent: None, saison: "Hiver", dec: -18.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Balance", latin: "Libra", abbr: Some("Lib"), parent: None, saison: "Printemps", dec: -15.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Loup", latin: "Lupus", abbr: Some("Lup"), parent: None, saison: "Été", dec: -40.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Table", latin: "Mensa", abbr: Some("Men"), parent: None, saison: "Circumpolaire S", dec: -75.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Microscope", latin: "Microscopium", abbr: Some("Mic"), parent: None, saison: "Automne", dec: -35.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Mouche", latin: "Musca", abbr: Some("Mus"), parent: None, saison: "Circumpolaire S", dec: -70.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Règle", latin: "Norma", abbr: Some("Nor"), parent: None, saison: "Été", dec: -50.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Octant", latin: "Octans", abbr: Some("Oct"), parent: None, saison: "Circumpolaire S", dec: -85.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Paon", latin: "Pavo", abbr: Some("Pav"), parent: None, saison: "Circumpolaire S", dec: -65.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Phénix", latin: "Phoenix", abbr: Some("Phe"), parent: None, saison: "Automne", dec: -45.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Peintre", latin: "Pictor", abbr: Some("Pic"), parent: None, saison: "Hiver", dec: -55.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Poisson Austral", latin: "Piscis Austrinus", abbr: Some("PsA"), parent: None, saison: "Automne", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Poupe", latin: "Puppis", abbr: Some("Pup"), parent: None, saison: "Hiver", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Boussole", latin: "Pyxis", abbr: Some("Pyx"), parent: None, saison: "Printemps", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Sagittaire", latin: "Sagittarius", abbr: Some("Sgr"), parent: None, saison: "Été", dec: -25.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Scorpion", latin: "Scorpius", abbr: Some("Sco"), parent: None, saison: "Été", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Sculpteur", latin: "Sculptor", abbr: Some("Scl"), parent: None, saison: "Automne", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Écu de Sobieski", latin: "Scutum", abbr: Some("Sct"), parent: None, saison: "Été", dec: -10.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Télescope", latin: "Telescopium", abbr: Some("Tel"), parent: None, saison: "Été", dec: -50.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Triangle Austral", latin: "Triangulum Australe", abbr: Some("TrA"), parent: None, saison: "Circumpolaire S", dec: -65.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Toucan", latin: "Tucana", abbr: Some("Tuc"), parent: None, saison: "Circumpolaire S", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Voiles", latin: "Vela", abbr: Some("Vel"), parent: None, saison: "Printemps", dec: -50.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Poisson Volant", latin: "Volans", abbr: Some("Vol"), parent: None, saison: "Circumpolaire S", dec: -70.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Réticule", latin: "Reticulum", abbr: Some("Ret"), parent: None, saison: "Circumpolaire S", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, nom: "Burin", latin: "Caelum", abbr: Some("Cae"), parent: None, saison: "Hiver", dec: -38.0 },

        Target { target_type: TargetType::Messier, id: Some("M1"), nom: "Nébuleuse du Crabe", latin: "Taurus", abbr: None, parent: Some("Taureau"), saison: "Hiver", dec: 22.0 },
        Target { target_type: TargetType::Messier, id: Some("M2"), nom: "Amas du Verseau", latin: "Aquarius", abbr: None, parent: Some("Verseau"), saison: "Automne", dec: -0.8 },
        Target { target_type: TargetType::Messier, id: Some("M3"), nom: "Amas globulaire", latin: "Canes Venatici", abbr: None, parent: Some("Chiens de Chasse"), saison: "Printemps", dec: 28.4 },
        Target { target_type: TargetType::Messier, id: Some("M4"), nom: "Amas du Scorpion", latin: "Scorpius", abbr: None, parent: Some("Scorpion"), saison: "Été", dec: -26.5 },
        Target { target_type: TargetType::Messier, id: Some("M5"), nom: "Amas du Serpent", latin: "Serpens", abbr: None, parent: Some("Serpent"), saison: "Printemps", dec: 2.1 },
        Target { target_type: TargetType::Messier, id: Some("M6"), nom: "Amas du Papillon", latin: "Scorpius", abbr: None, parent: Some("Scorpion"), saison: "Été", dec: -32.2 },
        Target { target_type: TargetType::Messier, id: Some("M7"), nom: "Amas de Ptolémée", latin: "Scorpius", abbr: None, parent: Some("Scorpion"), saison: "Été", dec: -34.8 },
        Target { target_type: TargetType::Messier, id: Some("M8"), nom: "Nébuleuse de la Lagune", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -24.4 },
        Target { target_type: TargetType::Messier, id: Some("M9"), nom: "Amas globulaire", latin: "Ophiuchus", abbr: None, parent: Some("Ophiuchus"), saison: "Été", dec: -18.5 },
        Target { target_type: TargetType::Messier, id: Some("M10"), nom: "Amas globulaire", latin: "Ophiuchus", abbr: None, parent: Some("Ophiuchus"), saison: "Été", dec: -4.1 },

        Target { target_type: TargetType::Messier, id: Some("M11"), nom: "Amas du Canard Sauvage", latin: "Scutum", abbr: None, parent: Some("Écu de Sobieski"), saison: "Été", dec: -6.3 },
        Target { target_type: TargetType::Messier, id: Some("M12"), nom: "Amas globulaire", latin: "Ophiuchus", abbr: None, parent: Some("Ophiuchus"), saison: "Été", dec: -2.0 },
        Target { target_type: TargetType::Messier, id: Some("M13"), nom: "Grand Amas d'Hercule", latin: "Hercules", abbr: None, parent: Some("Hercule"), saison: "Été", dec: 36.5 },
        Target { target_type: TargetType::Messier, id: Some("M14"), nom: "Amas globulaire", latin: "Ophiuchus", abbr: None, parent: Some("Ophiuchus"), saison: "Été", dec: -3.2 },
        Target { target_type: TargetType::Messier, id: Some("M15"), nom: "Amas de Pégase", latin: "Pegasus", abbr: None, parent: Some("Pégase"), saison: "Automne", dec: 12.2 },
        Target { target_type: TargetType::Messier, id: Some("M16"), nom: "Nébuleuse de l'Aigle", latin: "Serpens", abbr: None, parent: Some("Serpent"), saison: "Été", dec: -13.8 },
        Target { target_type: TargetType::Messier, id: Some("M17"), nom: "Nébuleuse Oméga", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -16.2 },
        Target { target_type: TargetType::Messier, id: Some("M18"), nom: "Amas ouvert", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -17.1 },
        Target { target_type: TargetType::Messier, id: Some("M19"), nom: "Amas globulaire", latin: "Ophiuchus", abbr: None, parent: Some("Ophiuchus"), saison: "Été", dec: -26.3 },
        Target { target_type: TargetType::Messier, id: Some("M20"), nom: "Nébuleuse Trifide", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -23.0 },

        Target { target_type: TargetType::Messier, id: Some("M21"), nom: "Amas ouvert", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -22.5 },
        Target { target_type: TargetType::Messier, id: Some("M22"), nom: "Grand Amas du Sagittaire", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -23.9 },
        Target { target_type: TargetType::Messier, id: Some("M23"), nom: "Amas ouvert", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -19.0 },
        Target { target_type: TargetType::Messier, id: Some("M24"), nom: "Nuage stellaire du Sagittaire", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -18.4 },
        Target { target_type: TargetType::Messier, id: Some("M25"), nom: "Amas ouvert", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -19.1 },
        Target { target_type: TargetType::Messier, id: Some("M26"), nom: "Amas ouvert", latin: "Scutum", abbr: None, parent: Some("Écu de Sobieski"), saison: "Été", dec: -9.4 },
        Target { target_type: TargetType::Messier, id: Some("M27"), nom: "Nébuleuse Dumbbell", latin: "Vulpecula", abbr: None, parent: Some("Petit Renard"), saison: "Été", dec: 22.7 },
        Target { target_type: TargetType::Messier, id: Some("M28"), nom: "Amas globulaire", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -24.9 },
        Target { target_type: TargetType::Messier, id: Some("M29"), nom: "Amas ouvert", latin: "Cygnus", abbr: None, parent: Some("Cygne"), saison: "Été", dec: 38.5 },
        Target { target_type: TargetType::Messier, id: Some("M30"), nom: "Amas du Capricorne", latin: "Capricornus", abbr: None, parent: Some("Capricorne"), saison: "Automne", dec: -23.2 },

        Target { target_type: TargetType::Messier, id: Some("M31"), nom: "Galaxie d'Andromède", latin: "Andromeda", abbr: None, parent: Some("Andromède"), saison: "Automne", dec: 41.3 },
        Target { target_type: TargetType::Messier, id: Some("M32"), nom: "Galaxie satellite d'Andromède", latin: "Andromeda", abbr: None, parent: Some("Andromède"), saison: "Automne", dec: 40.9 },
        Target { target_type: TargetType::Messier, id: Some("M33"), nom: "Galaxie du Triangle", latin: "Triangulum", abbr: None, parent: Some("Triangle"), saison: "Automne", dec: 30.7 },
        Target { target_type: TargetType::Messier, id: Some("M34"), nom: "Amas ouvert", latin: "Perseus", abbr: None, parent: Some("Persée"), saison: "Automne", dec: 42.8 },
        Target { target_type: TargetType::Messier, id: Some("M35"), nom: "Amas ouvert", latin: "Gemini", abbr: None, parent: Some("Gémeaux"), saison: "Hiver", dec: 24.3 },
        Target { target_type: TargetType::Messier, id: Some("M36"), nom: "Amas ouvert", latin: "Auriga", abbr: None, parent: Some("Cocher"), saison: "Hiver", dec: 34.1 },
        Target { target_type: TargetType::Messier, id: Some("M37"), nom: "Amas ouvert", latin: "Auriga", abbr: None, parent: Some("Cocher"), saison: "Hiver", dec: 32.5 },
        Target { target_type: TargetType::Messier, id: Some("M38"), nom: "Amas ouvert", latin: "Auriga", abbr: None, parent: Some("Cocher"), saison: "Hiver", dec: 35.8 },
        Target { target_type: TargetType::Messier, id: Some("M39"), nom: "Amas ouvert", latin: "Cygnus", abbr: None, parent: Some("Cygne"), saison: "Été", dec: 48.4 },
        Target { target_type: TargetType::Messier, id: Some("M40"), nom: "Étoile double Winnecke 4", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 58.1 },

        Target { target_type: TargetType::Messier, id: Some("M41"), nom: "Amas ouvert", latin: "Canis Major", abbr: None, parent: Some("Grand Chien"), saison: "Hiver", dec: -20.7 },
        Target { target_type: TargetType::Messier, id: Some("M42"), nom: "Nébuleuse d'Orion", latin: "Orion", abbr: None, parent: Some("Orion"), saison: "Hiver", dec: -5.4 },
        Target { target_type: TargetType::Messier, id: Some("M43"), nom: "Nébuleuse de Mairan", latin: "Orion", abbr: None, parent: Some("Orion"), saison: "Hiver", dec: -5.3 },
        Target { target_type: TargetType::Messier, id: Some("M44"), nom: "Amas de la Crèche", latin: "Cancer", abbr: None, parent: Some("Cancer"), saison: "Printemps", dec: 19.7 },
        Target { target_type: TargetType::Messier, id: Some("M45"), nom: "Les Pléiades", latin: "Taurus", abbr: None, parent: Some("Taureau"), saison: "Hiver", dec: 24.1 },
        Target { target_type: TargetType::Messier, id: Some("M46"), nom: "Amas ouvert", latin: "Puppis", abbr: None, parent: Some("Poupe"), saison: "Hiver", dec: -14.8 },
        Target { target_type: TargetType::Messier, id: Some("M47"), nom: "Amas ouvert", latin: "Puppis", abbr: None, parent: Some("Poupe"), saison: "Hiver", dec: -14.4 },
        Target { target_type: TargetType::Messier, id: Some("M48"), nom: "Amas ouvert", latin: "Hydra", abbr: None, parent: Some("Hydre"), saison: "Hiver", dec: -5.8 },
        Target { target_type: TargetType::Messier, id: Some("M49"), nom: "Galaxie elliptique", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 8.0 },
        Target { target_type: TargetType::Messier, id: Some("M50"), nom: "Amas ouvert", latin: "Monoceros", abbr: None, parent: Some("Licorne"), saison: "Hiver", dec: -8.3 },

        Target { target_type: TargetType::Messier, id: Some("M51"), nom: "Galaxie du Tourbillon", latin: "Canes Venatici", abbr: None, parent: Some("Chiens de Chasse"), saison: "Printemps", dec: 47.2 },
        Target { target_type: TargetType::Messier, id: Some("M52"), nom: "Amas ouvert", latin: "Cassiopeia", abbr: None, parent: Some("Cassiopée"), saison: "Automne", dec: 61.6 },
        Target { target_type: TargetType::Messier, id: Some("M53"), nom: "Amas globulaire", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 18.2 },
        Target { target_type: TargetType::Messier, id: Some("M54"), nom: "Amas globulaire", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -30.5 },
        Target { target_type: TargetType::Messier, id: Some("M55"), nom: "Amas globulaire", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -31.0 },
        Target { target_type: TargetType::Messier, id: Some("M56"), nom: "Amas globulaire", latin: "Lyra", abbr: None, parent: Some("Lyre"), saison: "Été", dec: 30.2 },
        Target { target_type: TargetType::Messier, id: Some("M57"), nom: "Nébuleuse de l'Anneau", latin: "Lyra", abbr: None, parent: Some("Lyre"), saison: "Été", dec: 33.0 },
        Target { target_type: TargetType::Messier, id: Some("M58"), nom: "Galaxie spirale", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 11.8 },
        Target { target_type: TargetType::Messier, id: Some("M59"), nom: "Galaxie elliptique", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 11.6 },
        Target { target_type: TargetType::Messier, id: Some("M60"), nom: "Galaxie elliptique", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 11.5 },

        Target { target_type: TargetType::Messier, id: Some("M61"), nom: "Galaxie spirale", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 4.5 },
        Target { target_type: TargetType::Messier, id: Some("M62"), nom: "Amas globulaire", latin: "Ophiuchus", abbr: None, parent: Some("Ophiuchus"), saison: "Été", dec: -30.1 },
        Target { target_type: TargetType::Messier, id: Some("M63"), nom: "Galaxie du Tournesol", latin: "Canes Venatici", abbr: None, parent: Some("Chiens de Chasse"), saison: "Printemps", dec: 42.0 },
        Target { target_type: TargetType::Messier, id: Some("M64"), nom: "Galaxie de l'Oeil Noir", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 21.7 },
        Target { target_type: TargetType::Messier, id: Some("M65"), nom: "Galaxie du Lion", latin: "Leo", abbr: None, parent: Some("Lion"), saison: "Printemps", dec: 13.1 },
        Target { target_type: TargetType::Messier, id: Some("M66"), nom: "Galaxie du Lion", latin: "Leo", abbr: None, parent: Some("Lion"), saison: "Printemps", dec: 13.0 },
        Target { target_type: TargetType::Messier, id: Some("M67"), nom: "Amas ouvert", latin: "Cancer", abbr: None, parent: Some("Cancer"), saison: "Printemps", dec: 11.8 },
        Target { target_type: TargetType::Messier, id: Some("M68"), nom: "Amas globulaire", latin: "Hydra", abbr: None, parent: Some("Hydre"), saison: "Printemps", dec: -26.7 },
        Target { target_type: TargetType::Messier, id: Some("M69"), nom: "Amas globulaire", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -32.3 },
        Target { target_type: TargetType::Messier, id: Some("M70"), nom: "Amas globulaire", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -32.3 },

        Target { target_type: TargetType::Messier, id: Some("M71"), nom: "Amas globulaire", latin: "Sagitta", abbr: None, parent: Some("Flèche"), saison: "Été", dec: 18.8 },
        Target { target_type: TargetType::Messier, id: Some("M72"), nom: "Amas globulaire", latin: "Aquarius", abbr: None, parent: Some("Verseau"), saison: "Automne", dec: -12.5 },
        Target { target_type: TargetType::Messier, id: Some("M73"), nom: "Astérisme", latin: "Aquarius", abbr: None, parent: Some("Verseau"), saison: "Automne", dec: -12.6 },
        Target { target_type: TargetType::Messier, id: Some("M74"), nom: "Galaxie spirale", latin: "Pisces", abbr: None, parent: Some("Poissons"), saison: "Automne", dec: 15.8 },
        Target { target_type: TargetType::Messier, id: Some("M75"), nom: "Amas globulaire", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -21.9 },
        Target { target_type: TargetType::Messier, id: Some("M76"), nom: "Petit Haltère", latin: "Perseus", abbr: None, parent: Some("Persée"), saison: "Automne", dec: 51.6 },
        Target { target_type: TargetType::Messier, id: Some("M77"), nom: "Galaxie spirale", latin: "Cetus", abbr: None, parent: Some("Baleine"), saison: "Automne", dec: -0.0 },
        Target { target_type: TargetType::Messier, id: Some("M78"), nom: "Nébuleuse par réflexion", latin: "Orion", abbr: None, parent: Some("Orion"), saison: "Hiver", dec: 0.1 },
        Target { target_type: TargetType::Messier, id: Some("M79"), nom: "Amas globulaire", latin: "Lepus", abbr: None, parent: Some("Lièvre"), saison: "Hiver", dec: -24.5 },
        Target { target_type: TargetType::Messier, id: Some("M80"), nom: "Amas globulaire", latin: "Scorpius", abbr: None, parent: Some("Scorpion"), saison: "Été", dec: -23.0 },

        Target { target_type: TargetType::Messier, id: Some("M81"), nom: "Galaxie de Bode", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 69.1 },
        Target { target_type: TargetType::Messier, id: Some("M82"), nom: "Galaxie du Cigare", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 69.7 },
        Target { target_type: TargetType::Messier, id: Some("M83"), nom: "Galaxie du Moulinet Austral", latin: "Hydra", abbr: None, parent: Some("Hydre"), saison: "Printemps", dec: -29.9 },
        Target { target_type: TargetType::Messier, id: Some("M84"), nom: "Galaxie lenticulaire", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 12.9 },
        Target { target_type: TargetType::Messier, id: Some("M85"), nom: "Galaxie lenticulaire", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 18.2 },
        Target { target_type: TargetType::Messier, id: Some("M86"), nom: "Galaxie lenticulaire", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 12.9 },
        Target { target_type: TargetType::Messier, id: Some("M87"), nom: "Virgo A", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 12.4 },
        Target { target_type: TargetType::Messier, id: Some("M88"), nom: "Galaxie spirale", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 14.4 },
        Target { target_type: TargetType::Messier, id: Some("M89"), nom: "Galaxie elliptique", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 12.6 },
        Target { target_type: TargetType::Messier, id: Some("M90"), nom: "Galaxie spirale", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 13.2 },

        Target { target_type: TargetType::Messier, id: Some("M91"), nom: "Galaxie spirale", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 14.5 },
        Target { target_type: TargetType::Messier, id: Some("M92"), nom: "Amas globulaire", latin: "Hercules", abbr: None, parent: Some("Hercule"), saison: "Été", dec: 43.1 },
        Target { target_type: TargetType::Messier, id: Some("M93"), nom: "Amas ouvert", latin: "Puppis", abbr: None, parent: Some("Poupe"), saison: "Hiver", dec: -23.8 },
        Target { target_type: TargetType::Messier, id: Some("M94"), nom: "Galaxie de l'Oeil de Croco", latin: "Canes Venatici", abbr: None, parent: Some("Chiens de Chasse"), saison: "Printemps", dec: 41.1 },
        Target { target_type: TargetType::Messier, id: Some("M95"), nom: "Galaxie spirale", latin: "Leo", abbr: None, parent: Some("Lion"), saison: "Printemps", dec: 11.7 },
        Target { target_type: TargetType::Messier, id: Some("M96"), nom: "Galaxie spirale", latin: "Leo", abbr: None, parent: Some("Lion"), saison: "Printemps", dec: 11.8 },
        Target { target_type: TargetType::Messier, id: Some("M97"), nom: "Nébuleuse du Hibou", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 55.0 },
        Target { target_type: TargetType::Messier, id: Some("M98"), nom: "Galaxie spirale", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 14.9 },
        Target { target_type: TargetType::Messier, id: Some("M99"), nom: "Galaxie spirale", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 14.4 },
        Target { target_type: TargetType::Messier, id: Some("M100"), nom: "Galaxie spirale", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 15.8 },

        Target { target_type: TargetType::Messier, id: Some("M101"), nom: "Galaxie du Moulinet", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 54.4 },
        Target { target_type: TargetType::Messier, id: Some("M102"), nom: "Galaxie du Fuseau", latin: "Draco", abbr: None, parent: Some("Dragon"), saison: "Circumpolaire N", dec: 55.8 },
        Target { target_type: TargetType::Messier, id: Some("M103"), nom: "Amas ouvert", latin: "Cassiopeia", abbr: None, parent: Some("Cassiopée"), saison: "Automne", dec: 60.7 },
        Target { target_type: TargetType::Messier, id: Some("M104"), nom: "Galaxie du Sombrero", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: -11.6 },
        Target { target_type: TargetType::Messier, id: Some("M105"), nom: "Galaxie elliptique", latin: "Leo", abbr: None, parent: Some("Lion"), saison: "Printemps", dec: 12.6 },
        Target { target_type: TargetType::Messier, id: Some("M106"), nom: "Galaxie spirale", latin: "Canes Venatici", abbr: None, parent: Some("Chiens de Chasse"), saison: "Printemps", dec: 47.3 },
        Target { target_type: TargetType::Messier, id: Some("M107"), nom: "Amas globulaire", latin: "Ophiuchus", abbr: None, parent: Some("Ophiuchus"), saison: "Été", dec: -13.0 },
        Target { target_type: TargetType::Messier, id: Some("M108"), nom: "Galaxie de la Planche de Surf", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 53.4 },
        Target { target_type: TargetType::Messier, id: Some("M109"), nom: "Galaxie spirale", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 53.4 },
        Target { target_type: TargetType::Messier, id: Some("M110"), nom: "Galaxie satellite d'Andromède", latin: "Andromeda", abbr: None, parent: Some("Andromède"), saison: "Automne", dec: 41.7 },

        Target { target_type: TargetType::Galaxy, id: Some("M31"), nom: "Galaxie d'Andromède", latin: "Andromeda", abbr: None, parent: Some("Andromède"), saison: "Automne", dec: 41.26 },
        Target { target_type: TargetType::Galaxy, id: Some("M33"), nom: "Galaxie du Triangle", latin: "Triangulum", abbr: None, parent: Some("Triangle"), saison: "Automne", dec: 30.66 },
        Target { target_type: TargetType::Galaxy, id: Some("M51"), nom: "Galaxie du Tourbillon", latin: "Canes Venatici", abbr: Some("NGC 5194"), parent: Some("Chiens de Chasse"), saison: "Printemps", dec: 47.19 },
        Target { target_type: TargetType::Galaxy, id: Some("M81"), nom: "Galaxie de Bode", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 69.06 },
        Target { target_type: TargetType::Galaxy, id: Some("M82"), nom: "Galaxie du Cigare", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 69.68 },
        Target { target_type: TargetType::Galaxy, id: Some("M101"), nom: "Galaxie du Moulinet", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 54.35 },
        Target { target_type: TargetType::Galaxy, id: Some("M63"), nom: "Galaxie du Tournesol", latin: "Canes Venatici", abbr: None, parent: Some("Chiens de Chasse"), saison: "Printemps", dec: 42.03 },
        Target { target_type: TargetType::Galaxy, id: Some("M64"), nom: "Galaxie de l'Oeil Noir", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 21.68 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 4565"), nom: "Galaxie de l'Aiguille", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 25.98 },
        Target { target_type: TargetType::Galaxy, id: Some("M104"), nom: "Galaxie du Sombrero", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: -11.62 }, // Sud

        // --- GALAXIES MAJEURES (Hémisphère Sud) ---
        Target { target_type: TargetType::Galaxy, id: Some("LMC"), nom: "Grand Nuage de Magellan", latin: "Dorado", abbr: None, parent: Some("Dorade"), saison: "Circumpolaire S", dec: -69.75 },
        Target { target_type: TargetType::Galaxy, id: Some("SMC"), nom: "Petit Nuage de Magellan", latin: "Tucana", abbr: None, parent: Some("Toucan"), saison: "Circumpolaire S", dec: -72.80 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 5128"), nom: "Centaurus A", latin: "Centaurus", abbr: None, parent: Some("Centaure"), saison: "Printemps", dec: -43.01 },
        Target { target_type: TargetType::Galaxy, id: Some("M83"), nom: "Galaxie du Moulinet Austral", latin: "Hydra", abbr: None, parent: Some("Hydre"), saison: "Printemps", dec: -29.86 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 253"), nom: "Galaxie du Sculpteur", latin: "Sculptor", abbr: None, parent: Some("Sculpteur"), saison: "Automne", dec: -25.29 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 300"), nom: "Galaxie du Sculpteur (Sud)", latin: "Sculptor", abbr: None, parent: Some("Sculpteur"), saison: "Automne", dec: -37.68 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 4945"), nom: "Galaxie de la Tresse", latin: "Centaurus", abbr: None, parent: Some("Centaure"), saison: "Printemps", dec: -49.47 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 1316"), nom: "Fornax A", latin: "Fornax", abbr: None, parent: Some("Fourneau"), saison: "Automne", dec: -37.20 },

        // --- TRIO DU LION & AUTRES GROUPES ---
        Target { target_type: TargetType::Galaxy, id: Some("M65"), nom: "Membre du Trio du Lion", latin: "Leo", abbr: None, parent: Some("Lion"), saison: "Printemps", dec: 13.09 },
        Target { target_type: TargetType::Galaxy, id: Some("M66"), nom: "Membre du Trio du Lion", latin: "Leo", abbr: None, parent: Some("Lion"), saison: "Printemps", dec: 12.99 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 3628"), nom: "Galaxie du Hamburger", latin: "Leo", abbr: None, parent: Some("Lion"), saison: "Printemps", dec: 13.59 },

        // --- CHAINES ET AMAS ---
        Target { target_type: TargetType::Galaxy, id: Some("M87"), nom: "Virgo A (Centre Amas Vierge)", latin: "Virgo", abbr: None, parent: Some("Vierge"), saison: "Printemps", dec: 12.39 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 4038"), nom: "Galaxies des Antennes", latin: "Corvus", abbr: None, parent: Some("Corbeau"), saison: "Printemps", dec: -18.87 },

        // --- HIVER (Le règne d'Orion et des licornes) ---
        Target { target_type: TargetType::Nebula, id: Some("M42"), nom: "Grande Nébuleuse d'Orion", latin: "Orion", abbr: None, parent: Some("Orion"), saison: "Hiver", dec: -5.39 },
        Target { target_type: TargetType::Nebula, id: Some("IC 434"), nom: "Tête de Cheval", latin: "Orion", abbr: None, parent: Some("Orion"), saison: "Hiver", dec: -2.45 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 2024"), nom: "Nébuleuse de la Flamme", latin: "Orion", abbr: None, parent: Some("Orion"), saison: "Hiver", dec: -1.86 },
        Target { target_type: TargetType::Nebula, id: Some("Sh2-276"), nom: "Boucle de Barnard", latin: "Orion", abbr: None, parent: Some("Orion"), saison: "Hiver", dec: -1.00 },
        Target { target_type: TargetType::Nebula, id: Some("IC 2118"), nom: "Tête de Sorcière", latin: "Eridanus", abbr: None, parent: Some("Éridan"), saison: "Hiver", dec: -7.25 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 2237"), nom: "La Rosette", latin: "Monoceros", abbr: None, parent: Some("Licorne"), saison: "Hiver", dec: 4.97 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 2264"), nom: "Le Cône / Arbre de Noël", latin: "Monoceros", abbr: None, parent: Some("Licorne"), saison: "Hiver", dec: 9.89 },
        Target { target_type: TargetType::Nebula, id: Some("IC 405"), nom: "L'Étoile Flamboyante", latin: "Auriga", abbr: None, parent: Some("Cocher"), saison: "Hiver", dec: 34.35 },
        Target { target_type: TargetType::Nebula, id: Some("IC 443"), nom: "Nébuleuse de la Méduse", latin: "Gemini", abbr: None, parent: Some("Gémeaux"), saison: "Hiver", dec: 22.47 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 2174"), nom: "Tête de Singe", latin: "Orion", abbr: None, parent: Some("Orion"), saison: "Hiver", dec: 20.50 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 1499"), nom: "Nébuleuse California", latin: "Perseus", abbr: None, parent: Some("Persée"), saison: "Hiver", dec: 36.42 },

        // --- PRINTEMPS (La saison des Galaxies, mais quelques nébuleuses du Sud) ---
        Target { target_type: TargetType::Nebula, id: Some("NGC 3372"), nom: "Grande Carène", latin: "Carina", abbr: None, parent: Some("Carène"), saison: "Printemps", dec: -59.87 },
        Target { target_type: TargetType::Nebula, id: Some("IC 2944"), nom: "Poulet qui Court", latin: "Centaurus", abbr: None, parent: Some("Centaure"), saison: "Printemps", dec: -63.02 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 3576"), nom: "Statue de la Liberté", latin: "Carina", abbr: None, parent: Some("Carène"), saison: "Printemps", dec: -61.30 },
        Target { target_type: TargetType::Nebula, id: Some("M97"), nom: "Nébuleuse du Hibou", latin: "Ursa Major", abbr: None, parent: Some("Grande Ourse"), saison: "Printemps", dec: 55.02 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 3242"), nom: "Fantôme de Jupiter", latin: "Hydra", abbr: None, parent: Some("Hydre"), saison: "Printemps", dec: -18.63 },
        Target { target_type: TargetType::Nebula, id: Some("M16"), nom: "L'Aigle", latin: "Serpens", abbr: None, parent: Some("Serpent"), saison: "Printemps", dec: -13.81 }, // Se lève en fin de nuit


        // --- ÉTÉ (Le long de la Voie Lactée) ---
        Target { target_type: TargetType::Nebula, id: Some("M8"), nom: "La Lagune", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -24.38 },
        Target { target_type: TargetType::Nebula, id: Some("M20"), nom: "Trifide", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -23.03 },
        Target { target_type: TargetType::Nebula, id: Some("M16"), nom: "L'Aigle", latin: "Serpens", abbr: None, parent: Some("Serpent"), saison: "Été", dec: -13.80 },
        Target { target_type: TargetType::Nebula, id: Some("M17"), nom: "Oméga / Cygne", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -16.18 },
        Target { target_type: TargetType::Nebula, id: Some("IC 4604"), nom: "Rho Ophiuchi", latin: "Ophiuchus", abbr: None, parent: Some("Ophiuchus"), saison: "Été", dec: -24.35 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6357"), nom: "Guerre et Paix", latin: "Scorpius", abbr: None, parent: Some("Scorpion"), saison: "Été", dec: -34.20 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6334"), nom: "Patte de Chat", latin: "Scorpius", abbr: None, parent: Some("Scorpion"), saison: "Été", dec: -35.95 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 7000"), nom: "North America", latin: "Cygnus", abbr: None, parent: Some("Cygne"), saison: "Été", dec: 44.33 },
        Target { target_type: TargetType::Nebula, id: Some("IC 5070"), nom: "Le Pélican", latin: "Cygnus", abbr: None, parent: Some("Cygne"), saison: "Été", dec: 44.13 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6960"), nom: "Dentelles du Cygne (Ouest)", latin: "Cygnus", abbr: None, parent: Some("Cygne"), saison: "Été", dec: 30.70 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6992"), nom: "Dentelles du Cygne (Est)", latin: "Cygnus", abbr: None, parent: Some("Cygne"), saison: "Été", dec: 31.70 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6888"), nom: "Le Croissant", latin: "Cygnus", abbr: None, parent: Some("Cygne"), saison: "Été", dec: 38.35 },
        Target { target_type: TargetType::Nebula, id: Some("IC 1318"), nom: "Région de Sadr", latin: "Cygnus", abbr: None, parent: Some("Cygne"), saison: "Été", dec: 40.25 },
        Target { target_type: TargetType::Nebula, id: Some("LDN 673"), nom: "Nébuleuse sombre de l'Aigle", latin: "Aquila", abbr: None, parent: Some("Aigle"), saison: "Été", dec: 1.00 },

        // --- AUTOMNE (Objets plus lointains ou circumpolaires) ---
        Target { target_type: TargetType::Nebula, id: Some("IC 1805"), nom: "Le Coeur", latin: "Cassiopeia", abbr: None, parent: Some("Cassiopée"), saison: "Automne", dec: 61.45 },
        Target { target_type: TargetType::Nebula, id: Some("IC 1848"), nom: "L'Âme", latin: "Cassiopeia", abbr: None, parent: Some("Cassiopée"), saison: "Automne", dec: 60.40 },
        Target { target_type: TargetType::Nebula, id: Some("IC 1396"), nom: "Trompe d'Éléphant", latin: "Cepheus", abbr: None, parent: Some("Céphée"), saison: "Automne", dec: 57.50 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 281"), nom: "Pacman", latin: "Cassiopeia", abbr: None, parent: Some("Cassiopée"), saison: "Automne", dec: 56.62 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 7635"), nom: "La Bulle", latin: "Cassiopeia", abbr: None, parent: Some("Cassiopée"), saison: "Automne", dec: 61.20 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 7023"), nom: "L'Iris", latin: "Cepheus", abbr: None, parent: Some("Céphée"), saison: "Automne", dec: 68.16 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 7293"), nom: "Hélice", latin: "Aquarius", abbr: None, parent: Some("Verseau"), saison: "Automne", dec: -20.80 },

        Target { target_type: TargetType::Cluster, id: Some("M45"), nom: "Les Pléiades", latin: "Taurus", abbr: None, parent: Some("Taureau"), saison: "Hiver", dec: 24.12 },
        Target { target_type: TargetType::Cluster, id: Some("M44"), nom: "Amas de la Crèche", latin: "Cancer", abbr: None, parent: Some("Cancer"), saison: "Printemps", dec: 19.67 },
        Target { target_type: TargetType::Cluster, id: Some("M35"), nom: "Amas des Gémeaux", latin: "Gemini", abbr: None, parent: Some("Gémeaux"), saison: "Hiver", dec: 24.33 },
        Target { target_type: TargetType::Cluster, id: Some("M36"), nom: "Amas du Cocher (M36)", latin: "Auriga", abbr: None, parent: Some("Cocher"), saison: "Hiver", dec: 34.13 },
        Target { target_type: TargetType::Cluster, id: Some("M37"), nom: "Amas du Cocher (M37)", latin: "Auriga", abbr: None, parent: Some("Cocher"), saison: "Hiver", dec: 32.55 },
        Target { target_type: TargetType::Cluster, id: Some("M38"), nom: "Amas de l'Étoile de Mer", latin: "Auriga", abbr: None, parent: Some("Cocher"), saison: "Hiver", dec: 35.83 },
        Target { target_type: TargetType::Cluster, id: Some("M41"), nom: "Petit Amas du Grand Chien", latin: "Canis Major", abbr: None, parent: Some("Grand Chien"), saison: "Hiver", dec: -20.73 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 869"), nom: "Double Amas de Persée (H)", latin: "Perseus", abbr: None, parent: Some("Persée"), saison: "Automne", dec: 57.13 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 884"), nom: "Double Amas de Persée (Chi)", latin: "Perseus", abbr: None, parent: Some("Persée"), saison: "Automne", dec: 57.15 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 2244"), nom: "Amas du Coeur de la Rosette", latin: "Monoceros", abbr: None, parent: Some("Licorne"), saison: "Hiver", dec: 4.87 },

        // --- PRINTEMPS (Les Géants Globulaires) ---
        Target { target_type: TargetType::Cluster, id: Some("M3"), nom: "Amas globulaire des Chiens de Chasse", latin: "Canes Venatici", abbr: None, parent: Some("Chiens de Chasse"), saison: "Printemps", dec: 28.38 },
        Target { target_type: TargetType::Cluster, id: Some("M5"), nom: "Amas globulaire du Serpent", latin: "Serpens", abbr: None, parent: Some("Serpent"), saison: "Printemps", dec: 2.08 },
        Target { target_type: TargetType::Cluster, id: Some("M13"), nom: "Grand Amas d'Hercule", latin: "Hercules", abbr: None, parent: Some("Hercule"), saison: "Été", dec: 36.46 },
        Target { target_type: TargetType::Cluster, id: Some("M53"), nom: "Amas de la Chevelure de Bérénice", latin: "Coma Berenices", abbr: None, parent: Some("Chevelure de Bérénice"), saison: "Printemps", dec: 18.17 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 5139"), nom: "Oméga Centauri (Le Roi)", latin: "Centaurus", abbr: None, parent: Some("Centaure"), saison: "Printemps", dec: -47.48 },

        // --- ÉTÉ (La Voie Lactée et ses bijoux) ---
        Target { target_type: TargetType::Cluster, id: Some("M11"), nom: "Amas du Canard Sauvage", latin: "Scutum", abbr: None, parent: Some("Écu de Sobieski"), saison: "Été", dec: -6.27 },
        Target { target_type: TargetType::Cluster, id: Some("M22"), nom: "Grand Amas du Sagittaire", latin: "Sagittarius", abbr: None, parent: Some("Sagittaire"), saison: "Été", dec: -23.90 },
        Target { target_type: TargetType::Cluster, id: Some("M6"), nom: "Amas du Papillon", latin: "Scorpius", abbr: None, parent: Some("Scorpion"), saison: "Été", dec: -32.22 },
        Target { target_type: TargetType::Cluster, id: Some("M7"), nom: "Amas de Ptolémée", latin: "Scorpius", abbr: None, parent: Some("Scorpion"), saison: "Été", dec: -34.82 },
        Target { target_type: TargetType::Cluster, id: Some("M92"), nom: "Amas d'Hercule (M92)", latin: "Hercules", abbr: None, parent: Some("Hercule"), saison: "Été", dec: 43.13 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 6231"), nom: "Faux Comète / Amas du Scorpion", latin: "Scorpius", abbr: None, parent: Some("Scorpion"), saison: "Été", dec: -41.80 },

        // --- AUTOMNE / SUD ---
        Target { target_type: TargetType::Cluster, id: Some("M15"), nom: "Amas de Pégase", latin: "Pegasus", abbr: None, parent: Some("Pégase"), saison: "Automne", dec: 12.17 },
        Target { target_type: TargetType::Cluster, id: Some("M2"), nom: "Amas du Verseau", latin: "Aquarius", abbr: None, parent: Some("Verseau"), saison: "Automne", dec: -0.82 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 104"), nom: "47 Tucanae", latin: "Tucana", abbr: None, parent: Some("Toucan"), saison: "Automne", dec: -72.08 }, // Sud
        Target { target_type: TargetType::Cluster, id: Some("M34"), nom: "Amas de Persée", latin: "Perseus", abbr: None, parent: Some("Persée"), saison: "Automne", dec: 42.78 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 457"), nom: "Amas de la Chouette / ET", latin: "Cassiopeia", abbr: None, parent: Some("Cassiopée"), saison: "Automne", dec: 58.33 },
    ]
});

fn get_color(t: f64, levels: &[f64]) -> HSLColor {
    // On cherche l'index du palier
    let mut idx = 0;
    for (i, &level) in levels.iter().enumerate() {
        if t <= level {
            idx = i;
            break;
        }
        idx = i;
    }
    
    let max_idx = (levels.len() - 1).max(1);
    let ratio = (idx as f64 / max_idx as f64).clamp(0.0, 1.0);
    
    // Simulation de la palette Magma (du noir/violet au jaune/blanc)
    let mut h = 0.8 - (ratio * 0.7);
    let mut s = 0.8;
    let mut l = 0.15 + (ratio * 0.7);
    
    // Si le temps est très court (entre 0 et 0.5s), on met en noir
    if t <= 0.5 {
        h = 0.0;
        s = 0.0;
        l = 0.0;
    }
    
    HSLColor(h, s, l)
}

fn calculer_npf(focale: f64, ouverture: f64, pixel: f64, dec: f64) -> f64 {
    let cos_dec = (dec.abs().to_radians()).cos().abs();
    DEFAULT_K_FACTOR * (16.9 * ouverture + 0.1 * focale + 13.7 * pixel) / (focale * cos_dec)
}

struct NpfApp {
    settings: AppSettings,
    selected_target: Option<Target>,
    chart_texture: Option<egui::TextureHandle>,
    needs_update: bool,
    pitch: f64,
    yaw: f64,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    first_frame: bool,
    show_settings: bool,
}

impl Default for NpfApp {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
            selected_target: None,
            chart_texture: None,
            needs_update: true,
            pitch: 0.5,
            yaw: 0.5,
            scale: 0.7,
            offset_x: 0.0,
            offset_y: 0.0,
            first_frame: true,
            show_settings: false,
        }
    }
}

impl NpfApp {
    fn is_target_visible(&self, target: &Target) -> bool {
        // Filtre Latitude : Un objet ne doit être affiché que si sa déclinaison est supérieure à la limite de l'horizon.
        // On utilise la formule : target.dec > (settings.latitude - 80.0)
        // Cela permet de ne garder que les objets qui montent à au moins 10° au-dessus de l'horizon.
        let latitude_visible = target.dec > (self.settings.latitude - 80.0);

        // Filtre Saison :
        // - Si selected_season est égal à "Toutes", on affiche tout (sous réserve du filtre latitude).
        // - Sinon, on affiche l'objet si target.saison correspond à selected_season OU si target.saison contient le mot "Circumpolaire".
        let season_visible = if self.settings.selected_season == "Toutes" {
            true
        } else {
            target.saison == self.settings.selected_season || target.saison.contains("Circumpolaire")
        };

        latitude_visible && season_visible
    }

    fn update_chart(&mut self, ctx: &egui::Context, steps: i32) {
        let width = 1200;
        let height = 900;
        let mut buffer = vec![0u8; width * height * 3];

        {
            let root_base = BitMapBackend::with_buffer(&mut buffer, (width as u32, height as u32)).into_drawing_area();
            root_base.fill(&WHITE).unwrap();

            // Translation du root area pour simuler le zoom sur curseur
            let root = root_base.margin(0, 0, 0, 0);
            
            let target_dec = self.selected_target.as_ref().map(|t| t.dec.abs()).unwrap_or(0.0);
            let target_name = self.selected_target.as_ref().map(|t| t.id.unwrap_or(t.nom)).unwrap_or("Équateur");

            let lens = &self.settings.lenses[self.settings.selected_lens_idx];
            let sensor = &self.settings.sensors[self.settings.selected_sensor_idx];
            
            let f_min = lens.f_min;
            let f_max = lens.f_max;
            let n_min = lens.n_min;
            let _n_max = lens.n_max;
            let pixel_size = sensor.pixel_size;

            // Niveaux de temps de pose pour la coloration (ajustés dynamiquement)
            // On fixe la déclinaison de référence à 60° pour l'échelle globale afin d'éviter les sauts lors de la sélection
            let z_max_dec = 60.0;
            let t_base = calculer_npf(f_min, n_min, pixel_size, z_max_dec);
            
            // z_max fixé à 1.25x le temps à 60° pour une vue d'ensemble stable
            let mut z_max = ((t_base * 1.25) / 5.0).ceil() * 5.0;
            if z_max < 5.0 { z_max = 5.0; }
            
            let mut levels = Vec::new();
            // On s'assure que 0.5 est présent dans les paliers pour une coloration cohérente
            levels.push(0.5);
            // Augmentation à 10 paliers pour une coloration plus fine et couvrant toute la plage de l'axe
            for i in 1..10 {
                let l = 0.5 + (z_max * 2.5 - 0.5) * (i as f64) / 9.0;
                levels.push(l);
            }

            let mut chart = ChartBuilder::on(&root)
                .margin(10)
                .x_label_area_size(80)
                .y_label_area_size(80)
                .build_cartesian_3d(f_min..f_max, 0.0..(z_max * 2.5), 0.0..90.0)
                .unwrap();

            chart.with_projection(|mut pb| {
                pb.pitch = self.pitch;
                pb.yaw = self.yaw;
                pb.scale = 0.7; // On fixe le scale de plotters pour éviter les déformations de grille
                
                pb.into_matrix()
            });

            chart.configure_axes()
                .label_style(("sans-serif", 18).into_font().color(&BLACK))
                .x_labels(5)
                .y_labels(5)
                .z_labels(5)
                .axis_panel_style(WHITE.mix(0.1))
                .light_grid_style(BLACK.mix(0.1))
                .bold_grid_style(BLACK.mix(0.2))
                .draw()
                .unwrap();

            // Légendes des axes 3D dynamiques
            let label_style = ("sans-serif", 20).into_font().color(&BLACK);
            
            // X: Focale - Placée au milieu de l'axe X, décalée en Y/Z pour être hors de la boîte
            chart.draw_series(std::iter::once(Text::new(
                "Focale (mm)",
                ((f_min + f_max) / 2.0, -z_max * 0.1, -10.0),
                label_style.clone().pos(plotters::style::text_anchor::Pos {
                    h_pos: plotters::style::text_anchor::HPos::Center,
                    v_pos: plotters::style::text_anchor::VPos::Top,
                }),
            ))).unwrap();

            // Y: Temps de pose - Placé au milieu de l'axe Y, décalé en X/Z
            chart.draw_series(std::iter::once(Text::new(
                "Temps de pose (s)",
                (f_min - (f_max-f_min)*0.1, z_max / 2.0, -10.0),
                label_style.clone().pos(plotters::style::text_anchor::Pos {
                    h_pos: plotters::style::text_anchor::HPos::Center,
                    v_pos: plotters::style::text_anchor::VPos::Bottom,
                }),
            ))).unwrap();

            // Z: Déclinaison - Placé au milieu de l'axe Z, décalé en X/Y sur l'axe en face
            chart.draw_series(std::iter::once(Text::new(
                "Déclinaison (°)",
                (f_max + (f_max-f_min)*0.1, -z_max * 0.1, 90.0 / 2.0),
                label_style.clone().pos(plotters::style::text_anchor::Pos {
                    h_pos: plotters::style::text_anchor::HPos::Center,
                    v_pos: plotters::style::text_anchor::VPos::Bottom,
                }),
            ))).unwrap();

            // Surface NPF
            let dec_steps = if steps < 30 { 10 } else { 25 };
            let f_steps = if steps < 30 { 10 } else { 25 };
            let t_threshold = z_max * 2.5; // Écrêtage à la limite exacte de l'axe

            // Type definitions to reduce complexity
            type Point3D = (f64, f64, f64);
            type Triangle3D = [Point3D; 3];
            type ColoredPolygons = std::collections::HashMap<usize, Vec<Vec<Point3D>>>;

            // Dictionnaire pour regrouper les polygones par couleur pour optimiser le dessin
            let mut colored_polygons: ColoredPolygons = std::collections::HashMap::new();

            // Fonction pour subdiviser un triangle par un seuil
            // Retourne (triangles_dessous, triangles_dessus)
            fn split_triangle(v: &Triangle3D, threshold: f64) -> (Vec<Triangle3D>, Vec<Triangle3D>) {
                let mut below = Vec::new();
                let mut above = Vec::new();

                let (mut b_idx, mut a_idx) = (Vec::new(), Vec::new());
                for (i, p) in v.iter().enumerate().take(3) {
                    if p.1 <= threshold { b_idx.push(i); } else { a_idx.push(i); }
                }

                if b_idx.len() == 3 {
                    below.push(*v);
                } else if a_idx.len() == 3 {
                    above.push(*v);
                } else if b_idx.len() == 1 {
                    // 1 point dessous, 2 points dessus
                    let b = v[b_idx[0]];
                    let a1 = v[a_idx[0]];
                    let a2 = v[a_idx[1]];

                    let t1 = (threshold - b.1) / (a1.1 - b.1);
                    let i1 = (b.0 + (a1.0 - b.0) * t1, threshold, b.2 + (a1.2 - b.2) * t1);

                    let t2 = (threshold - b.1) / (a2.1 - b.1);
                    let i2 = (b.0 + (a2.0 - b.0) * t2, threshold, b.2 + (a2.2 - b.2) * t2);

                    below.push([b, i1, i2]);
                    above.push([i1, a1, a2]);
                    above.push([i1, a2, i2]);
                } else {
                    // 2 points dessous, 1 point dessus
                    let b1 = v[b_idx[0]];
                    let b2 = v[b_idx[1]];
                    let a = v[a_idx[0]];

                    let t1 = (threshold - b1.1) / (a.1 - b1.1);
                    let i1 = (b1.0 + (a.0 - b1.0) * t1, threshold, b1.2 + (a.2 - b1.2) * t1);

                    let t2 = (threshold - b2.1) / (a.1 - b2.1);
                    let i2 = (b2.0 + (a.0 - b2.0) * t2, threshold, b2.2 + (a.2 - b2.2) * t2);

                    below.push([b1, b2, i1]);
                    below.push([b2, i2, i1]);
                    above.push([i1, i2, a]);
                }
                (below, above)
            }

            for i in 0..dec_steps {
                for j in 0..f_steps {
                    let d1 = 90.0 * (i as f64) / (dec_steps as f64);
                    let d2 = 90.0 * ((i + 1) as f64) / (dec_steps as f64);
                    let f1 = f_min + (f_max - f_min) * (j as f64) / (f_steps as f64);
                    let f2 = f_min + (f_max - f_min) * ((j + 1) as f64) / (f_steps as f64);

                    let n1 = lens.get_n(f1);
                    let n2 = lens.get_n(f2);

                    let mut t11 = calculer_npf(f1, n1, pixel_size, d1);
                    let mut t12 = calculer_npf(f1, n1, pixel_size, d2);
                    let mut t21 = calculer_npf(f2, n2, pixel_size, d1);
                    let mut t22 = calculer_npf(f2, n2, pixel_size, d2);

                    // Écrêtage
                    if t11 > t_threshold { t11 = t_threshold; }
                    if t12 > t_threshold { t12 = t_threshold; }
                    if t21 > t_threshold { t21 = t_threshold; }
                    if t22 > t_threshold { t22 = t_threshold; }

                    let tri1 = [(f1, t11, d1), (f1, t12, d2), (f2, t21, d1)];
                    let tri2 = [(f2, t22, d2), (f1, t12, d2), (f2, t21, d1)];

                    for initial_tri in [tri1, tri2] {
                        let mut current_tris = vec![initial_tri];
                        
                        // On découpe par chaque palier successif
                        for (idx, &level) in levels.iter().enumerate() {
                            let mut next_tris = Vec::new();
                            for tri in current_tris {
                                let (below, above) = split_triangle(&tri, level);
                                // On dessine ce qui est en dessous du palier actuel
                                for b_tri in below {
                                    colored_polygons.entry(idx).or_default().push(b_tri.to_vec());
                                }
                                // On garde ce qui est au dessus pour le prochain palier
                                next_tris.extend(above);
                            }
                            current_tris = next_tris;
                            if current_tris.is_empty() { break; }
                        }
                        
                        // Ajouter les triangles restants au dessus du dernier palier
                        for tri in current_tris {
                            colored_polygons.entry(levels.len()).or_default().push(tri.to_vec());
                        }
                    }
                }
            }

            // Dessin groupé par couleur (gain de performance massif)
            for (color_idx, polys) in colored_polygons {
                let color = if color_idx < levels.len() {
                    get_color(levels[color_idx] - 0.001, &levels)
                } else {
                    get_color(levels.last().unwrap() + 0.1, &levels)
                };
                
                chart.draw_series(polys.into_iter().map(|p| {
                    Polygon::new(p, color.mix(0.7).filled())
                })).unwrap();
            }

            // Ligne de la cible
            let line_points: Vec<(f64, f64, f64)> = (0..100).map(|i| {
                let f = f_min + (f_max - f_min) * (i as f64) / 99.0;
                let n = lens.get_n(f);
                let mut t = calculer_npf(f, n, pixel_size, target_dec);
                if t > t_threshold { t = t_threshold; }
                (f, t, target_dec)
            }).collect();

            chart.draw_series(LineSeries::new(line_points, CYAN.stroke_width(2))).unwrap();

            // Simulation d'une colorbar sur le côté (toujours fixe par rapport à l'image)
            let cb_x_start = width as i32 - 120;
            let cb_y_start = 200;
            let cb_height = height as i32 - 400;
            let cb_width = 40;

            for i in 0..cb_height {
                let ratio = 1.0 - (i as f64 / cb_height as f64);
                let t_val = ratio * (z_max * 2.5);
                
                // Pour la colorbar, on utilise toujours get_color qui maintenant fait des paliers
                let color = get_color(t_val, &levels);
                
                root_base.draw(&Rectangle::new(
                    [(cb_x_start, cb_y_start + i), (cb_x_start + cb_width, cb_y_start + i + 1)],
                    color.filled(),
                )).unwrap();
            }

            // Labels pour la colorbar
            for (i, &level) in levels.iter().enumerate() {
                let y = cb_y_start + cb_height - (i as i32 * cb_height / (levels.len() as i32 - 1));
                root_base.draw(&Text::new(
                    format!("{:.1}s", level),
                    (cb_x_start + 50, y - 10),
                    ("sans-serif", 20).into_font().color(&BLACK),
                )).unwrap();
            }

            // Légende manuelle en bas à droite pour la ligne cyan
            let leg_x_end = width as i32 - 40;
            let leg_y_bottom = height as i32 - 40;
            let leg_text = format!("Position {}", target_name);
            let leg_style = ("sans-serif", 20).into_font().color(&BLACK);
            let (text_w, _text_h) = root_base.estimate_text_size(&leg_text, &leg_style).unwrap_or((150, 20));
            
            let leg_x_start = leg_x_end - text_w as i32 - 40; // 40 pour la ligne
            
            // Dessin du segment cyan
            root_base.draw(&PathElement::new(
                vec![(leg_x_start, leg_y_bottom - 10), (leg_x_start + 30, leg_y_bottom - 10)],
                CYAN.stroke_width(2)
            )).unwrap();
            
            // Dessin du texte
            root_base.draw(&Text::new(
                leg_text,
                (leg_x_start + 40, leg_y_bottom - 20),
                leg_style,
            )).unwrap();

            root.present().unwrap();
            root_base.present().unwrap();
        }

        let mut rgba = Vec::with_capacity(width * height * 4);
        for i in 0..width * height {
            rgba.push(buffer[i * 3]);
            rgba.push(buffer[i * 3 + 1]);
            rgba.push(buffer[i * 3 + 2]);
            rgba.push(255);
        }

        let image = egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba);
        self.chart_texture = Some(ctx.load_texture("chart", image, Default::default()));
        self.needs_update = false;
    }
}

impl eframe::App for NpfApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_settings {
            egui::Window::new("Paramètres - Objectifs et Capteurs").show(ctx, |ui| {
                ui.heading("Capteurs");
                let mut to_remove_sensor = None;
                let num_sensors = self.settings.sensors.len();
                for i in 0..num_sensors {
                    let sensor = &mut self.settings.sensors[i];
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut sensor.name);
                        ui.add(egui::DragValue::new(&mut sensor.pixel_size).speed(0.01).prefix("Pixel: ").suffix(" µm"));
                        if ui.button("🗑").clicked() && num_sensors > 1 {
                            to_remove_sensor = Some(i);
                        }
                    });
                }
                if let Some(i) = to_remove_sensor {
                    self.settings.sensors.remove(i);
                    if self.settings.selected_sensor_idx >= self.settings.sensors.len() {
                        self.settings.selected_sensor_idx = 0;
                    }
                    self.needs_update = true;
                }
                if ui.button("+ Ajouter un capteur").clicked() {
                    self.settings.sensors.push(SensorConfig::default());
                }

                ui.separator();

                ui.heading("Objectifs");
                let mut to_remove_lens = None;
                let num_lenses = self.settings.lenses.len();
                for i in 0..num_lenses {
                    let lens = &mut self.settings.lenses[i];
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut lens.name);
                            if ui.button("🗑").clicked() && num_lenses > 1 {
                                to_remove_lens = Some(i);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut lens.f_min).speed(0.1).prefix("Fmin: ").suffix(" mm"));
                            ui.add(egui::DragValue::new(&mut lens.f_max).speed(0.1).prefix("Fmax: ").suffix(" mm"));
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut lens.n_min).speed(0.1).prefix("f/min: "));
                            ui.add(egui::DragValue::new(&mut lens.n_max).speed(0.1).prefix("f/max: "));
                        });
                    });
                }
                if let Some(i) = to_remove_lens {
                    self.settings.lenses.remove(i);
                    if self.settings.selected_lens_idx >= self.settings.lenses.len() {
                        self.settings.selected_lens_idx = 0;
                    }
                    self.needs_update = true;
                }
                if ui.button("+ Ajouter un objectif").clicked() {
                    self.settings.lenses.push(LensConfig {
                        name: "Nouvel Objectif".to_string(),
                        f_min: 50.0,
                        f_max: 50.0,
                        n_min: 1.8,
                        n_max: 1.8,
                    });
                }

                ui.add_space(10.0);
                if ui.button("Fermer et appliquer").clicked() {
                    self.show_settings = false;
                    self.needs_update = true;
                }
            });
        }

        if self.needs_update {
            self.update_chart(ctx, 25);
        }

        // Au tout premier frame, on s'assure que la fenêtre est visible et maximisée
        // une seule fois après le rendu initial pour éviter le flash blanc.
        if self.first_frame {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
            self.first_frame = false;
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Objectif", |ui| {
                    for (i, lens) in self.settings.lenses.iter().enumerate() {
                        if ui.selectable_label(self.settings.selected_lens_idx == i, &lens.name).clicked() {
                            self.settings.selected_lens_idx = i;
                            self.needs_update = true;
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("Capteur", |ui| {
                    for (i, sensor) in self.settings.sensors.iter().enumerate() {
                        if ui.selectable_label(self.settings.selected_sensor_idx == i, &sensor.name).clicked() {
                            self.settings.selected_sensor_idx = i;
                            self.needs_update = true;
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("Paramètres", |ui| {
                    if ui.button("Gérer les objectifs et capteurs").clicked() {
                        self.show_settings = true;
                        ui.close_menu();
                    }
                });

                ui.separator();

                ui.label("Saison:");
                egui::ComboBox::from_id_source("season_filter")
                    .selected_text(&self.settings.selected_season)
                    .show_ui(ui, |ui| {
                        let seasons = ["Toutes", "Printemps", "Été", "Automne", "Hiver", "Circumpolaire N", "Circumpolaire S"];
                        for season in seasons {
                            if ui.selectable_label(self.settings.selected_season == season, season).clicked() {
                                self.settings.selected_season = season.to_string();
                                self.needs_update = true;
                            }
                        }
                    });

                ui.label("Latitude:");
                if ui.add(egui::DragValue::new(&mut self.settings.latitude)
                    .speed(0.1)
                    .clamp_range(-90.0..=90.0)
                    .suffix("°")).changed() {
                    self.needs_update = true;
                }

                ui.menu_button("Type d'objet", |ui| {
                    ui.menu_button("Constellation", |ui| {
                        let mut constellations: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Constellation)
                            .filter(|t| self.is_target_visible(t))
                            .collect();
                        constellations.sort_by(|a, b| a.nom.cmp(b.nom));
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in constellations {
                                if ui.button(target.nom).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                    ui.menu_button("Messier", |ui| {
                        let mut messiers: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Messier)
                            .filter(|t| self.is_target_visible(t))
                            .collect();
                        messiers.sort_by(|a, b| {
                            let a_num: i32 = a.id.unwrap_or("").strip_prefix('M').and_then(|s| s.parse().ok()).unwrap_or(0);
                            let b_num: i32 = b.id.unwrap_or("").strip_prefix('M').and_then(|s| s.parse().ok()).unwrap_or(0);
                            a_num.cmp(&b_num)
                        });
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in messiers {
                                let label = format!("{} - {}", target.id.unwrap_or(""), target.nom);
                                if ui.button(label).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                    ui.menu_button("Nébuleuse", |ui| {
                        let mut nebulae: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Nebula)
                            .filter(|t| self.is_target_visible(t))
                            .collect();
                        nebulae.sort_by(|a, b| a.nom.cmp(b.nom));
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in nebulae {
                                if ui.button(target.nom).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                    ui.menu_button("Galaxie", |ui| {
                        let mut galaxies: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Galaxy)
                            .filter(|t| self.is_target_visible(t))
                            .collect();
                        galaxies.sort_by(|a, b| a.nom.cmp(b.nom));
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in galaxies {
                                if ui.button(target.nom).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                    ui.menu_button("Amas", |ui| {
                        let mut clusters: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Cluster)
                            .filter(|t| self.is_target_visible(t))
                            .collect();
                        clusters.sort_by(|a, b| a.nom.cmp(b.nom));
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in clusters {
                                if ui.button(target.nom).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                });
            });
        });

        egui::SidePanel::right("info_panel").show(ctx, |ui| {
            ui.set_min_width(200.0);
            ui.vertical_centered(|ui| {
                ui.heading("Informations Cible");
            });
            ui.separator();
            ui.add_space(10.0);

            if let Some(target) = &self.selected_target {
                let is_visible = self.is_target_visible(target);
                if !is_visible {
                    let msg = if target.saison != self.settings.selected_season && self.settings.selected_season != "Toutes" 
                        && !target.saison.starts_with("Circumpolaire") {
                        "⚠ Cible hors saison"
                    } else {
                        "⚠ Cible non visible depuis cette latitude"
                    };
                    ui.colored_label(egui::Color32::RED, msg);
                    ui.add_space(5.0);
                }
                ui.scope(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                    
                    if let Some(id) = target.id {
                        ui.horizontal(|ui| {
                            ui.strong("ID :");
                            ui.label(id);
                        });
                    }

                    ui.horizontal(|ui| {
                        ui.strong("Nom :");
                        ui.label(target.nom);
                    });

                    ui.horizontal(|ui| {
                        ui.strong("Latin :");
                        ui.label(target.latin);
                    });

                    if let Some(abbr) = target.abbr {
                        ui.horizontal(|ui| {
                            ui.strong("Abbr :");
                            ui.label(abbr);
                        });
                    }

                    if let Some(parent) = target.parent {
                        ui.horizontal(|ui| {
                            ui.strong("Constellation :");
                            ui.label(parent);
                        });
                    }

                    ui.horizontal(|ui| {
                        ui.strong("Saison :");
                        if !self.is_target_visible(target) {
                            ui.colored_label(egui::Color32::RED, target.saison);
                        } else {
                            ui.label(target.saison);
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.strong("Déclinaison :");
                        if target.dec <= (self.settings.latitude - 80.0) {
                            ui.colored_label(egui::Color32::RED, format!("{:.1}° (Trop bas)", target.dec));
                        } else {
                            ui.label(format!("{:.1}°", target.dec));
                        }
                    });

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Rappel des paramètres de prise de vue
                    ui.heading("Paramètres");
                    ui.add_space(5.0);
                    let lens = &self.settings.lenses[self.settings.selected_lens_idx];
                    let sensor = &self.settings.sensors[self.settings.selected_sensor_idx];
                    ui.label(format!("Objectif: {}", lens.name));
                    if lens.f_max > lens.f_min {
                        ui.label(format!("Focale: {:.0}-{:.0}mm", lens.f_min, lens.f_max));
                        ui.label(format!("Ouverture: f/{:.1}-f/{:.1}", lens.n_min, lens.n_max));
                    } else {
                        ui.label(format!("Focale: {:.0}mm", lens.f_min));
                        ui.label(format!("Ouverture: f/{:.1}", lens.n_min));
                    }
                    ui.label(format!("Capteur: {} ({:.2} µm)", sensor.name, sensor.pixel_size));
                    
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.heading("Temps d'exposition (NPF)");
                    ui.add_space(5.0);
                    
                    let pixel = sensor.pixel_size;
                    let dec = target.dec;
                    
                    if lens.f_max > lens.f_min {
                        let npf_min = calculer_npf(lens.f_min, lens.n_min, pixel, dec);
                        let npf_max = calculer_npf(lens.f_max, lens.n_max, pixel, dec);
                        
                        ui.horizontal(|ui| {
                            ui.strong(format!("{:.0}mm (f/{:.1}) :", lens.f_min, lens.n_min));
                            ui.label(format!("{:.2}s", npf_min));
                        });
                        ui.horizontal(|ui| {
                            ui.strong(format!("{:.0}mm (f/{:.1}) :", lens.f_max, lens.n_max));
                            ui.label(format!("{:.2}s", npf_max));
                        });
                    } else {
                        let npf = calculer_npf(lens.f_min, lens.n_min, pixel, dec);
                        ui.horizontal(|ui| {
                            ui.strong(format!("{:.0}mm (f/{:.1}) :", lens.f_min, lens.n_min));
                            ui.label(format!("{:.2}s", npf));
                        });
                    }
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.label("Sélectionnez un objet pour voir les détails.");
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let target_name = self.selected_target.as_ref().map(|t| t.id.unwrap_or(t.nom)).unwrap_or("Équateur");
            let lens = &self.settings.lenses[self.settings.selected_lens_idx];
            let titre = format!("{} | Cible : {}", lens.name, target_name);

            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(titre);
                ui.add_space(5.0);
                ui.label("Cliquez et déplacez la souris sur le graphique pour le faire pivoter.");
                ui.add_space(10.0);
            });

            if let Some(texture) = &self.chart_texture {
                // On calcule les UVs pour le zoom et l'offset
                // L'image de base est 800x600.
                // scale = 1.0 -> UV [0, 1]
                // scale > 1.0 -> UV réduit
                let uv_size = (1.0 / self.scale.max(0.1)).min(1.0);
                
                // L'offset_x/y est normalisé (-1 à 1 par rapport au centre)
                // On le convertit en décalage d'UV
                // Le centre de la fenêtre UV est (0.5 - offset_x/2, 0.5 - offset_y/2)
                let u_min = (0.5 - uv_size / 2.0 - self.offset_x * 0.5).clamp(0.0, 1.0 - uv_size);
                let v_min = (0.5 - uv_size / 2.0 - self.offset_y * 0.5).clamp(0.0, 1.0 - uv_size);
                
                let uv = egui::Rect::from_min_max(
                    egui::pos2(u_min as f32, v_min as f32),
                    egui::pos2((u_min + uv_size) as f32, (v_min + uv_size) as f32)
                );

                let img = egui::Image::new(texture)
                    .uv(uv)
                    .shrink_to_fit();
                let response = ui.add(img.sense(egui::Sense::drag()));

                if response.dragged() {
                    let delta = response.drag_delta();
                    // On ajuste la sensibilité (par exemple 0.01 radians par pixel)
                    // Inversion de l'axe horizontal selon la demande
                    self.yaw -= (delta.x as f64) * 0.01;
                    self.pitch += (delta.y as f64) * 0.01;

                    // On peut limiter le pitch pour éviter de retourner le graphe
                    self.pitch = self.pitch.clamp(0.01, std::f64::consts::PI - 0.01);
                    
                    // Normalisation du yaw (optionnel mais propre)
                    while self.yaw < 0.0 { self.yaw += std::f64::consts::PI * 2.0; }
                    while self.yaw > std::f64::consts::PI * 2.0 { self.yaw -= std::f64::consts::PI * 2.0; }

                    // Basse résolution dynamique lors du drag pour plus de fluidité
                    self.update_chart(ctx, 15);
                }

                // Gestion du zoom (scroll souris)
                let scroll_delta = ui.input(|i| i.scroll_delta.y);
                if scroll_delta != 0.0 {
                    let zoom_factor = 1.1f64;
                    
                    if scroll_delta > 0.0 {
                        // Zoom in: vers le curseur
                        let old_scale = self.scale;
                        self.scale *= zoom_factor;
                        self.scale = self.scale.clamp(0.1, 10.0);
                        
                        // Calculer la position relative de la souris sur le widget (-1.0 à 1.0)
                        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                            let rect = response.rect;
                            let rel_x = (mouse_pos.x - rect.center().x) / (rect.width() / 2.0);
                            let rel_y = (mouse_pos.y - rect.center().y) / (rect.height() / 2.0);
                            
                            // On ajuste l'offset pour simuler le zoom sur le curseur
                            // L'utilisateur demande d'inverser les axes de zoom car c'est pas intuitif.
                            // Le signe de rel_x et rel_y dans l'offset influence la direction du déplacement de la vue.
                            let actual_zoom = self.scale / old_scale;
                            self.offset_x -= rel_x as f64 * (1.0 - 1.0/actual_zoom);
                            self.offset_y -= rel_y as f64 * (1.0 - 1.0/actual_zoom);
                        }
                    } else {
                        // Zoom out: recentrage progressif
                        self.scale /= zoom_factor;
                        self.scale = self.scale.clamp(0.1, 10.0);
                        
                        // Recentrage : on réduit l'offset
                        self.offset_x *= 0.8;
                        self.offset_y *= 0.8;
                        if self.offset_x.abs() < 0.001 { self.offset_x = 0.0; }
                        if self.offset_y.abs() < 0.001 { self.offset_y = 0.0; }
                    }
                    
                    self.needs_update = true;
                }
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_maximized(true)
            .with_visible(false), // Masquer initialement pour éviter le flash blanc
        ..Default::default()
    };
    eframe::run_native(
        "Navigateur NPF",
        options,
        Box::new(|cc| {
            // Force le thème sombre immédiatement pour éviter le flash blanc
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            
            let mut app = if let Some(storage) = cc.storage {
                let settings: AppSettings = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
                NpfApp {
                    settings,
                    ..NpfApp::default()
                }
            } else {
                NpfApp::default()
            };

            // On génère le premier rendu AVANT l'ouverture de la fenêtre
            app.update_chart(&cc.egui_ctx, 25);
            
            Box::new(app)
        }),
    )
}
