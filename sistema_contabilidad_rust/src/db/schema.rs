pub const CREATE_TABLES: &str = "
CREATE TABLE IF NOT EXISTS db_version (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    version TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS usuarios (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre_usuario TEXT NOT NULL UNIQUE,
    contrasena_hash TEXT NOT NULL,
    activo INTEGER NOT NULL DEFAULT 1,
    fecha_registro TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS plan_cuentas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL,
    tipo TEXT NOT NULL CHECK(tipo IN ('activo','pasivo','capital','ingreso','gasto')),
    naturaleza TEXT NOT NULL DEFAULT 'deudora' CHECK(naturaleza IN ('deudora','acreedora')),
    nivel INTEGER NOT NULL DEFAULT 1,
    padre_id INTEGER REFERENCES plan_cuentas(id),
    activo INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS clientes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT,
    nombre TEXT NOT NULL,
    rfc TEXT,
    email TEXT,
    telefono TEXT,
    direccion TEXT,
    ciudad TEXT,
    limite_credito REAL NOT NULL DEFAULT 0,
    saldo_pendiente REAL NOT NULL DEFAULT 0,
    activo INTEGER NOT NULL DEFAULT 1,
    fecha_registro TEXT NOT NULL DEFAULT (date('now'))
);

CREATE TABLE IF NOT EXISTS proveedores (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT,
    nombre TEXT NOT NULL,
    contacto TEXT,
    rfc TEXT,
    email TEXT,
    telefono TEXT,
    direccion TEXT,
    ciudad TEXT,
    saldo_pendiente REAL NOT NULL DEFAULT 0,
    activo INTEGER NOT NULL DEFAULT 1,
    fecha_registro TEXT NOT NULL DEFAULT (date('now'))
);

CREATE TABLE IF NOT EXISTS categorias_productos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL UNIQUE,
    descripcion TEXT
);

CREATE TABLE IF NOT EXISTS productos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo TEXT,
    nombre TEXT NOT NULL,
    descripcion TEXT,
    categoria_id INTEGER REFERENCES categorias_productos(id),
    precio_compra REAL NOT NULL DEFAULT 0,
    precio_venta REAL NOT NULL DEFAULT 0,
    stock INTEGER NOT NULL DEFAULT 0,
    stock_minimo INTEGER NOT NULL DEFAULT 0,
    unidad TEXT NOT NULL DEFAULT 'pza',
    activo INTEGER NOT NULL DEFAULT 1,
    fecha_registro TEXT NOT NULL DEFAULT (date('now'))
);

CREATE TABLE IF NOT EXISTS ventas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    folio TEXT NOT NULL UNIQUE,
    cliente_id INTEGER REFERENCES clientes(id),
    cliente_nombre TEXT NOT NULL DEFAULT '',
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')),
    subtotal REAL NOT NULL DEFAULT 0,
    impuesto REAL NOT NULL DEFAULT 0,
    descuento REAL NOT NULL DEFAULT 0,
    total REAL NOT NULL DEFAULT 0,
    saldo_pendiente REAL NOT NULL DEFAULT 0,
    tipo TEXT NOT NULL DEFAULT 'contado' CHECK(tipo IN ('contado','credito')),
    estado TEXT NOT NULL DEFAULT 'completada' CHECK(estado IN ('pendiente','completada','cancelada')),
    metodo_pago TEXT,
    notas TEXT,
    fecha_vencimiento TEXT,
    fecha_pago TEXT
);

CREATE TABLE IF NOT EXISTS ventas_detalles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    venta_id INTEGER NOT NULL REFERENCES ventas(id),
    producto_id INTEGER REFERENCES productos(id),
    descripcion TEXT,
    producto_nombre TEXT NOT NULL,
    cantidad INTEGER NOT NULL DEFAULT 1,
    precio_unitario REAL NOT NULL DEFAULT 0,
    descuento REAL NOT NULL DEFAULT 0,
    importe REAL NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS categorias_gastos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL UNIQUE,
    descripcion TEXT
);

CREATE TABLE IF NOT EXISTS gastos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    numero TEXT,
    categoria_id INTEGER NOT NULL REFERENCES categorias_gastos(id),
    descripcion TEXT NOT NULL,
    monto REAL NOT NULL DEFAULT 0,
    subtotal REAL NOT NULL DEFAULT 0,
    impuesto REAL NOT NULL DEFAULT 0,
    total REAL NOT NULL DEFAULT 0,
    proveedor_id INTEGER REFERENCES proveedores(id),
    metodo_pago TEXT NOT NULL DEFAULT 'efectivo',
    referencia TEXT,
    comprobante TEXT,
    estado TEXT NOT NULL DEFAULT 'pendiente' CHECK(estado IN ('pendiente','pagado','cancelado')),
    notas TEXT,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')),
    fecha_vencimiento TEXT,
    fecha_pago TEXT
);

