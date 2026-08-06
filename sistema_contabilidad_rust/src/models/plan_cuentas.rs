use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCuentas {
    pub id: i64,
    pub codigo: String,
    pub nombre: String,
    pub tipo: String,
    pub naturaleza: String,
    pub nivel: i32,
    pub padre_id: Option<i64>,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asiento {
    pub id: i64,
    pub numero: Option<String>,
    pub fecha: String,
    pub concepto: String,
    pub descripcion: Option<String>,
    pub referencia: Option<String>,
    pub tipo: String,
    pub total_debe: f64,
    pub total_haber: f64,
    pub estado: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsientoLinea {
    pub id: i64,
    pub asiento_id: i64,
    pub cuenta_id: i64,
    pub cuenta_codigo: String,
    pub cuenta_nombre: String,
    pub descripcion: Option<String>,
    pub debe: f64,
    pub haber: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsientoNuevo {
    pub numero: Option<String>,
    pub fecha: String,
    pub concepto: String,
    pub descripcion: Option<String>,
    pub referencia: Option<String>,
    pub lineas: Vec<AsientoLineaNueva>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsientoLineaNueva {
    pub cuenta_id: i64,
    pub descripcion: Option<String>,
    pub debe: f64,
    pub haber: f64,
}
