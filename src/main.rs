#![windows_subsystem = "windows"]

use eframe::egui;
use fluent::{FluentArgs, FluentBundle, FluentResource};
use once_cell::sync::Lazy;
use plotters::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

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
    language: String,
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
            language: "fr".to_string(),
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
    pub name_key: &'static str,
    pub latin_key: &'static str,
    pub abbr: Option<&'static str>,
    pub parent_key: Option<&'static str>,
    pub season_key: &'static str,
    pub dec: f64,
}

pub static TARGETS: Lazy<Vec<Target>> = Lazy::new(|| {
    vec![
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-andromde", latin_key: "latin-andromeda", abbr: Some("And"), parent_key: None, season_key: "season-autumn", dec: 37.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-aigle", latin_key: "latin-aquila", abbr: Some("Aql"), parent_key: None, season_key: "season-summer", dec: 3.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-verseau", latin_key: "latin-aquarius", abbr: Some("Aqr"), parent_key: None, season_key: "season-autumn", dec: -10.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-blier", latin_key: "latin-aries", abbr: Some("Ari"), parent_key: None, season_key: "season-autumn", dec: 20.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-cocher", latin_key: "latin-auriga", abbr: Some("Aur"), parent_key: None, season_key: "season-winter", dec: 42.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-bouvier", latin_key: "latin-bootes", abbr: Some("Boo"), parent_key: None, season_key: "season-spring", dec: 28.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-girafe", latin_key: "latin-camelopardalis", abbr: Some("Cam"), parent_key: None, season_key: "season-circumpolar-n", dec: 70.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-cancer", latin_key: "latin-cancer", abbr: Some("Cnc"), parent_key: None, season_key: "season-spring", dec: 20.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-chiens-de-chasse", latin_key: "latin-canes-venatici", abbr: Some("CVn"), parent_key: None, season_key: "season-spring", dec: 40.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-petit-chien", latin_key: "latin-canis-minor", abbr: Some("CMi"), parent_key: None, season_key: "season-winter", dec: 5.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-cassiope", latin_key: "latin-cassiopeia", abbr: Some("Cas"), parent_key: None, season_key: "season-circumpolar-n", dec: 60.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-cphe", latin_key: "latin-cepheus", abbr: Some("Cep"), parent_key: None, season_key: "season-circumpolar-n", dec: 70.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-chevelure-de-brnice", latin_key: "latin-coma-berenices", abbr: Some("Com"), parent_key: None, season_key: "season-spring", dec: 23.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-couronne-borale", latin_key: "latin-corona-borealis", abbr: Some("CrB"), parent_key: None, season_key: "season-spring", dec: 30.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-cygne", latin_key: "latin-cygnus", abbr: Some("Cyg"), parent_key: None, season_key: "season-summer", dec: 42.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-dauphin", latin_key: "latin-delphinus", abbr: Some("Del"), parent_key: None, season_key: "season-summer", dec: 12.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-dragon", latin_key: "latin-draco", abbr: Some("Dra"), parent_key: None, season_key: "season-circumpolar-n", dec: 65.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-petit-cheval", latin_key: "latin-equuleus", abbr: Some("Equ"), parent_key: None, season_key: "season-autumn", dec: 7.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-gmeaux", latin_key: "latin-gemini", abbr: Some("Gem"), parent_key: None, season_key: "season-winter", dec: 22.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-hercule", latin_key: "latin-hercules", abbr: Some("Her"), parent_key: None, season_key: "season-summer", dec: 27.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-lzard", latin_key: "latin-lacerta", abbr: Some("Lac"), parent_key: None, season_key: "season-autumn", dec: 45.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-lion", latin_key: "latin-leo", abbr: Some("Leo"), parent_key: None, season_key: "season-spring", dec: 15.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-petit-lion", latin_key: "latin-leo-minor", abbr: Some("LMi"), parent_key: None, season_key: "season-spring", dec: 35.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-lynx", latin_key: "latin-lynx", abbr: Some("Lyn"), parent_key: None, season_key: "season-winter", dec: 45.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-lyre", latin_key: "latin-lyra", abbr: Some("Lyr"), parent_key: None, season_key: "season-summer", dec: 38.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-licorne", latin_key: "latin-monoceros", abbr: Some("Mon"), parent_key: None, season_key: "season-winter", dec: -3.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-ophiuchus", latin_key: "latin-ophiuchus", abbr: Some("Oph"), parent_key: None, season_key: "season-summer", dec: -7.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-orion", latin_key: "latin-orion", abbr: Some("Ori"), parent_key: None, season_key: "season-winter", dec: 5.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-pgase", latin_key: "latin-pegasus", abbr: Some("Peg"), parent_key: None, season_key: "season-autumn", dec: 20.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-perse", latin_key: "latin-perseus", abbr: Some("Per"), parent_key: None, season_key: "season-autumn", dec: 45.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-poissons", latin_key: "latin-pisces", abbr: Some("Psc"), parent_key: None, season_key: "season-autumn", dec: 15.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-flche", latin_key: "latin-sagitta", abbr: Some("Sge"), parent_key: None, season_key: "season-summer", dec: 18.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-serpent", latin_key: "latin-serpens", abbr: Some("Ser"), parent_key: None, season_key: "season-summer", dec: 0.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-sextant", latin_key: "latin-sextans", abbr: Some("Sex"), parent_key: None, season_key: "season-spring", dec: -2.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-taureau", latin_key: "latin-taurus", abbr: Some("Tau"), parent_key: None, season_key: "season-winter", dec: 16.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-triangle", latin_key: "latin-triangulum", abbr: Some("Tri"), parent_key: None, season_key: "season-autumn", dec: 32.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-grande-ourse", latin_key: "latin-ursa-major", abbr: Some("UMa"), parent_key: None, season_key: "season-circumpolar-n", dec: 50.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-petite-ourse", latin_key: "latin-ursa-minor", abbr: Some("UMi"), parent_key: None, season_key: "season-circumpolar-n", dec: 75.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-vierge", latin_key: "latin-virgo", abbr: Some("Vir"), parent_key: None, season_key: "season-spring", dec: 0.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-petit-renard", latin_key: "latin-vulpecula", abbr: Some("Vul"), parent_key: None, season_key: "season-summer", dec: 25.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-machine-pneumatique", latin_key: "latin-antlia", abbr: Some("Ant"), parent_key: None, season_key: "season-spring", dec: -35.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-oiseau-de-paradis", latin_key: "latin-apus", abbr: Some("Aps"), parent_key: None, season_key: "season-circumpolar-s", dec: -75.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-autel", latin_key: "latin-ara", abbr: Some("Ara"), parent_key: None, season_key: "season-summer", dec: -53.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-burin", latin_key: "latin-caelum", abbr: Some("Cae"), parent_key: None, season_key: "season-winter", dec: -38.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-grand-chien", latin_key: "latin-canis-major", abbr: Some("CMa"), parent_key: None, season_key: "season-winter", dec: -22.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-capricorne", latin_key: "latin-capricornus", abbr: Some("Cap"), parent_key: None, season_key: "season-autumn", dec: -20.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-carne", latin_key: "latin-carina", abbr: Some("Car"), parent_key: None, season_key: "season-circumpolar-s", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-centaure", latin_key: "latin-centaurus", abbr: Some("Cen"), parent_key: None, season_key: "season-spring", dec: -45.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-baleine", latin_key: "latin-cetus", abbr: Some("Cet"), parent_key: None, season_key: "season-autumn", dec: -8.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-camlon", latin_key: "latin-chamaeleon", abbr: Some("Cha"), parent_key: None, season_key: "season-circumpolar-s", dec: -75.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-circin", latin_key: "latin-circinus", abbr: Some("Cir"), parent_key: None, season_key: "season-circumpolar-s", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-colombe", latin_key: "latin-columba", abbr: Some("Col"), parent_key: None, season_key: "season-winter", dec: -35.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-couronne-australe", latin_key: "latin-corona-australis", abbr: Some("CrA"), parent_key: None, season_key: "season-summer", dec: -40.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-corbeau", latin_key: "latin-corvus", abbr: Some("Crv"), parent_key: None, season_key: "season-spring", dec: -18.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-coupe", latin_key: "latin-crater", abbr: Some("Crt"), parent_key: None, season_key: "season-spring", dec: -15.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-croix-du-sud", latin_key: "latin-crux", abbr: Some("Cru"), parent_key: None, season_key: "season-circumpolar-s", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-dorade", latin_key: "latin-dorado", abbr: Some("Dor"), parent_key: None, season_key: "season-circumpolar-s", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-ridan", latin_key: "latin-eridanus", abbr: Some("Eri"), parent_key: None, season_key: "season-winter", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-fourneau", latin_key: "latin-fornax", abbr: Some("For"), parent_key: None, season_key: "season-autumn", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-grue", latin_key: "latin-grus", abbr: Some("Gru"), parent_key: None, season_key: "season-autumn", dec: -45.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-horloge", latin_key: "latin-horologium", abbr: Some("Hor"), parent_key: None, season_key: "season-winter", dec: -50.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-hydre", latin_key: "latin-hydra", abbr: Some("Hya"), parent_key: None, season_key: "season-spring", dec: -20.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-hydre-mle", latin_key: "latin-hydrus", abbr: Some("Hyi"), parent_key: None, season_key: "season-circumpolar-s", dec: -70.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-indien", latin_key: "latin-indus", abbr: Some("Ind"), parent_key: None, season_key: "season-summer", dec: -55.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-livre", latin_key: "latin-lepus", abbr: Some("Lep"), parent_key: None, season_key: "season-winter", dec: -18.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-balance", latin_key: "latin-libra", abbr: Some("Lib"), parent_key: None, season_key: "season-spring", dec: -15.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-loup", latin_key: "latin-lupus", abbr: Some("Lup"), parent_key: None, season_key: "season-summer", dec: -40.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-table", latin_key: "latin-mensa", abbr: Some("Men"), parent_key: None, season_key: "season-circumpolar-s", dec: -75.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-microscope", latin_key: "latin-microscopium", abbr: Some("Mic"), parent_key: None, season_key: "season-autumn", dec: -35.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-mouche", latin_key: "latin-musca", abbr: Some("Mus"), parent_key: None, season_key: "season-circumpolar-s", dec: -70.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-rgle", latin_key: "latin-norma", abbr: Some("Nor"), parent_key: None, season_key: "season-summer", dec: -50.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-octant", latin_key: "latin-octans", abbr: Some("Oct"), parent_key: None, season_key: "season-circumpolar-s", dec: -85.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-paon", latin_key: "latin-pavo", abbr: Some("Pav"), parent_key: None, season_key: "season-circumpolar-s", dec: -65.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-phnix", latin_key: "latin-phoenix", abbr: Some("Phe"), parent_key: None, season_key: "season-autumn", dec: -45.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-peintre", latin_key: "latin-pictor", abbr: Some("Pic"), parent_key: None, season_key: "season-winter", dec: -55.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-poisson-austral", latin_key: "latin-piscis-austrinus", abbr: Some("PsA"), parent_key: None, season_key: "season-autumn", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-poupe", latin_key: "latin-puppis", abbr: Some("Pup"), parent_key: None, season_key: "season-winter", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-boussole", latin_key: "latin-pyxis", abbr: Some("Pyx"), parent_key: None, season_key: "season-spring", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-sagittaire", latin_key: "latin-sagittarius", abbr: Some("Sgr"), parent_key: None, season_key: "season-summer", dec: -25.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-scorpion", latin_key: "latin-scorpius", abbr: Some("Sco"), parent_key: None, season_key: "season-summer", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-sculpteur", latin_key: "latin-sculptor", abbr: Some("Scl"), parent_key: None, season_key: "season-autumn", dec: -30.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-cu-de-sobieski", latin_key: "latin-scutum", abbr: Some("Sct"), parent_key: None, season_key: "season-summer", dec: -10.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-tlescope", latin_key: "latin-telescopium", abbr: Some("Tel"), parent_key: None, season_key: "season-summer", dec: -50.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-triangle-austral", latin_key: "latin-triangulum-australe", abbr: Some("TrA"), parent_key: None, season_key: "season-circumpolar-s", dec: -65.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-toucan", latin_key: "latin-tucana", abbr: Some("Tuc"), parent_key: None, season_key: "season-circumpolar-s", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-voiles", latin_key: "latin-vela", abbr: Some("Vel"), parent_key: None, season_key: "season-spring", dec: -50.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-poisson-volant", latin_key: "latin-volans", abbr: Some("Vol"), parent_key: None, season_key: "season-circumpolar-s", dec: -70.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-rticule", latin_key: "latin-reticulum", abbr: Some("Ret"), parent_key: None, season_key: "season-circumpolar-s", dec: -60.0 },
        Target { target_type: TargetType::Constellation, id: None, name_key: "target-burin", latin_key: "latin-caelum", abbr: Some("Cae"), parent_key: None, season_key: "season-winter", dec: -38.0 },
        Target { target_type: TargetType::Messier, id: Some("M1"), name_key: "target-m1", latin_key: "latin-taurus", abbr: None, parent_key: Some("target-taureau"), season_key: "season-winter", dec: 22.0 },
        Target { target_type: TargetType::Messier, id: Some("M2"), name_key: "target-m2", latin_key: "latin-aquarius", abbr: None, parent_key: Some("target-verseau"), season_key: "season-autumn", dec: -0.8 },
        Target { target_type: TargetType::Messier, id: Some("M3"), name_key: "target-m3", latin_key: "latin-canes-venatici", abbr: None, parent_key: Some("target-chiens-de-chasse"), season_key: "season-spring", dec: 28.4 },
        Target { target_type: TargetType::Messier, id: Some("M4"), name_key: "target-m4", latin_key: "latin-scorpius", abbr: None, parent_key: Some("target-scorpion"), season_key: "season-summer", dec: -26.5 },
        Target { target_type: TargetType::Messier, id: Some("M5"), name_key: "target-m5", latin_key: "latin-serpens", abbr: None, parent_key: Some("target-serpent"), season_key: "season-spring", dec: 2.1 },
        Target { target_type: TargetType::Messier, id: Some("M6"), name_key: "target-m6", latin_key: "latin-scorpius", abbr: None, parent_key: Some("target-scorpion"), season_key: "season-summer", dec: -32.2 },
        Target { target_type: TargetType::Messier, id: Some("M7"), name_key: "target-m7", latin_key: "latin-scorpius", abbr: None, parent_key: Some("target-scorpion"), season_key: "season-summer", dec: -34.8 },
        Target { target_type: TargetType::Messier, id: Some("M8"), name_key: "target-m8", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -24.4 },
        Target { target_type: TargetType::Messier, id: Some("M9"), name_key: "target-m9", latin_key: "latin-ophiuchus", abbr: None, parent_key: Some("target-ophiuchus"), season_key: "season-summer", dec: -18.5 },
        Target { target_type: TargetType::Messier, id: Some("M10"), name_key: "target-m10", latin_key: "latin-ophiuchus", abbr: None, parent_key: Some("target-ophiuchus"), season_key: "season-summer", dec: -4.1 },
        Target { target_type: TargetType::Messier, id: Some("M11"), name_key: "target-m11", latin_key: "latin-scutum", abbr: None, parent_key: Some("target-cu-de-sobieski"), season_key: "season-summer", dec: -6.3 },
        Target { target_type: TargetType::Messier, id: Some("M12"), name_key: "target-m12", latin_key: "latin-ophiuchus", abbr: None, parent_key: Some("target-ophiuchus"), season_key: "season-summer", dec: -2.0 },
        Target { target_type: TargetType::Messier, id: Some("M13"), name_key: "target-m13", latin_key: "latin-hercules", abbr: None, parent_key: Some("target-hercule"), season_key: "season-summer", dec: 36.5 },
        Target { target_type: TargetType::Messier, id: Some("M14"), name_key: "target-m14", latin_key: "latin-ophiuchus", abbr: None, parent_key: Some("target-ophiuchus"), season_key: "season-summer", dec: -3.2 },
        Target { target_type: TargetType::Messier, id: Some("M15"), name_key: "target-m15", latin_key: "latin-pegasus", abbr: None, parent_key: Some("target-pgase"), season_key: "season-autumn", dec: 12.2 },
        Target { target_type: TargetType::Messier, id: Some("M16"), name_key: "target-m16", latin_key: "latin-serpens", abbr: None, parent_key: Some("target-serpent"), season_key: "season-summer", dec: -13.8 },
        Target { target_type: TargetType::Messier, id: Some("M17"), name_key: "target-m17", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -16.2 },
        Target { target_type: TargetType::Messier, id: Some("M18"), name_key: "target-m18", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -17.1 },
        Target { target_type: TargetType::Messier, id: Some("M19"), name_key: "target-m19", latin_key: "latin-ophiuchus", abbr: None, parent_key: Some("target-ophiuchus"), season_key: "season-summer", dec: -26.3 },
        Target { target_type: TargetType::Messier, id: Some("M20"), name_key: "target-m20", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -23.0 },
        Target { target_type: TargetType::Messier, id: Some("M21"), name_key: "target-m21", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -22.5 },
        Target { target_type: TargetType::Messier, id: Some("M22"), name_key: "target-m22", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -23.9 },
        Target { target_type: TargetType::Messier, id: Some("M23"), name_key: "target-m23", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -19.0 },
        Target { target_type: TargetType::Messier, id: Some("M24"), name_key: "target-m24", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -18.4 },
        Target { target_type: TargetType::Messier, id: Some("M25"), name_key: "target-m25", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -19.1 },
        Target { target_type: TargetType::Messier, id: Some("M26"), name_key: "target-m26", latin_key: "latin-scutum", abbr: None, parent_key: Some("target-cu-de-sobieski"), season_key: "season-summer", dec: -9.4 },
        Target { target_type: TargetType::Messier, id: Some("M27"), name_key: "target-m27", latin_key: "latin-vulpecula", abbr: None, parent_key: Some("target-petit-renard"), season_key: "season-summer", dec: 22.7 },
        Target { target_type: TargetType::Messier, id: Some("M28"), name_key: "target-m28", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -24.9 },
        Target { target_type: TargetType::Messier, id: Some("M29"), name_key: "target-m29", latin_key: "latin-cygnus", abbr: None, parent_key: Some("target-cygne"), season_key: "season-summer", dec: 38.5 },
        Target { target_type: TargetType::Messier, id: Some("M30"), name_key: "target-m30", latin_key: "latin-capricornus", abbr: None, parent_key: Some("target-capricorne"), season_key: "season-autumn", dec: -23.2 },
        Target { target_type: TargetType::Messier, id: Some("M31"), name_key: "target-m31", latin_key: "latin-andromeda", abbr: None, parent_key: Some("target-andromde"), season_key: "season-autumn", dec: 41.3 },
        Target { target_type: TargetType::Messier, id: Some("M32"), name_key: "target-m32", latin_key: "latin-andromeda", abbr: None, parent_key: Some("target-andromde"), season_key: "season-autumn", dec: 40.9 },
        Target { target_type: TargetType::Messier, id: Some("M33"), name_key: "target-m33", latin_key: "latin-triangulum", abbr: None, parent_key: Some("target-triangle"), season_key: "season-autumn", dec: 30.7 },
        Target { target_type: TargetType::Messier, id: Some("M34"), name_key: "target-m34", latin_key: "latin-perseus", abbr: None, parent_key: Some("target-perse"), season_key: "season-autumn", dec: 42.8 },
        Target { target_type: TargetType::Messier, id: Some("M35"), name_key: "target-m35", latin_key: "latin-gemini", abbr: None, parent_key: Some("target-gmeaux"), season_key: "season-winter", dec: 24.3 },
        Target { target_type: TargetType::Messier, id: Some("M36"), name_key: "target-m36", latin_key: "latin-auriga", abbr: None, parent_key: Some("target-cocher"), season_key: "season-winter", dec: 34.1 },
        Target { target_type: TargetType::Messier, id: Some("M37"), name_key: "target-m37", latin_key: "latin-auriga", abbr: None, parent_key: Some("target-cocher"), season_key: "season-winter", dec: 32.5 },
        Target { target_type: TargetType::Messier, id: Some("M38"), name_key: "target-m38", latin_key: "latin-auriga", abbr: None, parent_key: Some("target-cocher"), season_key: "season-winter", dec: 35.8 },
        Target { target_type: TargetType::Messier, id: Some("M39"), name_key: "target-m39", latin_key: "latin-cygnus", abbr: None, parent_key: Some("target-cygne"), season_key: "season-summer", dec: 48.4 },
        Target { target_type: TargetType::Messier, id: Some("M40"), name_key: "target-m40", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 58.1 },
        Target { target_type: TargetType::Messier, id: Some("M41"), name_key: "target-m41", latin_key: "latin-canis-major", abbr: None, parent_key: Some("target-grand-chien"), season_key: "season-winter", dec: -20.7 },
        Target { target_type: TargetType::Messier, id: Some("M42"), name_key: "target-m42", latin_key: "latin-orion", abbr: None, parent_key: Some("target-orion"), season_key: "season-winter", dec: -5.4 },
        Target { target_type: TargetType::Messier, id: Some("M43"), name_key: "target-m43", latin_key: "latin-orion", abbr: None, parent_key: Some("target-orion"), season_key: "season-winter", dec: -5.3 },
        Target { target_type: TargetType::Messier, id: Some("M44"), name_key: "target-m44", latin_key: "latin-cancer", abbr: None, parent_key: Some("target-cancer"), season_key: "season-spring", dec: 19.7 },
        Target { target_type: TargetType::Messier, id: Some("M45"), name_key: "target-m45", latin_key: "latin-taurus", abbr: None, parent_key: Some("target-taureau"), season_key: "season-winter", dec: 24.1 },
        Target { target_type: TargetType::Messier, id: Some("M46"), name_key: "target-m46", latin_key: "latin-puppis", abbr: None, parent_key: Some("target-poupe"), season_key: "season-winter", dec: -14.8 },
        Target { target_type: TargetType::Messier, id: Some("M47"), name_key: "target-m47", latin_key: "latin-puppis", abbr: None, parent_key: Some("target-poupe"), season_key: "season-winter", dec: -14.4 },
        Target { target_type: TargetType::Messier, id: Some("M48"), name_key: "target-m48", latin_key: "latin-hydra", abbr: None, parent_key: Some("target-hydre"), season_key: "season-winter", dec: -5.8 },
        Target { target_type: TargetType::Messier, id: Some("M49"), name_key: "target-m49", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 8.0 },
        Target { target_type: TargetType::Messier, id: Some("M50"), name_key: "target-m50", latin_key: "latin-monoceros", abbr: None, parent_key: Some("target-licorne"), season_key: "season-winter", dec: -8.3 },
        Target { target_type: TargetType::Messier, id: Some("M51"), name_key: "target-m51", latin_key: "latin-canes-venatici", abbr: None, parent_key: Some("target-chiens-de-chasse"), season_key: "season-spring", dec: 47.2 },
        Target { target_type: TargetType::Messier, id: Some("M52"), name_key: "target-m52", latin_key: "latin-cassiopeia", abbr: None, parent_key: Some("target-cassiope"), season_key: "season-autumn", dec: 61.6 },
        Target { target_type: TargetType::Messier, id: Some("M53"), name_key: "target-m53", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 18.2 },
        Target { target_type: TargetType::Messier, id: Some("M54"), name_key: "target-m54", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -30.5 },
        Target { target_type: TargetType::Messier, id: Some("M55"), name_key: "target-m55", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -31.0 },
        Target { target_type: TargetType::Messier, id: Some("M56"), name_key: "target-m56", latin_key: "latin-lyra", abbr: None, parent_key: Some("target-lyre"), season_key: "season-summer", dec: 30.2 },
        Target { target_type: TargetType::Messier, id: Some("M57"), name_key: "target-m57", latin_key: "latin-lyra", abbr: None, parent_key: Some("target-lyre"), season_key: "season-summer", dec: 33.0 },
        Target { target_type: TargetType::Messier, id: Some("M58"), name_key: "target-m58", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 11.8 },
        Target { target_type: TargetType::Messier, id: Some("M59"), name_key: "target-m59", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 11.6 },
        Target { target_type: TargetType::Messier, id: Some("M60"), name_key: "target-m60", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 11.5 },
        Target { target_type: TargetType::Messier, id: Some("M61"), name_key: "target-m61", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 4.5 },
        Target { target_type: TargetType::Messier, id: Some("M62"), name_key: "target-m62", latin_key: "latin-ophiuchus", abbr: None, parent_key: Some("target-ophiuchus"), season_key: "season-summer", dec: -30.1 },
        Target { target_type: TargetType::Messier, id: Some("M63"), name_key: "target-m63", latin_key: "latin-canes-venatici", abbr: None, parent_key: Some("target-chiens-de-chasse"), season_key: "season-spring", dec: 42.0 },
        Target { target_type: TargetType::Messier, id: Some("M64"), name_key: "target-m64", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 21.7 },
        Target { target_type: TargetType::Messier, id: Some("M65"), name_key: "target-m65", latin_key: "latin-leo", abbr: None, parent_key: Some("target-lion"), season_key: "season-spring", dec: 13.1 },
        Target { target_type: TargetType::Messier, id: Some("M66"), name_key: "target-m66", latin_key: "latin-leo", abbr: None, parent_key: Some("target-lion"), season_key: "season-spring", dec: 13.0 },
        Target { target_type: TargetType::Messier, id: Some("M67"), name_key: "target-m67", latin_key: "latin-cancer", abbr: None, parent_key: Some("target-cancer"), season_key: "season-spring", dec: 11.8 },
        Target { target_type: TargetType::Messier, id: Some("M68"), name_key: "target-m68", latin_key: "latin-hydra", abbr: None, parent_key: Some("target-hydre"), season_key: "season-spring", dec: -26.7 },
        Target { target_type: TargetType::Messier, id: Some("M69"), name_key: "target-m69", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -32.3 },
        Target { target_type: TargetType::Messier, id: Some("M70"), name_key: "target-m70", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -32.3 },
        Target { target_type: TargetType::Messier, id: Some("M71"), name_key: "target-m71", latin_key: "latin-sagitta", abbr: None, parent_key: Some("target-flche"), season_key: "season-summer", dec: 18.8 },
        Target { target_type: TargetType::Messier, id: Some("M72"), name_key: "target-m72", latin_key: "latin-aquarius", abbr: None, parent_key: Some("target-verseau"), season_key: "season-autumn", dec: -12.5 },
        Target { target_type: TargetType::Messier, id: Some("M73"), name_key: "target-m73", latin_key: "latin-aquarius", abbr: None, parent_key: Some("target-verseau"), season_key: "season-autumn", dec: -12.6 },
        Target { target_type: TargetType::Messier, id: Some("M74"), name_key: "target-m74", latin_key: "latin-pisces", abbr: None, parent_key: Some("target-poissons"), season_key: "season-autumn", dec: 15.8 },
        Target { target_type: TargetType::Messier, id: Some("M75"), name_key: "target-m75", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -21.9 },
        Target { target_type: TargetType::Messier, id: Some("M76"), name_key: "target-m76", latin_key: "latin-perseus", abbr: None, parent_key: Some("target-perse"), season_key: "season-autumn", dec: 51.6 },
        Target { target_type: TargetType::Messier, id: Some("M77"), name_key: "target-m77", latin_key: "latin-cetus", abbr: None, parent_key: Some("target-baleine"), season_key: "season-autumn", dec: -0.0 },
        Target { target_type: TargetType::Messier, id: Some("M78"), name_key: "target-m78", latin_key: "latin-orion", abbr: None, parent_key: Some("target-orion"), season_key: "season-winter", dec: 0.1 },
        Target { target_type: TargetType::Messier, id: Some("M79"), name_key: "target-m79", latin_key: "latin-lepus", abbr: None, parent_key: Some("target-livre"), season_key: "season-winter", dec: -24.5 },
        Target { target_type: TargetType::Messier, id: Some("M80"), name_key: "target-m80", latin_key: "latin-scorpius", abbr: None, parent_key: Some("target-scorpion"), season_key: "season-summer", dec: -23.0 },
        Target { target_type: TargetType::Messier, id: Some("M81"), name_key: "target-m81", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 69.1 },
        Target { target_type: TargetType::Messier, id: Some("M82"), name_key: "target-m82", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 69.7 },
        Target { target_type: TargetType::Messier, id: Some("M83"), name_key: "target-m83", latin_key: "latin-hydra", abbr: None, parent_key: Some("target-hydre"), season_key: "season-spring", dec: -29.9 },
        Target { target_type: TargetType::Messier, id: Some("M84"), name_key: "target-m84", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 12.9 },
        Target { target_type: TargetType::Messier, id: Some("M85"), name_key: "target-m85", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 18.2 },
        Target { target_type: TargetType::Messier, id: Some("M86"), name_key: "target-m86", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 12.9 },
        Target { target_type: TargetType::Messier, id: Some("M87"), name_key: "target-m87", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 12.4 },
        Target { target_type: TargetType::Messier, id: Some("M88"), name_key: "target-m88", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 14.4 },
        Target { target_type: TargetType::Messier, id: Some("M89"), name_key: "target-m89", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 12.6 },
        Target { target_type: TargetType::Messier, id: Some("M90"), name_key: "target-m90", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 13.2 },
        Target { target_type: TargetType::Messier, id: Some("M91"), name_key: "target-m91", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 14.5 },
        Target { target_type: TargetType::Messier, id: Some("M92"), name_key: "target-m92", latin_key: "latin-hercules", abbr: None, parent_key: Some("target-hercule"), season_key: "season-summer", dec: 43.1 },
        Target { target_type: TargetType::Messier, id: Some("M93"), name_key: "target-m93", latin_key: "latin-puppis", abbr: None, parent_key: Some("target-poupe"), season_key: "season-winter", dec: -23.8 },
        Target { target_type: TargetType::Messier, id: Some("M94"), name_key: "target-m94", latin_key: "latin-canes-venatici", abbr: None, parent_key: Some("target-chiens-de-chasse"), season_key: "season-spring", dec: 41.1 },
        Target { target_type: TargetType::Messier, id: Some("M95"), name_key: "target-m95", latin_key: "latin-leo", abbr: None, parent_key: Some("target-lion"), season_key: "season-spring", dec: 11.7 },
        Target { target_type: TargetType::Messier, id: Some("M96"), name_key: "target-m96", latin_key: "latin-leo", abbr: None, parent_key: Some("target-lion"), season_key: "season-spring", dec: 11.8 },
        Target { target_type: TargetType::Messier, id: Some("M97"), name_key: "target-m97", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 55.0 },
        Target { target_type: TargetType::Messier, id: Some("M98"), name_key: "target-m98", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 14.9 },
        Target { target_type: TargetType::Messier, id: Some("M99"), name_key: "target-m99", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 14.4 },
        Target { target_type: TargetType::Messier, id: Some("M100"), name_key: "target-m100", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 15.8 },
        Target { target_type: TargetType::Messier, id: Some("M101"), name_key: "target-m101", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 54.4 },
        Target { target_type: TargetType::Messier, id: Some("M102"), name_key: "target-m102", latin_key: "latin-draco", abbr: None, parent_key: Some("target-dragon"), season_key: "season-circumpolar-n", dec: 55.8 },
        Target { target_type: TargetType::Messier, id: Some("M103"), name_key: "target-m103", latin_key: "latin-cassiopeia", abbr: None, parent_key: Some("target-cassiope"), season_key: "season-autumn", dec: 60.7 },
        Target { target_type: TargetType::Messier, id: Some("M104"), name_key: "target-m104", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: -11.6 },
        Target { target_type: TargetType::Messier, id: Some("M105"), name_key: "target-m105", latin_key: "latin-leo", abbr: None, parent_key: Some("target-lion"), season_key: "season-spring", dec: 12.6 },
        Target { target_type: TargetType::Messier, id: Some("M106"), name_key: "target-m106", latin_key: "latin-canes-venatici", abbr: None, parent_key: Some("target-chiens-de-chasse"), season_key: "season-spring", dec: 47.3 },
        Target { target_type: TargetType::Messier, id: Some("M107"), name_key: "target-m107", latin_key: "latin-ophiuchus", abbr: None, parent_key: Some("target-ophiuchus"), season_key: "season-summer", dec: -13.0 },
        Target { target_type: TargetType::Messier, id: Some("M108"), name_key: "target-m108", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 53.4 },
        Target { target_type: TargetType::Messier, id: Some("M109"), name_key: "target-m109", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 53.4 },
        Target { target_type: TargetType::Messier, id: Some("M110"), name_key: "target-m110", latin_key: "latin-andromeda", abbr: None, parent_key: Some("target-andromde"), season_key: "season-autumn", dec: 41.7 },
        Target { target_type: TargetType::Galaxy, id: Some("M31"), name_key: "target-m31", latin_key: "latin-andromeda", abbr: None, parent_key: Some("target-andromde"), season_key: "season-autumn", dec: 41.26 },
        Target { target_type: TargetType::Galaxy, id: Some("M33"), name_key: "target-m33", latin_key: "latin-triangulum", abbr: None, parent_key: Some("target-triangle"), season_key: "season-autumn", dec: 30.66 },
        Target { target_type: TargetType::Galaxy, id: Some("M51"), name_key: "target-m51", latin_key: "latin-canes-venatici", abbr: Some("NGC 5194"), parent_key: Some("target-chiens-de-chasse"), season_key: "season-spring", dec: 47.19 },
        Target { target_type: TargetType::Galaxy, id: Some("M81"), name_key: "target-m81", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 69.06 },
        Target { target_type: TargetType::Galaxy, id: Some("M82"), name_key: "target-m82", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 69.68 },
        Target { target_type: TargetType::Galaxy, id: Some("M101"), name_key: "target-m101", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 54.35 },
        Target { target_type: TargetType::Galaxy, id: Some("M63"), name_key: "target-m63", latin_key: "latin-canes-venatici", abbr: None, parent_key: Some("target-chiens-de-chasse"), season_key: "season-spring", dec: 42.03 },
        Target { target_type: TargetType::Galaxy, id: Some("M64"), name_key: "target-m64", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 21.68 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 4565"), name_key: "target-ngc-4565", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 25.98 },
        Target { target_type: TargetType::Galaxy, id: Some("M104"), name_key: "target-m104", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: -11.62 },
        Target { target_type: TargetType::Galaxy, id: Some("LMC"), name_key: "target-lmc", latin_key: "latin-dorado", abbr: None, parent_key: Some("target-dorade"), season_key: "season-circumpolar-s", dec: -69.75 },
        Target { target_type: TargetType::Galaxy, id: Some("SMC"), name_key: "target-smc", latin_key: "latin-tucana", abbr: None, parent_key: Some("target-toucan"), season_key: "season-circumpolar-s", dec: -72.80 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 5128"), name_key: "target-ngc-5128", latin_key: "latin-centaurus", abbr: None, parent_key: Some("target-centaure"), season_key: "season-spring", dec: -43.01 },
        Target { target_type: TargetType::Galaxy, id: Some("M83"), name_key: "target-m83", latin_key: "latin-hydra", abbr: None, parent_key: Some("target-hydre"), season_key: "season-spring", dec: -29.86 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 253"), name_key: "target-ngc-253", latin_key: "latin-sculptor", abbr: None, parent_key: Some("target-sculpteur"), season_key: "season-autumn", dec: -25.29 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 300"), name_key: "target-ngc-300", latin_key: "latin-sculptor", abbr: None, parent_key: Some("target-sculpteur"), season_key: "season-autumn", dec: -37.68 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 4945"), name_key: "target-ngc-4945", latin_key: "latin-centaurus", abbr: None, parent_key: Some("target-centaure"), season_key: "season-spring", dec: -49.47 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 1316"), name_key: "target-ngc-1316", latin_key: "latin-fornax", abbr: None, parent_key: Some("target-fourneau"), season_key: "season-autumn", dec: -37.20 },
        Target { target_type: TargetType::Galaxy, id: Some("M65"), name_key: "target-m65", latin_key: "latin-leo", abbr: None, parent_key: Some("target-lion"), season_key: "season-spring", dec: 13.09 },
        Target { target_type: TargetType::Galaxy, id: Some("M66"), name_key: "target-m66", latin_key: "latin-leo", abbr: None, parent_key: Some("target-lion"), season_key: "season-spring", dec: 12.99 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 3628"), name_key: "target-ngc-3628", latin_key: "latin-leo", abbr: None, parent_key: Some("target-lion"), season_key: "season-spring", dec: 13.59 },
        Target { target_type: TargetType::Galaxy, id: Some("M87"), name_key: "target-m87", latin_key: "latin-virgo", abbr: None, parent_key: Some("target-vierge"), season_key: "season-spring", dec: 12.39 },
        Target { target_type: TargetType::Galaxy, id: Some("NGC 4038"), name_key: "target-ngc-4038", latin_key: "latin-corvus", abbr: None, parent_key: Some("target-corbeau"), season_key: "season-spring", dec: -18.87 },
        Target { target_type: TargetType::Nebula, id: Some("M42"), name_key: "target-m42", latin_key: "latin-orion", abbr: None, parent_key: Some("target-orion"), season_key: "season-winter", dec: -5.39 },
        Target { target_type: TargetType::Nebula, id: Some("IC 434"), name_key: "target-ic-434", latin_key: "latin-orion", abbr: None, parent_key: Some("target-orion"), season_key: "season-winter", dec: -2.45 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 2024"), name_key: "target-ngc-2024", latin_key: "latin-orion", abbr: None, parent_key: Some("target-orion"), season_key: "season-winter", dec: -1.86 },
        Target { target_type: TargetType::Nebula, id: Some("Sh2-276"), name_key: "target-sh2-276", latin_key: "latin-orion", abbr: None, parent_key: Some("target-orion"), season_key: "season-winter", dec: -1.00 },
        Target { target_type: TargetType::Nebula, id: Some("IC 2118"), name_key: "target-ic-2118", latin_key: "latin-eridanus", abbr: None, parent_key: Some("target-ridan"), season_key: "season-winter", dec: -7.25 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 2237"), name_key: "target-ngc-2237", latin_key: "latin-monoceros", abbr: None, parent_key: Some("target-licorne"), season_key: "season-winter", dec: 4.97 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 2264"), name_key: "target-ngc-2264", latin_key: "latin-monoceros", abbr: None, parent_key: Some("target-licorne"), season_key: "season-winter", dec: 9.89 },
        Target { target_type: TargetType::Nebula, id: Some("IC 405"), name_key: "target-ic-405", latin_key: "latin-auriga", abbr: None, parent_key: Some("target-cocher"), season_key: "season-winter", dec: 34.35 },
        Target { target_type: TargetType::Nebula, id: Some("IC 443"), name_key: "target-ic-443", latin_key: "latin-gemini", abbr: None, parent_key: Some("target-gmeaux"), season_key: "season-winter", dec: 22.47 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 2174"), name_key: "target-ngc-2174", latin_key: "latin-orion", abbr: None, parent_key: Some("target-orion"), season_key: "season-winter", dec: 20.50 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 1499"), name_key: "target-ngc-1499", latin_key: "latin-perseus", abbr: None, parent_key: Some("target-perse"), season_key: "season-winter", dec: 36.42 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 3372"), name_key: "target-ngc-3372", latin_key: "latin-carina", abbr: None, parent_key: Some("target-carne"), season_key: "season-spring", dec: -59.87 },
        Target { target_type: TargetType::Nebula, id: Some("IC 2944"), name_key: "target-ic-2944", latin_key: "latin-centaurus", abbr: None, parent_key: Some("target-centaure"), season_key: "season-spring", dec: -63.02 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 3576"), name_key: "target-ngc-3576", latin_key: "latin-carina", abbr: None, parent_key: Some("target-carne"), season_key: "season-spring", dec: -61.30 },
        Target { target_type: TargetType::Nebula, id: Some("M97"), name_key: "target-m97", latin_key: "latin-ursa-major", abbr: None, parent_key: Some("target-grande-ourse"), season_key: "season-spring", dec: 55.02 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 3242"), name_key: "target-ngc-3242", latin_key: "latin-hydra", abbr: None, parent_key: Some("target-hydre"), season_key: "season-spring", dec: -18.63 },
        Target { target_type: TargetType::Nebula, id: Some("M16"), name_key: "target-m16", latin_key: "latin-serpens", abbr: None, parent_key: Some("target-serpent"), season_key: "season-spring", dec: -13.81 },
        Target { target_type: TargetType::Nebula, id: Some("M8"), name_key: "target-m8", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -24.38 },
        Target { target_type: TargetType::Nebula, id: Some("M20"), name_key: "target-m20", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -23.03 },
        Target { target_type: TargetType::Nebula, id: Some("M16"), name_key: "target-m16", latin_key: "latin-serpens", abbr: None, parent_key: Some("target-serpent"), season_key: "season-summer", dec: -13.80 },
        Target { target_type: TargetType::Nebula, id: Some("M17"), name_key: "target-m17", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -16.18 },
        Target { target_type: TargetType::Nebula, id: Some("IC 4604"), name_key: "target-ic-4604", latin_key: "latin-ophiuchus", abbr: None, parent_key: Some("target-ophiuchus"), season_key: "season-summer", dec: -24.35 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6357"), name_key: "target-ngc-6357", latin_key: "latin-scorpius", abbr: None, parent_key: Some("target-scorpion"), season_key: "season-summer", dec: -34.20 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6334"), name_key: "target-ngc-6334", latin_key: "latin-scorpius", abbr: None, parent_key: Some("target-scorpion"), season_key: "season-summer", dec: -35.95 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 7000"), name_key: "target-ngc-7000", latin_key: "latin-cygnus", abbr: None, parent_key: Some("target-cygne"), season_key: "season-summer", dec: 44.33 },
        Target { target_type: TargetType::Nebula, id: Some("IC 5070"), name_key: "target-ic-5070", latin_key: "latin-cygnus", abbr: None, parent_key: Some("target-cygne"), season_key: "season-summer", dec: 44.13 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6960"), name_key: "target-ngc-6960", latin_key: "latin-cygnus", abbr: None, parent_key: Some("target-cygne"), season_key: "season-summer", dec: 30.70 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6992"), name_key: "target-ngc-6992", latin_key: "latin-cygnus", abbr: None, parent_key: Some("target-cygne"), season_key: "season-summer", dec: 31.70 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 6888"), name_key: "target-ngc-6888", latin_key: "latin-cygnus", abbr: None, parent_key: Some("target-cygne"), season_key: "season-summer", dec: 38.35 },
        Target { target_type: TargetType::Nebula, id: Some("IC 1318"), name_key: "target-ic-1318", latin_key: "latin-cygnus", abbr: None, parent_key: Some("target-cygne"), season_key: "season-summer", dec: 40.25 },
        Target { target_type: TargetType::Nebula, id: Some("LDN 673"), name_key: "target-ldn-673", latin_key: "latin-aquila", abbr: None, parent_key: Some("target-aigle"), season_key: "season-summer", dec: 1.00 },
        Target { target_type: TargetType::Nebula, id: Some("IC 1805"), name_key: "target-ic-1805", latin_key: "latin-cassiopeia", abbr: None, parent_key: Some("target-cassiope"), season_key: "season-autumn", dec: 61.45 },
        Target { target_type: TargetType::Nebula, id: Some("IC 1848"), name_key: "target-ic-1848", latin_key: "latin-cassiopeia", abbr: None, parent_key: Some("target-cassiope"), season_key: "season-autumn", dec: 60.40 },
        Target { target_type: TargetType::Nebula, id: Some("IC 1396"), name_key: "target-ic-1396", latin_key: "latin-cepheus", abbr: None, parent_key: Some("target-cphe"), season_key: "season-autumn", dec: 57.50 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 281"), name_key: "target-ngc-281", latin_key: "latin-cassiopeia", abbr: None, parent_key: Some("target-cassiope"), season_key: "season-autumn", dec: 56.62 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 7635"), name_key: "target-ngc-7635", latin_key: "latin-cassiopeia", abbr: None, parent_key: Some("target-cassiope"), season_key: "season-autumn", dec: 61.20 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 7023"), name_key: "target-ngc-7023", latin_key: "latin-cepheus", abbr: None, parent_key: Some("target-cphe"), season_key: "season-autumn", dec: 68.16 },
        Target { target_type: TargetType::Nebula, id: Some("NGC 7293"), name_key: "target-ngc-7293", latin_key: "latin-aquarius", abbr: None, parent_key: Some("target-verseau"), season_key: "season-autumn", dec: -20.80 },
        Target { target_type: TargetType::Cluster, id: Some("M45"), name_key: "target-m45", latin_key: "latin-taurus", abbr: None, parent_key: Some("target-taureau"), season_key: "season-winter", dec: 24.12 },
        Target { target_type: TargetType::Cluster, id: Some("M44"), name_key: "target-m44", latin_key: "latin-cancer", abbr: None, parent_key: Some("target-cancer"), season_key: "season-spring", dec: 19.67 },
        Target { target_type: TargetType::Cluster, id: Some("M35"), name_key: "target-m35", latin_key: "latin-gemini", abbr: None, parent_key: Some("target-gmeaux"), season_key: "season-winter", dec: 24.33 },
        Target { target_type: TargetType::Cluster, id: Some("M36"), name_key: "target-m36", latin_key: "latin-auriga", abbr: None, parent_key: Some("target-cocher"), season_key: "season-winter", dec: 34.13 },
        Target { target_type: TargetType::Cluster, id: Some("M37"), name_key: "target-m37", latin_key: "latin-auriga", abbr: None, parent_key: Some("target-cocher"), season_key: "season-winter", dec: 32.55 },
        Target { target_type: TargetType::Cluster, id: Some("M38"), name_key: "target-m38", latin_key: "latin-auriga", abbr: None, parent_key: Some("target-cocher"), season_key: "season-winter", dec: 35.83 },
        Target { target_type: TargetType::Cluster, id: Some("M41"), name_key: "target-m41", latin_key: "latin-canis-major", abbr: None, parent_key: Some("target-grand-chien"), season_key: "season-winter", dec: -20.73 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 869"), name_key: "target-ngc-869", latin_key: "latin-perseus", abbr: None, parent_key: Some("target-perse"), season_key: "season-autumn", dec: 57.13 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 884"), name_key: "target-ngc-884", latin_key: "latin-perseus", abbr: None, parent_key: Some("target-perse"), season_key: "season-autumn", dec: 57.15 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 2244"), name_key: "target-ngc-2244", latin_key: "latin-monoceros", abbr: None, parent_key: Some("target-licorne"), season_key: "season-winter", dec: 4.87 },
        Target { target_type: TargetType::Cluster, id: Some("M3"), name_key: "target-m3", latin_key: "latin-canes-venatici", abbr: None, parent_key: Some("target-chiens-de-chasse"), season_key: "season-spring", dec: 28.38 },
        Target { target_type: TargetType::Cluster, id: Some("M5"), name_key: "target-m5", latin_key: "latin-serpens", abbr: None, parent_key: Some("target-serpent"), season_key: "season-spring", dec: 2.08 },
        Target { target_type: TargetType::Cluster, id: Some("M13"), name_key: "target-m13", latin_key: "latin-hercules", abbr: None, parent_key: Some("target-hercule"), season_key: "season-summer", dec: 36.46 },
        Target { target_type: TargetType::Cluster, id: Some("M53"), name_key: "target-m53", latin_key: "latin-coma-berenices", abbr: None, parent_key: Some("target-chevelure-de-brnice"), season_key: "season-spring", dec: 18.17 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 5139"), name_key: "target-ngc-5139", latin_key: "latin-centaurus", abbr: None, parent_key: Some("target-centaure"), season_key: "season-spring", dec: -47.48 },
        Target { target_type: TargetType::Cluster, id: Some("M11"), name_key: "target-m11", latin_key: "latin-scutum", abbr: None, parent_key: Some("target-cu-de-sobieski"), season_key: "season-summer", dec: -6.27 },
        Target { target_type: TargetType::Cluster, id: Some("M22"), name_key: "target-m22", latin_key: "latin-sagittarius", abbr: None, parent_key: Some("target-sagittaire"), season_key: "season-summer", dec: -23.90 },
        Target { target_type: TargetType::Cluster, id: Some("M6"), name_key: "target-m6", latin_key: "latin-scorpius", abbr: None, parent_key: Some("target-scorpion"), season_key: "season-summer", dec: -32.22 },
        Target { target_type: TargetType::Cluster, id: Some("M7"), name_key: "target-m7", latin_key: "latin-scorpius", abbr: None, parent_key: Some("target-scorpion"), season_key: "season-summer", dec: -34.82 },
        Target { target_type: TargetType::Cluster, id: Some("M92"), name_key: "target-m92", latin_key: "latin-hercules", abbr: None, parent_key: Some("target-hercule"), season_key: "season-summer", dec: 43.13 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 6231"), name_key: "target-ngc-6231", latin_key: "latin-scorpius", abbr: None, parent_key: Some("target-scorpion"), season_key: "season-summer", dec: -41.80 },
        Target { target_type: TargetType::Cluster, id: Some("M15"), name_key: "target-m15", latin_key: "latin-pegasus", abbr: None, parent_key: Some("target-pgase"), season_key: "season-autumn", dec: 12.17 },
        Target { target_type: TargetType::Cluster, id: Some("M2"), name_key: "target-m2", latin_key: "latin-aquarius", abbr: None, parent_key: Some("target-verseau"), season_key: "season-autumn", dec: -0.82 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 104"), name_key: "target-ngc-104", latin_key: "latin-tucana", abbr: None, parent_key: Some("target-toucan"), season_key: "season-autumn", dec: -72.08 },
        Target { target_type: TargetType::Cluster, id: Some("M34"), name_key: "target-m34", latin_key: "latin-perseus", abbr: None, parent_key: Some("target-perse"), season_key: "season-autumn", dec: 42.78 },
        Target { target_type: TargetType::Cluster, id: Some("NGC 457"), name_key: "target-ngc-457", latin_key: "latin-cassiopeia", abbr: None, parent_key: Some("target-cassiope"), season_key: "season-autumn", dec: 58.33 },
    ]
});

fn get_color(t: f64, levels: &[f64]) -> HSLColor {
    // On cherche l'index du palier
    let mut idx = 0;
    for (i, &level) in levels.iter().enumerate() {
        if t < level {
            idx = i;
            break;
        }
        idx = i + 1;
    }
    
    let max_idx = levels.len();
    let ratio = (idx as f64 / max_idx as f64).clamp(0.0, 1.0);
    
    // Palette: du bleu (froid/sombre) au jaune/blanc (chaud/brillant)
    let h = 0.7 - (ratio * 0.75); // De 0.7 (bleu) à -0.05 (rouge/orangé)
    let s = 0.9;
    let l = 0.1 + (ratio * 0.8); // De 0.1 (sombre) à 0.9 (lumineux)
    
    // Si le temps est très court (entre 0 et 0.5s), on met en gris très sombre
    if t <= 0.5 {
        return HSLColor(0.0, 0.0, 0.05);
    }
    
    HSLColor(h, s, l)
}

fn calculer_npf(focale: f64, ouverture: f64, pixel: f64, dec: f64) -> f64 {
    let cos_dec = (dec.abs().to_radians()).cos().abs();
    let denom = focale * cos_dec;
    if denom.abs() < f64::EPSILON {
        return f64::MAX;
    }
    DEFAULT_K_FACTOR * (16.9 * ouverture + 0.1 * focale + 13.7 * pixel) / denom
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
    search_query: String,
    bundles: HashMap<String, FluentBundle<FluentResource>>,
}

impl Default for NpfApp {
    fn default() -> Self {
        let mut bundles = HashMap::new();
        
        // Chargement du français
        let res_fr = FluentResource::try_new(include_str!("i18n/fr.ftl").to_owned()).expect("Failed to parse fr.ftl");
        let lang_fr: LanguageIdentifier = "fr".parse().expect("Parsing failed");
        let mut bundle_fr = FluentBundle::new(vec![lang_fr]);
        bundle_fr.add_resource(res_fr).expect("Failed to add resource");
        bundles.insert("fr".to_string(), bundle_fr);

        // Chargement de l'anglais
        let res_en = FluentResource::try_new(include_str!("i18n/en.ftl").to_owned()).expect("Failed to parse en.ftl");
        let lang_en: LanguageIdentifier = "en".parse().expect("Parsing failed");
        let mut bundle_en = FluentBundle::new(vec![lang_en]);
        bundle_en.add_resource(res_en).expect("Failed to add resource");
        bundles.insert("en".to_string(), bundle_en);

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
            search_query: String::new(),
            bundles,
        }
    }
}

impl NpfApp {
    fn is_target_visible(&self, target: &Target) -> bool {
        let latitude_visible = target.dec > (self.settings.latitude - 80.0);

        let season_visible = if self.settings.selected_season == "Toutes" {
            true
        } else {
            let target_season = self.tr(target.season_key);
            let selected_season = self.get_season_tr(&self.settings.selected_season);
            target_season == selected_season || target.season_key.contains("circumpolaire")
        };

        latitude_visible && season_visible
    }

    fn matches_search(&self, target: &Target) -> bool {
        if self.search_query.is_empty() {
            return true;
        }
        let query = self.search_query.to_lowercase();
        let name = self.tr(target.name_key).to_lowercase();
        let latin = self.tr(target.latin_key).to_lowercase();
        
        name.contains(&query)
            || latin.contains(&query)
            || target.abbr.map(|a| a.to_lowercase().contains(&query)).unwrap_or(false)
            || target.id.map(|i| i.to_lowercase().contains(&query)).unwrap_or(false)
    }

    fn tr(&self, id: &str) -> String {
        let bundle = self.bundles.get(&self.settings.language).or_else(|| self.bundles.get("en")).unwrap();
        let pattern = bundle.get_message(id).and_then(|m| m.value()).expect("Translation missing");
        let mut errors = vec![];
        bundle.format_pattern(pattern, None, &mut errors).to_string()
    }

    fn tr_args(&self, id: &str, args: &FluentArgs) -> String {
        let bundle = self.bundles.get(&self.settings.language).or_else(|| self.bundles.get("en")).unwrap();
        let pattern = bundle.get_message(id).and_then(|m| m.value()).expect("Translation missing");
        let mut errors = vec![];
        bundle.format_pattern(pattern, Some(args), &mut errors).to_string()
    }

    fn get_season_tr(&self, season: &str) -> String {
        match season {
            "Toutes" => self.tr("season-all"),
            "Printemps" => self.tr("season-spring"),
            "Été" => self.tr("season-summer"),
            "Automne" => self.tr("season-autumn"),
            "Hiver" => self.tr("season-winter"),
            "Circumpolaire N" => self.tr("season-circumpolar-n"),
            "Circumpolaire S" => self.tr("season-circumpolar-s"),
            _ => season.to_string(),
        }
    }

    #[allow(unused)]
    fn get_type_tr(&self, t: &TargetType) -> String {
        match t {
            TargetType::Constellation => self.tr("type-constellation"),
            TargetType::Messier => self.tr("type-messier"),
            TargetType::Nebula => self.tr("type-nebula"),
            TargetType::Galaxy => self.tr("type-galaxy"),
            TargetType::Cluster => self.tr("type-cluster"),
        }
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
            let equator_tr = self.tr("chart-equator-label");
            let target_name_string = self.selected_target.as_ref()
                .map(|t| t.id.map(|id| id.to_string()).unwrap_or_else(|| self.tr(t.name_key)))
                .unwrap_or(equator_tr);
            let target_name = &target_name_string;

            let lens = &self.settings.lenses[self.settings.selected_lens_idx];
            let sensor = &self.settings.sensors[self.settings.selected_sensor_idx];
            
            let f_min = lens.f_min;
            let mut f_max = lens.f_max;
            if (f_max - f_min).abs() < 1e-6 {
                f_max = f_min + 1.0;
            }
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
            // Augmentation à 20 paliers pour une coloration beaucoup plus fine
            for i in 1..20 {
                let l = 0.5 + (z_max * 2.5 - 0.5) * (i as f64) / 19.0;
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
                .x_labels(10)
                .y_labels(10)
                .z_labels(10)
                .axis_panel_style(WHITE.mix(0.1))
                .light_grid_style(BLACK.mix(0.1))
                .bold_grid_style(BLACK.mix(0.2))
                .draw()
                .unwrap();

            // Légendes des axes 3D dynamiques
            let label_style = ("sans-serif", 20).into_font().color(&BLACK);
            
            // X: Focale - Placée au milieu de l'axe X, décalée en Y/Z pour être hors de la boîte
            chart.draw_series(std::iter::once(Text::new(
                self.tr("chart-focal-label"),
                ((f_min + f_max) / 2.0, -z_max * 0.1, -10.0),
                label_style.clone().pos(plotters::style::text_anchor::Pos {
                    h_pos: plotters::style::text_anchor::HPos::Center,
                    v_pos: plotters::style::text_anchor::VPos::Top,
                }),
            ))).unwrap();

            // Y: Temps de pose - Placé au milieu de l'axe Y, décalé en X/Z
            chart.draw_series(std::iter::once(Text::new(
                self.tr("chart-exposure-label"),
                (f_min - (f_max-f_min)*0.1, z_max / 2.0, -10.0),
                label_style.clone().pos(plotters::style::text_anchor::Pos {
                    h_pos: plotters::style::text_anchor::HPos::Center,
                    v_pos: plotters::style::text_anchor::VPos::Bottom,
                }),
            ))).unwrap();

            // Z: Déclinaison - Placé au milieu de l'axe Z, décalé en X/Y sur l'axe en face
            chart.draw_series(std::iter::once(Text::new(
                self.tr("chart-declination-label"),
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
            let leg_text = format!("{} : {}", self.tr("lens-label"), target_name);
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
            let max_height = ctx.screen_rect().height() * 0.75;
            egui::Window::new(self.tr("settings-window-title"))
                .max_height(max_height)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.heading(self.tr("sensors-heading"));
                        let mut to_remove_sensor = None;
                        let num_sensors = self.settings.sensors.len();
                        let pixel_prefix = self.tr("pixel-prefix");
                        let pixel_suffix = self.tr("pixel-suffix");
                        for i in 0..num_sensors {
                            let sensor = &mut self.settings.sensors[i];
                            ui.horizontal(|ui| {
                                ui.text_edit_singleline(&mut sensor.name);
                                ui.add(egui::DragValue::new(&mut sensor.pixel_size)
                                    .speed(0.01)
                                    .clamp_range(0.1..=100.0)
                                    .prefix(&pixel_prefix)
                                    .suffix(&pixel_suffix));
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
                        if ui.button(self.tr("add-sensor-button")).clicked() {
                            self.settings.sensors.push(SensorConfig::default());
                        }

                        ui.separator();

                        ui.heading(self.tr("lenses-heading"));
                        let mut to_remove_lens = None;
                        let num_lenses = self.settings.lenses.len();
                        let fmin_prefix = self.tr("fmin-prefix");
                        let fmin_suffix = self.tr("fmin-suffix");
                        let fmax_prefix = self.tr("fmax-prefix");
                        let fmax_suffix = self.tr("fmax-suffix");
                        let nmin_prefix = self.tr("nmin-prefix");
                        let nmax_prefix = self.tr("nmax-prefix");
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
                                    ui.add(egui::DragValue::new(&mut lens.f_min)
                                        .speed(0.1)
                                        .clamp_range(0.1..=10000.0)
                                        .prefix(&fmin_prefix)
                                        .suffix(&fmin_suffix));
                                    ui.add(egui::DragValue::new(&mut lens.f_max)
                                        .speed(0.1)
                                        .clamp_range(0.1..=10000.0)
                                        .prefix(&fmax_prefix)
                                        .suffix(&fmax_suffix));
                                });
                                ui.horizontal(|ui| {
                                    ui.add(egui::DragValue::new(&mut lens.n_min)
                                        .speed(0.1)
                                        .clamp_range(0.1..=64.0)
                                        .prefix(&nmin_prefix));
                                    ui.add(egui::DragValue::new(&mut lens.n_max)
                                        .speed(0.1)
                                        .clamp_range(0.1..=64.0)
                                        .prefix(&nmax_prefix));
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
                        if ui.button(self.tr("add-lens-button")).clicked() {
                            self.settings.lenses.push(LensConfig {
                                name: self.tr("new-lens-name"),
                                f_min: 50.0,
                                f_max: 50.0,
                                n_min: 1.8,
                                n_max: 1.8,
                            });
                        }

                        ui.separator();
                        if ui.button(self.tr("close-apply-button")).clicked() {
                            self.show_settings = false;
                            self.needs_update = true;
                        }
                    });
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
                ui.menu_button(self.tr("lens-label"), |ui| {
                    for (i, lens) in self.settings.lenses.iter().enumerate() {
                        if ui.selectable_label(self.settings.selected_lens_idx == i, &lens.name).clicked() {
                            self.settings.selected_lens_idx = i;
                            self.needs_update = true;
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button(self.tr("sensor-label"), |ui| {
                    for (i, sensor) in self.settings.sensors.iter().enumerate() {
                        if ui.selectable_label(self.settings.selected_sensor_idx == i, &sensor.name).clicked() {
                            self.settings.selected_sensor_idx = i;
                            self.needs_update = true;
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button(self.tr("settings-label"), |ui| {
                    if ui.button(self.tr("manage-lenses-sensors")).clicked() {
                        self.show_settings = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.label("Langue / Language");
                    ui.horizontal(|ui| {
                        if ui.selectable_label(self.settings.language == "fr", "Français").clicked() {
                            self.settings.language = "fr".to_string();
                            self.needs_update = true;
                            ui.close_menu();
                        }
                        if ui.selectable_label(self.settings.language == "en", "English").clicked() {
                            self.settings.language = "en".to_string();
                            self.needs_update = true;
                            ui.close_menu();
                        }
                    });
                });

                ui.separator();

                ui.label(self.tr("season-label"));
                egui::ComboBox::from_id_source("season_filter")
                    .selected_text(self.get_season_tr(&self.settings.selected_season))
                    .show_ui(ui, |ui| {
                        let seasons = [
                            ("Toutes", "season-all"), 
                            ("Printemps", "season-spring"), 
                            ("Été", "season-summer"), 
                            ("Automne", "season-autumn"), 
                            ("Hiver", "season-winter"), 
                            ("Circumpolaire N", "season-circumpolar-n"), 
                            ("Circumpolaire S", "season-circumpolar-s")
                        ];
                        for (key, tr_id) in seasons {
                            if ui.selectable_label(self.settings.selected_season == key, self.tr(tr_id)).clicked() {
                                self.settings.selected_season = key.to_string();
                                self.needs_update = true;
                            }
                        }
                    });

                ui.label(self.tr("latitude-label"));
                if ui.add(egui::DragValue::new(&mut self.settings.latitude)
                    .speed(0.1)
                    .clamp_range(-90.0..=90.0)
                    .suffix("°")).changed() {
                    self.needs_update = true;
                }

                ui.separator();
                ui.label(self.tr("search-placeholder"));
                if ui.text_edit_singleline(&mut self.search_query).changed() {
                    // La recherche change, on pourrait éventuellement forcer un refresh si besoin
                }
                if !self.search_query.is_empty() {
                    if ui.button("×").clicked() {
                        self.search_query.clear();
                    }

                    // Affichage direct des résultats de recherche s'il y en a peu
                    let search_results: Vec<&Target> = TARGETS.iter()
                        .filter(|t| self.is_target_visible(t))
                        .filter(|t| self.matches_search(t))
                        .take(10)
                        .collect();
                    
                    if !search_results.is_empty() {
                        ui.separator();
                        ui.menu_button(self.tr("search-results-label"), |ui| {
                            for target in search_results {
                                let name = self.tr(target.name_key);
                                let label = if let Some(id) = target.id {
                                    format!("{} - {}", id, name)
                                } else {
                                    name
                                };
                                if ui.button(label).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    }
                }

                ui.menu_button(self.tr("object-type-label"), |ui| {
                    ui.menu_button(self.tr("type-constellation"), |ui| {
                        let mut constellations: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Constellation)
                            .filter(|t| self.is_target_visible(t))
                            .filter(|t| self.matches_search(t))
                            .collect();
                        constellations.sort_by(|a, b| self.tr(a.name_key).cmp(&self.tr(b.name_key)));
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in constellations {
                                if ui.button(self.tr(target.name_key)).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                    ui.menu_button(self.tr("type-messier"), |ui| {
                        let mut messiers: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Messier)
                            .filter(|t| self.is_target_visible(t))
                            .filter(|t| self.matches_search(t))
                            .collect();
                        messiers.sort_by(|a, b| {
                            let a_num: i32 = a.id.unwrap_or("").strip_prefix('M').and_then(|s| s.parse().ok()).unwrap_or(0);
                            let b_num: i32 = b.id.unwrap_or("").strip_prefix('M').and_then(|s| s.parse().ok()).unwrap_or(0);
                            a_num.cmp(&b_num)
                        });
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in messiers {
                                let label = format!("{} - {}", target.id.unwrap_or(""), self.tr(target.name_key));
                                if ui.button(label).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                    ui.menu_button(self.tr("type-nebula"), |ui| {
                        let mut nebulae: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Nebula)
                            .filter(|t| self.is_target_visible(t))
                            .filter(|t| self.matches_search(t))
                            .collect();
                        nebulae.sort_by(|a, b| self.tr(a.name_key).cmp(&self.tr(b.name_key)));
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in nebulae {
                                if ui.button(self.tr(target.name_key)).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                    ui.menu_button(self.tr("type-galaxy"), |ui| {
                        let mut galaxies: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Galaxy)
                            .filter(|t| self.is_target_visible(t))
                            .filter(|t| self.matches_search(t))
                            .collect();
                        galaxies.sort_by(|a, b| self.tr(a.name_key).cmp(&self.tr(b.name_key)));
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in galaxies {
                                if ui.button(self.tr(target.name_key)).clicked() {
                                    self.selected_target = Some(target.clone());
                                    self.needs_update = true;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                    ui.menu_button(self.tr("type-cluster"), |ui| {
                        let mut clusters: Vec<&Target> = TARGETS.iter()
                            .filter(|t| t.target_type == TargetType::Cluster)
                            .filter(|t| self.is_target_visible(t))
                            .filter(|t| self.matches_search(t))
                            .collect();
                        clusters.sort_by(|a, b| self.tr(a.name_key).cmp(&self.tr(b.name_key)));
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for target in clusters {
                                if ui.button(self.tr(target.name_key)).clicked() {
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
                ui.heading(self.tr("target-info-title"));
            });
            ui.separator();
            ui.add_space(10.0);

            if let Some(target) = &self.selected_target {
                let is_visible = self.is_target_visible(target);
                if !is_visible {
                    let msg = if target.season_key != self.settings.selected_season && self.settings.selected_season != "Toutes" 
                        && !target.season_key.contains("circumpolaire") {
                        self.tr("target-out-of-season")
                    } else {
                        self.tr("target-not-visible")
                    };
                    ui.colored_label(egui::Color32::RED, msg);
                    ui.add_space(5.0);
                }
                ui.scope(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                    
                    if let Some(id) = target.id {
                        ui.horizontal(|ui| {
                            ui.strong(self.tr("target-id"));
                            ui.label(id);
                        });
                    }

                    ui.horizontal(|ui| {
                        ui.strong(self.tr("target-name"));
                        ui.label(self.tr(target.name_key));
                    });

                    ui.horizontal(|ui| {
                        ui.strong(self.tr("target-latin"));
                        ui.label(self.tr(target.latin_key));
                    });

                    if let Some(abbr) = target.abbr {
                        ui.horizontal(|ui| {
                            ui.strong(self.tr("target-abbr"));
                            ui.label(abbr);
                        });
                    }

                    if let Some(parent_key) = target.parent_key {
                        ui.horizontal(|ui| {
                            ui.strong(self.tr("target-constellation"));
                            ui.label(self.tr(parent_key));
                        });
                    }

                    ui.horizontal(|ui| {
                        ui.strong(self.tr("target-season"));
                        let season_label = self.tr(target.season_key);
                        if !self.is_target_visible(target) {
                            ui.colored_label(egui::Color32::RED, season_label);
                        } else {
                            ui.label(season_label);
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.strong(self.tr("target-declination"));
                        if target.dec <= (self.settings.latitude - 80.0) {
                            ui.colored_label(egui::Color32::RED, format!("{:.1}° {}", target.dec, self.tr("target-too-low")));
                        } else {
                            ui.label(format!("{:.1}°", target.dec));
                        }
                    });

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Rappel des paramètres de prise de vue
                    ui.heading(self.tr("target-params-heading"));
                    ui.add_space(5.0);
                    let lens = &self.settings.lenses[self.settings.selected_lens_idx];
                    let sensor = &self.settings.sensors[self.settings.selected_sensor_idx];
                    
                    let mut args = FluentArgs::new();
                    args.set("name", lens.name.clone());
                    ui.label(self.tr_args("target-lens-info", &args));
                    
                    if lens.f_max > lens.f_min {
                        let mut args_f = FluentArgs::new();
                        args_f.set("min", lens.f_min);
                        args_f.set("max", lens.f_max);
                        ui.label(self.tr_args("target-focal-range", &args_f));

                        let mut args_a = FluentArgs::new();
                        args_a.set("min", lens.n_min);
                        args_a.set("max", lens.n_max);
                        ui.label(self.tr_args("target-aperture-range", &args_a));
                    } else {
                        let mut args_f = FluentArgs::new();
                        args_f.set("val", lens.f_min);
                        ui.label(self.tr_args("target-focal-single", &args_f));

                        let mut args_a = FluentArgs::new();
                        args_a.set("val", lens.n_min);
                        ui.label(self.tr_args("target-aperture-single", &args_a));
                    }
                    
                    let mut args_s = FluentArgs::new();
                    args_s.set("name", sensor.name.clone());
                    args_s.set("pixel", sensor.pixel_size);
                    ui.label(self.tr_args("target-sensor-info", &args_s));
                    
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    let mut args_npf = FluentArgs::new();
                    args_npf.set("dec", format!("{:.1}", target.dec));
                    ui.heading(self.tr_args("target-exposure-at-dec", &args_npf));
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
                    ui.label(self.tr("select-target-hint"));
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let equator_tr = self.tr("chart-equator-label");
            let target_name_string = self.selected_target.as_ref()
                .map(|t| t.id.map(|id| id.to_string()).unwrap_or_else(|| self.tr(t.name_key)))
                .unwrap_or(equator_tr);
            let target_name = &target_name_string;
            let lens = &self.settings.lenses[self.settings.selected_lens_idx];
            
            let mut args = FluentArgs::new();
            args.set("lens", lens.name.clone());
            args.set("target", target_name.to_string());
            let titre = self.tr_args("chart-target-title", &args);

            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(titre);
                ui.add_space(5.0);
                ui.label(self.tr("chart-rotate-hint"));
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
