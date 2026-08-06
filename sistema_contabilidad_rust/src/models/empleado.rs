use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empleado {
    pub id: i64,
    pub cedula: String,
    pub nombre: String,
    pub cargo: String,
    pub telefono: String,
    pub sueldo_base: f64,
    pub fecha_ingreso: String,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpleadoNuevo {
    pub cedula: String,
    pub nombre: String,
    pub cargo: String,
    pub telefono: String,
    pub sueldo_base: f64,
    pub fecha_ingreso: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolPago {
    pub id: i64,
    pub empleado_id: i64,
    pub empleado_nombre: String,
    pub periodo: String,
    pub dias: i32,
    pub sueldo_bruto: f64,
    pub horas_extra: f64,
    pub comisiones: f64,
    pub total_ingresos: f64,
    pub iess: f64,
    pub prestamos: f64,
    pub otras_retenciones: f64,
    pub total_egresos: f64,
    pub total_neto: f64,
    pub estado: String,
    pub notas: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolPagoNuevo {
    pub empleado_id: i64,
    pub periodo: String,
    pub dias: i32,
    pub sueldo_bruto: f64,
    pub horas_extra: f64,
    pub comisiones: f64,
    pub iess: f64,
    pub prestamos: f64,
    pub otras_retenciones: f64,
    pub notas: String,
}
