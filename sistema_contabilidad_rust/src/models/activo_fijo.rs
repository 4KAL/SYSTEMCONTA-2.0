use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivoFijo {
    pub id: i64,
    pub descripcion: String,
    pub categoria: String,
    pub fecha_adquisicion: String,
    pub valor_adquisicion: f64,
    pub valor_residual: f64,
    pub vida_util_anios: f64,
    pub depreciacion_mensual: f64,
    pub depreciacion_acumulada: f64,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivoFijoNuevo {
    pub descripcion: String,
    pub categoria: String,
    pub fecha_adquisicion: String,
    pub valor_adquisicion: f64,
    pub valor_residual: f64,
    pub vida_util_anios: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Depreciacion {
    pub id: i64,
    pub activo_id: i64,
    pub activo_descripcion: String,
    pub periodo: String,
    pub monto: f64,
    pub acumulado: f64,
    pub fecha: String,
}
