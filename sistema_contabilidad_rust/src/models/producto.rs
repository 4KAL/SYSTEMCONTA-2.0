use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoriaProducto {
    pub id: i64,
    pub nombre: String,
    pub descripcion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producto {
    pub id: i64,
    pub codigo: Option<String>,
    pub nombre: String,
    pub descripcion: String,
    pub categoria_id: i64,
    pub precio_compra: f64,
    pub precio_venta: f64,
    pub stock: i32,
    pub stock_minimo: i32,
    pub unidad: String,
    pub activo: bool,
    pub fecha_registro: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductoNuevo {
    pub codigo: Option<String>,
    pub nombre: String,
    pub descripcion: String,
    pub categoria_id: i64,
    pub precio_compra: f64,
    pub precio_venta: f64,
    pub stock: i32,
    pub stock_minimo: i32,
    pub unidad: String,
}
