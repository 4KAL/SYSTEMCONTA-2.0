use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retencion {
    pub id: i64,
    pub numero: String,
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub cedula: String,
    pub fecha: String,
    pub base_imp_renta: f64,
    pub porcentaje_renta: f64,
    pub valor_renta: f64,
    pub base_imp_iva: f64,
    pub porcentaje_iva: f64,
    pub valor_iva: f64,
    pub tipo_comprobante: String,
    pub numero_comprobante: String,
    pub referencia: String,
    pub estado: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetencionNueva {
    pub numero: String,
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub cedula: String,
    pub fecha: String,
    pub base_imp_renta: f64,
    pub porcentaje_renta: f64,
    pub valor_renta: f64,
    pub base_imp_iva: f64,
    pub porcentaje_iva: f64,
    pub valor_iva: f64,
    pub tipo_comprobante: String,
    pub numero_comprobante: String,
    pub referencia: String,
    pub estado: String,
}
