use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuentaCredito {
    pub id: i64,
    pub nombre: String,
    pub tipo: String,
    pub cliente_id: Option<i64>,
    pub cliente_nombre: String,
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub limite: f64,
    pub saldo_actual: f64,
    pub notas: Option<String>,
    pub activa: bool,
    pub fecha_apertura: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditoMovimiento {
    pub id: i64,
    pub cuenta_id: i64,
    pub tipo: String,
    pub monto: f64,
    pub cantidad: f64,
    pub precio_unit: f64,
    pub saldo_acumulado: f64,
    pub colores: Option<String>,
    pub descripcion: String,
    pub referencia_id: Option<i64>,
    pub fecha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditoMovimientoNuevo {
    pub cuenta_id: i64,
    pub tipo: String,
    pub monto: f64,
    pub cantidad: f64,
    pub precio_unit: f64,
    pub descripcion: String,
    pub referencia_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuentaCreditoNueva {
    pub nombre: String,
    pub tipo: String,
    pub cliente_id: Option<i64>,
    pub proveedor_id: Option<i64>,
    pub limite: f64,
    pub notas: Option<String>,
}
