use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Garantia {
    pub id: i64,
    pub producto_id: Option<i64>,
    pub venta_id: Option<i64>,
    pub producto: String,
    pub producto_nombre: String,
    pub numero_serie: Option<String>,
    pub folio_venta: String,
    pub cliente_nombre: String,
    pub cedula: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub ciudad: Option<String>,
    pub monto_pago: f64,
    pub estado: String,
    pub observacion: Option<String>,
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub descripcion: String,
    pub activa: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarantiaNueva {
    pub producto_id: Option<i64>,
    pub venta_id: Option<i64>,
    pub producto: String,
    pub numero_serie: Option<String>,
    pub cliente_nombre: String,
    pub cedula: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub ciudad: Option<String>,
    pub monto_pago: f64,
    pub observacion: Option<String>,
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub descripcion: String,
}
