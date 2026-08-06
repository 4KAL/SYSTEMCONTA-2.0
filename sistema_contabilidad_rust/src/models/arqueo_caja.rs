use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArqueoCaja {
    pub id: i64,
    pub fecha: String,
    pub responsable: String,
    pub monto_esperado: f64,
    pub monto_real: f64,
    pub diferencia: f64,
    pub observacion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArqueoCajaNuevo {
    pub fecha: String,
    pub responsable: String,
    pub monto_esperado: f64,
    pub monto_real: f64,
    pub observacion: String,
}
