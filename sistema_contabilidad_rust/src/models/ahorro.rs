use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ahorro {
    pub id: i64,
    pub cliente_id: Option<i64>,
    pub cliente_nombre: String,
    pub saldo: f64,
    pub activo: bool,
    pub fecha_apertura: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhorroMovimiento {
    pub id: i64,
    pub ahorro_id: i64,
    pub tipo: String,
    pub monto: f64,
    pub saldo_acumulado: f64,
    pub cobro_id: Option<i64>,
    pub descripcion: String,
    pub fecha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhorroMovimientoNuevo {
    pub ahorro_id: i64,
    pub tipo: String,
    pub monto: f64,
    pub cobro_id: Option<i64>,
    pub descripcion: String,
}
