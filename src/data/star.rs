use super::enums::{SpectrType, StarType};
use super::planet::Planet;
use super::vector3::Vector3;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Star {
    pub id: i32,
    #[serde(skip_serializing)]
    pub index: i32,
    #[serde(skip_serializing)]
    pub seed: i32,
    #[serde(skip_serializing)]
    pub name_seed: i32,
    pub position: Vector3,
    pub name: String,
    #[serde(skip_serializing)]
    pub level: f32,
    #[serde(skip_serializing)]
    pub resource_coef: f32,
    pub mass: f32,
    pub lifetime: f32,
    pub age: f32,
    pub temperature: f32,
    pub star_type: StarType,
    pub spectr: SpectrType,
    #[serde(skip_serializing)]
    pub color: f32,
    #[serde(skip_serializing)]
    pub class_factor: f32,
    pub luminosity: f32,
    pub radius: f32,
    #[serde(skip_serializing)]
    pub habitable_radius: f32,
    #[serde(skip_serializing)]
    pub lignt_balance_radius: f32,
    #[serde(skip_serializing)]
    pub orbit_scaler: f32,
    pub dyson_radius: f32,
    pub planets: Vec<Planet>,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            id: 0,
            index: 0,
            seed: 0,
            name_seed: 0,
            position: Vector3::zero(),
            name: Default::default(),
            level: 0.0,
            resource_coef: 0.0,
            mass: 0.0,
            lifetime: 0.0,
            age: 0.0,
            temperature: 0.0,
            star_type: StarType::MainSeqStar,
            spectr: SpectrType::X,
            color: 0.0,
            class_factor: 0.0,
            luminosity: 0.0,
            radius: 0.0,
            habitable_radius: 0.0,
            lignt_balance_radius: 0.0,
            orbit_scaler: 0.0,
            dyson_radius: 0.0,
            planets: vec![],
        }
    }
}

impl Star {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn physics_radius(&self) -> f32 {
        return self.radius * 1200.0;
    }

    pub fn astro_id(&self) -> i32 {
        return self.id * 100;
    }
}