use rusqlite::{Connection, types::Value, TransactionBehavior};
use std::path::Path;
use std::time::Instant;
use crate::db::DatabaseManager;

pub const PYTHON_DB: &str = "contabilidad.db";
pub const RUST_DB: &str = "contabilidad_rust.db";

const CREATE_SCHEMA: &str = "
PRAGMA journal_mode=WAL;
PRAGMA foreign_keys=OFF;
CREATE TABLE IF NOT EXISTS db_version (id INTEGER PRIMARY KEY AUTOINCREMENT, version TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP);
CREATE TABLE IF NOT EXISTS plan_cuentas (id INTEGER PRIMARY KEY AUTOINCREMENT, codigo TEXT NOT NULL UNIQUE, nombre TEXT NOT NULL, tipo TEXT NOT NULL, naturaleza TEXT NOT NULL DEFAULT 'deudora', nivel INTEGER NOT NULL DEFAULT 1, padre_id INTEGER, activo INTEGER NOT NULL DEFAULT 1);
CREATE TABLE IF NOT EXISTS clientes (id INTEGER PRIMARY KEY AUTOINCREMENT, codigo TEXT, nombre TEXT NOT NULL, rfc TEXT, email TEXT, telefono TEXT, direccion TEXT, ciudad TEXT, limite_credito REAL NOT NULL DEFAULT 0, saldo_pendiente REAL NOT NULL DEFAULT 0, activo INTEGER NOT NULL DEFAULT 1, fecha_registro TEXT NOT NULL DEFAULT (date('now')));
CREATE TABLE IF NOT EXISTS proveedores (id INTEGER PRIMARY KEY AUTOINCREMENT, codigo TEXT, nombre TEXT NOT NULL, contacto TEXT, rfc TEXT, email TEXT, telefono TEXT, direccion TEXT, ciudad TEXT, saldo_pendiente REAL NOT NULL DEFAULT 0, activo INTEGER NOT NULL DEFAULT 1, fecha_registro TEXT NOT NULL DEFAULT (date('now')));
CREATE TABLE IF NOT EXISTS categorias_productos (id INTEGER PRIMARY KEY AUTOINCREMENT, nombre TEXT NOT NULL UNIQUE, descripcion TEXT);
CREATE TABLE IF NOT EXISTS productos (id INTEGER PRIMARY KEY AUTOINCREMENT, codigo TEXT, nombre TEXT NOT NULL, descripcion TEXT, categoria_id INTEGER, precio_compra REAL NOT NULL DEFAULT 0, precio_venta REAL NOT NULL DEFAULT 0, stock INTEGER NOT NULL DEFAULT 0, stock_minimo INTEGER NOT NULL DEFAULT 0, unidad TEXT NOT NULL DEFAULT 'pza', activo INTEGER NOT NULL DEFAULT 1, fecha_registro TEXT NOT NULL DEFAULT (date('now')));
CREATE TABLE IF NOT EXISTS ventas (id INTEGER PRIMARY KEY AUTOINCREMENT, folio TEXT NOT NULL UNIQUE, cliente_id INTEGER, cliente_nombre TEXT NOT NULL DEFAULT '', fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')), subtotal REAL NOT NULL DEFAULT 0, impuesto REAL NOT NULL DEFAULT 0, descuento REAL NOT NULL DEFAULT 0, total REAL NOT NULL DEFAULT 0, saldo_pendiente REAL NOT NULL DEFAULT 0, tipo TEXT NOT NULL DEFAULT 'contado', estado TEXT NOT NULL DEFAULT 'completada', metodo_pago TEXT, notas TEXT, fecha_vencimiento TEXT, fecha_pago TEXT);
CREATE TABLE IF NOT EXISTS ventas_detalles (id INTEGER PRIMARY KEY AUTOINCREMENT, venta_id INTEGER NOT NULL, producto_id INTEGER, descripcion TEXT, producto_nombre TEXT NOT NULL, cantidad INTEGER NOT NULL DEFAULT 1, precio_unitario REAL NOT NULL DEFAULT 0, descuento REAL NOT NULL DEFAULT 0, importe REAL NOT NULL DEFAULT 0);
CREATE TABLE IF NOT EXISTS categorias_gastos (id INTEGER PRIMARY KEY AUTOINCREMENT, nombre TEXT NOT NULL UNIQUE, descripcion TEXT);
CREATE TABLE IF NOT EXISTS gastos (id INTEGER PRIMARY KEY AUTOINCREMENT, numero TEXT, categoria_id INTEGER NOT NULL, descripcion TEXT NOT NULL, monto REAL NOT NULL DEFAULT 0, subtotal REAL NOT NULL DEFAULT 0, impuesto REAL NOT NULL DEFAULT 0, total REAL NOT NULL DEFAULT 0, proveedor_id INTEGER, metodo_pago TEXT NOT NULL DEFAULT 'efectivo', referencia TEXT, comprobante TEXT, estado TEXT NOT NULL DEFAULT 'pendiente', notas TEXT, fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')), fecha_vencimiento TEXT, fecha_pago TEXT);
CREATE TABLE IF NOT EXISTS pagos_recibidos (id INTEGER PRIMARY KEY AUTOINCREMENT, venta_id INTEGER, cliente_id INTEGER, monto REAL NOT NULL, metodo_pago TEXT, referencia TEXT, notas TEXT, fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')));
CREATE TABLE IF NOT EXISTS pagos_realizados (id INTEGER PRIMARY KEY AUTOINCREMENT, gasto_id INTEGER, proveedor_id INTEGER, monto REAL NOT NULL, metodo_pago TEXT, referencia TEXT, notas TEXT, fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')));
CREATE TABLE IF NOT EXISTS asientos (id INTEGER PRIMARY KEY AUTOINCREMENT, numero TEXT, fecha TEXT NOT NULL DEFAULT (date('now')), concepto TEXT NOT NULL, descripcion TEXT, referencia TEXT, tipo TEXT NOT NULL DEFAULT 'manual', total_debe REAL NOT NULL DEFAULT 0, total_haber REAL NOT NULL DEFAULT 0, estado TEXT NOT NULL DEFAULT 'registrado');
CREATE TABLE IF NOT EXISTS asiento_lineas (id INTEGER PRIMARY KEY AUTOINCREMENT, asiento_id INTEGER NOT NULL, cuenta_id INTEGER NOT NULL, descripcion TEXT, debe REAL NOT NULL DEFAULT 0, haber REAL NOT NULL DEFAULT 0);
CREATE TABLE IF NOT EXISTS ubicaciones (id INTEGER PRIMARY KEY AUTOINCREMENT, nombre TEXT NOT NULL, encargado TEXT, cedula TEXT, telefono TEXT, ciudad TEXT, direccion TEXT, activo INTEGER NOT NULL DEFAULT 1);
CREATE TABLE IF NOT EXISTS maquinas_ubicadas (id INTEGER PRIMARY KEY AUTOINCREMENT, ubicacion_id INTEGER NOT NULL, codigo TEXT, descripcion TEXT, modelo TEXT, numero_serie TEXT, color TEXT, fecha_ingreso TEXT, fecha_instalacion TEXT NOT NULL DEFAULT (datetime('now','localtime')), comision REAL NOT NULL DEFAULT 0, comision_estimada REAL NOT NULL DEFAULT 0, dia_cobro INTEGER NOT NULL DEFAULT 1, activo INTEGER NOT NULL DEFAULT 1, notas TEXT);
CREATE TABLE IF NOT EXISTS cobros_comisiones (id INTEGER PRIMARY KEY AUTOINCREMENT, maquina_id INTEGER NOT NULL, monto REAL NOT NULL, mes TEXT, periodo TEXT, observacion TEXT, notas TEXT, fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')));
CREATE TABLE IF NOT EXISTS cuentas_credito (id INTEGER PRIMARY KEY AUTOINCREMENT, nombre TEXT NOT NULL DEFAULT '', tipo TEXT NOT NULL DEFAULT 'cliente', cliente_id INTEGER, proveedor_id INTEGER, limite REAL NOT NULL DEFAULT 0, saldo_actual REAL NOT NULL DEFAULT 0, notas TEXT, activo INTEGER NOT NULL DEFAULT 1, fecha_apertura TEXT NOT NULL DEFAULT (date('now')));
CREATE TABLE IF NOT EXISTS credito_movimientos (id INTEGER PRIMARY KEY AUTOINCREMENT, cuenta_id INTEGER NOT NULL, tipo TEXT NOT NULL, monto REAL NOT NULL, cantidad REAL NOT NULL DEFAULT 0, precio_unit REAL NOT NULL DEFAULT 0, saldo_acumulado REAL NOT NULL DEFAULT 0, colores TEXT, descripcion TEXT, referencia_id INTEGER, fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')));
CREATE TABLE IF NOT EXISTS ahorros (id INTEGER PRIMARY KEY AUTOINCREMENT, cliente_id INTEGER, saldo REAL NOT NULL DEFAULT 0, activo INTEGER NOT NULL DEFAULT 1, fecha_apertura TEXT NOT NULL DEFAULT (date('now')));
CREATE TABLE IF NOT EXISTS ahorro_movimientos (id INTEGER PRIMARY KEY AUTOINCREMENT, ahorro_id INTEGER, tipo TEXT NOT NULL, monto REAL NOT NULL, saldo_acumulado REAL NOT NULL DEFAULT 0, cobro_id INTEGER, descripcion TEXT, fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')));
CREATE TABLE IF NOT EXISTS garantias (id INTEGER PRIMARY KEY AUTOINCREMENT, producto_id INTEGER, venta_id INTEGER, producto TEXT NOT NULL DEFAULT '', numero_serie TEXT, cliente_nombre TEXT NOT NULL, cedula TEXT, telefono TEXT, direccion TEXT, ciudad TEXT, monto_pago REAL NOT NULL DEFAULT 0, estado TEXT NOT NULL DEFAULT 'vigente', observacion TEXT, descripcion TEXT, fecha_inicio TEXT NOT NULL, fecha_fin TEXT NOT NULL, activa INTEGER NOT NULL DEFAULT 1);
CREATE TABLE IF NOT EXISTS movimientos_inventario (id INTEGER PRIMARY KEY AUTOINCREMENT, producto_id INTEGER REFERENCES productos(id), producto_nombre TEXT NOT NULL DEFAULT '', tipo TEXT NOT NULL CHECK(tipo IN ('entrada','salida','ajuste')), cantidad INTEGER NOT NULL, motivo TEXT, referencia TEXT, fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')));
CREATE TABLE IF NOT EXISTS configuracion (clave TEXT PRIMARY KEY, valor TEXT);
CREATE TABLE IF NOT EXISTS deudas_empresa (id INTEGER PRIMARY KEY AUTOINCREMENT, numero TEXT NOT NULL, proveedor_id INTEGER REFERENCES proveedores(id), proveedor_nombre TEXT NOT NULL DEFAULT '', concepto TEXT NOT NULL, descripcion TEXT, categoria_id INTEGER REFERENCES categorias_gastos(id), categoria_nombre TEXT NOT NULL DEFAULT '', fecha_deuda TEXT NOT NULL DEFAULT (date('now')), fecha_vencimiento TEXT, monto_total REAL NOT NULL DEFAULT 0, saldo_pendiente REAL NOT NULL DEFAULT 0, referencia TEXT, notas TEXT, estado TEXT NOT NULL DEFAULT 'pendiente' CHECK(estado IN ('pendiente','pagada','cancelada')), activa INTEGER NOT NULL DEFAULT 1, fecha_registro TEXT NOT NULL DEFAULT (datetime('now','localtime')));
";

pub struct MigracionResultado {
    pub filas_migradas: u64,
    pub mensaje: String,
}

/// Busca la base de datos antigua del sistema Python en las ubicaciones habituales.
pub fn buscar_origen_python() -> Option<String> {
    let candidatos: Vec<std::path::PathBuf> = {
        let mut v = Vec::new();
        if let Some(roaming) = std::env::var_os("APPDATA") {
            v.push(std::path::PathBuf::from(&roaming).join("SistemaConta").join(PYTHON_DB));
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                v.push(dir.join(PYTHON_DB));
            }
        }
        v.push(std::path::PathBuf::from(PYTHON_DB));
        v
    };
    for p in candidatos {
        if p.exists() {
            return Some(p.to_string_lossy().to_string());
        }
    }
    None
}

/// Si el destino no existe y hay una base antigua Python, la migra automáticamente.
/// Devuelve `Ok(None)` si no hay nada que migrar.
pub fn auto_migrar(destino: &str) -> Result<Option<MigracionResultado>, String> {
    if Path::new(destino).exists() {
        return Ok(None);
    }
    let origen = match buscar_origen_python() {
        Some(o) => o,
        None => return Ok(None),
    };
    migrar_desde_python(&origen, destino).map(Some)
}

pub fn migrar_desde_python(origen: &str, destino: &str) -> Result<MigracionResultado, String> {
    if !Path::new(origen).exists() {
        return Err(format!("No se encuentra la base de datos origen '{}'", origen));
    }
    if Path::new(destino).exists() {
        std::fs::remove_file(destino).ok();
    }

    let mut reporte = String::new();
    reporte.push_str(&format!("=== Migración: Python → Rust ===\nOrigen:  {}\nDestino: {}\n\n", origen, destino));

    let start = Instant::now();

    let py = Connection::open(origen).map_err(|e| format!("Error al abrir Python DB: {}", e))?;
    let rust = Connection::open(destino).map_err(|e| format!("Error al crear Rust DB: {}", e))?;

    for stmt in CREATE_SCHEMA.split(';') {
        let s = stmt.trim();
        if !s.is_empty() {
            rust.execute_batch(s).ok();
        }
    }

    let mut total = 0u64;

    total += migrate_plan_cuentas(&py, &rust);
    total += migrate_categorias_productos(&py, &rust);
    total += migrate_categorias_gastos(&py, &rust);
    total += migrate_clientes(&py, &rust);
    total += migrate_proveedores(&py, &rust);
    total += migrate_productos(&py, &rust);
    total += migrate_ubicaciones(&py, &rust);
    total += migrate_ventas(&py, &rust);
    total += migrate_ventas_detalles(&py, &rust);
    total += migrate_movimientos_inventario(&rust);
    total += migrate_gastos(&py, &rust);
    total += migrate_maquinas(&py, &rust);
    total += migrate_cobros_comisiones(&py, &rust);
    total += migrate_garantias(&py, &rust);
    total += migrate_cuentas_credito(&py, &rust);
    total += migrate_credito_movimientos(&py, &rust);
    total += migrate_pagos_recibidos(&py, &rust);
    total += migrate_pagos_realizados(&py, &rust);
    total += migrate_asiento_lineas(&py, &rust);
    total += migrate_asientos(&py, &rust);
    total += migrate_db_version(&py, &rust);

    if table_has_rows(&py, "ahorro") {
        rust.execute("INSERT OR IGNORE INTO ahorros (id, cliente_id, saldo, activo, fecha_apertura) VALUES (1, NULL, 0, 1, date('now'))", []).ok();
        total += 1;
        reporte.push_str("  ahorros                         cuenta general creada\n");
        total += migrate_ahorro_movimientos(&py, &rust);
    }

    rust.execute_batch("PRAGMA foreign_keys=ON;").ok();

    limpiar_nulls_texto(&rust);

    let upd_ventas = rust.execute(
        "UPDATE ventas SET saldo_pendiente = total - COALESCE((SELECT SUM(monto) FROM pagos_recibidos WHERE venta_id=ventas.id),0) WHERE saldo_pendiente IS NULL OR total<>COALESCE((SELECT SUM(monto) FROM pagos_recibidos WHERE venta_id=ventas.id),0)+saldo_pendiente",
        [],
    ).ok();
    let upd_clientes = rust.execute(
        "UPDATE clientes SET saldo_pendiente = COALESCE((SELECT SUM(saldo_pendiente) FROM ventas WHERE cliente_id=clientes.id AND tipo='credito'),0) WHERE id IN (SELECT cliente_id FROM ventas WHERE cliente_id IS NOT NULL GROUP BY cliente_id)",
        [],
    ).ok();
    if upd_ventas.unwrap_or(0) > 0 { reporte.push_str("  saldos recalculados en ventas\n"); }
    if upd_clientes.unwrap_or(0) > 0 { reporte.push_str("  saldos recalculados en clientes\n"); }

    let upd_prov = rust.execute(
        "UPDATE proveedores SET saldo_pendiente = COALESCE((SELECT COALESCE(SUM(g.total),0)-COALESCE(SUM(p.monto),0) FROM gastos g LEFT JOIN pagos_realizados p ON p.gasto_id=g.id WHERE g.proveedor_id=proveedores.id),0)",
        [],
    ).ok();
    let upd_cred = rust.execute(
        "UPDATE cuentas_credito SET saldo_actual = COALESCE((SELECT m.saldo_acumulado FROM credito_movimientos m WHERE m.cuenta_id=cuentas_credito.id ORDER BY m.id DESC LIMIT 1),0)",
        [],
    ).ok();
    if upd_prov.unwrap_or(0) > 0 { reporte.push_str("  saldos recalculados en proveedores\n"); }
    if upd_cred.unwrap_or(0) > 0 { reporte.push_str("  saldos recalculados en cuentas_credito\n"); }

    let n_deudas = rust.execute(
        "INSERT OR IGNORE INTO deudas_empresa (numero, proveedor_id, proveedor_nombre, concepto, monto_total, saldo_pendiente, notas, estado, fecha_deuda, referencia)
         SELECT 'DEUDA-' || c.id, c.proveedor_id, COALESCE(c.nombre,''), COALESCE(c.notas,''), c.saldo_actual, c.saldo_actual, COALESCE(c.notas,''), 'pendiente', date('now'), ''
         FROM cuentas_credito c WHERE c.saldo_actual > 0.0",
        [],
    ).ok();
    if n_deudas.unwrap_or(0) > 0 { reporte.push_str(&format!("  deudas_empresa generadas: {}\n", n_deudas.unwrap())); }

    limpiar_nulls_texto(&rust);

    let elapsed = start.elapsed();
    reporte.push_str("\n=== Migración completada ===\n");
    reporte.push_str(&format!("  Filas migradas: {}\n", total));
    reporte.push_str(&format!("  Tiempo: {:.2}s\n", elapsed.as_secs_f64()));
    reporte.push_str(&verificar_conteos(&py, &rust));

    Ok(MigracionResultado { filas_migradas: total, mensaje: reporte })
}

fn verificar_conteos(py: &Connection, rust: &Connection) -> String {
    let pares: &[(&str, Option<&str>, &str, Option<&str>)] = &[
        ("plan_cuentas", Some("codigo"), "plan_cuentas", Some("codigo")),
        ("clientes", None, "clientes", None),
        ("proveedores", None, "proveedores", None),
        ("categorias_producto", Some("nombre"), "categorias_productos", Some("nombre")),
        ("categorias_gasto", Some("nombre"), "categorias_gastos", Some("nombre")),
        ("productos", None, "productos", None),
        ("ubicaciones", None, "ubicaciones", None),
        ("ventas", Some("numero"), "ventas", Some("folio")),
        ("venta_detalles", None, "ventas_detalles", None),
        ("gastos", None, "gastos", None),
        ("maquinas_ubicadas", None, "maquinas_ubicadas", None),
        ("cobros_comision", None, "cobros_comisiones", None),
        ("garantias", None, "garantias", None),
        ("cuentas_credito", None, "cuentas_credito", None),
        ("credito_movimientos", None, "credito_movimientos", None),
        ("pagos_recibidos", None, "pagos_recibidos", None),
        ("pagos_realizados", None, "pagos_realizados", None),
        ("asientos", None, "asientos", None),
        ("asiento_lineas", None, "asiento_lineas", None),
    ];
    let mut s = String::from("\n=== Verificación de conteos (origen → destino) ===\n");
    let mut hay_error = false;
    for (o, ok, d, dk) in pares {
        let sql_o = match ok { Some(k) => format!("SELECT COUNT(DISTINCT \"{}\") FROM \"{}\"", k, o), None => format!("SELECT COUNT(*) FROM \"{}\"", o) };
        let oc = py.query_row(&sql_o, [], |r| r.get::<_, i64>(0)).unwrap_or(0);
        let sql_d = match dk { Some(k) => format!("SELECT COUNT(DISTINCT \"{}\") FROM \"{}\"", k, d), None => format!("SELECT COUNT(*) FROM \"{}\"", d) };
        let dc = rust.query_row(&sql_d, [], |r| r.get::<_, i64>(0)).unwrap_or(-1);
        let estado = if oc == dc { "OK" } else { "DIFERENCIA!" };
        if oc != dc { hay_error = true; }
        s.push_str(&format!("  {:26} {:>5} → {:>5}  {}\n", format!("{}.{}", o, d), oc, dc, estado));
    }
    if hay_error {
        s.push_str("\n  !! HAY DIFERENCIAS DE CONTEO. Revisar arriba.\n");
    } else {
        s.push_str("\n  Todos los conteos coinciden.\n");
    }
    s
}

fn table_has_rows(py: &Connection, name: &str) -> bool {
    if name.is_empty() { return false; }
    let count: i64 = py
        .query_row(&format!("SELECT COUNT(*) FROM \"{}\"", name), [], |r| r.get(0))
        .unwrap_or(0);
    count > 0
}

fn table_count(py: &Connection, name: &str) -> u64 {
    if !table_has_rows(py, name) { return 0; }
    let count: i64 = py
        .query_row(&format!("SELECT COUNT(*) FROM \"{}\"", name), [], |r| r.get(0))
        .unwrap_or(0);
    count as u64
}

fn read_rows(py: &Connection, sql: &str) -> Vec<Vec<Value>> {
    let mut stmt = py.prepare(sql).unwrap();
    let n = stmt.column_count();
    stmt.query_map([], |r| {
        let mut row = Vec::with_capacity(n);
        for i in 0..n { row.push(r.get::<_, Value>(i).unwrap_or(Value::Null)); }
        Ok(row)
    }).unwrap().filter_map(|r| r.ok()).collect()
}

fn limpiar_nulls_texto(rust: &Connection) {
    for tbl in ["productos", "clientes", "proveedores", "categorias_productos", "categorias_gastos",
                "ventas", "ventas_detalles", "gastos", "pagos_recibidos", "pagos_realizados",
                "asientos", "asiento_lineas", "ubicaciones", "maquinas_ubicadas", "cobros_comisiones",
                "cuentas_credito", "credito_movimientos", "ahorros", "ahorro_movimientos", "garantias",
                "movimientos_inventario", "deudas_empresa", "plan_cuentas"] {
        let cols: Vec<(String, String)> = rust.prepare(&format!("PRAGMA table_info({})", tbl)).unwrap()
            .query_map([], |r| Ok((r.get::<_, String>(1)?, r.get::<_, String>(2)?))).unwrap()
            .filter_map(|r| r.ok()).collect();
        for (c, typ) in cols {
            let t = typ.to_uppercase();
            let es_texto = t.contains("TEXT") || t.contains("CHAR") || t.contains("CLOB") || t.is_empty();
            if es_texto {
                rust.execute(&format!("UPDATE {} SET \"{}\" = '' WHERE \"{}\" IS NULL", tbl, c, c), []).ok();
            }
        }
    }
}

macro_rules! migrate_table {
    ($py:expr, $rust:expr, $name:expr, $py_sql:expr, $rust_sql:expr, $ncols:expr) => {{
        let name = $name;
        let count = table_count($py, name);
        if count == 0 { 0 } else {
            let rows = read_rows($py, $py_sql);
            let tx = rusqlite::Transaction::new_unchecked($rust, TransactionBehavior::Immediate).unwrap();
            let mut ok = 0u64;
            let mut err = 0u64;
            for r in &rows {
                let params: Vec<&dyn rusqlite::types::ToSql> = r.iter().map(|v| v as &dyn rusqlite::types::ToSql).collect();
                match tx.execute($rust_sql, params.as_slice()) {
                    Ok(0) => { err += 1; if err <= 5 { println!("    !! fila omitida (sin insertar): {}", name); } }
                    Ok(_) => ok += 1,
                    Err(e) => { err += 1; if err <= 5 { println!("    !! fila omitida: {}: {}", name, e); } }
                }
            }
            tx.commit().ok();
            println!("  {:30} {} filas ({} ok, {} omitidas)", name, count, ok, err);
            ok
        }
    }};
}

fn migrate_plan_cuentas(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "plan_cuentas",
        "SELECT id, codigo, nombre, tipo, naturaleza, activo FROM plan_cuentas",
        "INSERT OR IGNORE INTO plan_cuentas (id, codigo, nombre, tipo, naturaleza, nivel, padre_id, activo) VALUES (?1, ?2, ?3, ?4, ?5, 1, NULL, ?6)",
        6)
}

fn migrate_categorias_productos(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "categorias_producto",
        "SELECT id, nombre, descripcion FROM categorias_producto",
        "INSERT OR IGNORE INTO categorias_productos (id, nombre, descripcion) VALUES (?1, ?2, ?3)",
        3)
}

fn migrate_categorias_gastos(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "categorias_gasto",
        "SELECT id, nombre, descripcion FROM categorias_gasto",
        "INSERT OR IGNORE INTO categorias_gastos (id, nombre, descripcion) VALUES (?1, ?2, ?3)",
        3)
}

fn migrate_clientes(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "clientes",
        "SELECT id, codigo, nombre, rfc, email, telefono, direccion, ciudad, limite_credito, activo, fecha_registro FROM clientes",
        "INSERT OR IGNORE INTO clientes (id, codigo, nombre, rfc, email, telefono, direccion, ciudad, limite_credito, saldo_pendiente, activo, fecha_registro) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0, ?10, ?11)",
        11)
}

fn migrate_proveedores(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "proveedores",
        "SELECT id, codigo, nombre, rfc, email, telefono, direccion, ciudad, activo, fecha_registro FROM proveedores",
        "INSERT OR IGNORE INTO proveedores (id, codigo, nombre, contacto, rfc, email, telefono, direccion, ciudad, saldo_pendiente, activo, fecha_registro) VALUES (?1, ?2, ?3, '', ?4, ?5, ?6, ?7, ?8, 0, ?9, ?10)",
        10)
}

fn migrate_productos(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "productos",
        "SELECT id, codigo, nombre, descripcion, categoria_id, precio_compra, precio_venta, stock, stock_minimo, unidad, activo FROM productos",
        "INSERT OR IGNORE INTO productos (id, codigo, nombre, descripcion, categoria_id, precio_compra, precio_venta, stock, stock_minimo, unidad, activo) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        11)
}

fn migrate_ubicaciones(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "ubicaciones",
        "SELECT id, nombre, encargado, cedula, telefono, ciudad, direccion, activo FROM ubicaciones",
        "INSERT OR IGNORE INTO ubicaciones (id, nombre, encargado, cedula, telefono, ciudad, direccion, activo) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        8)
}

fn migrate_ventas(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "ventas",
        "SELECT id, numero, fecha, cliente_id, subtotal, impuesto, descuento, total, estado, metodo_pago, notas, fecha_vencimiento, fecha_pago FROM ventas",
        "INSERT OR IGNORE INTO ventas (id, folio, cliente_id, cliente_nombre, fecha, subtotal, impuesto, descuento, total, saldo_pendiente, tipo, estado, metodo_pago, notas, fecha_vencimiento, fecha_pago) VALUES (?1, ?2, ?4, '', ?3, ?5, ?6, ?7, ?8, 0, 'contado', COALESCE(?9,'pendiente'), ?10, COALESCE(?11,''), ?12, ?13)",
        13)
}

fn migrate_ventas_detalles(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "venta_detalles",
        "SELECT d.id, d.venta_id, d.producto_id, d.descripcion, COALESCE(d.descripcion, p.nombre, ''), d.cantidad, d.precio_unitario, d.descuento, d.subtotal
         FROM venta_detalles d LEFT JOIN productos p ON p.id = d.producto_id",
        "INSERT OR IGNORE INTO ventas_detalles (id, venta_id, producto_id, descripcion, producto_nombre, cantidad, precio_unitario, descuento, importe) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        9)
}

fn migrate_gastos(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "gastos",
        "SELECT id, numero, fecha, proveedor_id, categoria_id, descripcion, subtotal, impuesto, total, estado, metodo_pago, comprobante, notas, fecha_vencimiento, fecha_pago FROM gastos",
        "INSERT OR IGNORE INTO gastos (id, numero, categoria_id, descripcion, monto, subtotal, impuesto, total, proveedor_id, metodo_pago, referencia, comprobante, estado, notas, fecha, fecha_vencimiento, fecha_pago) VALUES (?1, ?2, COALESCE(?5,1), COALESCE(?6,''), ?9, COALESCE(?7,0), COALESCE(?8,0), COALESCE(?9,0), ?4, COALESCE(?11,'efectivo'), '', ?12, COALESCE(?10,'pendiente'), COALESCE(?13,''), ?3, ?14, ?15)",
        15)
}

