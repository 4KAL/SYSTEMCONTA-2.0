#[path = "../db/mod.rs"]
mod db;
#[path = "../models/mod.rs"]
mod models;

use db::DatabaseManager;
use models::{DeudaEmpresaNueva, DeudaPagoNuevo};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("--clean-deudas") {
        let db = DatabaseManager::new("contabilidad_rust.db").expect("abrir BD");
        db.eliminar_deuda_pago(99999).unwrap_or_default();
        let deudas = db.listar_deudas_empresa().unwrap();
        for d in deudas {
            for p in db.listar_deuda_pagos(d.id).unwrap() {
                let _ = db.eliminar_deuda_pago(p.id);
            }
            let _ = db.eliminar_deuda_empresa(d.id);
        }
        println!("DEUDAS DE PRUEBA ELIMINADAS");
        return;
    }
    let db = DatabaseManager::new("contabilidad_rust.db").expect("abrir BD");
    let deuda = DeudaEmpresaNueva {
        proveedor_id: None,
        proveedor_nombre: "Distribuidora Prueba".to_string(),
        concepto: "10 teclados USB".to_string(),
        descripcion: Some("Mercadería para la tienda".to_string()),
        categoria_id: None,
        categoria_nombre: String::new(),
        fecha_deuda: "2026-07-20".to_string(),
        fecha_vencimiento: Some("2026-08-20".to_string()),
        monto_total: 1000.0,
        referencia: "FAC-001".to_string(),
        notas: "Pagar en partes".to_string(),
    };
    let id = match db.crear_deuda_empresa(&deuda) {
        Ok(id) => id,
        Err(e) => { println!("ERROR crear deuda: {}", e); return; }
    };
    println!("DEUDA CREADA id={}", id);

    for (monto, metodo) in [(200.0, "efectivo"), (100.0, "transferencia")] {
        let p = DeudaPagoNuevo { deuda_id: id, monto, metodo_pago: Some(metodo.to_string()), referencia: String::new(), notas: String::new() };
        match db.crear_deuda_pago(&p) {
            Ok(_) => println!("PAGO ${} por {} OK", monto, metodo),
            Err(e) => println!("ERROR pago: {}", e),
        }
    }

    let d = db.listar_deudas_empresa().unwrap().into_iter().find(|d| d.id == id).unwrap();
    println!("SALDO DESPUES DE 300 PAGADO (debe ser 700): ${:.2}", d.saldo_pendiente);
    println!("ESTADO (debe ser pendiente): {}", d.estado);
    let pagos = db.listar_deuda_pagos(id).unwrap();
    println!("PAGOS REGISTRADOS: {} (debe ser 2)", pagos.len());

    if let Some(p) = pagos.first() {
        db.eliminar_deuda_pago(p.id).unwrap();
        let d = db.listar_deudas_empresa().unwrap().into_iter().find(|d| d.id == id).unwrap();
        println!("SALDO TRAS BORRAR PAGO DE ${} (debe ser 900): ${:.2}", p.monto, d.saldo_pendiente);
    }

    let p_total = DeudaPagoNuevo { deuda_id: id, monto: 900.0, metodo_pago: Some("tarjeta".to_string()), referencia: String::new(), notas: String::new() };
    db.crear_deuda_pago(&p_total).unwrap();
    let d = db.listar_deudas_empresa().unwrap().into_iter().find(|d| d.id == id).unwrap();
    println!("SALDO FINAL (debe ser 0): ${:.2} | ESTADO (debe ser pagada): {}", d.saldo_pendiente, d.estado);
}
