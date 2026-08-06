use rusqlite::Connection;
use std::path::PathBuf;

fn main() {
    let out: PathBuf = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "contabilidad.db".to_string())
        .into();
    if let Some(p) = out.parent() {
        let _ = std::fs::create_dir_all(p);
    }
    let conn = Connection::open(&out).expect("abrir db");
    let c = &conn;

    // ---- Esquema legacy (columnas que lee migrar.rs en src/db/migrar.rs) ----
    let schema = "
    CREATE TABLE IF NOT EXISTS plan_cuentas (id INTEGER PRIMARY KEY, codigo TEXT, nombre TEXT, tipo TEXT, naturaleza TEXT, activo INTEGER);
    CREATE TABLE IF NOT EXISTS categorias_producto (id INTEGER PRIMARY KEY, nombre TEXT, descripcion TEXT);
    CREATE TABLE IF NOT EXISTS categorias_gasto (id INTEGER PRIMARY KEY, nombre TEXT, descripcion TEXT);
    CREATE TABLE IF NOT EXISTS clientes (id INTEGER PRIMARY KEY, codigo TEXT, nombre TEXT, rfc TEXT, email TEXT, telefono TEXT, direccion TEXT, ciudad TEXT, limite_credito REAL, activo INTEGER, fecha_registro TEXT);
    CREATE TABLE IF NOT EXISTS proveedores (id INTEGER PRIMARY KEY, codigo TEXT, nombre TEXT, rfc TEXT, email TEXT, telefono TEXT, direccion TEXT, ciudad TEXT, activo INTEGER, fecha_registro TEXT);
    CREATE TABLE IF NOT EXISTS productos (id INTEGER PRIMARY KEY, codigo TEXT, nombre TEXT, descripcion TEXT, categoria_id INTEGER, precio_compra REAL, precio_venta REAL, stock INTEGER, stock_minimo INTEGER, unidad TEXT, activo INTEGER);
    CREATE TABLE IF NOT EXISTS ubicaciones (id INTEGER PRIMARY KEY, nombre TEXT, encargado TEXT, cedula TEXT, telefono TEXT, ciudad TEXT, direccion TEXT, activo INTEGER);
    CREATE TABLE IF NOT EXISTS ventas (id INTEGER PRIMARY KEY, numero TEXT, fecha TEXT, cliente_id INTEGER, subtotal REAL, impuesto REAL, descuento REAL, total REAL, estado TEXT, metodo_pago TEXT, notas TEXT, fecha_vencimiento TEXT, fecha_pago TEXT);
    CREATE TABLE IF NOT EXISTS venta_detalles (id INTEGER PRIMARY KEY, venta_id INTEGER, producto_id INTEGER, descripcion TEXT, cantidad INTEGER, precio_unitario REAL, descuento REAL, subtotal REAL);
    CREATE TABLE IF NOT EXISTS gastos (id INTEGER PRIMARY KEY, numero TEXT, fecha TEXT, proveedor_id INTEGER, categoria_id INTEGER, descripcion TEXT, subtotal REAL, impuesto REAL, total REAL, estado TEXT, metodo_pago TEXT, comprobante TEXT, notas TEXT, fecha_vencimiento TEXT, fecha_pago TEXT);
    CREATE TABLE IF NOT EXISTS maquinas_ubicadas (id INTEGER PRIMARY KEY, ubicacion_id INTEGER, descripcion TEXT, color TEXT, fecha_ingreso TEXT, fecha_instalacion TEXT, comision REAL, comision_estimada REAL, dia_cobro INTEGER, activo INTEGER, notas TEXT);
    CREATE TABLE IF NOT EXISTS cobros_comision (id INTEGER PRIMARY KEY, maquina_id INTEGER, fecha TEXT, mes TEXT, monto REAL, observacion TEXT);
    CREATE TABLE IF NOT EXISTS garantias (id INTEGER PRIMARY KEY, venta_id INTEGER, fecha_venta TEXT, fecha_vencimiento TEXT, cliente_nombre TEXT, cedula TEXT, telefono TEXT, direccion TEXT, ciudad TEXT, producto TEXT, numero_serie TEXT, monto_pago REAL, estado TEXT, observacion TEXT);
    CREATE TABLE IF NOT EXISTS cuentas_credito (id INTEGER PRIMARY KEY, nombre TEXT, tipo TEXT, proveedor_id INTEGER, cliente_id INTEGER, notas TEXT, activo INTEGER);
    CREATE TABLE IF NOT EXISTS credito_movimientos (id INTEGER PRIMARY KEY, cuenta_id INTEGER, fecha TEXT, tipo TEXT, descripcion TEXT, cantidad REAL, precio_unit REAL, monto REAL, saldo_acumulado REAL, colores TEXT);
    CREATE TABLE IF NOT EXISTS pagos_recibidos (id INTEGER PRIMARY KEY, fecha TEXT, venta_id INTEGER, cliente_id INTEGER, monto REAL, metodo_pago TEXT, referencia TEXT, notas TEXT);
    CREATE TABLE IF NOT EXISTS pagos_realizados (id INTEGER PRIMARY KEY, fecha TEXT, gasto_id INTEGER, proveedor_id INTEGER, monto REAL, metodo_pago TEXT, referencia TEXT, notas TEXT);
    CREATE TABLE IF NOT EXISTS asientos (id INTEGER PRIMARY KEY, numero TEXT, fecha TEXT, descripcion TEXT, referencia TEXT, tipo TEXT);
    CREATE TABLE IF NOT EXISTS asiento_lineas (id INTEGER PRIMARY KEY, asiento_id INTEGER, cuenta_id INTEGER, descripcion TEXT, debe REAL, haber REAL);
    CREATE TABLE IF NOT EXISTS db_version (id INTEGER PRIMARY KEY, version TEXT, created_at TEXT, updated_at TEXT);
    CREATE TABLE IF NOT EXISTS ahorro (id INTEGER PRIMARY KEY, fecha TEXT, tipo TEXT, descripcion TEXT, monto REAL, saldo_acumulado REAL, cobro_id INTEGER);
    ";
    c.execute_batch(schema).expect("schema");

    // ---- CategorÃ­as ----
    c.execute_batch("INSERT INTO categorias_producto (nombre,descripcion) VALUES ('General','Productos generales')").ok();
    c.execute_batch("INSERT INTO categorias_gasto (nombre,descripcion) VALUES ('Servicios','Luz agua internet')").ok();
    c.execute_batch("INSERT INTO categorias_gasto (nombre,descripcion) VALUES ('Insumos','Materiales')").ok();

    // ---- Clientes / Proveedores ----
    c.execute_batch("INSERT INTO clientes (codigo,nombre,rfc,ciudad,limite_credito,activo,fecha_registro) VALUES ('C001','Acme Corp','RFC1', 'Toluca', 50000, 1, date('now'))").ok();
    c.execute_batch("INSERT INTO clientes (codigo,nombre,rfc,ciudad,limite_credito,activo,fecha_registro) VALUES ('C002','Beta S de RL','RFC2', 'CDMX', 20000, 1, date('now'))").ok();
    c.execute_batch("INSERT INTO proveedores (codigo,nombre,rfc,ciudad,activo,fecha_registro) VALUES ('P001','Proveer Suministros','RPF1', 'Toluca', 1, date('now'))").ok();
    c.execute_batch("INSERT INTO proveedores (codigo,nombre,rfc,ciudad,activo,fecha_registro) VALUES ('P002','Tech Devices','RPF2', 'Monterrey', 1, date('now'))").ok();

    // ---- Productos ----
    c.execute_batch("INSERT INTO productos (codigo,nombre,descripcion,categoria_id,precio_compra,precio_venta,stock,stock_minimo,unidad,activo) VALUES ('PROD1','Laptop','Laptop 15 pulgadas',1,5000,6500,10,5,'pz',1)").ok();
    c.execute_batch("INSERT INTO productos (codigo,nombre,descripcion,categoria_id,precio_compra,precio_venta,stock,stock_minimo,unidad,activo) VALUES ('PROD2','Mouse','Mouse inalÃ¡mbrico',1,150,250,50,20,'pz',1)").ok();
    c.execute_batch("INSERT INTO productos (codigo,nombre,descripcion,categoria_id,precio_compra,precio_venta,stock,stock_minimo,unidad,activo) VALUES ('PROD3','Servicio InstalaciÃ³n','Mano de obra',2,0,800,0,0,'svc',1)").ok();

    // ---- Ubicaciones / mÃ¡quinas ----
    c.execute_batch("INSERT INTO ubicaciones (nombre,encargado,telefono,ciudad,activo) VALUES ('Suc. Toluca','Carlos PÃ©rez','555-1001','Toluca',1)").ok();
    c.execute_batch("INSERT INTO maquinas_ubicadas (ubicacion_id,descripcion,color,fecha_ingreso,fecha_instalacion,comision,comision_estimada,dia_cobro,activo,notas) VALUES (1,'Router empresa','Negro','2025-01-15','2025-02-01',500,500,5,1,'Migrado')").ok();

    // ---- Ventas + detalles ----
    c.execute_batch("INSERT INTO ventas (numero,fecha,cliente_id,subtotal,impuesto,descuento,total,estado,metodo_pago,notas,fecha_vencimiento) VALUES ('V-0001', datetime('now','localtime'), 1, 10000, 1500, 0, 11500, 'completada', 'efectivo', 'Ticket #1', date('now','+15 days'))").ok();
    c.execute_batch("INSERT INTO venta_detalles (venta_id,producto_id,descripcion,cantidad,precio_unitario,descuento,subtotal) VALUES (1,1,'Laptop 15 pulgadas',1,10000,0,10000)").ok();
    c.execute_batch("INSERT INTO ventas (numero,fecha,cliente_id,subtotal,impuesto,descuento,total,estado,metodo_pago,notas,fecha_vencimiento) VALUES ('V-0002', datetime('now','localtime'), 2, 750, 112.5, 0, 862.5, 'completada', 'tarjeta', 'Ticket #2', date('now','+10 days'))").ok();
    c.execute_batch("INSERT INTO venta_detalles (venta_id,producto_id,descripcion,cantidad,precio_unitario,descuento,subtotal) VALUES (2,2,'Mouse inalÃ¡mbrico',3,250,0,750)").ok();
    // venta a crÃ©dito (para probar saldo_pendiente en clientes)
    c.execute_batch("INSERT INTO ventas (numero,fecha,cliente_id,subtotal,impuesto,descuento,total,estado,metodo_pago,notas,fecha_vencimiento) VALUES ('V-0003', datetime('now','localtime'), 1, 5000, 750, 0, 5750, 'completada', NULL, 'A crÃ©dito', date('now','+30 days'))").ok();
    c.execute_batch("INSERT INTO venta_detalles (venta_id,producto_id,descripcion,cantidad,precio_unitario,descuento,subtotal) VALUES (3,1,'Laptop 15 pulgadas',1,5000,0,5000)").ok();

    // ---- Gastos ----
    c.execute_batch("INSERT INTO gastos (numero,fecha,proveedor_id,categoria_id,descripcion,subtotal,impuesto,total,estado,metodo_pago,notas,fecha_vencimiento) VALUES ('G-001', datetime('now','localtime'), 1, 1, 'Servicios del mes', 9500, 0, 9500, 'pagado', 'transferencia', 'Luz/agua/Internet', NULL)").ok();
    c.execute_batch("INSERT INTO gastos (numero,fecha,proveedor_id,categoria_id,descripcion,subtotal,impuesto,total,estado,metodo_pago,notas,fecha_vencimiento) VALUES ('G-002', datetime('now','localtime'), 2, 2, 'Materiales', 3000, 480, 3480, 'pagado', 'efectivo', 'Refacciones', NULL)").ok();

    // ---- Cobros de comisiÃ³n (pendientes) ----
    c.execute_batch("INSERT INTO cobros_comision (maquina_id,fecha,mes,monto,observacion) VALUES (1, datetime('now','localtime'), strftime('%Y-%m','now'), 500, 'ComisiÃ³n mensual mÃ¡quina 1')").ok();

    // ---- Pagos recibidos / realizados ----
    c.execute_batch("INSERT INTO pagos_recibidos (fecha,venta_id,cliente_id,monto,metodo_pago,referencia,notas) VALUES (datetime('now','localtime'),1,1,5000,'efectivo','','Pago parcial V-0001')").ok();
    c.execute_batch("INSERT INTO pagos_realizados (fecha,gasto_id,proveedor_id,monto,metodo_pago,referencia) VALUES (datetime('now','localtime'),1,1,9500,'transferencia','')").ok();

    // ---- CrÃ©ditos / cuentas_credito + movimientos ----
    c.execute_batch("INSERT INTO cuentas_credito (nombre,tipo,cliente_id,notas,activo) VALUES ('CrÃ©dito Acme','cliente',1,'Cuenta crÃ©dito',1)").ok();
    c.execute_batch("INSERT INTO credito_movimientos (cuenta_id,fecha,tipo,descripcion, cantidad, precio_unit, monto, saldo_acumulado, colores) VALUES (1, datetime('now','localtime'), 'cargo','Venta laptop',1,5000,5000,5000,'')").ok();

    // ---- Asientos ----
    c.execute_batch("INSERT INTO asientos (numero,fecha,descripcion,referencia,tipo) VALUES ('A-001', date('now'), 'Apertura', 'init', 'manual')").ok();
    c.execute_batch("INSERT INTO asiento_lineas (asiento_id,cuenta_id,descripcion,debe,haber) VALUES (1,1,'Plan de cuentas',0,0)").ok();

    // ---- Ahorro (legacy 'ahorro') ----
    c.execute_batch("INSERT INTO ahorro (fecha,tipo,descripcion,monto,saldo_acumulado,cobro_id) VALUES (datetime('now','localtime'),'deposito','Ahorro inicial',1000,1000,NULL)").ok();

    // ---- db_version ----
    c.execute_batch("INSERT INTO db_version (version,created_at,updated_at) VALUES ('1.0', datetime('now','localtime'), datetime('now','localtime'))").ok();

    println!("Legacy DB de simulaciÃ³n creada en: {}", out.display());
    println!("Tablas con datos de prueba (clientes, proveedores, productos, ventas+detalles, gastos, cobros_comision, maquinas, pagos, crÃ©ditos, asientos, ahorro, db_version).");
}