CREATE TABLE IF NOT EXISTS pagos_recibidos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    venta_id INTEGER REFERENCES ventas(id),
    cliente_id INTEGER REFERENCES clientes(id),
    monto REAL NOT NULL,
    metodo_pago TEXT,
    referencia TEXT,
    notas TEXT,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS pagos_realizados (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gasto_id INTEGER REFERENCES gastos(id),
    proveedor_id INTEGER REFERENCES proveedores(id),
    monto REAL NOT NULL,
    metodo_pago TEXT,
    referencia TEXT,
    notas TEXT,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS asientos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    numero TEXT,
    fecha TEXT NOT NULL DEFAULT (date('now')),
    concepto TEXT NOT NULL,
    descripcion TEXT,
    referencia TEXT,
    tipo TEXT NOT NULL DEFAULT 'manual',
    total_debe REAL NOT NULL DEFAULT 0,
    total_haber REAL NOT NULL DEFAULT 0,
    estado TEXT NOT NULL DEFAULT 'registrado' CHECK(estado IN ('registrado','cancelado'))
);

CREATE TABLE IF NOT EXISTS asiento_lineas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asiento_id INTEGER NOT NULL REFERENCES asientos(id),
    cuenta_id INTEGER NOT NULL REFERENCES plan_cuentas(id),
    descripcion TEXT,
    debe REAL NOT NULL DEFAULT 0,
    haber REAL NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS ubicaciones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    encargado TEXT,
    cedula TEXT,
    telefono TEXT,
    ciudad TEXT,
    direccion TEXT,
    activo INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS maquinas_ubicadas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ubicacion_texto TEXT,
    codigo TEXT,
    descripcion TEXT,
    modelo TEXT,
    numero_serie TEXT,
    color TEXT,
    fecha_ingreso TEXT,
    fecha_instalacion TEXT NOT NULL DEFAULT (date('now','localtime')),
    comision REAL NOT NULL DEFAULT 0,
    comision_estimada REAL NOT NULL DEFAULT 0,
    dia_cobro INTEGER NOT NULL DEFAULT 1,
    activo INTEGER NOT NULL DEFAULT 1,
    notas TEXT
);

CREATE TABLE IF NOT EXISTS cobros_comisiones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    maquina_id INTEGER NOT NULL REFERENCES maquinas_ubicadas(id),
    monto REAL NOT NULL,
    mes TEXT,
    periodo TEXT,
    observacion TEXT,
    notas TEXT,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS cuentas_credito (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL DEFAULT '',
    tipo TEXT NOT NULL DEFAULT 'cliente' CHECK(tipo IN ('cliente','proveedor')),
    cliente_id INTEGER REFERENCES clientes(id),
    proveedor_id INTEGER REFERENCES proveedores(id),
    limite REAL NOT NULL DEFAULT 0,
    saldo_actual REAL NOT NULL DEFAULT 0,
    notas TEXT,
    activo INTEGER NOT NULL DEFAULT 1,
    fecha_apertura TEXT NOT NULL DEFAULT (date('now'))
);

CREATE TABLE IF NOT EXISTS credito_movimientos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cuenta_id INTEGER NOT NULL REFERENCES cuentas_credito(id),
    tipo TEXT NOT NULL CHECK(tipo IN ('cargo','abono','pedido','pago')),
    monto REAL NOT NULL,
    cantidad REAL NOT NULL DEFAULT 0,
    precio_unit REAL NOT NULL DEFAULT 0,
    saldo_acumulado REAL NOT NULL DEFAULT 0,
    colores TEXT,
    descripcion TEXT,
    referencia_id INTEGER,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS ahorros (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cliente_id INTEGER REFERENCES clientes(id),
    saldo REAL NOT NULL DEFAULT 0,
    activo INTEGER NOT NULL DEFAULT 1,
    fecha_apertura TEXT NOT NULL DEFAULT (date('now'))
);