fn migrate_maquinas(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "maquinas_ubicadas",
        "SELECT id, ubicacion_id, descripcion, color, fecha_ingreso, fecha_ingreso, comision_estimada, comision_estimada, dia_cobro, activo, notas FROM maquinas_ubicadas",
        "INSERT OR IGNORE INTO maquinas_ubicadas (id, ubicacion_id, codigo, descripcion, modelo, numero_serie, color, fecha_ingreso, fecha_instalacion, comision, comision_estimada, dia_cobro, activo, notas) VALUES (?1, ?2, NULL, ?3, '', '', ?4, ?5, COALESCE(?6, 'now'), ?7, ?8, ?9, ?10, ?11)",
        11)
}

fn migrate_cobros_comisiones(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "cobros_comision",
        "SELECT id, maquina_id, fecha, mes, monto, observacion FROM cobros_comision",
        "INSERT OR IGNORE INTO cobros_comisiones (id, maquina_id, monto, mes, periodo, observacion, notas, fecha) VALUES (?1, ?2, ?5, ?4, ?4, ?6, '', ?3)",
        6)
}

fn migrate_garantias(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "garantias",
        "SELECT id, venta_id, fecha_venta, fecha_vencimiento, cliente_nombre, cedula, telefono, direccion, ciudad, producto, numero_serie, monto_pago, estado, observacion, estado FROM garantias",
        "INSERT OR IGNORE INTO garantias (id, producto_id, venta_id, producto, numero_serie, cliente_nombre, cedula, telefono, direccion, ciudad, monto_pago, estado, observacion, descripcion, fecha_inicio, fecha_fin, activa) VALUES (?1, NULL, ?2, ?10, ?11, ?5, ?6, ?7, ?8, ?9, ?12, ?13, ?14, '', ?3, ?4, CASE WHEN ?15='vigente' THEN 1 ELSE 0 END)",
        15)
}

