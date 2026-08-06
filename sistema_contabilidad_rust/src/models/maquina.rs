use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaquinaUbicada {
    pub id: i64,
    pub ubicacion_texto: String,
    pub codigo: Option<String>,
    pub descripcion: String,
    pub modelo: String,
    pub numero_serie: String,
    pub color: Option<String>,
    pub fecha_ingreso: Option<String>,
    pub fecha_instalacion: String,
    pub comision: f64,
    pub comision_estimada: f64,
    pub dia_cobro: i32,
    pub activo: bool,
    pub notas: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaquinaNueva {
    pub ubicacion_texto: String,
    pub codigo: Option<String>,
    pub descripcion: String,
    pub modelo: String,
    pub numero_serie: String,
    pub color: Option<String>,
    pub comision: f64,
    pub comision_estimada: f64,
    pub dia_cobro: i32,
    pub fecha_instalacion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobroComision {
    pub id: i64,
    pub maquina_id: i64,
    pub monto: f64,
    pub fecha: String,
    pub mes: Option<String>,
    pub periodo: String,
    pub observacion: Option<String>,
    pub notas: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobroComisionNuevo {
    pub maquina_id: i64,
    pub monto: f64,
    pub mes: Option<String>,
    pub periodo: String,
    pub observacion: Option<String>,
    pub notas: String,
}
