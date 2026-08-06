use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compra {
    pub id: i64,
    pub numero: String,
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub fecha: String,
    pub subtotal: f64,
    pub impuesto: f64,
    pub descuento: f64,
    pub total: f64,
    pub metodo_pago: Option<String>,
    pub referencia: Option<String>,
    pub notas: String,
    pub estado: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompraDetalle {
    pub id: i64,
    pub compra_id: i64,
    pub producto_id: Option<i64>,
    pub producto_nombre: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub descuento: f64,
    pub importe: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompraNueva {
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub notas: String,
    pub descuento: f64,
    pub metodo_pago: Option<String>,
    pub iva: f64,
    pub detalles: Vec<CompraDetalleNuevo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompraDetalleNuevo {
    pub producto_id: Option<i64>,
    pub producto_nombre: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub descuento: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovimientoInventario {
    pub id: i64,
    pub producto_id: Option<i64>,
    pub producto_nombre: String,
    pub tipo: String,
    pub cantidad: i32,
    pub motivo: Option<String>,
    pub referencia: Option<String>,
    pub fecha: String,
}
