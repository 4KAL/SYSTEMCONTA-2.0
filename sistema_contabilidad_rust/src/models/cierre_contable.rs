use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CierreContable {
    pub id: i64,
    pub anio: i32,
    pub fecha: String,
    pub ingresos: f64,
    pub gastos: f64,
    pub utilidad: f64,
    pub estado: String,
    pub notas: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CierreContableNuevo {
    pub anio: i32,
    pub notas: String,
}