CREATE TABLE IF NOT EXISTS ahorro_movimientos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ahorro_id INTEGER REFERENCES ahorros(id),
    tipo TEXT NOT NULL CHECK(tipo IN ('deposito','retiro','comision')),
    monto REAL NOT NULL,
    saldo_acumulado REAL NOT NULL DEFAULT 0,
    cobro_id INTEGER REFERENCES cobros_comisiones(id),
    descripcion TEXT,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS garantias (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    producto_id INTEGER REFERENCES productos(id),
    venta_id INTEGER REFERENCES ventas(id),
    producto TEXT NOT NULL DEFAULT '',
    numero_serie TEXT,
    cliente_nombre TEXT NOT NULL,
    cedula TEXT,
    telefono TEXT,
    direccion TEXT,
    ciudad TEXT,
    monto_pago REAL NOT NULL DEFAULT 0,
    estado TEXT NOT NULL DEFAULT 'vigente' CHECK(estado IN ('vigente','vencida','cancelada')),
    observacion TEXT,
    descripcion TEXT,
    fecha_inicio TEXT NOT NULL,
    fecha_fin TEXT NOT NULL,
    activa INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS deudas_empresa (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    numero TEXT NOT NULL,
    proveedor_id INTEGER REFERENCES proveedores(id),
    proveedor_nombre TEXT NOT NULL DEFAULT '',
    concepto TEXT NOT NULL,
    descripcion TEXT,
    categoria_id INTEGER REFERENCES categorias_gastos(id),
    categoria_nombre TEXT NOT NULL DEFAULT '',
    fecha_deuda TEXT NOT NULL DEFAULT (date('now')),
    fecha_vencimiento TEXT,
    monto_total REAL NOT NULL DEFAULT 0,
    saldo_pendiente REAL NOT NULL DEFAULT 0,
    referencia TEXT,
    notas TEXT,
    estado TEXT NOT NULL DEFAULT 'pendiente' CHECK(estado IN ('pendiente','pagada','cancelada')),
    activa INTEGER NOT NULL DEFAULT 1,
    fecha_registro TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS deuda_pagos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    deuda_id INTEGER NOT NULL REFERENCES deudas_empresa(id),
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')),
    monto REAL NOT NULL,
    metodo_pago TEXT,
    referencia TEXT,
    notas TEXT
);

CREATE TABLE IF NOT EXISTS configuracion (
    clave TEXT PRIMARY KEY,
    valor TEXT
);

CREATE TABLE IF NOT EXISTS compras (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    numero TEXT NOT NULL,
    proveedor_id INTEGER REFERENCES proveedores(id),
    proveedor_nombre TEXT NOT NULL DEFAULT '',
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')),
    subtotal REAL NOT NULL DEFAULT 0,
    impuesto REAL NOT NULL DEFAULT 0,
    descuento REAL NOT NULL DEFAULT 0,
    total REAL NOT NULL DEFAULT 0,
    metodo_pago TEXT,
    referencia TEXT,
    notas TEXT,
    estado TEXT NOT NULL DEFAULT 'completada'
);

CREATE TABLE IF NOT EXISTS compra_detalles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    compra_id INTEGER NOT NULL REFERENCES compras(id),
    producto_id INTEGER REFERENCES productos(id),
    producto_nombre TEXT NOT NULL,
    cantidad INTEGER NOT NULL DEFAULT 1,
    precio_unitario REAL NOT NULL DEFAULT 0,
    descuento REAL NOT NULL DEFAULT 0,
    importe REAL NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS movimientos_inventario (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    producto_id INTEGER REFERENCES productos(id),
    producto_nombre TEXT NOT NULL DEFAULT '',
    tipo TEXT NOT NULL CHECK(tipo IN ('entrada','salida','ajuste')),
    cantidad INTEGER NOT NULL,
    motivo TEXT,
    referencia TEXT,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS cotizaciones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    numero TEXT NOT NULL,
    cliente_id INTEGER REFERENCES clientes(id),
    cliente_nombre TEXT NOT NULL DEFAULT '',
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')),
    validez_dias INTEGER NOT NULL DEFAULT 7,
    subtotal REAL NOT NULL DEFAULT 0,
    impuesto REAL NOT NULL DEFAULT 0,
    descuento REAL NOT NULL DEFAULT 0,
    total REAL NOT NULL DEFAULT 0,
    estado TEXT NOT NULL DEFAULT 'vigente' CHECK(estado IN ('vigente','convertida','vencida','cancelada')),
    notas TEXT
);

CREATE TABLE IF NOT EXISTS cotizacion_detalles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cotizacion_id INTEGER NOT NULL REFERENCES cotizaciones(id),
    producto_id INTEGER REFERENCES productos(id),
    producto_nombre TEXT NOT NULL,
    cantidad INTEGER NOT NULL DEFAULT 1,
    precio_unitario REAL NOT NULL DEFAULT 0,
    descuento REAL NOT NULL DEFAULT 0,
    importe REAL NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS retenciones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    numero TEXT NOT NULL,
    proveedor_id INTEGER REFERENCES proveedores(id),
    proveedor_nombre TEXT NOT NULL DEFAULT '',
    cedula TEXT NOT NULL DEFAULT '',
    fecha TEXT NOT NULL,
    base_imp_renta REAL NOT NULL DEFAULT 0,
    porcentaje_renta REAL NOT NULL DEFAULT 0,
    valor_renta REAL NOT NULL DEFAULT 0,
    base_imp_iva REAL NOT NULL DEFAULT 0,
    porcentaje_iva REAL NOT NULL DEFAULT 0,
    valor_iva REAL NOT NULL DEFAULT 0,
    tipo_comprobante TEXT NOT NULL DEFAULT 'factura',
    numero_comprobante TEXT NOT NULL DEFAULT '',
    referencia TEXT NOT NULL DEFAULT '',
    estado TEXT NOT NULL DEFAULT 'emitida',
    creado_en TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS empleados (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cedula TEXT NOT NULL DEFAULT '',
    nombre TEXT NOT NULL,
    cargo TEXT NOT NULL DEFAULT '',
    telefono TEXT NOT NULL DEFAULT '',
    sueldo_base REAL NOT NULL DEFAULT 0,
    fecha_ingreso TEXT NOT NULL DEFAULT (date('now')),
    activo INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS roles_pago (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    empleado_id INTEGER NOT NULL REFERENCES empleados(id),
    periodo TEXT NOT NULL,
    dias INTEGER NOT NULL DEFAULT 30,
    sueldo_bruto REAL NOT NULL DEFAULT 0,
    horas_extra REAL NOT NULL DEFAULT 0,
    comisiones REAL NOT NULL DEFAULT 0,
    total_ingresos REAL NOT NULL DEFAULT 0,
    iess REAL NOT NULL DEFAULT 0,
    prestamos REAL NOT NULL DEFAULT 0,
    otras_retenciones REAL NOT NULL DEFAULT 0,
    total_egresos REAL NOT NULL DEFAULT 0,
    total_neto REAL NOT NULL DEFAULT 0,
    estado TEXT NOT NULL DEFAULT 'generado',
    notas TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS activos_fijos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    descripcion TEXT NOT NULL,
    categoria TEXT NOT NULL DEFAULT 'equipo',
    fecha_adquisicion TEXT NOT NULL DEFAULT (date('now')),
    valor_adquisicion REAL NOT NULL DEFAULT 0,
    valor_residual REAL NOT NULL DEFAULT 0,
    vida_util_anios REAL NOT NULL DEFAULT 5,
    depreciacion_mensual REAL NOT NULL DEFAULT 0,
    depreciacion_acumulada REAL NOT NULL DEFAULT 0,
    activo INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS depreciaciones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activo_id INTEGER NOT NULL REFERENCES activos_fijos(id),
    periodo TEXT NOT NULL,
    monto REAL NOT NULL DEFAULT 0,
    acumulado REAL NOT NULL DEFAULT 0,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS cierres_contables (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    anio INTEGER NOT NULL,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')),
    ingresos REAL NOT NULL DEFAULT 0,
    gastos REAL NOT NULL DEFAULT 0,
    utilidad REAL NOT NULL DEFAULT 0,
    estado TEXT NOT NULL DEFAULT 'cerrado',
    notas TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS cuentas_bancarias (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    banco TEXT NOT NULL DEFAULT '',
    numero_cuenta TEXT NOT NULL DEFAULT '',
    tipo TEXT NOT NULL DEFAULT 'ahorros',
    saldo_inicial REAL NOT NULL DEFAULT 0,
    activo INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS movimientos_bancarios (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cuenta_id INTEGER NOT NULL REFERENCES cuentas_bancarias(id),
    fecha TEXT NOT NULL,
    descripcion TEXT NOT NULL DEFAULT '',
    tipo TEXT NOT NULL CHECK(tipo IN ('ingreso','egreso')),
    monto REAL NOT NULL DEFAULT 0,
    conciliado INTEGER NOT NULL DEFAULT 0,
    referencia TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS arqueos_caja (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fecha TEXT NOT NULL,
    responsable TEXT NOT NULL DEFAULT '',
    monto_esperado REAL NOT NULL DEFAULT 0,
    monto_real REAL NOT NULL DEFAULT 0,
    diferencia REAL NOT NULL DEFAULT 0,
    observacion TEXT NOT NULL DEFAULT '',
    creado_en TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

CREATE TABLE IF NOT EXISTS qr_tokens (
    token TEXT PRIMARY KEY,
    usuario TEXT NOT NULL,
    expira REAL NOT NULL,
    usado INTEGER NOT NULL DEFAULT 0,
    creado_en TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);
";

pub const SEED_DATA: &str = "
INSERT OR IGNORE INTO plan_cuentas (codigo, nombre, tipo, naturaleza, nivel) VALUES
('1000', 'ACTIVO', 'activo', 'deudora', 1),
('1100', 'Caja y Bancos', 'activo', 'deudora', 2),
('1110', 'Caja General', 'activo', 'deudora', 3),
('1120', 'Banco', 'activo', 'deudora', 3),
('1200', 'Cuentas por Cobrar', 'activo', 'deudora', 2),
('1210', 'Clientes', 'activo', 'deudora', 3),
('1300', 'Inventario', 'activo', 'deudora', 2),
('1310', 'Mercancías', 'activo', 'deudora', 3),
('1400', 'Activo Fijo', 'activo', 'deudora', 2),
('1410', 'Maquinaria y Equipo', 'activo', 'deudora', 3),
('1420', 'Depreciación Acumulada', 'activo', 'deudora', 3),
('2000', 'PASIVO', 'pasivo', 'acreedora', 1),
('2100', 'Cuentas por Pagar', 'pasivo', 'acreedora', 2),
('2110', 'Proveedores', 'pasivo', 'acreedora', 3),
('2200', 'IVA por Pagar', 'pasivo', 'acreedora', 3),
('2300', 'Deudas Bancarias', 'pasivo', 'acreedora', 3),
('3000', 'CAPITAL', 'capital', 'acreedora', 1),
('3100', 'Capital Social', 'capital', 'acreedora', 2),
('3200', 'Utilidades Retenidas', 'capital', 'acreedora', 2),
('3300', 'Utilidad del Ejercicio', 'capital', 'acreedora', 3),
('4000', 'INGRESOS', 'ingreso', 'acreedora', 1),
('4100', 'Ventas', 'ingreso', 'acreedora', 2),
('4110', 'Ventas de Maquinaria', 'ingreso', 'acreedora', 3),
('4120', 'Ventas de Productos', 'ingreso', 'acreedora', 3),
('4200', 'Otros Ingresos', 'ingreso', 'acreedora', 2),
('5000', 'GASTOS', 'gasto', 'deudora', 1),
('5100', 'Costo de Ventas', 'gasto', 'deudora', 2),
('5200', 'Gastos de Operación', 'gasto', 'deudora', 2),
('5210', 'Sueldos y Salarios', 'gasto', 'deudora', 3),
('5220', 'Renta', 'gasto', 'deudora', 3),
('5230', 'Servicios (agua, luz, internet)', 'gasto', 'deudora', 3),
('5240', 'Mantenimiento y Reparaciones', 'gasto', 'deudora', 3),
('5250', 'Combustible y Transporte', 'gasto', 'deudora', 3),
('5260', 'Publicidad y Marketing', 'gasto', 'deudora', 3),
('5270', 'Materiales y Suministros', 'gasto', 'deudora', 3),
('5300', 'Gastos Financieros', 'gasto', 'deudora', 3),
('5400', 'Impuestos', 'gasto', 'deudora', 3);

INSERT OR IGNORE INTO categorias_productos (nombre, descripcion) VALUES
('Electrónicos', 'Componentes y dispositivos electrónicos'),
('Máquinas', 'Máquinas completas y refacciones'),
('Accesorios', 'Accesorios y consumibles'),
('Servicios', 'Mano de obra y servicios técnicos');

INSERT OR IGNORE INTO categorias_gastos (nombre, descripcion) VALUES
('Renta', 'Pago de renta de local'),
('Sueldos', 'Pago de nómina y sueldos'),
('Servicios', 'Luz, agua, internet, teléfono'),
('Insumos', 'Compra de materiales y refacciones'),
('Transporte', 'Gasolina, fletes, transporte'),
('Comida', 'Alimentación del personal'),
('Marketing', 'Publicidad y promociones'),
('Otros', 'Gastos varios no categorizados');
";
