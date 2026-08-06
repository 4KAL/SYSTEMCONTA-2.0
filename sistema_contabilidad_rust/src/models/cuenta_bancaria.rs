use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuentaBancaria {
    pub id: i64,
    pub nombre: String,
    pub banco: String,
    pub numero_cuenta: String,
    pub tipo: String,
    pub saldo_inicial: f64,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuentaBancariaNueva {
    pub nombre: String,
    pub banco: String,
    pub numero_cuenta: String,
    pub tipo: String,
    pub saldo_inicial: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovimientoBancario {
    pub id: i64,
    pub cuenta_id: i64,
    pub cuenta_nombre: String,
    pub fecha: String,
    pub descripcion: String,
    pub tipo: String,
    pub monto: f64,
    pub conciliado: bool,
    pub referencia: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovimientoBancarioNuevo {
    pub cuenta_id: i64,
    pub fecha: String,
    pub descripcion: String,
    pub tipo: String,
    pub monto: f64,
    pub conciliado: bool,
    pub referencia: String,
}
