use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proveedor {
    pub id: i64,
    pub codigo: Option<String>,
    pub nombre: String,
    pub contacto: String,
    pub rfc: String,
    pub email: String,
    pub telefono: String,
    pub direccion: String,
    pub ciudad: String,
    pub saldo_pendiente: f64,
    pub activo: bool,
    pub fecha_registro: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveedorNuevo {
    pub codigo: Option<String>,
    pub nombre: String,
    pub contacto: String,
    pub rfc: String,
    pub email: String,
    pub telefono: String,
    pub direccion: String,
    pub ciudad: String,
}