fn migrate_cuentas_credito(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "cuentas_credito",
        "SELECT id, nombre, tipo, proveedor_id, cliente_id, notas, activo FROM cuentas_credito",
        "INSERT OR IGNORE INTO cuentas_credito (id, nombre, tipo, cliente_id, proveedor_id, limite, saldo_actual, notas, activo, fecha_apertura) VALUES (?1, ?2, ?3, ?5, ?4, 0, 0, ?6, ?7, date('now'))",
        7)
}

fn migrate_credito_movimientos(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "credito_movimientos",
        "SELECT id, cuenta_id, fecha, tipo, descripcion, CAST(COALESCE(NULLIF(cantidad,''),'0') AS REAL), COALESCE(precio_unit,0), monto, saldo_acumulado, colores FROM credito_movimientos",
        "INSERT OR IGNORE INTO credito_movimientos (id, cuenta_id, tipo, monto, cantidad, precio_unit, saldo_acumulado, colores, descripcion, referencia_id, fecha) VALUES (?1, ?2, ?4, ?8, ?6, ?7, ?9, ?10, ?5, NULL, ?3)",
        10)
}

fn migrate_pagos_recibidos(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "pagos_recibidos",
        "SELECT id, fecha, venta_id, cliente_id, monto, metodo_pago, referencia, notas FROM pagos_recibidos",
        "INSERT OR IGNORE INTO pagos_recibidos (id, venta_id, cliente_id, monto, metodo_pago, referencia, notas, fecha) VALUES (?1, ?3, ?4, ?5, ?6, ?7, ?8, ?2)",
        8)
}

