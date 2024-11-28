use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub buildings: Vec<Building>,
}

#[derive(Debug, Deserialize)]
pub struct Building {
    pub name: String,
    pub floors: Vec<Floor>,
}

#[derive(Debug, Deserialize)]
pub struct Floor {
    pub name: String,
    pub rooms: Vec<Room>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    pub name: String,
    pub devices: Device, // Dispositivos como un único mapa
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub lights: Option<Vec<String>>,
    pub climate: Option<Vec<String>>,
    pub shades: Option<Vec<String>>,
}

pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("No se pudo leer el archivo: {}", e))?;

    let config: Config = serde_yaml::from_str(&content)
        .map_err(|e| format!("Error al deserializar el archivo YAML: {}", e))?;

    Ok(config)
}
