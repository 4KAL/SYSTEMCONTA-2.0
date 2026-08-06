use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeudaEmpresa {
    pub id: i64,
    pub numero: String,
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub concepto: String,
    pub descripcion: Option<String>,
    pub categoria_id: Option<i64>,
    pub categoria_nombre: String,
    pub fecha_deuda: String,
    pub fecha_vencimiento: Option<String>,
    pub monto_total: f64,
    pub saldo_pendiente: f64,
    pub referencia: String,
    pub notas: String,
    pub estado: String,
    pub activa: bool,
    pub fecha_registro: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeudaEmpresaNueva {
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub concepto: String,
    pub descripcion: Option<String>,
    pub categoria_id: Option<i64>,
    pub categoria_nombre: String,
    pub fecha_deuda: String,
    pub fecha_vencimiento: Option<String>,
    pub monto_total: f64,
    pub referencia: String,
    pub notas: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeudaPago {
    pub id: i64,
    pub deuda_id: i64,
    pub fecha: String,
    pub monto: f64,
    pub metodo_pago: Option<String>,
    pub referencia: String,
    pub notas: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeudaPagoNuevo {
    pub deuda_id: i64,
    pub monto: f64,
    pub metodo_pago: Option<String>,
    pub referencia: String,
    pub notas: String,
}