fn migrate_pagos_realizados(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "pagos_realizados",
        "SELECT id, fecha, gasto_id, proveedor_id, monto, metodo_pago, referencia, notas FROM pagos_realizados",
        "INSERT OR IGNORE INTO pagos_realizados (id, gasto_id, proveedor_id, monto, metodo_pago, referencia, notas, fecha) VALUES (?1, ?3, ?4, ?5, ?6, ?7, ?8, ?2)",
        8)
}

fn migrate_asientos(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "asientos",
        "SELECT a.id, a.numero, a.fecha, a.descripcion, a.referencia, a.tipo, COALESCE((SELECT SUM(debe) FROM asiento_lineas WHERE asiento_id=a.id),0), COALESCE((SELECT SUM(haber) FROM asiento_lineas WHERE asiento_id=a.id),0) FROM asientos a",
        "INSERT OR IGNORE INTO asientos (id, numero, fecha, concepto, descripcion, referencia, tipo, total_debe, total_haber, estado) VALUES (?1, ?2, ?3, ?4, NULL, ?5, ?6, ?7, ?8, 'registrado')",
        8)
}

fn migrate_asiento_lineas(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "asiento_lineas",
        "SELECT id, asiento_id, cuenta_id, descripcion, debe, haber FROM asiento_lineas",
        "INSERT OR IGNORE INTO asiento_lineas (id, asiento_id, cuenta_id, descripcion, debe, haber) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        6)
}

