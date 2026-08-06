pub mod migrar;
pub mod schema;

use rusqlite::{Connection, Result, params};
use std::sync::Mutex;

use crate::models::*;

pub struct DatabaseManager {
    conn: Mutex<Connection>,
}

impl DatabaseManager {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Self { conn: Mutex::new(conn) };
        db.initialize()?;
        Ok(db)
    }

    pub fn initialize(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(schema::CREATE_TABLES)?;
        let _ = conn.execute_batch("ALTER TABLE plan_cuentas ADD COLUMN naturaleza TEXT NOT NULL DEFAULT 'deudora' CHECK(naturaleza IN ('deudora','acreedora'))");
        let maq_cols: Vec<String> = conn.prepare("PRAGMA table_info(maquinas_ubicadas)")?
            .query_map([], |r| r.get(1))?
            .collect::<Result<Vec<_>>>()?;
        if !maq_cols.iter().any(|c| c == "ubicacion_texto") {
            conn.execute_batch(
                "ALTER TABLE maquinas_ubicadas ADD COLUMN ubicacion_texto TEXT;
                 UPDATE maquinas_ubicadas SET ubicacion_texto = COALESCE((SELECT nombre FROM ubicaciones WHERE id = maquinas_ubicadas.ubicacion_id), '');"
            )?;
        }
        if maq_cols.iter().any(|c| c == "ubicacion_id") {
            conn.execute_batch("ALTER TABLE maquinas_ubicadas DROP COLUMN ubicacion_id")?;
        }
        conn.execute_batch("UPDATE maquinas_ubicadas SET fecha_instalacion = substr(fecha_instalacion, 1, 10) WHERE length(fecha_instalacion) > 10")?;

        // Migraciones para bases creadas con el esquema anterior.
        Self::migrar_esquema_anterior(&conn)?;

        let _ = conn.execute_batch("INSERT OR IGNORE INTO configuracion(clave, valor) VALUES ('iva','15');");
        conn.execute_batch(schema::SEED_DATA)?;
        Ok(())
    }

    fn migrar_esquema_anterior(conn: &rusqlite::Connection) -> Result<()> {
        let tiene = |tabla: &str, col: &str| -> bool {
            let Ok(mut st) = conn.prepare(&format!("PRAGMA table_info({})", tabla)) else {
                return false;
            };
            let Ok(rows) = st.query_map([], |r| r.get::<_, String>(1)) else {
                return false;
            };
            rows.flatten().any(|c| c == col)
        };
        let agregar = |tabla: &str, sql: &str| -> Result<()> {
            let col = sql.split_whitespace().next().unwrap_or("");
            if !tiene(tabla, col) {
                conn.execute_batch(&format!("ALTER TABLE {} ADD COLUMN {}", tabla, sql))?;
            }
            Ok(())
        };

        if !tiene("clientes", "saldo_pendiente") {
            conn.execute_batch("ALTER TABLE clientes ADD COLUMN saldo_pendiente REAL NOT NULL DEFAULT 0")?;
        }

        if !tiene("ventas", "impuesto") {
            conn.execute_batch("ALTER TABLE ventas ADD COLUMN impuesto REAL NOT NULL DEFAULT 0")?;
            let _ = conn.execute_batch("UPDATE ventas SET impuesto = COALESCE(iva, 0)");
        }
        agregar("ventas", "descuento REAL NOT NULL DEFAULT 0")?;
        agregar("ventas", "metodo_pago TEXT")?;
        agregar("ventas", "fecha_vencimiento TEXT")?;
        agregar("ventas", "fecha_pago TEXT")?;

        agregar("gastos", "numero TEXT")?;
        agregar("gastos", "subtotal REAL NOT NULL DEFAULT 0")?;
        agregar("gastos", "impuesto REAL NOT NULL DEFAULT 0")?;
        agregar("gastos", "total REAL NOT NULL DEFAULT 0")?;
        agregar("gastos", "comprobante TEXT")?;
        agregar("gastos", "estado TEXT NOT NULL DEFAULT 'pagado'")?;
        agregar("gastos", "fecha_vencimiento TEXT")?;
        agregar("gastos", "fecha_pago TEXT")?;
        let _ = conn.execute_batch("UPDATE gastos SET total = CASE WHEN total = 0 THEN monto ELSE total END, subtotal = CASE WHEN subtotal = 0 THEN monto ELSE subtotal END");

        if !tiene("pagos_recibidos", "metodo_pago") {
            conn.execute_batch("ALTER TABLE pagos_recibidos ADD COLUMN metodo_pago TEXT")?;
            let _ = conn.execute_batch("UPDATE pagos_recibidos SET metodo_pago = metodo");
        }
        agregar("pagos_recibidos", "cliente_id INTEGER REFERENCES clientes(id)")?;
        agregar("pagos_recibidos", "notas TEXT")?;

        agregar("maquinas_ubicadas", "color TEXT")?;
        agregar("maquinas_ubicadas", "fecha_ingreso TEXT")?;
        agregar("maquinas_ubicadas", "comision_estimada REAL NOT NULL DEFAULT 0")?;
        agregar("maquinas_ubicadas", "dia_cobro INTEGER NOT NULL DEFAULT 1")?;
        agregar("maquinas_ubicadas", "notas TEXT")?;
        let _ = conn.execute_batch("UPDATE maquinas_ubicadas SET comision_estimada = CASE WHEN comision_estimada = 0 THEN comision ELSE comision_estimada END");

        agregar("cobros_comisiones", "mes TEXT")?;
        agregar("cobros_comisiones", "observacion TEXT")?;
        let _ = conn.execute_batch("UPDATE cobros_comisiones SET mes = substr(periodo, 1, 7), observacion = notas WHERE mes IS NULL");
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Clientes
    // -----------------------------------------------------------------------
    pub fn listar_clientes(&self) -> Result<Vec<Cliente>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, codigo, nombre, COALESCE(rfc, ''), COALESCE(email, ''), COALESCE(telefono, ''),
                    COALESCE(direccion, ''), COALESCE(ciudad, ''), limite_credito, saldo_pendiente, activo, fecha_registro
             FROM clientes ORDER BY nombre"
        )?;
        let rows = stmt.query_map([], |r| Ok(Cliente {
            id: r.get(0)?, codigo: r.get(1)?, nombre: r.get(2)?, rfc: r.get(3)?,
            email: r.get(4)?, telefono: r.get(5)?, direccion: r.get(6)?, ciudad: r.get(7)?,
            limite_credito: r.get(8)?, saldo_pendiente: r.get(9)?, activo: r.get(10)?, fecha_registro: r.get(11)?,
        }))?;
        rows.collect()
    }

    pub fn crear_cliente(&self, c: &ClienteNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let codigo = match &c.codigo {
            Some(cod) if !cod.trim().is_empty() => cod.clone(),
            _ => format!("CLI-{}", chrono::Local::now().format("%Y%m%d%H%M%S")),
        };
        conn.execute(
            "INSERT INTO clientes (codigo, nombre, rfc, email, telefono, direccion, ciudad, limite_credito)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![codigo, c.nombre, c.rfc, c.email, c.telefono, c.direccion, c.ciudad, c.limite_credito]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_cliente(&self, id: i64, c: &ClienteNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE clientes SET codigo=COALESCE(?1, codigo), nombre=?2, rfc=?3, email=?4, telefono=?5, direccion=?6, ciudad=?7, limite_credito=?8 WHERE id=?9",
            params![c.codigo, c.nombre, c.rfc, c.email, c.telefono, c.direccion, c.ciudad, c.limite_credito, id]
        )?;
        Ok(())
    }

    pub fn eliminar_cliente(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE clientes SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Proveedores
    // -----------------------------------------------------------------------
    pub fn listar_proveedores(&self) -> Result<Vec<Proveedor>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, codigo, nombre, contacto, rfc, email, telefono, direccion, ciudad, saldo_pendiente, activo, fecha_registro
             FROM proveedores ORDER BY nombre"
        )?;
        let rows = stmt.query_map([], |r| Ok(Proveedor {
            id: r.get(0)?, codigo: r.get(1)?, nombre: r.get(2)?, contacto: r.get(3)?,
            rfc: r.get(4)?, email: r.get(5)?, telefono: r.get(6)?, direccion: r.get(7)?,
            ciudad: r.get(8)?, saldo_pendiente: r.get(9)?, activo: r.get(10)?, fecha_registro: r.get(11)?,
        }))?;
        rows.collect()
    }

    pub fn crear_proveedor(&self, p: &ProveedorNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO proveedores (codigo, nombre, contacto, rfc, email, telefono, direccion, ciudad)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![p.codigo, p.nombre, p.contacto, p.rfc, p.email, p.telefono, p.direccion, p.ciudad]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_proveedor(&self, id: i64, p: &ProveedorNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE proveedores SET codigo=?1, nombre=?2, contacto=?3, rfc=?4, email=?5, telefono=?6, direccion=?7, ciudad=?8 WHERE id=?9",
            params![p.codigo, p.nombre, p.contacto, p.rfc, p.email, p.telefono, p.direccion, p.ciudad, id]
        )?;
        Ok(())
    }

    pub fn eliminar_proveedor(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE proveedores SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Productos
    // -----------------------------------------------------------------------
    pub fn listar_productos(&self) -> Result<Vec<Producto>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT p.id, p.codigo, p.nombre, COALESCE(p.descripcion, ''), COALESCE(p.categoria_id, 0), p.precio_compra, p.precio_venta,
                    p.stock, p.stock_minimo, p.unidad, p.activo, p.fecha_registro
             FROM productos p ORDER BY p.nombre"
        )?;
        let rows = stmt.query_map([], |r| Ok(Producto {
            id: r.get(0)?, codigo: r.get(1)?, nombre: r.get(2)?, descripcion: r.get(3)?,
            categoria_id: r.get(4)?, precio_compra: r.get(5)?, precio_venta: r.get(6)?,
            stock: r.get(7)?, stock_minimo: r.get(8)?, unidad: r.get(9)?, activo: r.get(10)?,
            fecha_registro: r.get(11)?,
        }))?;
        rows.collect()
    }

    pub fn crear_producto(&self, p: &ProductoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO productos (codigo, nombre, descripcion, categoria_id, precio_compra, precio_venta, stock, stock_minimo, unidad)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![p.codigo, p.nombre, p.descripcion, p.categoria_id, p.precio_compra, p.precio_venta, p.stock, p.stock_minimo, p.unidad]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_producto(&self, id: i64, p: &ProductoNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE productos SET codigo=?1, nombre=?2, descripcion=?3, categoria_id=?4, precio_compra=?5, precio_venta=?6, stock=?7, stock_minimo=?8, unidad=?9 WHERE id=?10",
            params![p.codigo, p.nombre, p.descripcion, p.categoria_id, p.precio_compra, p.precio_venta, p.stock, p.stock_minimo, p.unidad, id]
        )?;
        Ok(())
    }

    pub fn eliminar_producto(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE productos SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn listar_categorias_productos(&self) -> Result<Vec<CategoriaProducto>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, nombre, descripcion FROM categorias_productos ORDER BY nombre")?;
        let rows = stmt.query_map([], |r| Ok(CategoriaProducto {
            id: r.get(0)?, nombre: r.get(1)?, descripcion: r.get(2)?,
        }))?;
        rows.collect()
    }

    // -----------------------------------------------------------------------
    // Configuracion
    // -----------------------------------------------------------------------
    pub fn obtener_configuracion(&self) -> Result<Configuracion> {
        let conn = self.conn.lock().unwrap();
        let mut cfg = Configuracion::default();
        let mut stmt = conn.prepare("SELECT clave, valor FROM configuracion")?;
        let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
        for row in rows {
            let (clave, valor) = row?;
            match clave.as_str() {
                "empresa_nombre" => cfg.empresa_nombre = valor,
                "ruc" => cfg.ruc = valor,
                "direccion" => cfg.direccion = valor,
                "telefono" => cfg.telefono = valor,
                "email" => cfg.email = valor,
                "ciudad" => cfg.ciudad = valor,
                "iva" => cfg.iva = valor.parse::<f64>().unwrap_or(cfg.iva),
                _ => {}
            }
        }
        Ok(cfg)
    }

    pub fn guardar_configuracion(&self, c: &Configuracion) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let pares: [(&str, String); 7] = [
            ("empresa_nombre", c.empresa_nombre.clone()),
            ("ruc", c.ruc.clone()),
            ("direccion", c.direccion.clone()),
            ("telefono", c.telefono.clone()),
            ("email", c.email.clone()),
            ("ciudad", c.ciudad.clone()),
            ("iva", c.iva.to_string()),
        ];
        for (clave, valor) in pares {
            conn.execute(
                "INSERT INTO configuracion (clave, valor) VALUES (?1, ?2)
                 ON CONFLICT(clave) DO UPDATE SET valor=excluded.valor",
                params![clave, valor]
            )?;
        }
        Ok(())
    }

    pub fn obtener_iva(&self) -> f64 {
        let conn = self.conn.lock().unwrap();
        Self::_iva_con(&conn)
    }

    fn _iva_con(conn: &rusqlite::Connection) -> f64 {
        conn.query_row("SELECT valor FROM configuracion WHERE clave='iva'", [], |r| r.get::<_, String>(0))
            .map(|v| v.parse::<f64>().unwrap_or(15.0))
            .unwrap_or(15.0)
    }

    // -----------------------------------------------------------------------
    // Usuarios
    // -----------------------------------------------------------------------
    pub fn hay_usuarios(&self) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let n: i64 = conn.query_row("SELECT COUNT(*) FROM usuarios", [], |r| r.get(0))?;
        Ok(n > 0)
    }

    pub fn crear_usuario(&self, nombre_usuario: &str, contrasena: &str) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO usuarios (nombre_usuario, contrasena_hash) VALUES (?1, ?2)",
            params![nombre_usuario.trim(), hash_password(contrasena)],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn verificar_usuario(&self, nombre_usuario: &str, contrasena: &str) -> Result<Option<Usuario>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, nombre_usuario, activo FROM usuarios WHERE nombre_usuario = ?1 AND contrasena_hash = ?2"
        )?;
        let mut rows = stmt.query_map(params![nombre_usuario.trim(), hash_password(contrasena)], |r| {
            Ok(Usuario {
                id: r.get(0)?, nombre_usuario: r.get(1)?, activo: r.get::<_, i64>(2)? != 0,
            })
        })?;
        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    // -----------------------------------------------------------------------
    // Tokens QR (acceso desde el celular con un solo escaneo)
    // -----------------------------------------------------------------------
    pub fn crear_qr_token(&self, nombre_usuario: &str, expira_segundos: i64) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let ahora = chrono::Utc::now().timestamp();
        let _ = conn.execute("DELETE FROM qr_tokens WHERE usado = 1 OR expira < ?1", params![ahora as f64]);
        let token = format!("{:016x}{:016x}", rand::random::<u64>(), rand::random::<u64>());
        conn.execute(
            "INSERT INTO qr_tokens (token, usuario, expira) VALUES (?1, ?2, ?3)",
            params![token, nombre_usuario.trim(), (ahora + expira_segundos) as f64],
        )?;
        Ok(token)
    }

    pub fn usar_qr_token(&self, token: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let ahora = chrono::Utc::now().timestamp() as f64;
        let mut stmt = conn.prepare(
            "SELECT usuario FROM qr_tokens WHERE token = ?1 AND usado = 0 AND expira > ?2"
        )?;
        let mut rows = stmt.query_map(params![token, ahora], |r| r.get::<_, String>(0))?;
        if let Some(row) = rows.next() {
            let usuario = row?;
            let _ = conn.execute("UPDATE qr_tokens SET usado = 1 WHERE token = ?1", params![token]);
            Ok(Some(usuario))
        } else {
            Ok(None)
        }
    }

    pub fn listar_usuarios(&self) -> Result<Vec<Usuario>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, nombre_usuario, activo FROM usuarios ORDER BY id")?;
        let rows = stmt.query_map([], |r| Ok(Usuario {
            id: r.get(0)?, nombre_usuario: r.get(1)?, activo: r.get::<_, i64>(2)? != 0,
        }))?;
        rows.collect()
    }

    // -----------------------------------------------------------------------
    // Ventas
    // -----------------------------------------------------------------------
    pub fn listar_ventas(&self) -> Result<Vec<Venta>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, folio, cliente_id, cliente_nombre, fecha, subtotal, impuesto, descuento, total,
                    saldo_pendiente, tipo, estado, metodo_pago, COALESCE(notas, ''), fecha_vencimiento, fecha_pago
             FROM ventas ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(Venta {
            id: r.get(0)?, folio: r.get(1)?, cliente_id: r.get(2)?, cliente_nombre: r.get(3)?,
            fecha: r.get(4)?, subtotal: r.get(5)?, impuesto: r.get(6)?, descuento: r.get(7)?,
            total: r.get(8)?, saldo_pendiente: r.get(9)?, tipo: r.get(10)?, estado: r.get(11)?,
            metodo_pago: r.get(12)?, notas: r.get(13)?, fecha_vencimiento: r.get(14)?, fecha_pago: r.get(15)?,
        }))?;
        rows.collect()
    }

    pub fn crear_venta(&self, v: &VentaNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        self._insertar_venta_con(&conn, v)
    }

    fn _insertar_venta_con(&self, conn: &rusqlite::Connection, v: &VentaNueva) -> Result<i64> {
        let folio = format!("V-{}", chrono::Local::now().format("%Y%m%d%H%M%S%3f"));
        let mut subtotal = 0.0;
        for d in &v.detalles {
            subtotal += d.cantidad as f64 * d.precio_unitario;
        }
        let descuento_total = v.descuento;
        let base_imponible = (subtotal - descuento_total).max(0.0);
        let iva = if v.iva > 0.0 { v.iva } else { Self::_iva_con(conn) };
        let impuesto = base_imponible * iva / 100.0;
        let total = base_imponible + impuesto;
        let saldo = if v.tipo == "credito" { total } else { 0.0 };

        conn.execute(
            "INSERT INTO ventas (folio, cliente_id, cliente_nombre, subtotal, impuesto, descuento, total, saldo_pendiente, tipo, estado, metodo_pago, notas)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'completada', ?10, ?11)",
            params![folio, v.cliente_id, v.cliente_nombre, subtotal, impuesto, descuento_total, total, saldo, v.tipo, v.metodo_pago, v.notas]
        )?;
        let venta_id = conn.last_insert_rowid();

        for d in &v.detalles {
            let importe = (d.cantidad as f64 * d.precio_unitario) - d.descuento;
            conn.execute(
                "INSERT INTO ventas_detalles (venta_id, producto_id, producto_nombre, cantidad, precio_unitario, descuento, importe)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![venta_id, d.producto_id, d.producto_nombre, d.cantidad, d.precio_unitario, d.descuento, importe]
            )?;
            self._aplicar_stock(conn, d.producto_id, -d.cantidad as i32, "salida", "Venta", &folio)?;
        }
        Ok(venta_id)
    }

    pub fn obtener_detalles_venta(&self, venta_id: i64) -> Result<Vec<VentaDetalle>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, venta_id, producto_id, descripcion, producto_nombre, cantidad, precio_unitario, descuento, importe
             FROM ventas_detalles WHERE venta_id=?1"
        )?;
        let rows = stmt.query_map(params![venta_id], |r| Ok(VentaDetalle {
            id: r.get(0)?, venta_id: r.get(1)?, producto_id: r.get(2)?,
            descripcion: r.get(3)?, producto_nombre: r.get(4)?, cantidad: r.get(5)?,
            precio_unitario: r.get(6)?, descuento: r.get(7)?, importe: r.get(8)?,
        }))?;
        rows.collect()
    }

    // -----------------------------------------------------------------------
    // Gastos
    // -----------------------------------------------------------------------
    pub fn listar_gastos(&self) -> Result<Vec<Gasto>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT g.id, g.numero, g.categoria_id, c.nombre, g.descripcion, g.monto, g.subtotal, g.impuesto, g.total,
                    g.proveedor_id, COALESCE(p.nombre, ''), g.fecha, COALESCE(g.metodo_pago, ''), COALESCE(g.referencia, ''),
                    g.comprobante, g.estado, COALESCE(g.notas, ''), g.fecha_vencimiento, g.fecha_pago
             FROM gastos g
             LEFT JOIN categorias_gastos c ON c.id = g.categoria_id
             LEFT JOIN proveedores p ON p.id = g.proveedor_id
             ORDER BY g.fecha DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(Gasto {
            id: r.get(0)?, numero: r.get(1)?, categoria_id: r.get(2)?, categoria_nombre: r.get(3)?,
            descripcion: r.get(4)?, monto: r.get(5)?, subtotal: r.get(6)?, impuesto: r.get(7)?,
            total: r.get(8)?, proveedor_id: r.get(9)?, proveedor_nombre: r.get(10)?,
            fecha: r.get(11)?, metodo_pago: r.get(12)?, referencia: r.get(13)?,
            comprobante: r.get(14)?, estado: r.get(15)?, notas: r.get(16)?,
            fecha_vencimiento: r.get(17)?, fecha_pago: r.get(18)?,
        }))?;
        rows.collect()
    }

    pub fn crear_gasto(&self, g: &GastoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let total = g.monto + g.impuesto;
        let estado = if g.fecha_vencimiento.is_some() { "pendiente" } else { "pagado" };
        conn.execute(
            "INSERT INTO gastos (numero, categoria_id, descripcion, monto, subtotal, impuesto, total, proveedor_id, metodo_pago, referencia, comprobante, estado, notas, fecha_vencimiento)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![g.numero, g.categoria_id, g.descripcion, g.monto, g.subtotal, g.impuesto, total,
                    g.proveedor_id, g.metodo_pago, g.referencia, g.comprobante, estado, g.notas, g.fecha_vencimiento]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn listar_categorias_gastos(&self) -> Result<Vec<CategoriaGasto>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, nombre, descripcion FROM categorias_gastos ORDER BY nombre")?;
        let rows = stmt.query_map([], |r| Ok(CategoriaGasto {
            id: r.get(0)?, nombre: r.get(1)?, descripcion: r.get(2)?,
        }))?;
        rows.collect()
    }

    // -----------------------------------------------------------------------
    // Pagos Recibidos
    // -----------------------------------------------------------------------
    pub fn listar_pagos_recibidos(&self) -> Result<Vec<PagoRecibido>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, venta_id, cliente_id, monto, metodo_pago, COALESCE(referencia, ''), COALESCE(notas, ''), fecha
             FROM pagos_recibidos ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(PagoRecibido {
            id: r.get(0)?, venta_id: r.get(1)?, cliente_id: r.get(2)?,
            monto: r.get(3)?, metodo_pago: r.get(4)?, referencia: r.get(5)?,
            notas: r.get(6)?, fecha: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn crear_pago_recibido(&self, p: &PagoRecibidoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO pagos_recibidos (venta_id, cliente_id, monto, metodo_pago, referencia, notas)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![p.venta_id, p.cliente_id, p.monto, p.metodo_pago, p.referencia, p.notas]
        )?;
        let id = conn.last_insert_rowid();
        if let Some(venta_id) = p.venta_id {
            conn.execute(
                "UPDATE ventas SET saldo_pendiente = MAX(0, saldo_pendiente - ?1) WHERE id=?2",
                params![p.monto, venta_id]
            )?;
        }
        Ok(id)
    }

    // -----------------------------------------------------------------------
    // Pagos Realizados
    // -----------------------------------------------------------------------
    pub fn listar_pagos_realizados(&self) -> Result<Vec<PagoRealizado>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, gasto_id, proveedor_id, monto, metodo_pago, referencia, notas, fecha
             FROM pagos_realizados ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(PagoRealizado {
            id: r.get(0)?, gasto_id: r.get(1)?, proveedor_id: r.get(2)?,
            monto: r.get(3)?, metodo_pago: r.get(4)?, referencia: r.get(5)?,
            notas: r.get(6)?, fecha: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn crear_pago_realizado(&self, p: &PagoRealizadoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO pagos_realizados (gasto_id, proveedor_id, monto, metodo_pago, referencia, notas)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![p.gasto_id, p.proveedor_id, p.monto, p.metodo_pago, p.referencia, p.notas]
        )?;
        let id = conn.last_insert_rowid();
        if let Some(gasto_id) = p.gasto_id {
            let cubierto: f64 = conn.query_row(
                "SELECT COALESCE(SUM(monto), 0) FROM pagos_realizados WHERE gasto_id=?1", params![gasto_id], |r| r.get(0)
            ).unwrap_or(0.0);
            let total_gasto: f64 = conn.query_row(
                "SELECT total FROM gastos WHERE id=?1", params![gasto_id], |r| r.get(0)
            ).unwrap_or(0.0);
            if cubierto >= total_gasto {
                conn.execute(
                    "UPDATE gastos SET estado='pagado', fecha_pago=datetime('now','localtime') WHERE id=?1 AND estado='pendiente'",
                    params![gasto_id]
                )?;
            }
        }
        Ok(id)
    }

    // -----------------------------------------------------------------------
    // Plan de Cuentas
    // -----------------------------------------------------------------------
    pub fn listar_plan_cuentas(&self) -> Result<Vec<PlanCuentas>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, codigo, nombre, tipo, naturaleza, nivel, padre_id, activo FROM plan_cuentas ORDER BY codigo"
        )?;
        let rows = stmt.query_map([], |r| Ok(PlanCuentas {
            id: r.get(0)?, codigo: r.get(1)?, nombre: r.get(2)?, tipo: r.get(3)?,
            naturaleza: r.get(4)?, nivel: r.get(5)?, padre_id: r.get(6)?, activo: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn crear_cuenta(&self, c: &PlanCuentas) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO plan_cuentas (codigo, nombre, tipo, naturaleza, nivel, padre_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![c.codigo, c.nombre, c.tipo, c.naturaleza, c.nivel, c.padre_id]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_cuenta(&self, id: i64, c: &PlanCuentas) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE plan_cuentas SET codigo=?1, nombre=?2, tipo=?3, naturaleza=?4, nivel=?5, padre_id=?6, activo=?7 WHERE id=?8",
            params![c.codigo, c.nombre, c.tipo, c.naturaleza, c.nivel, c.padre_id, c.activo, id]
        )?;
        Ok(())
    }

    pub fn eliminar_cuenta(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE plan_cuentas SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Asientos Contables
    // -----------------------------------------------------------------------
    pub fn listar_asientos(&self) -> Result<Vec<Asiento>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, numero, fecha, concepto, descripcion, referencia, tipo, total_debe, total_haber, estado
             FROM asientos ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(Asiento {
            id: r.get(0)?, numero: r.get(1)?, fecha: r.get(2)?, concepto: r.get(3)?,
            descripcion: r.get(4)?, referencia: r.get(5)?, tipo: r.get(6)?,
            total_debe: r.get(7)?, total_haber: r.get(8)?, estado: r.get(9)?,
        }))?;
        rows.collect()
    }

    pub fn crear_asiento(&self, a: &AsientoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let mut total_debe = 0.0;
        let mut total_haber = 0.0;
        for l in &a.lineas {
            total_debe += l.debe;
            total_haber += l.haber;
        }
        conn.execute(
            "INSERT INTO asientos (numero, fecha, concepto, descripcion, referencia, tipo, total_debe, total_haber, estado)
             VALUES (?1, ?2, ?3, ?4, ?5, 'manual', ?6, ?7, 'registrado')",
            params![a.numero, a.fecha, a.concepto, a.descripcion, a.referencia, total_debe, total_haber]
        )?;
        let asiento_id = conn.last_insert_rowid();
        for l in &a.lineas {
            conn.execute(
                "INSERT INTO asiento_lineas (asiento_id, cuenta_id, descripcion, debe, haber) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![asiento_id, l.cuenta_id, l.descripcion, l.debe, l.haber]
            )?;
        }
        Ok(asiento_id)
    }

    pub fn obtener_lineas_asiento(&self, asiento_id: i64) -> Result<Vec<AsientoLinea>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT al.id, al.asiento_id, al.cuenta_id, pc.codigo, pc.nombre, al.descripcion, al.debe, al.haber
             FROM asiento_lineas al JOIN plan_cuentas pc ON pc.id = al.cuenta_id WHERE al.asiento_id=?1"
        )?;
        let rows = stmt.query_map(params![asiento_id], |r| Ok(AsientoLinea {
            id: r.get(0)?, asiento_id: r.get(1)?, cuenta_id: r.get(2)?,
            cuenta_codigo: r.get(3)?, cuenta_nombre: r.get(4)?, descripcion: r.get(5)?,
            debe: r.get(6)?, haber: r.get(7)?,
        }))?;
        rows.collect()
    }

    // -----------------------------------------------------------------------
    // Dashboard - KPIs
    // -----------------------------------------------------------------------
    pub fn kpi_ventas_hoy(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(SUM(total), 0) FROM ventas WHERE date(fecha) = date('now','localtime') AND estado='completada'",
            [], |r| r.get(0)
        )
    }

    pub fn kpi_gastos_hoy(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(SUM(COALESCE(total, monto)), 0) FROM gastos WHERE date(fecha) = date('now','localtime') AND estado='pagado'",
            [], |r| r.get(0)
        )
    }

    pub fn kpi_cxc(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(SUM(saldo_pendiente), 0) FROM ventas WHERE tipo='credito' AND estado='completada'",
            [], |r| r.get(0)
        )
    }

    pub fn kpi_cxp(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(SUM(COALESCE(total, monto)), 0) FROM gastos WHERE estado='pendiente'",
            [], |r| r.get(0)
        )
    }

    pub fn kpi_utilidad_mes(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        let ventas = conn.query_row::<f64, _, _>(
            "SELECT COALESCE(SUM(total), 0) FROM ventas WHERE strftime('%Y-%m', fecha) = strftime('%Y-%m', 'now','localtime') AND estado='completada'",
            [], |r| r.get(0)
        )?;
        let gastos = conn.query_row::<f64, _, _>(
            "SELECT COALESCE(SUM(COALESCE(total, monto)), 0) FROM gastos WHERE strftime('%Y-%m', fecha) = strftime('%Y-%m', 'now','localtime') AND estado='pagado'",
            [], |r| r.get(0)
        )?;
        Ok(ventas - gastos)
    }

    pub fn kpi_clientes(&self) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.query_row("SELECT COUNT(*) FROM clientes WHERE activo=1", [], |r| r.get(0))
    }

    pub fn kpi_ventas_mes(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(SUM(total), 0) FROM ventas WHERE strftime('%Y-%m', fecha) = strftime('%Y-%m', 'now','localtime') AND estado='completada'",
            [], |r| r.get(0)
        )
    }

    pub fn kpi_gastos_mes(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(SUM(COALESCE(total, monto)), 0) FROM gastos WHERE strftime('%Y-%m', fecha) = strftime('%Y-%m', 'now','localtime') AND estado='pagado'",
            [], |r| r.get(0)
        )
    }

    pub fn kpi_ventas_anio(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(SUM(total), 0) FROM ventas WHERE strftime('%Y', fecha) = strftime('%Y', 'now','localtime') AND estado='completada'",
            [], |r| r.get(0)
        )
    }

    pub fn kpi_gastos_anio(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(SUM(COALESCE(total, monto)), 0) FROM gastos WHERE strftime('%Y', fecha) = strftime('%Y', 'now','localtime') AND estado='pagado'",
            [], |r| r.get(0)
        )
    }

    pub fn kpi_utilidad_anio(&self) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        let ventas = conn.query_row::<f64, _, _>(
            "SELECT COALESCE(SUM(total), 0) FROM ventas WHERE strftime('%Y', fecha) = strftime('%Y','now','localtime') AND estado='completada'",
            [], |r| r.get(0)
        )?;
        let gastos = conn.query_row::<f64, _, _>(
            "SELECT COALESCE(SUM(COALESCE(total, monto)), 0) FROM gastos WHERE strftime('%Y', fecha) = strftime('%Y','now','localtime') AND estado='pagado'",
            [], |r| r.get(0)
        )?;
        Ok(ventas - gastos)
    }

    // -----------------------------------------------------------------------
    // Dashboard - Charts & Activity
    // -----------------------------------------------------------------------
    pub fn ventas_por_mes(&self) -> Result<Vec<(String, f64)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT strftime('%Y-%m', fecha) as mes, COALESCE(SUM(total), 0)
             FROM ventas WHERE estado='completada'
             GROUP BY mes ORDER BY mes DESC LIMIT 12"
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
        rows.collect()
    }

    pub fn gastos_por_categoria(&self) -> Result<Vec<(String, f64)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT c.nombre, COALESCE(SUM(COALESCE(g.total, g.monto)), 0)
             FROM gastos g JOIN categorias_gastos c ON c.id = g.categoria_id
             WHERE strftime('%Y-%m', g.fecha) = strftime('%Y-%m', 'now','localtime')
             GROUP BY c.nombre ORDER BY SUM(COALESCE(g.total, g.monto)) DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
        rows.collect()
    }

    pub fn alertas_stock(&self) -> Result<Vec<Producto>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT p.id, p.codigo, p.nombre, p.descripcion, p.categoria_id, p.precio_compra, p.precio_venta,
                    p.stock, p.stock_minimo, p.unidad, p.activo, p.fecha_registro
             FROM productos p WHERE p.activo=1 AND p.stock <= p.stock_minimo ORDER BY p.stock ASC"
        )?;
        let rows = stmt.query_map([], |r| Ok(Producto {
            id: r.get(0)?, codigo: r.get(1)?, nombre: r.get(2)?, descripcion: r.get(3)?,
            categoria_id: r.get(4)?, precio_compra: r.get(5)?, precio_venta: r.get(6)?,
            stock: r.get(7)?, stock_minimo: r.get(8)?, unidad: r.get(9)?, activo: r.get(10)?,
            fecha_registro: r.get(11)?,
        }))?;
        rows.collect()
    }

    pub fn alertas_creditos_vencidos(&self) -> Result<Vec<(Cliente, f64)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT c.id, c.codigo, c.nombre, c.rfc, c.email, c.telefono, c.direccion, c.ciudad,
                    c.limite_credito, c.saldo_pendiente, c.activo, c.fecha_registro,
                    COALESCE(SUM(v.saldo_pendiente), 0)
             FROM clientes c
             JOIN ventas v ON v.cliente_id = c.id AND v.tipo='credito' AND v.estado='completada' AND v.saldo_pendiente > 0
             WHERE c.activo=1
             GROUP BY c.id HAVING SUM(v.saldo_pendiente) > 0"
        )?;
        let rows = stmt.query_map([], |r| {
            let cli = Cliente {
                id: r.get(0)?, codigo: r.get(1)?, nombre: r.get(2)?, rfc: r.get(3)?,
                email: r.get(4)?, telefono: r.get(5)?, direccion: r.get(6)?, ciudad: r.get(7)?,
                limite_credito: r.get(8)?, saldo_pendiente: r.get(9)?, activo: r.get(10)?, fecha_registro: r.get(11)?,
            };
            let saldo: f64 = r.get(12)?;
            Ok((cli, saldo))
        })?;
        rows.collect()
    }

    pub fn actividad_reciente(&self) -> Result<Vec<(String, String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT 'venta' as tipo, folio, fecha FROM ventas WHERE estado='completada'
             UNION ALL
             SELECT 'gasto', descripcion, fecha FROM gastos WHERE estado='pagado'
             ORDER BY fecha DESC LIMIT 20"
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
        rows.collect()
    }

    // -----------------------------------------------------------------------
    // Ubicaciones
    // -----------------------------------------------------------------------
    pub fn listar_ubicaciones(&self) -> Result<Vec<Ubicacion>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, nombre, encargado, cedula, telefono, ciudad, direccion, activo FROM ubicaciones ORDER BY nombre"
        )?;
        let rows = stmt.query_map([], |r| Ok(Ubicacion {
            id: r.get(0)?, nombre: r.get(1)?, encargado: r.get(2)?, cedula: r.get(3)?,
            telefono: r.get(4)?, ciudad: r.get(5)?, direccion: r.get(6)?, activo: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn crear_ubicacion(&self, u: &UbicacionNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO ubicaciones (nombre, encargado, cedula, telefono, ciudad, direccion) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![u.nombre, u.encargado, u.cedula, u.telefono, u.ciudad, u.direccion]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_ubicacion(&self, id: i64, u: &UbicacionNueva) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE ubicaciones SET nombre=?1, encargado=?2, cedula=?3, telefono=?4, ciudad=?5, direccion=?6 WHERE id=?7",
            params![u.nombre, u.encargado, u.cedula, u.telefono, u.ciudad, u.direccion, id]
        )?;
        Ok(())
    }

    pub fn eliminar_ubicacion(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE ubicaciones SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Máquinas
    // -----------------------------------------------------------------------
    pub fn listar_maquinas(&self) -> Result<Vec<MaquinaUbicada>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT m.id, m.ubicacion_texto, m.codigo, m.descripcion, m.modelo,
                    m.numero_serie, m.color, m.fecha_ingreso, m.fecha_instalacion, m.comision,
                    m.comision_estimada, m.dia_cobro, m.activo, m.notas
             FROM maquinas_ubicadas m ORDER BY m.codigo"
        )?;
        let rows = stmt.query_map([], |r| Ok(MaquinaUbicada {
            id: r.get(0)?, ubicacion_texto: r.get(1)?,
            codigo: r.get(2)?, descripcion: r.get(3)?, modelo: r.get(4)?,
            numero_serie: r.get(5)?, color: r.get(6)?, fecha_ingreso: r.get(7)?,
            fecha_instalacion: r.get(8)?, comision: r.get(9)?, comision_estimada: r.get(10)?,
            dia_cobro: r.get(11)?, activo: r.get(12)?, notas: r.get(13)?,
        }))?;
        rows.collect()
    }

    pub fn crear_maquina(&self, m: &MaquinaNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let fecha = m.fecha_instalacion.as_deref().unwrap_or("");
        conn.execute(
            "INSERT INTO maquinas_ubicadas (ubicacion_texto, codigo, descripcion, modelo, numero_serie, color, comision, comision_estimada, dia_cobro, fecha_instalacion)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, COALESCE(NULLIF(?10, ''), date('now','localtime')))",
            params![m.ubicacion_texto, m.codigo, m.descripcion, m.modelo, m.numero_serie, m.color, m.comision, m.comision_estimada, m.dia_cobro, fecha]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_maquina(&self, id: i64, m: &MaquinaNueva) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let fecha = m.fecha_instalacion.as_deref().unwrap_or("");
        conn.execute(
            "UPDATE maquinas_ubicadas SET ubicacion_texto=?1, codigo=?2, descripcion=?3, modelo=?4, numero_serie=?5, color=?6, comision=?7, comision_estimada=?8, dia_cobro=?9, fecha_instalacion=COALESCE(NULLIF(?10, ''), fecha_instalacion) WHERE id=?11",
            params![m.ubicacion_texto, m.codigo, m.descripcion, m.modelo, m.numero_serie, m.color, m.comision, m.comision_estimada, m.dia_cobro, fecha, id]
        )?;
        Ok(())
    }

    pub fn checkpoint(&self) {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE)");
    }

    pub fn cobros_pendientes_mes(&self, periodo: &str, dia_hoy: i32) -> Result<Vec<MaquinaUbicada>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT m.id, m.ubicacion_texto, m.codigo, m.descripcion, m.modelo,
                    m.numero_serie, m.color, m.fecha_ingreso, m.fecha_instalacion, m.comision,
                    m.comision_estimada, m.dia_cobro, m.activo, m.notas
             FROM maquinas_ubicadas m
             WHERE m.activo = 1 AND m.dia_cobro <= ?2
               AND NOT EXISTS (SELECT 1 FROM cobros_comisiones c WHERE c.maquina_id = m.id AND COALESCE(c.periodo, '') = ?1)
             ORDER BY m.dia_cobro"
        )?;
        let rows = stmt.query_map(params![periodo, dia_hoy], |r| Ok(MaquinaUbicada {
            id: r.get(0)?, ubicacion_texto: r.get(1)?,
            codigo: r.get(2)?, descripcion: r.get(3)?, modelo: r.get(4)?,
            numero_serie: r.get(5)?, color: r.get(6)?, fecha_ingreso: r.get(7)?,
            fecha_instalacion: r.get(8)?, comision: r.get(9)?, comision_estimada: r.get(10)?,
            dia_cobro: r.get(11)?, activo: r.get(12)?, notas: r.get(13)?,
        }))?;
        rows.collect()
    }

    pub fn eliminar_maquina(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE maquinas_ubicadas SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Cobros de Comisiones - alerta de inicio (solo vencidos HOY, no pagados)
    pub fn resumen_cobros_vencidos_hoy(&self, periodo: &str, dia_hoy: i32) -> Result<(i64, f64)> {
        let conn = self.conn.lock().unwrap();
        let n = conn.query_row(
            "SELECT COUNT(*) FROM maquinas_ubicadas m WHERE m.activo=1 AND m.dia_cobro = ?2 AND NOT EXISTS (SELECT 1 FROM cobros_comisiones c WHERE c.maquina_id=m.id AND COALESCE(c.periodo,'')=?1)",
            params![periodo, dia_hoy], |r| r.get::<_, i64>(0)
        )?;
        let monto = conn.query_row(
            "SELECT COALESCE(SUM(m.comision_estimada),0) FROM maquinas_ubicadas m WHERE m.activo=1 AND m.dia_cobro = ?2 AND NOT EXISTS (SELECT 1 FROM cobros_comisiones c WHERE c.maquina_id=m.id AND COALESCE(c.periodo,'')=?1)",
            params![periodo, dia_hoy], |r| r.get::<_, f64>(0)
        )?;
        Ok((n, monto))
    }

    pub fn listar_todas_comisiones(&self) -> Result<Vec<CobroComision>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, maquina_id, monto, fecha, mes, periodo, observacion, notas
             FROM cobros_comisiones ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(CobroComision {
            id: r.get(0)?, maquina_id: r.get(1)?, monto: r.get(2)?, fecha: r.get(3)?,
            mes: r.get(4)?, periodo: r.get(5)?, observacion: r.get(6)?, notas: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn eliminar_cobro_comision(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM cobros_comisiones WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn actualizar_cobro_comision(&self, id: i64, c: &CobroComisionNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE cobros_comisiones SET maquina_id=?1, monto=?2, mes=?3, periodo=?4, observacion=?5, notas=?6 WHERE id=?7",
            params![c.maquina_id, c.monto, c.mes, c.periodo, c.observacion, c.notas, id]
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    pub fn listar_cobros_comisiones(&self, maquina_id: i64) -> Result<Vec<CobroComision>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, maquina_id, monto, fecha, mes, periodo, observacion, notas
             FROM cobros_comisiones WHERE maquina_id=?1 ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map(params![maquina_id], |r| Ok(CobroComision {
            id: r.get(0)?, maquina_id: r.get(1)?, monto: r.get(2)?, fecha: r.get(3)?,
            mes: r.get(4)?, periodo: r.get(5)?, observacion: r.get(6)?, notas: r.get(7)?,
        }))?;
        rows.collect()
    }

    // -----------------------------------------------------------------------
    // Garantías
    // -----------------------------------------------------------------------
    pub fn listar_garantias(&self) -> Result<Vec<Garantia>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT g.id, g.producto_id, COALESCE(p.nombre, ''), g.venta_id, COALESCE(v.folio, ''),
                    g.producto, g.numero_serie, g.cliente_nombre, g.cedula, g.telefono, g.direccion, g.ciudad,
                    g.monto_pago, g.estado, g.observacion, g.fecha_inicio, g.fecha_fin, g.descripcion, g.activa
             FROM garantias g
             LEFT JOIN productos p ON p.id = g.producto_id
             LEFT JOIN ventas v ON v.id = g.venta_id
             ORDER BY g.fecha_fin ASC"
        )?;
        let rows = stmt.query_map([], |r| Ok(Garantia {
            id: r.get(0)?, producto_id: r.get(1)?, producto_nombre: r.get(2)?,
            venta_id: r.get(3)?, folio_venta: r.get(4)?, producto: r.get(5)?,
            numero_serie: r.get(6)?, cliente_nombre: r.get(7)?, cedula: r.get(8)?,
            telefono: r.get(9)?, direccion: r.get(10)?, ciudad: r.get(11)?,
            monto_pago: r.get(12)?, estado: r.get(13)?, observacion: r.get(14)?,
            fecha_inicio: r.get(15)?, fecha_fin: r.get(16)?, descripcion: r.get(17)?, activa: r.get(18)?,
        }))?;
        rows.collect()
    }

    pub fn crear_garantia(&self, g: &GarantiaNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO garantias (producto_id, venta_id, producto, numero_serie, cliente_nombre, cedula, telefono, direccion, ciudad, monto_pago, observacion, fecha_inicio, fecha_fin, descripcion)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![g.producto_id, g.venta_id, g.producto, g.numero_serie, g.cliente_nombre,
                    g.cedula, g.telefono, g.direccion, g.ciudad, g.monto_pago, g.observacion,
                    g.fecha_inicio, g.fecha_fin, g.descripcion]
        )?;
        Ok(conn.last_insert_rowid())
    }

    // -----------------------------------------------------------------------
    // Crédito
    // -----------------------------------------------------------------------
    pub fn listar_cuentas_credito(&self) -> Result<Vec<CuentaCredito>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT cc.id, cc.nombre, cc.tipo, cc.cliente_id, COALESCE(c.nombre, ''), cc.proveedor_id, COALESCE(p.nombre, ''),
                    cc.limite, cc.saldo_actual, cc.notas, cc.activo, cc.fecha_apertura
             FROM cuentas_credito cc
             LEFT JOIN clientes c ON c.id = cc.cliente_id
             LEFT JOIN proveedores p ON p.id = cc.proveedor_id
             ORDER BY cc.nombre"
        )?;
        let rows = stmt.query_map([], |r| Ok(CuentaCredito {
            id: r.get(0)?, nombre: r.get(1)?, tipo: r.get(2)?, cliente_id: r.get(3)?,
            cliente_nombre: r.get(4)?, proveedor_id: r.get(5)?, proveedor_nombre: r.get(6)?,
            limite: r.get(7)?, saldo_actual: r.get(8)?, notas: r.get(9)?,
            activa: r.get(10)?, fecha_apertura: r.get(11)?,
        }))?;
        rows.collect()
    }

    pub fn crear_cuenta_credito(&self, cc: &CuentaCreditoNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO cuentas_credito (nombre, tipo, cliente_id, proveedor_id, limite, notas)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![cc.nombre, cc.tipo, cc.cliente_id, cc.proveedor_id, cc.limite, cc.notas]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn listar_credito_movimientos(&self, cuenta_id: i64) -> Result<Vec<CreditoMovimiento>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, cuenta_id, tipo, monto, cantidad, precio_unit, saldo_acumulado, colores, descripcion, referencia_id, fecha
             FROM credito_movimientos WHERE cuenta_id=?1 ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map(params![cuenta_id], |r| Ok(CreditoMovimiento {
            id: r.get(0)?, cuenta_id: r.get(1)?, tipo: r.get(2)?, monto: r.get(3)?,
            cantidad: r.get(4)?, precio_unit: r.get(5)?, saldo_acumulado: r.get(6)?,
            colores: r.get(7)?, descripcion: r.get(8)?, referencia_id: r.get(9)?, fecha: r.get(10)?,
        }))?;
        rows.collect()
    }

    // -----------------------------------------------------------------------
    // Ahorro
    // -----------------------------------------------------------------------
    pub fn listar_ahorros(&self) -> Result<Vec<Ahorro>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT a.id, a.cliente_id, COALESCE(c.nombre, ''), a.saldo, a.activo, a.fecha_apertura
             FROM ahorros a LEFT JOIN clientes c ON c.id = a.cliente_id ORDER BY c.nombre"
        )?;
        let rows = stmt.query_map([], |r| Ok(Ahorro {
            id: r.get(0)?, cliente_id: r.get(1)?, cliente_nombre: r.get(2)?,
            saldo: r.get(3)?, activo: r.get(4)?, fecha_apertura: r.get(5)?,
        }))?;
        rows.collect()
    }

    pub fn crear_ahorro(&self, a: &Ahorro) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO ahorros (cliente_id, saldo) VALUES (?1, ?2)",
            params![a.cliente_id, a.saldo]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn listar_ahorro_movimientos(&self, ahorro_id: i64) -> Result<Vec<AhorroMovimiento>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, ahorro_id, tipo, monto, saldo_acumulado, cobro_id, descripcion, fecha
             FROM ahorro_movimientos WHERE ahorro_id=?1 ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map(params![ahorro_id], |r| Ok(AhorroMovimiento {
            id: r.get(0)?, ahorro_id: r.get(1)?, tipo: r.get(2)?, monto: r.get(3)?,
            saldo_acumulado: r.get(4)?, cobro_id: r.get(5)?, descripcion: r.get(6)?, fecha: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn crear_ahorro_movimiento(&self, m: &AhorroMovimientoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO ahorro_movimientos (ahorro_id, tipo, monto, cobro_id, descripcion)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![m.ahorro_id, m.tipo, m.monto, m.cobro_id, m.descripcion]
        )?;
        if m.tipo == "abono" {
            conn.execute("UPDATE ahorros SET saldo = saldo + ?1 WHERE id=?2", params![m.monto, m.ahorro_id])?;
        } else if m.tipo == "retiro" {
            conn.execute("UPDATE ahorros SET saldo = MAX(0, saldo - ?1) WHERE id=?2", params![m.monto, m.ahorro_id])?;
        }
        Ok(conn.last_insert_rowid())
    }

    pub fn eliminar_ahorro(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE ahorros SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn actualizar_ahorro(&self, id: i64, a: &Ahorro) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE ahorros SET cliente_id=?1, saldo=?2 WHERE id=?3",
            params![a.cliente_id, a.saldo, id]
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Ventas - delete / update
    // -----------------------------------------------------------------------
    pub fn eliminar_venta(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let folio: Option<String> = conn.query_row(
            "SELECT folio FROM ventas WHERE id=?1", params![id], |r| r.get(0)
        ).ok();
        let dets = self._detalles_venta(&conn, id)?;
        conn.execute("UPDATE ventas SET estado='cancelada' WHERE id=?1", params![id])?;
        for d in &dets {
            self._aplicar_stock(&conn, d.producto_id, d.cantidad, "entrada", "Venta cancelada", &format!("Venta {}", folio.clone().unwrap_or_default()))?;
        }
        if let Some(f) = &folio {
            conn.execute("DELETE FROM movimientos_inventario WHERE tipo='salida' AND referencia=?1", params![f])?;
        }
        Ok(())
    }

    pub fn actualizar_venta(&self, id: i64, v: &VentaNueva) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let dets_antes = self._detalles_venta(&conn, id)?;
        let mut subtotal = 0.0;
        for d in &v.detalles {
            subtotal += d.cantidad as f64 * d.precio_unitario;
        }
        let descuento_total = v.descuento;
        let base_imponible = (subtotal - descuento_total).max(0.0);
        let iva = if v.iva > 0.0 { v.iva } else { Self::_iva_con(&conn) };
        let impuesto = base_imponible * iva / 100.0;
        let total = base_imponible + impuesto;
        let saldo = if v.tipo == "credito" { total } else { 0.0 };
        conn.execute(
            "UPDATE ventas SET cliente_id=?1, cliente_nombre=?2, subtotal=?3, impuesto=?4, descuento=?5, total=?6, saldo_pendiente=?7, tipo=?8, metodo_pago=?9, notas=?10 WHERE id=?11",
            params![v.cliente_id, v.cliente_nombre, subtotal, impuesto, descuento_total, total, saldo, v.tipo, v.metodo_pago, v.notas, id]
        )?;
        conn.execute("DELETE FROM ventas_detalles WHERE venta_id=?1", params![id])?;
        for d in &dets_antes {
            self._aplicar_stock(&conn, d.producto_id, d.cantidad, "entrada", "Venta editada", "Ajuste")?;
        }
        for d in &v.detalles {
            let importe = (d.cantidad as f64 * d.precio_unitario) - d.descuento;
            conn.execute(
                "INSERT INTO ventas_detalles (venta_id, producto_id, producto_nombre, cantidad, precio_unitario, descuento, importe)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![id, d.producto_id, d.producto_nombre, d.cantidad, d.precio_unitario, d.descuento, importe]
            )?;
            self._aplicar_stock(&conn, d.producto_id, -d.cantidad as i32, "salida", "Venta", "Venta editada")?;
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Gastos - delete / update
    // -----------------------------------------------------------------------
    pub fn eliminar_gasto(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE gastos SET estado='cancelado' WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn actualizar_gasto(&self, id: i64, g: &GastoNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let total = g.monto + g.impuesto;
        let estado = if g.fecha_vencimiento.is_some() { "pendiente" } else { "pagado" };
        conn.execute(
            "UPDATE gastos SET numero=?1, categoria_id=?2, descripcion=?3, monto=?4, subtotal=?5, impuesto=?6, total=?7, proveedor_id=?8, metodo_pago=?9, referencia=?10, comprobante=?11, estado=?12, notas=?13, fecha_vencimiento=?14 WHERE id=?15",
            params![g.numero, g.categoria_id, g.descripcion, g.monto, g.subtotal, g.impuesto, total, g.proveedor_id, g.metodo_pago, g.referencia, g.comprobante, estado, g.notas, g.fecha_vencimiento, id]
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Garantias - delete
    // -----------------------------------------------------------------------
    pub fn eliminar_garantia(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE garantias SET activa=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn actualizar_garantia(&self, id: i64, g: &GarantiaNueva) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE garantias SET producto_id=?1, venta_id=?2, producto=?3, numero_serie=?4, cliente_nombre=?5, cedula=?6, telefono=?7, direccion=?8, ciudad=?9, monto_pago=?10, observacion=?11, fecha_inicio=?12, fecha_fin=?13, descripcion=?14 WHERE id=?15",
            params![g.producto_id, g.venta_id, g.producto, g.numero_serie, g.cliente_nombre, g.cedula, g.telefono, g.direccion, g.ciudad, g.monto_pago, g.observacion, g.fecha_inicio, g.fecha_fin, g.descripcion, id]
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Asientos - delete / update
    // -----------------------------------------------------------------------
    pub fn eliminar_asiento(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE asientos SET estado='cancelado' WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn actualizar_asiento(&self, id: i64, a: &AsientoNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let mut total_debe = 0.0;
        let mut total_haber = 0.0;
        for l in &a.lineas {
            total_debe += l.debe;
            total_haber += l.haber;
        }
        conn.execute(
            "UPDATE asientos SET fecha=?1, concepto=?2, descripcion=?3, referencia=?4, total_debe=?5, total_haber=?6 WHERE id=?7",
            params![a.fecha, a.concepto, a.descripcion, a.referencia, total_debe, total_haber, id]
        )?;
        conn.execute("DELETE FROM asiento_lineas WHERE asiento_id=?1", params![id])?;
        for l in &a.lineas {
            conn.execute(
                "INSERT INTO asiento_lineas (asiento_id, cuenta_id, descripcion, debe, haber) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, l.cuenta_id, l.descripcion, l.debe, l.haber]
            )?;
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Pagos Recibidos - delete / update
    // -----------------------------------------------------------------------
    pub fn eliminar_pago_recibido(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let (monto, venta_id): (f64, Option<i64>) = conn.query_row(
            "SELECT monto, venta_id FROM pagos_recibidos WHERE id=?1", params![id],
            |r| Ok((r.get(0)?, r.get(1)?))
        ).unwrap_or((0.0, None));
        conn.execute("DELETE FROM pagos_recibidos WHERE id=?1", params![id])?;
        if let Some(venta_id) = venta_id {
            conn.execute(
                "UPDATE ventas SET saldo_pendiente = saldo_pendiente + ?1 WHERE id=?2",
                params![monto, venta_id]
            )?;
        }
        Ok(())
    }

    pub fn actualizar_pago_recibido(&self, id: i64, p: &PagoRecibidoNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE pagos_recibidos SET venta_id=?1, cliente_id=?2, monto=?3, metodo_pago=?4, referencia=?5, notas=?6 WHERE id=?7",
            params![p.venta_id, p.cliente_id, p.monto, p.metodo_pago, p.referencia, p.notas, id]
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Pagos Realizados - delete / update
    // -----------------------------------------------------------------------
    pub fn eliminar_pago_realizado(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let (_monto, gasto_id): (f64, Option<i64>) = conn.query_row(
            "SELECT monto, gasto_id FROM pagos_realizados WHERE id=?1", params![id],
            |r| Ok((r.get(0)?, r.get(1)?))
        ).unwrap_or((0.0, None));
        conn.execute("DELETE FROM pagos_realizados WHERE id=?1", params![id])?;
        if let Some(gasto_id) = gasto_id {
            let cubierto: f64 = conn.query_row(
                "SELECT COALESCE(SUM(monto), 0) FROM pagos_realizados WHERE gasto_id=?1", params![gasto_id], |r| r.get(0)
            ).unwrap_or(0.0);
            let total_gasto: f64 = conn.query_row(
                "SELECT total FROM gastos WHERE id=?1", params![gasto_id], |r| r.get(0)
            ).unwrap_or(0.0);
            if cubierto < total_gasto {
                conn.execute(
                    "UPDATE gastos SET estado='pendiente', fecha_pago=NULL WHERE id=?1 AND fecha_vencimiento IS NOT NULL",
                    params![gasto_id]
                )?;
            }
        }
        Ok(())
    }

    pub fn actualizar_pago_realizado(&self, id: i64, p: &PagoRealizadoNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE pagos_realizados SET gasto_id=?1, proveedor_id=?2, monto=?3, metodo_pago=?4, referencia=?5, notas=?6 WHERE id=?7",
            params![p.gasto_id, p.proveedor_id, p.monto, p.metodo_pago, p.referencia, p.notas, id]
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Deudas de la Empresa
    // -----------------------------------------------------------------------
    pub fn listar_deudas_empresa(&self) -> Result<Vec<DeudaEmpresa>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, numero, proveedor_id, proveedor_nombre, concepto, descripcion,
                    categoria_id, categoria_nombre, fecha_deuda, fecha_vencimiento,
                    monto_total, saldo_pendiente, referencia, notas, estado, activa, fecha_registro
             FROM deudas_empresa WHERE activa=1 ORDER BY estado='pagada', fecha_deuda DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(DeudaEmpresa {
            id: r.get(0)?, numero: r.get(1)?, proveedor_id: r.get(2)?, proveedor_nombre: r.get(3)?,
            concepto: r.get(4)?, descripcion: r.get(5)?, categoria_id: r.get(6)?,
            categoria_nombre: r.get(7)?, fecha_deuda: r.get(8)?, fecha_vencimiento: r.get(9)?,
            monto_total: r.get(10)?, saldo_pendiente: r.get(11)?, referencia: r.get(12)?,
            notas: r.get(13)?, estado: r.get(14)?, activa: r.get(15)?, fecha_registro: r.get(16)?,
        }))?;
        rows.collect()
    }

    pub fn crear_deuda_empresa(&self, d: &DeudaEmpresaNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let numero = format!("DEU-{}", chrono::Local::now().format("%Y%m%d%H%M%S"));
        let saldo = d.monto_total.max(0.0);
        conn.execute(
            "INSERT INTO deudas_empresa (numero, proveedor_id, proveedor_nombre, concepto, descripcion,
                    categoria_id, categoria_nombre, fecha_deuda, fecha_vencimiento, monto_total,
                    saldo_pendiente, referencia, notas, estado)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'pendiente')",
            params![numero, d.proveedor_id, d.proveedor_nombre, d.concepto, d.descripcion,
                    d.categoria_id, d.categoria_nombre.clone(), d.fecha_deuda, d.fecha_vencimiento, d.monto_total,
                    saldo, d.referencia, d.notas]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_deuda_empresa(&self, id: i64, d: &DeudaEmpresaNueva) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let pagado: f64 = conn.query_row(
            "SELECT COALESCE(SUM(monto), 0) FROM deuda_pagos WHERE deuda_id=?1", params![id], |r| r.get(0)
        ).unwrap_or(0.0);
        let nuevo_saldo = (d.monto_total.max(0.0) - pagado).max(0.0);
        let estado = if nuevo_saldo <= 0.01 { "pagada" } else { "pendiente" };
        conn.execute(
            "UPDATE deudas_empresa SET proveedor_id=?1, proveedor_nombre=?2, concepto=?3, descripcion=?4,
                    categoria_id=?5, categoria_nombre=?6, fecha_deuda=?7, fecha_vencimiento=?8,
                    monto_total=?9, saldo_pendiente=?10, referencia=?11, notas=?12, estado=?13 WHERE id=?14",
            params![d.proveedor_id, d.proveedor_nombre, d.concepto, d.descripcion,
                    d.categoria_id, d.categoria_nombre.clone(), d.fecha_deuda, d.fecha_vencimiento, d.monto_total,
                    nuevo_saldo, d.referencia, d.notas, estado, id]
        )?;
        Ok(())
    }

    pub fn eliminar_deuda_empresa(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE deudas_empresa SET activa=0, estado='cancelada' WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn listar_deuda_pagos(&self, deuda_id: i64) -> Result<Vec<DeudaPago>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, deuda_id, fecha, monto, metodo_pago, referencia, notas
             FROM deuda_pagos WHERE deuda_id=?1 ORDER BY fecha ASC, id ASC"
        )?;
        let rows = stmt.query_map(params![deuda_id], |r| Ok(DeudaPago {
            id: r.get(0)?, deuda_id: r.get(1)?, fecha: r.get(2)?, monto: r.get(3)?,
            metodo_pago: r.get(4)?, referencia: r.get(5)?, notas: r.get(6)?,
        }))?;
        rows.collect()
    }

    pub fn listar_todos_deuda_pagos(&self) -> Result<Vec<DeudaPago>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, deuda_id, fecha, monto, metodo_pago, referencia, notas FROM deuda_pagos"
        )?;
        let rows = stmt.query_map([], |r| Ok(DeudaPago {
            id: r.get(0)?, deuda_id: r.get(1)?, fecha: r.get(2)?, monto: r.get(3)?,
            metodo_pago: r.get(4)?, referencia: r.get(5)?, notas: r.get(6)?,
        }))?;
        rows.collect()
    }

    pub fn crear_deuda_pago(&self, p: &DeudaPagoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO deuda_pagos (deuda_id, monto, metodo_pago, referencia, notas)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![p.deuda_id, p.monto, p.metodo_pago, p.referencia, p.notas]
        )?;
        let id = conn.last_insert_rowid();
        conn.execute(
            "UPDATE deudas_empresa SET saldo_pendiente = MAX(0, saldo_pendiente - ?1),
                    estado = CASE WHEN saldo_pendiente - ?1 <= 0.01 THEN 'pagada' ELSE 'pendiente' END
             WHERE id=?2",
            params![p.monto, p.deuda_id]
        )?;
        Ok(id)
    }

    pub fn eliminar_deuda_pago(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let (monto, deuda_id): (f64, i64) = conn.query_row(
            "SELECT monto, deuda_id FROM deuda_pagos WHERE id=?1", params![id],
            |r| Ok((r.get(0)?, r.get(1)?))
        ).unwrap_or((0.0, 0));
        conn.execute("DELETE FROM deuda_pagos WHERE id=?1", params![id])?;
        if deuda_id > 0 {
            conn.execute(
                "UPDATE deudas_empresa SET saldo_pendiente = saldo_pendiente + ?1,
                        estado = 'pendiente' WHERE id=?2",
                params![monto, deuda_id]
            )?;
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Creditos - delete / movimientos
    // -----------------------------------------------------------------------
    pub fn eliminar_cuenta_credito(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE cuentas_credito SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn actualizar_cuenta_credito(&self, id: i64, cc: &CuentaCreditoNueva) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE cuentas_credito SET nombre=?1, tipo=?2, cliente_id=?3, proveedor_id=?4, limite=?5, notas=?6 WHERE id=?7",
            params![cc.nombre, cc.tipo, cc.cliente_id, cc.proveedor_id, cc.limite, cc.notas, id]
        )?;
        Ok(())
    }

    pub fn crear_credito_movimiento(&self, m: &CreditoMovimientoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let saldo_anterior: f64 = conn.query_row(
            "SELECT saldo_actual FROM cuentas_credito WHERE id=?1", params![m.cuenta_id], |r| r.get(0)
        ).unwrap_or(0.0);
        let nuevo_saldo = if m.tipo == "abono" || m.tipo == "pago" {
            (saldo_anterior - m.monto).max(0.0)
        } else { saldo_anterior + m.monto };
        conn.execute(
            "INSERT INTO credito_movimientos (cuenta_id, tipo, monto, cantidad, precio_unit, saldo_acumulado, descripcion, referencia_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![m.cuenta_id, m.tipo, m.monto, m.cantidad, m.precio_unit, nuevo_saldo, m.descripcion, m.referencia_id]
        )?;
        conn.execute("UPDATE cuentas_credito SET saldo_actual=?1 WHERE id=?2", params![nuevo_saldo, m.cuenta_id])?;
        Ok(conn.last_insert_rowid())
    }

    // -----------------------------------------------------------------------
    // Cobros de Comisiones
    // -----------------------------------------------------------------------
    pub fn crear_cobro_comision(&self, c: &CobroComisionNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO cobros_comisiones (maquina_id, monto, mes, periodo, observacion, notas)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![c.maquina_id, c.monto, c.mes, c.periodo, c.observacion, c.notas]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn listar_maquinas_por_ubicacion(&self, ubicacion_id: i64) -> Result<Vec<MaquinaUbicada>> {
        let _ = (self, ubicacion_id);
        Ok(Vec::new())
    }

    // -----------------------------------------------------------------------
    // Reportes
    // -----------------------------------------------------------------------
    pub fn reporte_libro_diario(&self, desde: &str, hasta: &str) -> Result<Vec<Asiento>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, numero, fecha, concepto, descripcion, referencia, tipo, total_debe, total_haber, estado
             FROM asientos WHERE fecha BETWEEN ?1 AND ?2 AND estado='publicado' ORDER BY fecha, id"
        )?;
        let rows = stmt.query_map(params![desde, hasta], |r| Ok(Asiento {
            id: r.get(0)?, numero: r.get(1)?, fecha: r.get(2)?, concepto: r.get(3)?,
            descripcion: r.get(4)?, referencia: r.get(5)?, tipo: r.get(6)?,
            total_debe: r.get(7)?, total_haber: r.get(8)?, estado: r.get(9)?,
        }))?;
        rows.collect()
    }

    pub fn reporte_balance_general(&self) -> Result<(Vec<PlanCuentas>, Vec<f64>, Vec<f64>)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, codigo, nombre, tipo, naturaleza, nivel, padre_id, activo FROM plan_cuentas ORDER BY codigo"
        )?;
        let cuentas: Vec<PlanCuentas> = stmt.query_map([], |r| Ok(PlanCuentas {
            id: r.get(0)?, codigo: r.get(1)?, nombre: r.get(2)?, tipo: r.get(3)?,
            naturaleza: r.get(4)?, nivel: r.get(5)?, padre_id: r.get(6)?, activo: r.get(7)?,
        }))?.collect::<Result<Vec<_>>>().unwrap_or_default();

        let mut debe = Vec::new();
        let mut haber = Vec::new();
        for c in &cuentas {
            let d: f64 = conn.query_row(
                "SELECT COALESCE(SUM(debe),0) FROM asiento_lineas al JOIN asientos a ON a.id=al.asiento_id WHERE al.cuenta_id=?1 AND a.estado='publicado'",
                params![c.id], |r| r.get(0)
            ).unwrap_or(0.0);
            let h: f64 = conn.query_row(
                "SELECT COALESCE(SUM(haber),0) FROM asiento_lineas al JOIN asientos a ON a.id=al.asiento_id WHERE al.cuenta_id=?1 AND a.estado='publicado'",
                params![c.id], |r| r.get(0)
            ).unwrap_or(0.0);
            debe.push(d);
            haber.push(h);
        }
        Ok((cuentas, debe, haber))
    }

    // -----------------------------------------------------------------------
    // Inventario (stock + movimientos)
    // -----------------------------------------------------------------------
    fn _aplicar_stock(
        &self,
        conn: &rusqlite::Connection,
        producto_id: Option<i64>,
        delta: i32,
        tipo: &str,
        motivo: &str,
        referencia: &str,
    ) -> Result<()> {
        if let Some(pid) = producto_id {
            conn.execute(
                "UPDATE productos SET stock = MAX(0, stock + ?1) WHERE id=?2 AND activo=1",
                params![delta, pid]
            )?;
            let nombre: Option<String> = conn.query_row(
                "SELECT nombre FROM productos WHERE id=?1", params![pid], |r| r.get(0)
            ).ok();
            conn.execute(
                "INSERT INTO movimientos_inventario (producto_id, producto_nombre, tipo, cantidad, motivo, referencia)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![pid, nombre.unwrap_or_default(), tipo, delta.abs(), motivo, referencia]
            )?;
        }
        Ok(())
    }

    fn _detalles_venta(&self, conn: &rusqlite::Connection, venta_id: i64) -> Result<Vec<VentaDetalle>> {
        let mut stmt = conn.prepare(
            "SELECT id, venta_id, producto_id, descripcion, producto_nombre, cantidad, precio_unitario, descuento, importe
             FROM ventas_detalles WHERE venta_id=?1"
        )?;
        let rows = stmt.query_map(params![venta_id], |r| Ok(VentaDetalle {
            id: r.get(0)?, venta_id: r.get(1)?, producto_id: r.get(2)?,
            descripcion: r.get(3)?, producto_nombre: r.get(4)?, cantidad: r.get(5)?,
            precio_unitario: r.get(6)?, descuento: r.get(7)?, importe: r.get(8)?,
        }))?;
        rows.collect()
    }

    pub fn listar_movimientos_inventario(&self) -> Result<Vec<MovimientoInventario>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, producto_id, producto_nombre, tipo, cantidad, motivo, referencia, fecha
             FROM movimientos_inventario ORDER BY fecha DESC, id DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(MovimientoInventario {
            id: r.get(0)?, producto_id: r.get(1)?, producto_nombre: r.get(2)?,
            tipo: r.get(3)?, cantidad: r.get(4)?, motivo: r.get(5)?,
            referencia: r.get(6)?, fecha: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn listar_movimientos_producto(&self, producto_id: i64) -> Result<Vec<MovimientoInventario>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, producto_id, producto_nombre, tipo, cantidad, motivo, referencia, fecha
             FROM movimientos_inventario WHERE producto_id=?1 ORDER BY fecha DESC, id DESC"
        )?;
        let rows = stmt.query_map(params![producto_id], |r| Ok(MovimientoInventario {
            id: r.get(0)?, producto_id: r.get(1)?, producto_nombre: r.get(2)?,
            tipo: r.get(3)?, cantidad: r.get(4)?, motivo: r.get(5)?,
            referencia: r.get(6)?, fecha: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn ajustar_stock(&self, producto_id: i64, nuevo_stock: i32, motivo: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let actual: i32 = conn.query_row(
            "SELECT stock FROM productos WHERE id=?1", params![producto_id], |r| r.get(0)
        )?;
        conn.execute(
            "UPDATE productos SET stock=?1 WHERE id=?2", params![nuevo_stock, producto_id]
        )?;
        let nombre: String = conn.query_row(
            "SELECT nombre FROM productos WHERE id=?1", params![producto_id], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO movimientos_inventario (producto_id, producto_nombre, tipo, cantidad, motivo, referencia)
             VALUES (?1, ?2, 'ajuste', ?3, ?4, 'Ajuste manual')",
            params![producto_id, nombre, (nuevo_stock - actual).abs(), motivo]
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Compras (entradas de inventario)
    // -----------------------------------------------------------------------
    pub fn listar_compras(&self) -> Result<Vec<Compra>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, numero, proveedor_id, proveedor_nombre, fecha, subtotal, impuesto, descuento, total,
                    metodo_pago, referencia, notas, estado
             FROM compras ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(Compra {
            id: r.get(0)?, numero: r.get(1)?, proveedor_id: r.get(2)?, proveedor_nombre: r.get(3)?,
            fecha: r.get(4)?, subtotal: r.get(5)?, impuesto: r.get(6)?, descuento: r.get(7)?,
            total: r.get(8)?, metodo_pago: r.get(9)?, referencia: r.get(10)?,
            notas: r.get(11)?, estado: r.get(12)?,
        }))?;
        rows.collect()
    }

    pub fn crear_compra(&self, c: &CompraNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let numero = format!("C-{}", chrono::Local::now().format("%Y%m%d%H%M%S%3f"));
        let mut subtotal = 0.0;
        for d in &c.detalles {
            subtotal += d.cantidad as f64 * d.precio_unitario;
        }
        let descuento_total = c.descuento;
        let base_imponible = (subtotal - descuento_total).max(0.0);
        let iva = if c.iva > 0.0 { c.iva } else { Self::_iva_con(&conn) };
        let impuesto = base_imponible * iva / 100.0;
        let total = base_imponible + impuesto;

        conn.execute(
            "INSERT INTO compras (numero, proveedor_id, proveedor_nombre, subtotal, impuesto, descuento, total, metodo_pago, referencia, notas)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![numero, c.proveedor_id, c.proveedor_nombre, subtotal, impuesto, descuento_total, total, c.metodo_pago, c.metodo_pago, c.notas]
        )?;
        let compra_id = conn.last_insert_rowid();

        for d in &c.detalles {
            let importe = (d.cantidad as f64 * d.precio_unitario) - d.descuento;
            conn.execute(
                "INSERT INTO compra_detalles (compra_id, producto_id, producto_nombre, cantidad, precio_unitario, descuento, importe)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![compra_id, d.producto_id, d.producto_nombre, d.cantidad, d.precio_unitario, d.descuento, importe]
            )?;
            self._aplicar_stock(&conn, d.producto_id, d.cantidad, "entrada", "Compra", &numero)?;
        }
        Ok(compra_id)
    }

    pub fn obtener_detalles_compra(&self, compra_id: i64) -> Result<Vec<CompraDetalle>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, compra_id, producto_id, producto_nombre, cantidad, precio_unitario, descuento, importe
             FROM compra_detalles WHERE compra_id=?1"
        )?;
        let rows = stmt.query_map(params![compra_id], |r| Ok(CompraDetalle {
            id: r.get(0)?, compra_id: r.get(1)?, producto_id: r.get(2)?,
            producto_nombre: r.get(3)?, cantidad: r.get(4)?,
            precio_unitario: r.get(5)?, descuento: r.get(6)?, importe: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn eliminar_compra(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let numero: String = conn.query_row(
            "SELECT numero FROM compras WHERE id=?1", params![id], |r| r.get(0)
        )?;
        let mut stmt = conn.prepare("SELECT producto_id, cantidad FROM compra_detalles WHERE compra_id=?1")?;
        let rows = stmt.query_map(params![id], |r| Ok((r.get::<_, Option<i64>>(0)?, r.get::<_, i32>(1)?)))?;
        for row in rows {
            let (pid, cant) = row?;
            self._aplicar_stock(&conn, pid, -cant, "salida", "Compra eliminada", &numero)?;
        }
        conn.execute("DELETE FROM compra_detalles WHERE compra_id=?1", params![id])?;
        conn.execute("DELETE FROM compras WHERE id=?1", params![id])?;
        conn.execute("DELETE FROM movimientos_inventario WHERE tipo='entrada' AND referencia=?1", params![numero])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Cotizaciones
    // -----------------------------------------------------------------------
    pub fn listar_cotizaciones(&self) -> Result<Vec<Cotizacion>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, numero, cliente_id, cliente_nombre, fecha, validez_dias, subtotal, impuesto, descuento, total, estado, notas
             FROM cotizaciones ORDER BY fecha DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(Cotizacion {
            id: r.get(0)?, numero: r.get(1)?, cliente_id: r.get(2)?, cliente_nombre: r.get(3)?,
            fecha: r.get(4)?, validez_dias: r.get(5)?, subtotal: r.get(6)?, impuesto: r.get(7)?,
            descuento: r.get(8)?, total: r.get(9)?, estado: r.get(10)?, notas: r.get(11)?,
        }))?;
        rows.collect()
    }

    pub fn crear_cotizacion(&self, c: &CotizacionNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let numero = format!("COT-{}", chrono::Local::now().format("%Y%m%d%H%M%S%3f"));
        let mut subtotal = 0.0;
        for d in &c.detalles {
            subtotal += d.cantidad as f64 * d.precio_unitario;
        }
        let descuento_total = c.descuento;
        let base_imponible = (subtotal - descuento_total).max(0.0);
        let iva = if c.iva > 0.0 { c.iva } else { Self::_iva_con(&conn) };
        let impuesto = base_imponible * iva / 100.0;
        let total = base_imponible + impuesto;

        conn.execute(
            "INSERT INTO cotizaciones (numero, cliente_id, cliente_nombre, validez_dias, subtotal, impuesto, descuento, total, notas)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![numero, c.cliente_id, c.cliente_nombre, c.validez_dias, subtotal, impuesto, descuento_total, total, c.notas]
        )?;
        let cotizacion_id = conn.last_insert_rowid();

        for d in &c.detalles {
            let importe = (d.cantidad as f64 * d.precio_unitario) - d.descuento;
            conn.execute(
                "INSERT INTO cotizacion_detalles (cotizacion_id, producto_id, producto_nombre, cantidad, precio_unitario, descuento, importe)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![cotizacion_id, d.producto_id, d.producto_nombre, d.cantidad, d.precio_unitario, d.descuento, importe]
            )?;
        }
        Ok(cotizacion_id)
    }

    pub fn obtener_detalles_cotizacion(&self, cotizacion_id: i64) -> Result<Vec<CotizacionDetalle>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, cotizacion_id, producto_id, producto_nombre, cantidad, precio_unitario, descuento, importe
             FROM cotizacion_detalles WHERE cotizacion_id=?1"
        )?;
        let rows = stmt.query_map(params![cotizacion_id], |r| Ok(CotizacionDetalle {
            id: r.get(0)?, cotizacion_id: r.get(1)?, producto_id: r.get(2)?,
            producto_nombre: r.get(3)?, cantidad: r.get(4)?,
            precio_unitario: r.get(5)?, descuento: r.get(6)?, importe: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn eliminar_cotizacion(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM cotizacion_detalles WHERE cotizacion_id=?1", params![id])?;
        conn.execute("DELETE FROM cotizaciones WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn convertir_cotizacion_en_venta(&self, cotizacion_id: i64) -> Result<VentaDesdeCotizacion> {
        let conn = self.conn.lock().unwrap();
        let (cliente_id, cliente_nombre): (Option<i64>, String) = conn.query_row(
            "SELECT cliente_id, cliente_nombre FROM cotizaciones WHERE id=?1 AND estado='vigente'",
            params![cotizacion_id], |r| Ok((r.get(0)?, r.get(1)?))
        )?;
        let mut stmt = conn.prepare(
            "SELECT producto_id, producto_nombre, cantidad, precio_unitario, descuento FROM cotizacion_detalles WHERE cotizacion_id=?1"
        )?;
        let detalles = stmt.query_map(params![cotizacion_id], |r| Ok(VentaDetalleNuevo {
            producto_id: r.get(0)?, producto_nombre: r.get(1)?, cantidad: r.get(2)?,
            precio_unitario: r.get(3)?, descuento: r.get(4)?,
        }))?.collect::<Result<Vec<_>>>()?;
        drop(stmt);
        let venta_nueva = VentaNueva {
            cliente_id,
            cliente_nombre,
            tipo: "contado".into(),
            notas: format!("Venta generada desde cotizacion COT-{}", cotizacion_id),
            descuento: 0.0,
            metodo_pago: None,
            iva: Self::_iva_con(&conn),
            detalles,
        };
        let venta_id = self._insertar_venta_con(&conn, &venta_nueva)?;
        let folio: String = conn.query_row(
            "SELECT folio FROM ventas WHERE id=?1", params![venta_id], |r| r.get(0)
        )?;
        conn.execute(
            "UPDATE cotizaciones SET estado='convertida' WHERE id=?1", params![cotizacion_id]
        )?;
        Ok(VentaDesdeCotizacion { cotizacion_id, venta_id, folio })
    }

    // -----------------------------------------------------------------------
    // Reportes financieros
    // -----------------------------------------------------------------------
    pub fn reporte_estado_resultados(&self, desde: &str, hasta: &str) -> Result<EstadoResultados> {
        let conn = self.conn.lock().unwrap();
        let ventas: f64 = conn.query_row(
            "SELECT COALESCE(SUM(total),0) FROM ventas WHERE fecha BETWEEN ?1 AND ?2 AND estado!='cancelada'",
            params![desde, hasta], |r| r.get(0)
        )?;
        let costo: f64 = conn.query_row(
            "SELECT COALESCE(SUM(vd.cantidad * COALESCE(p.precio_compra, vd.precio_unitario)),0)
             FROM ventas_detalles vd
             JOIN ventas v ON v.id = vd.venta_id
             LEFT JOIN productos p ON p.id = vd.producto_id
             WHERE v.fecha BETWEEN ?1 AND ?2 AND v.estado!='cancelada'",
            params![desde, hasta], |r| r.get(0)
        )?;
        let gastos: f64 = conn.query_row(
            "SELECT COALESCE(SUM(total),0) FROM gastos WHERE fecha BETWEEN ?1 AND ?2 AND estado!='cancelado'",
            params![desde, hasta], |r| r.get(0)
        )?;
        Ok(EstadoResultados {
            ventas_total: ventas,
            costo_ventas: costo,
            utilidad_bruta: ventas - costo,
            gastos_total: gastos,
            utilidad_neta: ventas - costo - gastos,
            ingresos_otros: 0.0,
        })
    }

    pub fn reporte_balance_resumen(&self) -> Result<(f64, f64, f64, f64, f64, f64, f64)> {
        let conn = self.conn.lock().unwrap();
        let cxc: f64 = conn.query_row(
            "SELECT COALESCE(SUM(saldo_pendiente),0) FROM ventas WHERE estado!='cancelada'", [], |r| r.get(0)
        )?;
        let inventario: f64 = conn.query_row(
            "SELECT COALESCE(SUM(stock * precio_compra),0) FROM productos WHERE activo=1", [], |r| r.get(0)
        )?;
        let pagos_recibidos: f64 = conn.query_row(
            "SELECT COALESCE(SUM(monto),0) FROM pagos_recibidos", [], |r| r.get(0)
        )?;
        let pagos_realizados: f64 = conn.query_row(
            "SELECT COALESCE(SUM(monto),0) FROM pagos_realizados", [], |r| r.get(0)
        )?;
        let cxp_deudas: f64 = conn.query_row(
            "SELECT COALESCE(SUM(saldo),0) FROM deudas_empresa WHERE activa=1", [], |r| r.get(0)
        )?;
        let cxp_gastos_saldo: f64 = conn.query_row(
            "SELECT COALESCE(SUM(CASE WHEN estado='pendiente' THEN total ELSE 0 END),0) FROM gastos WHERE estado!='cancelado'",
            [], |r| r.get(0)
        )?;
        let efectivo = pagos_recibidos - pagos_realizados;
        let total_activos = efectivo + cxc + inventario;
        let total_pasivos = cxp_deudas + cxp_gastos_saldo;
        let patrimonio = total_activos - total_pasivos;
        Ok((efectivo, cxc, inventario, total_activos, cxp_deudas + cxp_gastos_saldo, total_pasivos, patrimonio))
    }

    pub fn reporte_libro_mayor(&self, cuenta_id: i64, desde: &str, hasta: &str) -> Result<Vec<MayorLinea>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT a.fecha, a.concepto, al.debe, al.haber, a.id
             FROM asiento_lineas al JOIN asientos a ON a.id = al.asiento_id
             WHERE al.cuenta_id=?1 AND a.fecha BETWEEN ?2 AND ?3 AND a.estado='publicado'
             ORDER BY a.fecha, a.id"
        )?;
        let rows = stmt.query_map(params![cuenta_id, desde, hasta], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, f64>(2)?, r.get::<_, f64>(3)?))
        })?;
        let mut saldo = 0.0;
        let mut lineas = Vec::new();
        for row in rows {
            let (fecha, concepto, debe, haber) = row?;
            saldo += debe - haber;
            lineas.push(MayorLinea { fecha, concepto, debe, haber, saldo });
        }
        Ok(lineas)
    }

    pub fn reporte_antiguedad_saldos(&self) -> Result<(Vec<SaldoPendiente>, Vec<SaldoPendiente>)> {
        let conn = self.conn.lock().unwrap();
        let cxc: Vec<SaldoPendiente> = {
            let mut stmt = conn.prepare(
                "SELECT cliente_nombre, saldo_pendiente, fecha FROM ventas WHERE saldo_pendiente > 0.01 AND estado!='cancelada'"
            )?;
            stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, f64>(1)?, r.get::<_, String>(2)?)))?
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(|(nombre, total, fecha)| {
                    let dias = fecha_dias_desde(&fecha);
                    SaldoPendiente { nombre, total, dias }
                })
                .collect()
        };
        let cxp: Vec<SaldoPendiente> = {
            let mut stmt = conn.prepare(
                "SELECT proveedor_nombre, saldo FROM deudas_empresa WHERE activa=1 AND saldo > 0.01"
            )?;
            stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, f64>(1)?, r.get::<_, String>(2)?)))?
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(|(nombre, total, fecha)| {
                    let dias = fecha_dias_desde(&fecha);
                    SaldoPendiente { nombre, total, dias }
                })
                .collect()
        };
        Ok((cxc, cxp))
    }

    // -----------------------------------------------------------------------
    // Retenciones (SRI)
    // -----------------------------------------------------------------------
    pub fn listar_retenciones(&self) -> Result<Vec<Retencion>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, numero, proveedor_id, proveedor_nombre, cedula, fecha,
                    base_imp_renta, porcentaje_renta, valor_renta,
                    base_imp_iva, porcentaje_iva, valor_iva,
                    tipo_comprobante, numero_comprobante, referencia, estado
             FROM retenciones ORDER BY fecha DESC, id DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(Retencion {
            id: r.get(0)?, numero: r.get(1)?, proveedor_id: r.get(2)?,
            proveedor_nombre: r.get(3)?, cedula: r.get(4)?, fecha: r.get(5)?,
            base_imp_renta: r.get(6)?, porcentaje_renta: r.get(7)?, valor_renta: r.get(8)?,
            base_imp_iva: r.get(9)?, porcentaje_iva: r.get(10)?, valor_iva: r.get(11)?,
            tipo_comprobante: r.get(12)?, numero_comprobante: r.get(13)?,
            referencia: r.get(14)?, estado: r.get(15)?,
        }))?;
        rows.collect()
    }

    pub fn crear_retencion(&self, r: &RetencionNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO retenciones (numero, proveedor_id, proveedor_nombre, cedula, fecha,
                base_imp_renta, porcentaje_renta, valor_renta, base_imp_iva, porcentaje_iva, valor_iva,
                tipo_comprobante, numero_comprobante, referencia, estado)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![r.numero, r.proveedor_id, r.proveedor_nombre, r.cedula, r.fecha,
                r.base_imp_renta, r.porcentaje_renta, r.valor_renta,
                r.base_imp_iva, r.porcentaje_iva, r.valor_iva,
                r.tipo_comprobante, r.numero_comprobante, r.referencia, r.estado]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_retencion(&self, id: i64, r: &RetencionNueva) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE retenciones SET numero=?1, proveedor_id=?2, proveedor_nombre=?3, cedula=?4, fecha=?5,
                base_imp_renta=?6, porcentaje_renta=?7, valor_renta=?8,
                base_imp_iva=?9, porcentaje_iva=?10, valor_iva=?11,
                tipo_comprobante=?12, numero_comprobante=?13, referencia=?14, estado=?15
             WHERE id=?16",
            params![r.numero, r.proveedor_id, r.proveedor_nombre, r.cedula, r.fecha,
                r.base_imp_renta, r.porcentaje_renta, r.valor_renta,
                r.base_imp_iva, r.porcentaje_iva, r.valor_iva,
                r.tipo_comprobante, r.numero_comprobante, r.referencia, r.estado, id]
        )?;
        Ok(())
    }

    pub fn eliminar_retencion(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM retenciones WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn proximo_numero_retencion(&self) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let ultimo: Option<String> = conn.query_row(
            "SELECT numero FROM retenciones ORDER BY id DESC LIMIT 1",
            [], |r| r.get(0)
        ).ok();
        let n = ultimo.and_then(|s| s.trim_start_matches("R-").parse::<i64>().ok()).unwrap_or(0) + 1;
        Ok(format!("R-{:04}", n))
    }

    // -----------------------------------------------------------------------
    // Empleados y Nómina
    // -----------------------------------------------------------------------
    pub fn listar_empleados(&self) -> Result<Vec<Empleado>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, cedula, nombre, cargo, telefono, sueldo_base, fecha_ingreso, activo
             FROM empleados ORDER BY nombre"
        )?;
        let rows = stmt.query_map([], |r| Ok(Empleado {
            id: r.get(0)?, cedula: r.get(1)?, nombre: r.get(2)?, cargo: r.get(3)?,
            telefono: r.get(4)?, sueldo_base: r.get(5)?, fecha_ingreso: r.get(6)?, activo: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn crear_empleado(&self, e: &EmpleadoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO empleados (cedula, nombre, cargo, telefono, sueldo_base, fecha_ingreso)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![e.cedula, e.nombre, e.cargo, e.telefono, e.sueldo_base, e.fecha_ingreso]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_empleado(&self, id: i64, e: &EmpleadoNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE empleados SET cedula=?1, nombre=?2, cargo=?3, telefono=?4, sueldo_base=?5, fecha_ingreso=?6 WHERE id=?7",
            params![e.cedula, e.nombre, e.cargo, e.telefono, e.sueldo_base, e.fecha_ingreso, id]
        )?;
        Ok(())
    }

    pub fn eliminar_empleado(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE empleados SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn listar_roles_pago(&self) -> Result<Vec<RolPago>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT r.id, r.empleado_id, COALESCE(e.nombre, ''), r.periodo, r.dias,
                    r.sueldo_bruto, r.horas_extra, r.comisiones, r.total_ingresos,
                    r.iess, r.prestamos, r.otras_retenciones, r.total_egresos, r.total_neto,
                    r.estado, r.notas
             FROM roles_pago r LEFT JOIN empleados e ON e.id = r.empleado_id
             ORDER BY r.periodo DESC, r.id DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(RolPago {
            id: r.get(0)?, empleado_id: r.get(1)?, empleado_nombre: r.get(2)?, periodo: r.get(3)?,
            dias: r.get(4)?, sueldo_bruto: r.get(5)?, horas_extra: r.get(6)?, comisiones: r.get(7)?,
            total_ingresos: r.get(8)?, iess: r.get(9)?, prestamos: r.get(10)?,
            otras_retenciones: r.get(11)?, total_egresos: r.get(12)?, total_neto: r.get(13)?,
            estado: r.get(14)?, notas: r.get(15)?,
        }))?;
        rows.collect()
    }

    pub fn crear_rol_pago(&self, r: &RolPagoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO roles_pago (empleado_id, periodo, dias, sueldo_bruto, horas_extra, comisiones,
                total_ingresos, iess, prestamos, otras_retenciones, total_egresos, total_neto, estado, notas)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![r.empleado_id, r.periodo, r.dias, r.sueldo_bruto, r.horas_extra, r.comisiones,
                r.sueldo_bruto + r.horas_extra + r.comisiones,
                r.iess, r.prestamos, r.otras_retenciones,
                r.iess + r.prestamos + r.otras_retenciones,
                r.sueldo_bruto + r.horas_extra + r.comisiones - r.iess - r.prestamos - r.otras_retenciones,
                "generado", r.notas]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_rol_pago(&self, id: i64, r: &RolPagoNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE roles_pago SET empleado_id=?1, periodo=?2, dias=?3, sueldo_bruto=?4, horas_extra=?5,
                comisiones=?6, total_ingresos=?7, iess=?8, prestamos=?9, otras_retenciones=?10,
                total_egresos=?11, total_neto=?12, notas=?13 WHERE id=?14",
            params![r.empleado_id, r.periodo, r.dias, r.sueldo_bruto, r.horas_extra, r.comisiones,
                r.sueldo_bruto + r.horas_extra + r.comisiones,
                r.iess, r.prestamos, r.otras_retenciones,
                r.iess + r.prestamos + r.otras_retenciones,
                r.sueldo_bruto + r.horas_extra + r.comisiones - r.iess - r.prestamos - r.otras_retenciones,
                r.notas, id]
        )?;
        Ok(())
    }

    pub fn marcar_rol_pagado(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE roles_pago SET estado='pagado' WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn eliminar_rol_pago(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM roles_pago WHERE id=?1", params![id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Activos fijos y depreciación
    // -----------------------------------------------------------------------
    pub fn listar_activos_fijos(&self) -> Result<Vec<ActivoFijo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, descripcion, categoria, fecha_adquisicion, valor_adquisicion,
                    valor_residual, vida_util_anios, depreciacion_mensual, depreciacion_acumulada, activo
             FROM activos_fijos ORDER BY fecha_adquisicion DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(ActivoFijo {
            id: r.get(0)?, descripcion: r.get(1)?, categoria: r.get(2)?,
            fecha_adquisicion: r.get(3)?, valor_adquisicion: r.get(4)?, valor_residual: r.get(5)?,
            vida_util_anios: r.get(6)?, depreciacion_mensual: r.get(7)?,
            depreciacion_acumulada: r.get(8)?, activo: r.get(9)?,
        }))?;
        rows.collect()
    }

    pub fn crear_activo_fijo(&self, a: &ActivoFijoNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let mensual = if a.vida_util_anios > 0.0 {
            (a.valor_adquisicion - a.valor_residual) / (a.vida_util_anios * 12.0)
        } else { 0.0 };
        conn.execute(
            "INSERT INTO activos_fijos (descripcion, categoria, fecha_adquisicion, valor_adquisicion,
                valor_residual, vida_util_anios, depreciacion_mensual)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![a.descripcion, a.categoria, a.fecha_adquisicion, a.valor_adquisicion,
                a.valor_residual, a.vida_util_anios, mensual]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_activo_fijo(&self, id: i64, a: &ActivoFijoNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let mensual = if a.vida_util_anios > 0.0 {
            (a.valor_adquisicion - a.valor_residual) / (a.vida_util_anios * 12.0)
        } else { 0.0 };
        conn.execute(
            "UPDATE activos_fijos SET descripcion=?1, categoria=?2, fecha_adquisicion=?3,
                valor_adquisicion=?4, valor_residual=?5, vida_util_anios=?6, depreciacion_mensual=?7
             WHERE id=?8",
            params![a.descripcion, a.categoria, a.fecha_adquisicion, a.valor_adquisicion,
                a.valor_residual, a.vida_util_anios, mensual, id]
        )?;
        Ok(())
    }

    pub fn eliminar_activo_fijo(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE activos_fijos SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn listar_depreciaciones(&self) -> Result<Vec<Depreciacion>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT d.id, d.activo_id, COALESCE(a.descripcion, ''), d.periodo, d.monto, d.acumulado, d.fecha
             FROM depreciaciones d LEFT JOIN activos_fijos a ON a.id = d.activo_id
             ORDER BY d.periodo DESC, d.id DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(Depreciacion {
            id: r.get(0)?, activo_id: r.get(1)?, activo_descripcion: r.get(2)?, periodo: r.get(3)?,
            monto: r.get(4)?, acumulado: r.get(5)?, fecha: r.get(6)?,
        }))?;
        rows.collect()
    }

    pub fn registrar_depreciacion_mensual(&self, activo_id: i64, periodo: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let (mensual, acumulada): (f64, f64) = conn.query_row(
            "SELECT depreciacion_mensual, depreciacion_acumulada FROM activos_fijos WHERE id=?1",
            params![activo_id], |r| Ok((r.get(0)?, r.get(1)?))
        )?;
        let nuevo_acumulado = acumulada + mensual;
        conn.execute(
            "INSERT INTO depreciaciones (activo_id, periodo, monto, acumulado) VALUES (?1, ?2, ?3, ?4)",
            params![activo_id, periodo, mensual, nuevo_acumulado]
        )?;
        conn.execute(
            "UPDATE activos_fijos SET depreciacion_acumulada=?1 WHERE id=?2",
            params![nuevo_acumulado, activo_id]
        )?;
        Ok(())
    }

    pub fn eliminar_depreciacion(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM depreciaciones WHERE id=?1", params![id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Cierre contable
    // -----------------------------------------------------------------------
    pub fn listar_cierres(&self) -> Result<Vec<CierreContable>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, anio, fecha, ingresos, gastos, utilidad, estado, notas
             FROM cierres_contables ORDER BY anio DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(CierreContable {
            id: r.get(0)?, anio: r.get(1)?, fecha: r.get(2)?, ingresos: r.get(3)?,
            gastos: r.get(4)?, utilidad: r.get(5)?, estado: r.get(6)?, notas: r.get(7)?,
        }))?;
        rows.collect()
    }

    pub fn crear_cierre(&self, c: &CierreContableNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let (ingresos, gastos): (f64, f64) = conn.query_row(
            "SELECT COALESCE((SELECT SUM(total) FROM ventas WHERE substr(fecha,1,4) = ?1), 0),
                    COALESCE((SELECT SUM(monto) FROM gastos WHERE substr(fecha,1,4) = ?1), 0)",
            params![c.anio.to_string()], |r| Ok((r.get(0)?, r.get(1)?))
        )?;
        let utilidad = ingresos - gastos;
        conn.execute(
            "INSERT INTO cierres_contables (anio, ingresos, gastos, utilidad, estado, notas)
             VALUES (?1, ?2, ?3, ?4, 'cerrado', ?5)",
            params![c.anio, ingresos, gastos, utilidad, c.notas]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn eliminar_cierre(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM cierres_contables WHERE id=?1", params![id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Conciliación bancaria
    // -----------------------------------------------------------------------
    pub fn listar_cuentas_bancarias(&self) -> Result<Vec<CuentaBancaria>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, nombre, banco, numero_cuenta, tipo, saldo_inicial, activo
             FROM cuentas_bancarias ORDER BY nombre"
        )?;
        let rows = stmt.query_map([], |r| Ok(CuentaBancaria {
            id: r.get(0)?, nombre: r.get(1)?, banco: r.get(2)?, numero_cuenta: r.get(3)?,
            tipo: r.get(4)?, saldo_inicial: r.get(5)?, activo: r.get(6)?,
        }))?;
        rows.collect()
    }

    pub fn crear_cuenta_bancaria(&self, c: &CuentaBancariaNueva) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO cuentas_bancarias (nombre, banco, numero_cuenta, tipo, saldo_inicial)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![c.nombre, c.banco, c.numero_cuenta, c.tipo, c.saldo_inicial]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_cuenta_bancaria(&self, id: i64, c: &CuentaBancariaNueva) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE cuentas_bancarias SET nombre=?1, banco=?2, numero_cuenta=?3, tipo=?4, saldo_inicial=?5 WHERE id=?6",
            params![c.nombre, c.banco, c.numero_cuenta, c.tipo, c.saldo_inicial, id]
        )?;
        Ok(())
    }

    pub fn eliminar_cuenta_bancaria(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE cuentas_bancarias SET activo=0 WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn listar_movimientos_bancarios(&self, cuenta_id: i64) -> Result<Vec<MovimientoBancario>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT m.id, m.cuenta_id, COALESCE(c.nombre, ''), m.fecha, m.descripcion, m.tipo,
                    m.monto, m.conciliado, m.referencia
             FROM movimientos_bancarios m LEFT JOIN cuentas_bancarias c ON c.id = m.cuenta_id
             WHERE m.cuenta_id=?1 ORDER BY m.fecha DESC, m.id DESC"
        )?;
        let rows = stmt.query_map(params![cuenta_id], |r| Ok(MovimientoBancario {
            id: r.get(0)?, cuenta_id: r.get(1)?, cuenta_nombre: r.get(2)?, fecha: r.get(3)?,
            descripcion: r.get(4)?, tipo: r.get(5)?, monto: r.get(6)?, conciliado: r.get(7)?,
            referencia: r.get(8)?,
        }))?;
        rows.collect()
    }

    pub fn crear_movimiento_bancario(&self, m: &MovimientoBancarioNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO movimientos_bancarios (cuenta_id, fecha, descripcion, tipo, monto, conciliado, referencia)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![m.cuenta_id, m.fecha, m.descripcion, m.tipo, m.monto, m.conciliado, m.referencia]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn actualizar_movimiento_bancario(&self, id: i64, m: &MovimientoBancarioNuevo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE movimientos_bancarios SET cuenta_id=?1, fecha=?2, descripcion=?3, tipo=?4, monto=?5, conciliado=?6, referencia=?7 WHERE id=?8",
            params![m.cuenta_id, m.fecha, m.descripcion, m.tipo, m.monto, m.conciliado, m.referencia, id]
        )?;
        Ok(())
    }

    pub fn eliminar_movimiento_bancario(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM movimientos_bancarios WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn saldo_cuenta_bancaria(&self, cuenta_id: i64) -> Result<f64> {
        let conn = self.conn.lock().unwrap();
        let inicial: f64 = conn.query_row(
            "SELECT saldo_inicial FROM cuentas_bancarias WHERE id=?1",
            params![cuenta_id], |r| r.get(0)
        )?;
        let ingresos: f64 = conn.query_row(
            "SELECT COALESCE(SUM(monto), 0) FROM movimientos_bancarios WHERE cuenta_id=?1 AND tipo='ingreso'",
            params![cuenta_id], |r| r.get(0)
        )?;
        let egresos: f64 = conn.query_row(
            "SELECT COALESCE(SUM(monto), 0) FROM movimientos_bancarios WHERE cuenta_id=?1 AND tipo='egreso'",
            params![cuenta_id], |r| r.get(0)
        )?;
        Ok(inicial + ingresos - egresos)
    }

    // -----------------------------------------------------------------------
    // Arqueo de caja
    // -----------------------------------------------------------------------
    pub fn listar_arqueos(&self) -> Result<Vec<ArqueoCaja>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, fecha, responsable, monto_esperado, monto_real, diferencia, observacion
             FROM arqueos_caja ORDER BY fecha DESC, id DESC"
        )?;
        let rows = stmt.query_map([], |r| Ok(ArqueoCaja {
            id: r.get(0)?, fecha: r.get(1)?, responsable: r.get(2)?, monto_esperado: r.get(3)?,
            monto_real: r.get(4)?, diferencia: r.get(5)?, observacion: r.get(6)?,
        }))?;
        rows.collect()
    }

    pub fn crear_arqueo(&self, a: &ArqueoCajaNuevo) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let diferencia = a.monto_real - a.monto_esperado;
        conn.execute(
            "INSERT INTO arqueos_caja (fecha, responsable, monto_esperado, monto_real, diferencia, observacion)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![a.fecha, a.responsable, a.monto_esperado, a.monto_real, diferencia, a.observacion]
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn eliminar_arqueo(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM arqueos_caja WHERE id=?1", params![id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Libros de compras / ventas y ATS
    // -----------------------------------------------------------------------
    pub fn libro_compras(&self, desde: &str, hasta: &str) -> Result<Vec<LibroComprasLinea>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT numero, proveedor_nombre, date(fecha), subtotal, impuesto, total
             FROM compras
             WHERE estado != 'cancelada'
               AND (?1 = '' OR date(fecha) >= date(?1))
               AND (?2 = '' OR date(fecha) <= date(?2))
             ORDER BY date(fecha), id"
        )?;
        let rows = stmt.query_map(params![desde, hasta], |r| Ok(LibroComprasLinea {
            numero: r.get(0)?, proveedor_nombre: r.get(1)?, fecha: r.get(2)?,
            subtotal: r.get(3)?, iva: r.get(4)?, total: r.get(5)?,
        }))?;
        rows.collect()
    }

    pub fn libro_ventas(&self, desde: &str, hasta: &str) -> Result<Vec<LibroVentasLinea>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT folio, cliente_nombre, date(fecha), subtotal, impuesto, total
             FROM ventas
             WHERE estado != 'cancelada'
               AND (?1 = '' OR date(fecha) >= date(?1))
               AND (?2 = '' OR date(fecha) <= date(?2))
             ORDER BY date(fecha), id"
        )?;
        let rows = stmt.query_map(params![desde, hasta], |r| Ok(LibroVentasLinea {
            folio: r.get(0)?, cliente_nombre: r.get(1)?, fecha: r.get(2)?,
            subtotal: r.get(3)?, iva: r.get(4)?, total: r.get(5)?,
        }))?;
        rows.collect()
    }

    pub fn resumen_ats(&self, desde: &str, hasta: &str) -> Result<ResumenAts> {
        let conn = self.conn.lock().unwrap();
        let r: ResumenAts = conn.query_row(
            "SELECT
                COALESCE((SELECT SUM(total) FROM ventas WHERE estado != 'cancelada' AND (?1 = '' OR date(fecha) >= date(?1)) AND (?2 = '' OR date(fecha) <= date(?2))), 0),
                COALESCE((SELECT SUM(impuesto) FROM ventas WHERE estado != 'cancelada' AND (?1 = '' OR date(fecha) >= date(?1)) AND (?2 = '' OR date(fecha) <= date(?2))), 0),
                COALESCE((SELECT SUM(total) FROM compras WHERE estado != 'cancelada' AND (?1 = '' OR date(fecha) >= date(?1)) AND (?2 = '' OR date(fecha) <= date(?2))), 0),
                COALESCE((SELECT SUM(impuesto) FROM compras WHERE estado != 'cancelada' AND (?1 = '' OR date(fecha) >= date(?1)) AND (?2 = '' OR date(fecha) <= date(?2))), 0),
                COALESCE((SELECT SUM(subtotal) FROM ventas WHERE estado != 'cancelada' AND impuesto = 0 AND (?1 = '' OR date(fecha) >= date(?1)) AND (?2 = '' OR date(fecha) <= date(?2))), 0)",
            params![desde, hasta],
            |r| Ok(ResumenAts {
                ventas: r.get(0)?, iva_ventas: r.get(1)?, compras: r.get(2)?,
                iva_compras: r.get(3)?, ventas_exentas: r.get(4)?,
            })
        )?;
        Ok(r)
    }
}

fn fecha_dias_desde(fecha: &str) -> i64 {
    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let f1 = chrono::NaiveDate::parse_from_str(fecha.get(0..10).unwrap_or(fecha), "%Y-%m-%d").unwrap_or(chrono::Local::now().date_naive());
    let f2 = chrono::NaiveDate::parse_from_str(hoy.get(0..10).unwrap_or(&hoy), "%Y-%m-%d").unwrap_or(chrono::Local::now().date_naive());
    (f2 - f1).num_days().max(0)
}

fn hash_password(contrasena: &str) -> String {
    const SAL: &str = "contab_ec_2026";
    let mut hash: u64 = 0xcbf29ce484222325;
    let data = format!("{}{}", SAL, contrasena);
    for b in data.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}
