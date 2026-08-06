use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venta {
    pub id: i64,
    pub folio: String,
    pub cliente_id: Option<i64>,
    pub cliente_nombre: String,
    pub fecha: String,
    pub subtotal: f64,
    pub impuesto: f64,
    pub descuento: f64,
    pub total: f64,
    pub saldo_pendiente: f64,
    pub tipo: String,
    pub estado: String,
    pub metodo_pago: Option<String>,
    pub notas: String,
    pub fecha_vencimiento: Option<String>,
    pub fecha_pago: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentaDetalle {
    pub id: i64,
    pub venta_id: i64,
    pub producto_id: Option<i64>,
    pub descripcion: Option<String>,
    pub producto_nombre: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub descuento: f64,
    pub importe: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentaNueva {
    pub cliente_id: Option<i64>,
    pub cliente_nombre: String,
    pub tipo: String,
    pub notas: String,
    pub descuento: f64,
    pub metodo_pago: Option<String>,
    pub iva: f64,
    pub detalles: Vec<VentaDetalleNuevo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentaDetalleNuevo {
    pub producto_id: Option<i64>,
    pub producto_nombre: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub descuento: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagoRecibido {
    pub id: i64,
    pub venta_id: Option<i64>,
    pub cliente_id: Option<i64>,
    pub monto: f64,
    pub metodo_pago: Option<String>,
    pub referencia: String,
    pub notas: String,
    pub fecha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagoRecibidoNuevo {
    pub venta_id: Option<i64>,
    pub cliente_id: Option<i64>,
    pub monto: f64,
    pub metodo_pago: Option<String>,
    pub referencia: String,
    pub notas: String,
}