fn migrate_db_version(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "db_version",
        "SELECT id, version, created_at, updated_at FROM db_version",
        "INSERT OR IGNORE INTO db_version (id, version, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        4)
}

fn migrate_ahorro_movimientos(py: &Connection, rust: &Connection) -> u64 {
    migrate_table!(py, rust, "ahorro_movimientos",
        "SELECT id, fecha, COALESCE(tipo,'deposito'), descripcion, COALESCE(monto,0), saldo_acumulado, cobro_id FROM ahorro",
        "INSERT OR IGNORE INTO ahorro_movimientos (id, ahorro_id, tipo, monto, saldo_acumulado, cobro_id, descripcion, fecha) VALUES (?1, 1, ?3, ?5, ?6, ?7, ?4, ?2)",
        7)
}

fn migrate_movimientos_inventario(rust: &Connection) -> u64 {
    if !table_has_rows(rust, "ventas_detalles") { return 0; }
    let ok = rust.execute(
        "INSERT OR IGNORE INTO movimientos_inventario (producto_id, producto_nombre, tipo, cantidad, motivo, referencia)
         SELECT d.producto_id, d.producto_nombre, 'salida', -d.cantidad, 'Venta', v.folio
         FROM ventas_detalles d JOIN ventas v ON v.id=d.venta_id",
        [],
    ).unwrap_or(0);
    if ok > 0 { println!("  movimientos_inventario           {} filas", ok); }
    ok as u64
}

