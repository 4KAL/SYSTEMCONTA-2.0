#[path = "../db/mod.rs"]
mod db;
#[path = "../models/mod.rs"]
mod models;

use db::DatabaseManager;

fn main() {
    let db = DatabaseManager::new("C:/Users/ADMIN/AppData/Local/Temp/opencode/real_rust.db").expect("abrir BD");
    macro_rules! t {
        ($name:expr, $e:expr) => {
            match $e {
                Ok(v) => println!("{:26} OK ({})", $name, v.len()),
                Err(e) => println!("{:26} ERR: {}", $name, e),
            }
        };
    }
    t!("productos", db.listar_productos());
    t!("clientes", db.listar_clientes());
    t!("proveedores", db.listar_proveedores());
    t!("categorias_productos", db.listar_categorias_productos());
    t!("categorias_gastos", db.listar_categorias_gastos());
    t!("ventas", db.listar_ventas());
    t!("gastos", db.listar_gastos());
    t!("pagos_recibidos", db.listar_pagos_recibidos());
    t!("pagos_realizados", db.listar_pagos_realizados());
    t!("plan_cuentas", db.listar_plan_cuentas());
    t!("asientos", db.listar_asientos());
    t!("ubicaciones", db.listar_ubicaciones());
    t!("maquinas", db.listar_maquinas());
    t!("cobros_comisiones", db.listar_todas_comisiones());
    t!("garantias", db.listar_garantias());
    t!("cuentas_credito", db.listar_cuentas_credito());
    t!("ahorros", db.listar_ahorros());
    t!("deudas_empresa", db.listar_deudas_empresa());
    t!("movimientos_inventario", db.listar_movimientos_inventario());
    t!("compras", db.listar_compras());
    t!("cotizaciones", db.listar_cotizaciones());
    t!("retenciones", db.listar_retenciones());
    t!("empleados", db.listar_empleados());
    t!("cuentas_bancarias", db.listar_cuentas_bancarias());
}
