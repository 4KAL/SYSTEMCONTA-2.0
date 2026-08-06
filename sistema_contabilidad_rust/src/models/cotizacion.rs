use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cotizacion {
    pub id: i64,
    pub numero: String,
    pub cliente_id: Option<i64>,
    pub cliente_nombre: String,
    pub fecha: String,
    pub validez_dias: i32,
    pub subtotal: f64,
    pub impuesto: f64,
    pub descuento: f64,
    pub total: f64,
    pub estado: String,
    pub notas: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CotizacionDetalle {
    pub id: i64,
    pub cotizacion_id: i64,
    pub producto_id: Option<i64>,
    pub producto_nombre: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub descuento: f64,
    pub importe: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CotizacionNueva {
    pub cliente_id: Option<i64>,
    pub cliente_nombre: String,
    pub validez_dias: i32,
    pub notas: String,
    pub descuento: f64,
    pub iva: f64,
    pub detalles: Vec<CotizacionDetalleNuevo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CotizacionDetalleNuevo {
    pub producto_id: Option<i64>,
    pub producto_nombre: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub descuento: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentaDesdeCotizacion {
    pub cotizacion_id: i64,
    pub venta_id: i64,
    pub folio: String,
}
