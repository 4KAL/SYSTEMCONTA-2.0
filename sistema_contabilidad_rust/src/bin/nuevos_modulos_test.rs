#[path = "../db/mod.rs"]
mod db;
#[path = "../models/mod.rs"]
mod models;
#[path = "../pdf.rs"]
mod pdf;

use db::DatabaseManager;
use models::*;

fn main() {
    let ruta = std::env::temp_dir().join("contab_test_nuevos.db");
    let _ = std::fs::remove_file(&ruta);
    let db = DatabaseManager::new(ruta.to_str().unwrap()).expect("abrir BD");

    // Retenciones
    let n = db.proximo_numero_retencion().unwrap();
    println!("proximo numero retencion: {}", n);
    let ret = RetencionNueva {
        numero: n, proveedor_id: None, proveedor_nombre: "Proveedor Test".into(),
        cedula: "1790010010001".into(), fecha: "2026-08-01".into(),
        base_imp_renta: 1000.0, porcentaje_renta: 1.0, valor_renta: 10.0,
        base_imp_iva: 1000.0, porcentaje_iva: 30.0, valor_iva: 300.0,
        tipo_comprobante: "factura".into(), numero_comprobante: "001-001-000000001".into(),
        referencia: "prueba".into(), estado: "emitida".into(),
    };
    let rid = db.crear_retencion(&ret).unwrap();
    assert!(rid > 0);
    assert_eq!(db.listar_retenciones().unwrap().len(), 1);
    assert!(db.proximo_numero_retencion().unwrap().starts_with("R-0002"));
    println!("retenciones OK");

    // Empleados y roles
    let emp = EmpleadoNuevo {
        cedula: "1700000001".into(), nombre: "Juan Perez".into(), cargo: "Tecnico".into(),
        telefono: "0999999999".into(), sueldo_base: 600.0, fecha_ingreso: "2026-01-01".into(),
    };
    let eid = db.crear_empleado(&emp).unwrap();
    assert!(eid > 0);
    let rol = RolPagoNuevo {
        empleado_id: eid, periodo: "2026-08".into(), dias: 30,
        sueldo_bruto: 600.0, horas_extra: 50.0, comisiones: 20.0,
        iess: 0.0, prestamos: 10.0, otras_retenciones: 5.0, notas: String::new(),
    };
    let rolid = db.crear_rol_pago(&rol).unwrap();
    let roles = db.listar_roles_pago().unwrap();
    assert_eq!(roles.len(), 1);
    let r = &roles[0];
    assert!((r.total_ingresos - 670.0).abs() < 0.001, "total_ingresos {}", r.total_ingresos);
    assert!((r.total_neto - 655.0).abs() < 0.001, "total_neto {}", r.total_neto);
    db.marcar_rol_pagado(rolid).unwrap();
    assert_eq!(db.listar_roles_pago().unwrap()[0].estado, "pagado");
    println!("nomina OK (ingresos {}, neto {})", r.total_ingresos, r.total_neto);

    // Activos fijos
    let act = ActivoFijoNuevo {
        descripcion: "Laptop".into(), categoria: "equipo".into(),
        fecha_adquisicion: "2026-01-01".into(), valor_adquisicion: 1200.0,
        valor_residual: 0.0, vida_util_anios: 3.0,
    };
    let aid = db.crear_activo_fijo(&act).unwrap();
    let activos = db.listar_activos_fijos().unwrap();
    assert!((activos[0].depreciacion_mensual - 33.3333).abs() < 0.01, "mensual {}", activos[0].depreciacion_mensual);
    db.registrar_depreciacion_mensual(aid, "2026-08").unwrap();
    let deps = db.listar_depreciaciones().unwrap();
    assert_eq!(deps.len(), 1);
    assert!((deps[0].acumulado - 33.3333).abs() < 0.01);
    println!("depreciacion OK (mensual {:.2})", activos[0].depreciacion_mensual);

    // Cierre contable
    let cierre = CierreContableNuevo { anio: 2026, notas: "cierre de prueba".into() };
    db.crear_cierre(&cierre).unwrap();
    assert_eq!(db.listar_cierres().unwrap().len(), 1);
    println!("cierre contable OK");

    // Cuentas bancarias y movimientos
    let cta = CuentaBancariaNueva {
        nombre: "Caja Principal".into(), banco: "Banco Pichincha".into(),
        numero_cuenta: "1234567890".into(), tipo: "corriente".into(), saldo_inicial: 500.0,
    };
    let cid = db.crear_cuenta_bancaria(&cta).unwrap();
    let mov = MovimientoBancarioNuevo {
        cuenta_id: cid, fecha: "2026-08-01".into(), descripcion: "Deposito".into(),
        tipo: "ingreso".into(), monto: 100.0, conciliado: false, referencia: "dep-1".into(),
    };
    db.crear_movimiento_bancario(&mov).unwrap();
    let mov2 = MovimientoBancarioNuevo {
        cuenta_id: cid, fecha: "2026-08-02".into(), descripcion: "Pago proveedor".into(),
        tipo: "egreso".into(), monto: 50.0, conciliado: false, referencia: "pag-1".into(),
    };
    let mid = db.crear_movimiento_bancario(&mov2).unwrap();
    let saldo = db.saldo_cuenta_bancaria(cid).unwrap();
    assert!((saldo - 550.0).abs() < 0.001, "saldo {}", saldo);
    let mvs = db.listar_movimientos_bancarios(cid).unwrap();
    assert_eq!(mvs.len(), 2);
    let m = &mvs[0];
    let upd = MovimientoBancarioNuevo {
        cuenta_id: m.cuenta_id, fecha: m.fecha.clone(), descripcion: m.descripcion.clone(),
        tipo: m.tipo.clone(), monto: m.monto, conciliado: true, referencia: m.referencia.clone(),
    };
    db.actualizar_movimiento_bancario(m.id, &upd).unwrap();
    assert!(db.listar_movimientos_bancarios(cid).unwrap().iter().any(|x| x.conciliado));
    db.eliminar_movimiento_bancario(mid).unwrap();
    println!("conciliacion bancaria OK (saldo {})", saldo);

    // Arqueo de caja
    let arq = ArqueoCajaNuevo {
        fecha: "2026-08-02".into(), responsable: "Cajero".into(),
        monto_esperado: 500.0, monto_real: 505.0, observacion: "sobrante".into(),
    };
    let arid = db.crear_arqueo(&arq).unwrap();
    let arqueos = db.listar_arqueos().unwrap();
    assert!((arqueos[0].diferencia - 5.0).abs() < 0.001, "diferencia {}", arqueos[0].diferencia);
    println!("arqueo de caja OK (diferencia {})", arqueos[0].diferencia);

    // Libros y ATS
    let compras = db.libro_compras("", "").unwrap();
    let ventas = db.libro_ventas("", "").unwrap();
    let ats = db.resumen_ats("", "").unwrap();
    println!("libros OK (compras {}, ventas {}, ats ventas {:.2})", compras.len(), ventas.len(), ats.ventas);

    // PDF retencion
    let rets = db.listar_retenciones().unwrap();
    let empresa = db.obtener_configuracion().unwrap();
    let p = pdf::generar_retencion_pdf(&rets[0], &empresa).unwrap();
    println!("PDF retencion generado: {}", p.display());
    assert!(p.exists());

    // XML factura
    let v = Venta {
        id: 1, folio: "V-0001".into(), cliente_id: None,
        cliente_nombre: "Cliente Test".into(), fecha: "2026-08-01".into(),
        subtotal: 100.0, impuesto: 15.0, descuento: 0.0, total: 115.0,
        saldo_pendiente: 0.0, tipo: "contado".into(), estado: "completada".into(),
        metodo_pago: None, notas: String::new(), fecha_vencimiento: None, fecha_pago: None,
    };
    let det = vec![VentaDetalle {
        id: 1, venta_id: 1, producto_id: None, descripcion: None,
        producto_nombre: "Producto & Co <test>".into(), cantidad: 2,
        precio_unitario: 50.0, descuento: 0.0, importe: 100.0,
    }];
    let x = pdf::generar_xml_factura(&v, &det, None, &empresa).unwrap();
    let contenido = std::fs::read_to_string(&x).unwrap();
    assert!(contenido.contains("<factura id=\"comprobante\" version=\"1.0.0\">"));
    assert!(contenido.contains("Producto &amp; Co &lt;test&gt;"));
    assert!(contenido.contains("<importeTotal>115.00</importeTotal>"));
    println!("XML factura OK: {}", x.display());

    // Backup manual
    let _ = std::fs::copy(&ruta, ruta.with_extension("backup"));

    println!("TODAS LAS PRUEBAS NUEVAS PASARON");
    let _ = std::fs::remove_file(&ruta);
}
