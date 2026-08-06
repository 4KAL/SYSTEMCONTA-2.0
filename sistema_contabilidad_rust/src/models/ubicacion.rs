use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ubicacion {
    pub id: i64,
    pub nombre: String,
    pub encargado: Option<String>,
    pub cedula: Option<String>,
    pub telefono: String,
    pub ciudad: String,
    pub direccion: String,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UbicacionNueva {
    pub nombre: String,
    pub encargado: Option<String>,
    pub cedula: Option<String>,
    pub telefono: String,
    pub ciudad: String,
    pub direccion: String,
}