impl DatabaseManager {
    /// Migración EN CALIENTE (append): copia movimientos desde una DB legacy (`contabilidad.db`)
    /// a la base activa (`contabilidad_rust.db`) SIN borrarla. Segura para disparar desde la API
    /// con la app ya en marcha (SQLite WAL permite lectores concurrentes).
    pub fn migrar_desde_archivo(&self, origen: &str) -> Result<MigracionResultado, String> {
        if !Path::new(origen).exists() {
            return Err(format!(
                "No se encuentra la base origen '{}' (cwd: {})",
                origen,
                std::env::current_dir().unwrap().display()
            ));
        }
        let py = Connection::open(origen).map_err(|e| format!("Error al abrir DB origen: {}", e))?;
        let rust = self.conn.lock().unwrap();
        let start = Instant::now();

        let mut reporte = format!("=== Migración en caliente: {} → contabilidad_rust.db ===\n", origen);

        for stmt in CREATE_SCHEMA.split(';') {
            let s = stmt.trim();
            if !s.is_empty() { rust.execute_batch(s).ok(); }
        }

        let mut total = 0u64;
        total += migrate_plan_cuentas(&py, &rust);
        total += migrate_categorias_productos(&py, &rust);
        total += migrate_categorias_gastos(&py, &rust);
        total += migrate_clientes(&py, &rust);
        total += migrate_proveedores(&py, &rust);
        total += migrate_productos(&py, &rust);
        total += migrate_ubicaciones(&py, &rust);
        total += migrate_ventas(&py, &rust);
        total += migrate_ventas_detalles(&py, &rust);
        total += migrate_movimientos_inventario(&rust);
        total += migrate_gastos(&py, &rust);
        total += migrate_maquinas(&py, &rust);
        total += migrate_cobros_comisiones(&py, &rust);
        total += migrate_garantias(&py, &rust);
        total += migrate_cuentas_credito(&py, &rust);
        total += migrate_credito_movimientos(&py, &rust);
        total += migrate_pagos_recibidos(&py, &rust);
        total += migrate_pagos_realizados(&py, &rust);
        total += migrate_asiento_lineas(&py, &rust);
        total += migrate_asientos(&py, &rust);
        total += migrate_db_version(&py, &rust);

        if table_has_rows(&py, "ahorro") {
            rust.execute("INSERT OR IGNORE INTO ahorros (id, cliente_id, saldo, activo, fecha_apertura) VALUES (1, NULL, 0, 1, date('now'))", []).ok();
            total += 1;
            total += migrate_ahorro_movimientos(&py, &rust);
        }

        rust.execute_batch("PRAGMA foreign_keys=ON;").ok();
        limpiar_nulls_texto(&rust);

        let upd_ventas = rust.execute("UPDATE ventas SET saldo_pendiente = total - COALESCE((SELECT SUM(monto) FROM pagos_recibidos WHERE venta_id=ventas.id),0) WHERE saldo_pendiente IS NULL OR total<>COALESCE((SELECT SUM(monto) FROM pagos_recibidos WHERE venta_id=ventas.id),0)+saldo_pendiente", []).ok();
        let upd_clientes = rust.execute("UPDATE clientes SET saldo_pendiente = COALESCE((SELECT SUM(saldo_pendiente) FROM ventas WHERE cliente_id=clientes.id AND tipo='credito'),0) WHERE id IN (SELECT cliente_id FROM ventas WHERE cliente_id IS NOT NULL GROUP BY cliente_id)", []).ok();
        let upd_prov = rust.execute("UPDATE proveedores SET saldo_pendiente = COALESCE((SELECT COALESCE(SUM(g.total),0)-COALESCE(SUM(p.monto),0) FROM gastos g LEFT JOIN pagos_realizados p ON p.gasto_id=g.id WHERE g.proveedor_id=proveedores.id),0)", []).ok();
        let upd_cred = rust.execute("UPDATE cuentas_credito SET saldo_actual = COALESCE((SELECT m.saldo_acumulado FROM credito_movimientos m WHERE m.cuenta_id=cuentas_credito.id ORDER BY m.id DESC LIMIT 1),0)", []).ok();
        if upd_ventas.unwrap_or(0) > 0 { reporte.push_str("  saldos recalculados en ventas\n"); }
        if upd_clientes.unwrap_or(0) > 0 { reporte.push_str("  saldos recalculados en clientes\n"); }
        if upd_prov.unwrap_or(0) > 0 { reporte.push_str("  saldos recalculados en proveedores\n"); }
        if upd_cred.unwrap_or(0) > 0 { reporte.push_str("  saldos recalculados en cuentas_credito\n"); }

        limpiar_nulls_texto(&rust);
        let elapsed = start.elapsed();
        reporte.push_str(&format!("\n=== Migración completada ===\n  Filas migradas: {}\n  Tiempo: {:.2}s\n", total, elapsed.as_secs_f64()));
        reporte.push_str(&verificar_conteos(&py, &rust));

        Ok(MigracionResultado { filas_migradas: total, mensaje: reporte })
    }
}
