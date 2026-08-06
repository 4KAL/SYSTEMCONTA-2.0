use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoriaGasto {
    pub id: i64,
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gasto {
    pub id: i64,
    pub numero: Option<String>,
    pub categoria_id: i64,
    pub categoria_nombre: String,
    pub descripcion: String,
    pub monto: f64,
    pub subtotal: f64,
    pub impuesto: f64,
    pub total: f64,
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub fecha: String,
    pub metodo_pago: String,
    pub referencia: String,
    pub comprobante: Option<String>,
    pub estado: String,
    pub notas: String,
    pub fecha_vencimiento: Option<String>,
    pub fecha_pago: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GastoNuevo {
    pub numero: Option<String>,
    pub categoria_id: i64,
    pub descripcion: String,
    pub monto: f64,
    pub subtotal: f64,
    pub impuesto: f64,
    pub proveedor_id: Option<i64>,
    pub metodo_pago: String,
    pub referencia: String,
    pub comprobante: Option<String>,
    pub notas: String,
    pub fecha_vencimiento: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagoRealizado {
    pub id: i64,
    pub gasto_id: Option<i64>,
    pub proveedor_id: Option<i64>,
    pub monto: f64,
    pub metodo_pago: Option<String>,
    pub referencia: String,
    pub notas: String,
    pub fecha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagoRealizadoNuevo {
    pub gasto_id: Option<i64>,
    pub proveedor_id: Option<i64>,
    pub monto: f64,
    pub metodo_pago: Option<String>,
    pub referencia: String,
    pub notas: String,
}
