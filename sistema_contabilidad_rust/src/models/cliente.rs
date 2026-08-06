use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliente {
    pub id: i64,
    pub codigo: Option<String>,
    pub nombre: String,
    pub rfc: String,
    pub email: String,
    pub telefono: String,
    pub direccion: String,
    pub ciudad: String,
    pub limite_credito: f64,
    pub saldo_pendiente: f64,
    pub activo: bool,
    pub fecha_registro: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClienteNuevo {
    pub codigo: Option<String>,
    pub nombre: String,
    pub rfc: String,
    pub email: String,
    pub telefono: String,
    pub direccion: String,
    pub ciudad: String,
    pub limite_credito: f64,
}
