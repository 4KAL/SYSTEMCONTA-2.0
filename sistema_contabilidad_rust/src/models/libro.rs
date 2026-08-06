use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibroComprasLinea {
    pub numero: String,
    pub proveedor_nombre: String,
    pub fecha: String,
    pub subtotal: f64,
    pub iva: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibroVentasLinea {
    pub folio: String,
    pub cliente_nombre: String,
    pub fecha: String,
    pub subtotal: f64,
    pub iva: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResumenAts {
    pub ventas: f64,
    pub iva_ventas: f64,
    pub compras: f64,
    pub iva_compras: f64,
    pub ventas_exentas: f64,
}
