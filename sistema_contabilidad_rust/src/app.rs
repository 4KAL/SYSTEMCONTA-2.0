use iced::{Element, Task};
use iced::widget::{button, container, row, text, column, Space};
use iced::{Length, Alignment};
use chrono::Datelike;

use std::collections::HashMap;
use crate::db::DatabaseManager;
use crate::theme::*;
use crate::ui::forms::{SelectOption, confirm_dialog};
use crate::ui::navigation::{NavItem, sidebar_view};
pub use crate::ui::*;
use crate::models::{ClienteNuevo, ProveedorNuevo, ProductoNuevo, VentaNueva, VentaDetalleNuevo, GastoNuevo, UbicacionNueva, MaquinaNueva, GarantiaNueva, CuentaCreditoNueva, Ahorro, AsientoNuevo, AsientoLineaNueva, PagoRecibidoNuevo, PagoRealizadoNuevo, PlanCuentas, CreditoMovimientoNuevo, AhorroMovimientoNuevo, CobroComisionNuevo, DeudaEmpresaNueva, DeudaPagoNuevo, CompraNueva, CompraDetalleNuevo, CotizacionNueva, CotizacionDetalleNuevo, Configuracion, RetencionNueva, EmpleadoNuevo, RolPagoNuevo, ActivoFijoNuevo, CierreContableNuevo, CuentaBancariaNueva, MovimientoBancarioNuevo, ArqueoCajaNuevo};


#[derive(Debug, Clone)]
pub enum Message {
    Navigate(NavItem),
    RefreshDashboard,
    CerrarSolicitado,
    CrearCliente,
    EditarCliente(i64),
    EliminarCliente(i64),
    BuscarClientes(String),
    ClienteFormMsg(clientes::ClienteFormMessage),
    CrearProveedor,
    EditarProveedor(i64),
    EliminarProveedor(i64),
    BuscarProveedores(String),
    ProveedorFormMsg(proveedores::ProveedorFormMessage),
    CrearProducto,
    EditarProducto(i64),
    EliminarProducto(i64),
    BuscarProductos(String),
    ProductoFormMsg(productos::ProductoFormMessage),
    NuevaVenta,
    FiltrarVentaDesde(String),
    FiltrarVentaHasta(String),
    VentaFormMsg(ventas::VentaFormMessage),
    NuevoGasto,
    FiltrarGastoDesde(String),
    FiltrarGastoHasta(String),
    GastoFormMsg(gastos::GastoFormMessage),
    NuevaUbicacion,
    EditarUbicacion(i64),
    EliminarUbicacion(i64),
    UbicacionFormMsg(ubicaciones::UbicacionFormMessage),
    NuevaMaquina,
    EditarMaquina(i64),
    EliminarMaquina(i64),
    MaquinaFormMsg(maquinas::MaquinaFormMessage),
    NuevaCuenta,
    EditarCuenta(i64),
    EliminarCuenta(i64),
    BuscarCuenta(String),
    CuentaFormMsg(plan_cuentas::PlanCuentasFormMessage),
    NuevaGarantia,
    GarantiaFormMsg(garantias::GarantiaFormMessage),
    NuevoCredito,
    CreditoFormMsg(creditos::CreditoFormMessage),
    NuevoAhorro,
    AhorroFormMsg(ahorros::AhorroFormMessage),
    NuevoAsiento,
    FiltrarAsientoDesde(String),
    FiltrarAsientoHasta(String),
    AsientoFormMsg(asientos::AsientoFormMessage),
    EditarVenta(i64),
    EliminarVenta(i64),
    BuscarVenta(String),
    VerDetalleVenta(i64),
    EmitirFactura(i64),
    EmitirXml(i64),
    EmitirGarantia(i64),
    AbonarVenta(i64),
    EditarGasto(i64),
    EliminarGasto(i64),
    BuscarGasto(String),
    EditarGarantia(i64),
    EliminarGarantia(i64),
    BuscarGarantia(String),
    EditarCredito(i64),
    EliminarCredito(i64),
    BuscarCredito(String),
    VerMovimientosCredito(i64),
    CerrarMovimientosCredito,
    NuevoMovimientoCredito,
    CreditoMovFormMsg(creditos::CreditoMovFormMessage),
    EditarAhorro(i64),
    EliminarAhorro(i64),
    BuscarAhorro(String),
    VerMovimientosAhorro(i64),
    CerrarMovimientosAhorro,
    NuevoMovimientoAhorro,
    AhorroMovFormMsg(ahorros::AhorroMovFormMessage),
    EditarAsiento(i64),
    EliminarAsiento(i64),
    BuscarAsiento(String),
    VerDetalleAsiento(i64),
    CerrarDetalleAsiento,
    NuevoPagoRecibido,
    FiltrarPagoRecibidoDesde(String),
    FiltrarPagoRecibidoHasta(String),
    EliminarPagoRecibido(i64),
    BuscarPagoRecibido(String),
    PagoRecibidoFormMsg(pagos_recibidos::PagoRecibidoFormMessage),
    NuevoPagoRealizado,
    FiltrarPagoRealizadoDesde(String),
    FiltrarPagoRealizadoHasta(String),
    EliminarPagoRealizado(i64),
    BuscarPagoRealizado(String),
    PagoRealizadoFormMsg(pagos_realizados::PagoRealizadoFormMessage),
    NuevaDeuda,
    EditarDeuda(i64),
    EliminarDeuda(i64),
    BuscarDeuda(String),
    FiltrarDeudaEstado(String),
    DeudaFormMsg(deudas::DeudaFormMessage),
    VerDetalleDeuda(i64),
    CerrarDetalleDeuda,
    NuevoPagoDeuda,
    EliminarPagoDeuda(i64),
    DeudaPagoFormMsg(deudas::DeudaPagoFormMessage),
    ReportesMsg(reportes::ReportesMessage),
    NuevoCobroComision,
    EliminarCobroComision(i64),
    BuscarCobroComision(String),
    CobroComisionFormMsg(cobro_comisiones::CobroComisionFormMessage),
    NuevaCompra,
    EliminarCompra(i64),
    BuscarCompra(String),
    VerDetalleCompra(i64),
    CompraFormMsg(compras::CompraFormMessage),
    NuevaCotizacion,
    EliminarCotizacion(i64),
    BuscarCotizacion(String),
    FiltrarCotizacionEstado(String),
    VerDetalleCotizacion(i64),
    ConvertirCotizacion(i64),
    CotizacionFormMsg(cotizaciones::CotizacionFormMessage),
    NuevaRetencion,
    EliminarRetencion(i64),
    BuscarRetencion(String),
    ImprimirRetencion(i64),
    RetencionFormMsg(retenciones::RetencionFormMessage),
    NominaTab(nomina::NominaTab),
    NuevoEmpleado,
    EliminarEmpleado(i64),
    EmpleadoFormMsg(nomina::EmpleadoFormMessage),
    NuevoRol,
    EliminarRol(i64),
    MarcarRolPagado(i64),
    RolFormMsg(nomina::RolFormMessage),
    DepreciacionTab(depreciacion::DepreciacionTab),
    NuevoActivo,
    EliminarActivo(i64),
    EliminarDepreciacion(i64),
    DepreciarActivo(i64),
    DepreciacionPeriodo(String),
    ActivoFormMsg(depreciacion::ActivoFormMessage),
    NuevoCierre,
    EliminarCierre(i64),
    CierreFormMsg(cierre_contable::CierreFormMessage),
    ConciliacionTab(conciliacion::ConciliacionTab),
    NuevaCuentaBancaria,
    EliminarCuentaBancaria(i64),
    CuentaBancariaFormMsg(conciliacion::CuentaFormMessage),
    SeleccionarCuentaBancaria(i64),
    NuevoMovimientoBancario,
    EliminarMovimientoBancario(i64),
    ToggleConciliado(i64),
    MovimientoBancarioFormMsg(conciliacion::MovimientoFormMessage),
    NuevoArqueo,
    EliminarArqueo(i64),
    BuscarArqueo(String),
    ArqueoFormMsg(caja_chica::ArqueoFormMessage),
    ConfiguracionMsg(configuracion::ConfiguracionMessage),
    SetupMsg(primera_vez::SetupMessage),
    LoginMsg(login::LoginMessage),
    CerrarSesion,
    ConectarCelular,
    CelularListo(String, String),
    CelularError(String),
    CerrarDialogoCelular,
    FinPresentacion,
    ConfirmarSi,
    ConfirmarNo,
    LimpiarNotificacion,
}

#[derive(Debug, Clone)]
pub enum ConfirmTarget { Cliente, Proveedor, Producto, Venta, Gasto, Ubicacion, Maquina, Cuenta, Garantia, Credito, Ahorro, Asiento, PagoRecibido, PagoRealizado, Comision, Deuda, DeudaPago, Compra, Cotizacion, Retencion, Empleado, Rol, Activo, Depreciacion, Cierre, CuentaBancaria, MovimientoBancario, Arqueo }

#[derive(Debug, Clone, PartialEq)]
pub enum Fase { Presentacion, Instalacion, Login, Principal }

pub struct App {
    db: DatabaseManager,
    nav: NavItem,
    pub notificacion: Option<(String, iced::Color)>,
    pub confirmar: Option<(i64, ConfirmTarget)>,
    pub dashboard: dashboard::DashboardData,
    pub clientes: clientes::ClientesState,
    pub proveedores: proveedores::ProveedoresState,
    pub productos: productos::ProductosState,
    pub ventas: ventas::VentasState,
    pub gastos: gastos::GastosState,
    pub plan_cuentas: plan_cuentas::PlanCuentasState,
    pub ubicaciones: ubicaciones::UbicacionesState,
    pub maquinas: maquinas::MaquinasState,
    pub garantias: garantias::GarantiasState,
    pub creditos: creditos::CreditosState,
    pub ahorros: ahorros::AhorrosState,
    pub asientos: asientos::AsientosState,
    pub pagos_recibidos: pagos_recibidos::PagosRecibidosState,
    pub pagos_realizados: pagos_realizados::PagosRealizadosState,
    pub deudas: deudas::DeudasState,
    pub reportes: reportes::ReportesState,
    pub cobro_comisiones: cobro_comisiones::CobroComisionesState,
    pub compras: compras::ComprasState,
    pub cotizaciones: cotizaciones::CotizacionesState,
    pub retenciones: retenciones::RetencionesState,
    pub nomina: nomina::NominaState,
    pub depreciacion: depreciacion::DepreciacionState,
    pub cierre_contable: cierre_contable::CierreState,
    pub conciliacion: conciliacion::ConciliacionState,
    pub caja_chica: caja_chica::CajaChicaState,
    pub configuracion: configuracion::ConfiguracionState,
    pub setup: primera_vez::SetupState,
    pub login: login::LoginState,
    pub celular: celular::CelularState,
    pub fase: Fase,
    pub usuario_actual: String,
    pub empresa: Configuracion,
}

impl Default for App {
    fn default() -> Self {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        let db_path = exe_dir.join("..").join("contabilidad_rust.db");
        if !db_path.exists() {
            match crate::db::migrar::auto_migrar(&db_path.to_string_lossy()) {
                Ok(Some(resultado)) => {
                    let _ = std::fs::write(exe_dir.join("migracion_reporte.txt"), &resultado.mensaje);
                    println!("Migración automática completada: {} filas migradas.", resultado.filas_migradas);
                }
                Ok(None) => {}
                Err(e) => {
                    let _ = std::fs::write(exe_dir.join("migracion_reporte.txt"), format!("Error en migración automática: {}\n", e));
                    println!("Error en migración automática: {}", e);
                }
            }
        }
        let db = DatabaseManager::new(&db_path.to_string_lossy()).expect("Error al conectar con la base de datos");
        let mut app = Self {
            db,
            nav: NavItem::Dashboard,
            notificacion: None,
            confirmar: None,
            dashboard: dashboard::DashboardData::default(),
            clientes: clientes::ClientesState::default(),
            proveedores: proveedores::ProveedoresState::default(),
            productos: productos::ProductosState::default(),
            ventas: ventas::VentasState::default(),
            gastos: gastos::GastosState::default(),
            plan_cuentas: plan_cuentas::PlanCuentasState::default(),
            ubicaciones: ubicaciones::UbicacionesState::default(),
            maquinas: maquinas::MaquinasState::default(),
            garantias: garantias::GarantiasState::default(),
            creditos: creditos::CreditosState::default(),
            ahorros: ahorros::AhorrosState::default(),
            asientos: asientos::AsientosState::default(),
            pagos_recibidos: pagos_recibidos::PagosRecibidosState::default(),
            pagos_realizados: pagos_realizados::PagosRealizadosState::default(),
            deudas: deudas::DeudasState::default(),
            reportes: reportes::ReportesState::default(),
            cobro_comisiones: cobro_comisiones::CobroComisionesState::default(),
            compras: compras::ComprasState::default(),
            cotizaciones: cotizaciones::CotizacionesState::default(),
            retenciones: retenciones::RetencionesState::default(),
            nomina: nomina::NominaState::default(),
            depreciacion: depreciacion::DepreciacionState::default(),
            cierre_contable: cierre_contable::CierreState::default(),
            conciliacion: conciliacion::ConciliacionState::default(),
            caja_chica: caja_chica::CajaChicaState::default(),
            configuracion: configuracion::ConfiguracionState::default(),
            setup: primera_vez::SetupState::default(),
            login: login::LoginState::default(),
            celular: celular::CelularState::default(),
            fase: Fase::Instalacion,
            usuario_actual: String::new(),
            empresa: Configuracion::default(),
        };
        app.fase = if app.db.hay_usuarios().unwrap_or(false) { Fase::Login } else { Fase::Presentacion };
        app.empresa = app.db.obtener_configuracion().unwrap_or_default();
        app.configuracion.form = configuracion::ConfiguracionFormData {
            empresa_nombre: app.empresa.empresa_nombre.clone(),
            ruc: app.empresa.ruc.clone(),
            direccion: app.empresa.direccion.clone(),
            telefono: app.empresa.telefono.clone(),
            email: app.empresa.email.clone(),
            ciudad: app.empresa.ciudad.clone(),
            iva: app.empresa.iva.to_string(),
        };
        if app.fase == Fase::Principal {
            app.load_current_section();
        }
        app.hacer_respaldo_automatico();
        app
    }
}

impl App {
    fn load_dashboard(&mut self) {
        self.dashboard.ventas_hoy = self.db.kpi_ventas_hoy().unwrap_or(0.0);
        self.dashboard.gastos_hoy = self.db.kpi_gastos_hoy().unwrap_or(0.0);
        self.dashboard.cxc = self.db.kpi_cxc().unwrap_or(0.0);
        self.dashboard.cxp = self.db.kpi_cxp().unwrap_or(0.0);
        self.dashboard.utilidad_mes = self.db.kpi_utilidad_mes().unwrap_or(0.0);
        self.dashboard.ventas_mes = self.db.kpi_ventas_mes().unwrap_or(0.0);
        self.dashboard.gastos_mes = self.db.kpi_gastos_mes().unwrap_or(0.0);
        self.dashboard.ventas_anio = self.db.kpi_ventas_anio().unwrap_or(0.0);
        self.dashboard.gastos_anio = self.db.kpi_gastos_anio().unwrap_or(0.0);
        self.dashboard.utilidad_anio = self.db.kpi_utilidad_anio().unwrap_or(0.0);
        self.dashboard.total_clientes = self.db.kpi_clientes().unwrap_or(0);
        self.dashboard.ventas_mensuales = self.db.ventas_por_mes().unwrap_or_default();
        self.dashboard.gastos_categorias = self.db.gastos_por_categoria().unwrap_or_default();
        self.dashboard.actividad = self.db.actividad_reciente().unwrap_or_default();
        self.dashboard.alertas_stock = self.db.alertas_stock().unwrap_or_default().iter().map(|p| (p.nombre.clone(), p.stock)).collect();
        self.dashboard.alertas_creditos = self.db.alertas_creditos_vencidos().unwrap_or_default().iter().map(|(c, s)| (c.nombre.clone(), *s)).collect();
        let ahora = chrono::Local::now();
        let periodo = ahora.format("%Y-%m").to_string();
        self.dashboard.alertas_cobros = self.db.cobros_pendientes_mes(&periodo, ahora.day() as i32).unwrap_or_default()
            .iter().map(|m| (
                m.codigo.clone().map(|c| format!("{} · {}", c, m.descripcion)).unwrap_or_else(|| m.descripcion.clone()),
                if m.ubicacion_texto.is_empty() { String::from("") } else { m.ubicacion_texto.clone() },
                m.dia_cobro, m.comision_estimada,
            )).collect();
    }

    fn load_clientes(&mut self) { self.clientes.clientes = self.db.listar_clientes().unwrap_or_default(); }
    fn load_proveedores(&mut self) { self.proveedores.proveedores = self.db.listar_proveedores().unwrap_or_default(); }
    fn load_productos(&mut self) { self.productos.productos = self.db.listar_productos().unwrap_or_default(); }
    fn load_ventas(&mut self) { self.ventas.ventas = self.db.listar_ventas().unwrap_or_default(); }
    fn load_gastos(&mut self) { self.gastos.gastos = self.db.listar_gastos().unwrap_or_default(); }
    fn load_plan_cuentas(&mut self) { self.plan_cuentas.cuentas = self.db.listar_plan_cuentas().unwrap_or_default(); }

    fn open_cuenta_form(&mut self, editing_id: Option<i64>) {
        self.plan_cuentas.errores = HashMap::new();
        if let Some(id) = editing_id {
            if let Some(c) = self.plan_cuentas.cuentas.iter().find(|c| c.id == id) {
                self.plan_cuentas.form = plan_cuentas::PlanCuentasFormData {
                    codigo: c.codigo.clone(), nombre: c.nombre.clone(), tipo: c.tipo.clone(),
                    naturaleza: c.naturaleza.clone(), nivel: c.nivel.to_string(),
                    padre_id: c.padre_id.map(|x| x.to_string()).unwrap_or_default(), activo: c.activo,
                };
            }
        } else { self.plan_cuentas.form = plan_cuentas::PlanCuentasFormData::default(); }
        self.plan_cuentas.editing_id = editing_id;
        self.plan_cuentas.show_form = true;
    }

    fn save_cuenta_form(&mut self) {
        self.plan_cuentas.errores = HashMap::new();
        if self.plan_cuentas.form.codigo.trim().is_empty() {
            self.plan_cuentas.errores.insert("codigo".to_string(), "El código es obligatorio".to_string());
            return;
        }
        if self.plan_cuentas.form.nombre.trim().is_empty() {
            self.plan_cuentas.errores.insert("nombre".to_string(), "El nombre es obligatorio".to_string());
            return;
        }
        let f = &self.plan_cuentas.form;
        let nuevo = PlanCuentas {
            id: 0, codigo: f.codigo.clone(), nombre: f.nombre.clone(), tipo: f.tipo.clone(),
            naturaleza: f.naturaleza.clone(), nivel: f.nivel.parse().unwrap_or(1),
            padre_id: Some(f.padre_id.parse().unwrap_or(0)).filter(|&x| x > 0), activo: f.activo,
        };
        let r = if let Some(id) = self.plan_cuentas.editing_id { self.db.actualizar_cuenta(id, &nuevo).map(|_| ()) } else { self.db.crear_cuenta(&nuevo).map(|_| ()) };
        if r.is_ok() { self.plan_cuentas.show_form = false; self.load_plan_cuentas(); self.notificacion = Some(("Cuenta guardada correctamente".to_string(), COLOR_SUCCESS)); }
    }
    fn load_ubicaciones(&mut self) { self.ubicaciones.ubicaciones = self.db.listar_ubicaciones().unwrap_or_default(); }
    fn load_maquinas(&mut self) { self.maquinas.maquinas = self.db.listar_maquinas().unwrap_or_default(); }
    fn load_garantias(&mut self) { self.garantias.garantias = self.db.listar_garantias().unwrap_or_default(); }
    fn load_creditos(&mut self) { self.creditos.cuentas = self.db.listar_cuentas_credito().unwrap_or_default(); }
    fn load_ahorros(&mut self) { self.ahorros.ahorros = self.db.listar_ahorros().unwrap_or_default(); }
    fn load_asientos(&mut self) { self.asientos.asientos = self.db.listar_asientos().unwrap_or_default(); }
    fn load_pagos_recibidos(&mut self) { self.pagos_recibidos.pagos = self.db.listar_pagos_recibidos().unwrap_or_default(); }
    fn load_pagos_realizados(&mut self) { self.pagos_realizados.pagos = self.db.listar_pagos_realizados().unwrap_or_default(); }
    fn load_deudas(&mut self) {
        self.deudas.deudas = self.db.listar_deudas_empresa().unwrap_or_default();
        self.deudas.pagos = self.db.listar_todos_deuda_pagos().unwrap_or_default();
    }

    fn load_reportes_libro_diario(&mut self) { self.reportes.libro_diario = self.db.reporte_libro_diario(&self.reportes.desde, &self.reportes.hasta).unwrap_or_default(); }

    fn load_reportes_balance(&mut self) { let (b, d, h) = self.db.reporte_balance_general().unwrap_or_default(); self.reportes.balance = b; self.reportes.balance_debe = d; self.reportes.balance_haber = h; }

    fn load_reportes_balance_resumen(&mut self) { self.reportes.balance_resumen = self.db.reporte_balance_resumen().ok(); }

    fn load_reportes_resultados(&mut self) { self.reportes.resultado = self.db.reporte_estado_resultados(&self.reportes.desde, &self.reportes.hasta).ok(); }

    fn load_reportes_mayor(&mut self) {
        if self.reportes.mayor_cuenta > 0 {
            self.reportes.mayor = self.db.reporte_libro_mayor(self.reportes.mayor_cuenta, &self.reportes.desde, &self.reportes.hasta).unwrap_or_default();
        } else { self.reportes.mayor = Vec::new(); }
    }

    fn load_reportes_antiguedad(&mut self) { let (cxc, cxp) = self.db.reporte_antiguedad_saldos().unwrap_or_default(); self.reportes.antiguedad_cxc = cxc; self.reportes.antiguedad_cxp = cxp; }

    fn load_reportes_libro_compras(&mut self) { self.reportes.libro_compras = self.db.libro_compras(&self.reportes.desde, &self.reportes.hasta).unwrap_or_default(); }

    fn load_reportes_libro_ventas(&mut self) { self.reportes.libro_ventas = self.db.libro_ventas(&self.reportes.desde, &self.reportes.hasta).unwrap_or_default(); }

    fn load_reportes_ats(&mut self) { self.reportes.resumen_ats = self.db.resumen_ats(&self.reportes.desde, &self.reportes.hasta).unwrap_or_default(); }

    fn exportar_ats_csv(&mut self) {
        let dir = std::env::current_exe().ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("Documentos");
        let _ = std::fs::create_dir_all(&dir);
        let nombre = format!("ats_libros_{}.csv", chrono::Local::now().format("%Y%m%d_%H%M%S"));
        let mut contenido = String::new();
        contenido.push_str("=== LIBRO DE VENTAS ===\n");
        contenido.push_str("Folio;Cliente;Fecha;Subtotal;IVA;Total\n");
        for v in &self.reportes.libro_ventas {
            contenido.push_str(&format!("{};{};{};{:.2};{:.2};{:.2}\n", v.folio, v.cliente_nombre.replace(';', ","), v.fecha, v.subtotal, v.iva, v.total));
        }
        contenido.push_str("\n=== LIBRO DE COMPRAS ===\n");
        contenido.push_str("Numero;Proveedor;Fecha;Subtotal;IVA;Total\n");
        for c in &self.reportes.libro_compras {
            contenido.push_str(&format!("{};{};{};{:.2};{:.2};{:.2}\n", c.numero, c.proveedor_nombre.replace(';', ","), c.fecha, c.subtotal, c.iva, c.total));
        }
        let r = self.reportes.resumen_ats.clone();
        contenido.push_str(&format!("\n=== RESUMEN ATS ===\nVentas;{:.2}\nIVA Ventas;{:.2}\nCompras;{:.2}\nIVA Compras;{:.2}\n", r.ventas, r.iva_ventas, r.compras, r.iva_compras));
        match std::fs::write(dir.join(&nombre), contenido) {
            Ok(_) => { self.notificacion = Some((format!("ATS exportado: Documentos/{}", nombre), COLOR_SUCCESS)); }
            Err(e) => { self.notificacion = Some((format!("Error al exportar ATS: {}", e), COLOR_DANGER)); }
        }
    }

    fn load_cobro_comisiones(&mut self) { self.cobro_comisiones.comisiones = self.db.listar_todas_comisiones().unwrap_or_default(); }

    fn load_compras(&mut self) {
        self.compras.compras = self.db.listar_compras().unwrap_or_default();
        self.compras.movimientos = self.db.listar_movimientos_inventario().unwrap_or_default();
    }

    fn load_cotizaciones(&mut self) { self.cotizaciones.cotizaciones = self.db.listar_cotizaciones().unwrap_or_default(); }

    fn load_retenciones(&mut self) { self.retenciones.retenciones = self.db.listar_retenciones().unwrap_or_default(); }

    fn load_nomina(&mut self) {
        self.nomina.empleados = self.db.listar_empleados().unwrap_or_default();
        self.nomina.roles = self.db.listar_roles_pago().unwrap_or_default();
    }

    fn load_depreciacion(&mut self) {
        self.depreciacion.activos = self.db.listar_activos_fijos().unwrap_or_default();
        self.depreciacion.historial = self.db.listar_depreciaciones().unwrap_or_default();
    }

    fn load_cierres(&mut self) { self.cierre_contable.cierres = self.db.listar_cierres().unwrap_or_default(); }

    fn load_conciliacion(&mut self) {
        self.conciliacion.cuentas = self.db.listar_cuentas_bancarias().unwrap_or_default();
        if self.conciliacion.cuenta_seleccionada > 0 {
            self.conciliacion.movimientos = self.db.listar_movimientos_bancarios(self.conciliacion.cuenta_seleccionada).unwrap_or_default();
            self.conciliacion.saldo_actual = self.db.saldo_cuenta_bancaria(self.conciliacion.cuenta_seleccionada).unwrap_or(0.0);
        } else {
            self.conciliacion.movimientos = Vec::new();
            self.conciliacion.saldo_actual = 0.0;
        }
    }

    fn load_arqueos(&mut self) { self.caja_chica.arqueos = self.db.listar_arqueos().unwrap_or_default(); }

    fn load_configuracion(&mut self) {
        self.empresa = self.db.obtener_configuracion().unwrap_or_default();
        self.configuracion.form = configuracion::ConfiguracionFormData {
            empresa_nombre: self.empresa.empresa_nombre.clone(),
            ruc: self.empresa.ruc.clone(),
            direccion: self.empresa.direccion.clone(),
            telefono: self.empresa.telefono.clone(),
            email: self.empresa.email.clone(),
            ciudad: self.empresa.ciudad.clone(),
            iva: self.empresa.iva.to_string(),
        };
    }

    fn save_setup(&mut self) {
        self.setup.errores = HashMap::new();
        self.setup.mensaje = String::new();
        let f = self.setup.form.clone();
        if f.empresa_nombre.trim().is_empty() {
            self.setup.errores.insert("empresa".into(), "El nombre de la empresa es obligatorio".into());
        }
        if f.ruc.trim().is_empty() {
            self.setup.errores.insert("ruc".into(), "El RUC es obligatorio".into());
        }
        if f.telefono.trim().is_empty() {
            self.setup.errores.insert("telefono".into(), "El teléfono es obligatorio".into());
        }
        if f.usuario.trim().is_empty() {
            self.setup.errores.insert("usuario".into(), "Cree un nombre de usuario".into());
        }
        if f.contrasena.is_empty() {
            self.setup.errores.insert("contrasena".into(), "La contraseña es obligatoria".into());
        }
        if f.contrasena != f.confirmar {
            self.setup.errores.insert("confirmar".into(), "Las contraseñas no coinciden".into());
        }
        if !self.setup.errores.is_empty() {
            return;
        }
        let cfg = Configuracion {
            empresa_nombre: f.empresa_nombre.trim().to_string(),
            ruc: f.ruc.trim().to_string(),
            direccion: f.direccion.trim().to_string(),
            telefono: f.telefono.trim().to_string(),
            email: f.email.trim().to_string(),
            ciudad: f.ciudad.trim().to_string(),
            iva: f.iva.parse().unwrap_or(15.0),
        };
        match self.db.guardar_configuracion(&cfg) {
            Ok(_) => match self.db.crear_usuario(&f.usuario, &f.contrasena) {
                Ok(_) => {
                    self.empresa = cfg;
                    self.fase = Fase::Login;
                    self.setup.mensaje = "Sistema configurado correctamente. Inicie sesión con su usuario.".to_string();
                    self.setup.es_error = false;
                    self.load_configuracion();
                }
                Err(e) => {
                    self.setup.mensaje = format!("Error al crear el usuario: {}", e);
                    self.setup.es_error = true;
                }
            },
            Err(e) => {
                self.setup.mensaje = format!("Error al guardar la configuración: {}", e);
                self.setup.es_error = true;
            }
        }
    }

    fn do_login(&mut self) {
        self.login.mensaje = String::new();
        if self.login.usuario.trim().is_empty() || self.login.contrasena.is_empty() {
            self.login.mensaje = "Ingrese su usuario y contraseña".into();
            self.login.es_error = true;
            return;
        }
        match self.db.verificar_usuario(&self.login.usuario, &self.login.contrasena) {
            Ok(Some(u)) => {
                self.usuario_actual = u.nombre_usuario.clone();
                self.login.contrasena = String::new();
                self.login.mensaje = String::new();
                self.login.es_error = false;
                self.fase = Fase::Principal;
                self.nav = NavItem::Dashboard;
                self.load_current_section();
                self.notificar_cobros_comisiones(u.nombre_usuario.clone());
            }
            Ok(None) => {
                self.login.mensaje = "Usuario o contraseña incorrectos".into();
                self.login.es_error = true;
            }
            Err(e) => {
                self.login.mensaje = format!("Error al iniciar sesión: {}", e);
                self.login.es_error = true;
            }
        }
    }

    fn conectar_celular(&mut self) -> Task<Message> {
        use std::os::windows::process::CommandExt;
        use iced::futures::SinkExt;

        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        let db_path = exe_dir.join("..").join("contabilidad_rust.db");
        let server_exe = exe_dir.join("server.exe");
        let cloudflared_exe = exe_dir.join("cloudflared.exe");
        let log_path = exe_dir.join("cloudflared.log");

        if !server_exe.exists() {
            self.notificacion = Some(("No se encontró server.exe junto al ejecutable".to_string(), COLOR_DANGER));
            return Task::none();
        }
        if !cloudflared_exe.exists() {
            self.notificacion = Some(("No se encontró cloudflared.exe junto al ejecutable".to_string(), COLOR_DANGER));
            return Task::none();
        }

        self.celular.activo = true;
        self.celular.mensaje = "Iniciando servidor y túnel...".to_string();
        self.celular.url = None;
        self.celular.qr = None;
        self.celular.qr_size = 0;

        let token = self.db.crear_qr_token(&self.usuario_actual, 300).unwrap_or_default();
        if token.is_empty() {
            self.celular.activo = false;
            self.notificacion = Some(("No se pudo crear el token de acceso".to_string(), COLOR_DANGER));
            return Task::none();
        }

        if !App::puerto_ocupado(8095) {
            let _ = std::process::Command::new(&server_exe)
                .arg(db_path.to_string_lossy().to_string())
                .arg("8095")
                .creation_flags(0x08000000)
                .spawn();
        }

        let _ = std::fs::remove_file(&log_path);
        if let Ok(log_file) = std::fs::File::create(&log_path) {
            let mut cmd = std::process::Command::new(&cloudflared_exe);
            cmd.arg("tunnel")
               .arg("--url")
               .arg("http://localhost:8095")
               .stdout(std::process::Stdio::null())
               .stderr(std::process::Stdio::from(log_file))
               .creation_flags(0x08000000);
            let _ = cmd.spawn();
        }

        let log_path_t = log_path.clone();
        let token_t = token.clone();
        Task::run(
            iced::stream::channel(1, move |mut sender: iced::futures::channel::mpsc::Sender<Message>| async move {
                let mut encontrada: Option<String> = None;
                for _ in 0..60 {
                    if let Ok(content) = std::fs::read_to_string(&log_path_t) {
                        if let Some(u) = celular::extraer_url_cloudflared(&content) {
                            encontrada = Some(u);
                            break;
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
                match encontrada {
                    Some(u) => {
                        let payload = serde_json::json!({"url": u, "qr": token_t}).to_string();
                        let _ = sender.send(Message::CelularListo(u, payload)).await;
                    }
                    None => {
                        match Self::obtener_ip_lan() {
                            Some(ip) => {
                                let url_fallback = format!("http://{}:8095", ip);
                                let payload = serde_json::json!({"url": url_fallback.clone(), "qr": token_t}).to_string();
                                let _ = sender.send(Message::CelularListo(url_fallback, payload)).await;
                            }
                            None => {
                                let _ = sender.send(Message::CelularError(
                                    "No se obtuvo la URL del túnel ni la IP local. Conecte el celular a la misma Wi-Fi de la PC o revise su conexión a internet.".to_string(),
                                )).await;
                            }
                        }
                    }
                }
            }),
            |msg| msg,
        )
    }

    fn puerto_ocupado(port: u16) -> bool {
        std::net::TcpStream::connect(("127.0.0.1", port)).is_ok()
    }

    fn obtener_ip_lan() -> Option<String> {
        use std::process::Command;
        let out = Command::new("cmd").args(["/C", "ipconfig"]).output().ok()?;
        let txt = String::from_utf8_lossy(&out.stdout);
        for line in txt.lines() {
            let l = line.trim();
            let lower = l.to_lowercase();
            if l.contains("IPv4") || lower.contains("dirección ipv4") || lower.contains("dir. ipv4") {
                if let Some(idx) = l.rfind(':') {
                    let ip = l[idx + 1..].trim();
                    if !ip.is_empty() && ip != "127.0.0.1" {
                        return Some(ip.to_string());
                    }
                }
            }
        }
        None
    }

    fn load_current_section(&mut self) {
        match self.nav {
            NavItem::Dashboard => self.load_dashboard(),
            NavItem::Clientes => self.load_clientes(),
            NavItem::Proveedores => self.load_proveedores(),
            NavItem::Productos => self.load_productos(),
            NavItem::Ventas => self.load_ventas(),
            NavItem::Gastos => self.load_gastos(),
            NavItem::PlanCuentas => self.load_plan_cuentas(),
            NavItem::Ubicaciones => self.load_ubicaciones(),
            NavItem::Maquinas => self.load_maquinas(),
            NavItem::Garantias => self.load_garantias(),
            NavItem::Creditos => self.load_creditos(),
            NavItem::Ahorros => self.load_ahorros(),
            NavItem::Asientos => self.load_asientos(),
            NavItem::PagosRecibidos => self.load_pagos_recibidos(),
            NavItem::PagosRealizados => self.load_pagos_realizados(),
            NavItem::DeudasEmpresa => self.load_deudas(),
            NavItem::Reportes => match self.reportes.tab {
                reportes::ReporteTab::LibroDiario => self.load_reportes_libro_diario(),
                reportes::ReporteTab::BalanceGeneral => self.load_reportes_balance_resumen(),
                reportes::ReporteTab::Comprobacion => self.load_reportes_balance(),
                reportes::ReporteTab::EstadoResultados => self.load_reportes_resultados(),
                reportes::ReporteTab::LibroMayor => self.load_reportes_mayor(),
                reportes::ReporteTab::Antiguedad => self.load_reportes_antiguedad(),
                reportes::ReporteTab::LibroCompras => self.load_reportes_libro_compras(),
                reportes::ReporteTab::LibroVentas => self.load_reportes_libro_ventas(),
                reportes::ReporteTab::Ats => self.load_reportes_ats(),
            },
            NavItem::CobroComisiones => self.load_cobro_comisiones(),
            NavItem::Compras => self.load_compras(),
            NavItem::Cotizaciones => self.load_cotizaciones(),
            NavItem::Retenciones => self.load_retenciones(),
            NavItem::Nomina => self.load_nomina(),
            NavItem::Depreciacion => self.load_depreciacion(),
            NavItem::CierreContable => self.load_cierres(),
            NavItem::Conciliacion => self.load_conciliacion(),
            NavItem::CajaChica => self.load_arqueos(),
            NavItem::Configuracion => self.load_configuracion(),
        }
    }

    // ---- Clientes ----
    fn open_cliente_form(&mut self, editing_id: Option<i64>) {
        self.clientes.errores = HashMap::new();
        if let Some(id) = editing_id {
            if let Some(c) = self.clientes.clientes.iter().find(|c| c.id == id) {
                self.clientes.form = clientes::ClienteFormData {
                    codigo: c.codigo.clone().unwrap_or_default(), nombre: c.nombre.clone(), rfc: c.rfc.clone(),
                    email: c.email.clone(), telefono: c.telefono.clone(), direccion: c.direccion.clone(),
                    ciudad: c.ciudad.clone(), limite_credito: c.limite_credito.to_string(),
                };
            }
        } else { self.clientes.form = clientes::ClienteFormData::default(); }
        self.clientes.editing_id = editing_id;
        self.clientes.show_form = true;
    }

    fn save_cliente_form(&mut self) {
        self.clientes.errores = HashMap::new();
        if self.clientes.form.nombre.trim().is_empty() {
            self.clientes.errores.insert("nombre".to_string(), "El nombre es obligatorio".to_string());
            return;
        }
        let f = &self.clientes.form;
        let nuevo = ClienteNuevo {
            codigo: Some(f.codigo.clone()).filter(|s| !s.is_empty()),
            nombre: f.nombre.clone(), rfc: f.rfc.clone(),
            email: f.email.clone(), telefono: f.telefono.clone(), direccion: f.direccion.clone(),
            ciudad: f.ciudad.clone(), limite_credito: f.limite_credito.parse().unwrap_or(0.0),
        };
        let r = if let Some(id) = self.clientes.editing_id { self.db.actualizar_cliente(id, &nuevo).map(|_| ()) } else { self.db.crear_cliente(&nuevo).map(|_| ()) };
        if r.is_ok() { self.clientes.show_form = false; self.load_clientes(); self.notificacion = Some(("Cliente guardado correctamente".to_string(), COLOR_SUCCESS)); }
    }

    // ---- Proveedores ----
    fn open_proveedor_form(&mut self, editing_id: Option<i64>) {
        self.proveedores.errores = HashMap::new();
        if let Some(id) = editing_id {
            if let Some(p) = self.proveedores.proveedores.iter().find(|p| p.id == id) {
                self.proveedores.form = proveedores::ProveedorFormData {
                    codigo: p.codigo.clone().unwrap_or_default(), nombre: p.nombre.clone(), contacto: p.contacto.clone(),
                    rfc: p.rfc.clone(), email: p.email.clone(), telefono: p.telefono.clone(), direccion: p.direccion.clone(),
                };
            }
        } else { self.proveedores.form = proveedores::ProveedorFormData::default(); }
        self.proveedores.editing_id = editing_id;
        self.proveedores.show_form = true;
    }

    fn save_proveedor_form(&mut self) {
        self.proveedores.errores = HashMap::new();
        if self.proveedores.form.nombre.trim().is_empty() {
            self.proveedores.errores.insert("nombre".to_string(), "El nombre es obligatorio".to_string());
            return;
        }
        let f = &self.proveedores.form;
        let nuevo = ProveedorNuevo {
            codigo: Some(f.codigo.clone()).filter(|s| !s.is_empty()),
            nombre: f.nombre.clone(), contacto: f.contacto.clone(),
            rfc: f.rfc.clone(), email: f.email.clone(), telefono: f.telefono.clone(),
            direccion: f.direccion.clone(), ciudad: String::new(),
        };
        let r = if let Some(id) = self.proveedores.editing_id { self.db.actualizar_proveedor(id, &nuevo).map(|_| ()) } else { self.db.crear_proveedor(&nuevo).map(|_| ()) };
        if r.is_ok() { self.proveedores.show_form = false; self.load_proveedores(); self.notificacion = Some(("Proveedor guardado correctamente".to_string(), COLOR_SUCCESS)); }
    }

    // ---- Productos ----
    fn open_producto_form(&mut self, editing_id: Option<i64>) {
        self.productos.errores = HashMap::new();
        if let Some(id) = editing_id {
            if let Some(p) = self.productos.productos.iter().find(|p| p.id == id) {
                self.productos.form = productos::ProductoFormData {
                    codigo: p.codigo.clone().unwrap_or_default(), nombre: p.nombre.clone(), descripcion: p.descripcion.clone(),
                    precio_compra: p.precio_compra.to_string(), precio_venta: p.precio_venta.to_string(),
                    stock: p.stock.to_string(), stock_minimo: p.stock_minimo.to_string(), unidad: p.unidad.clone(),
                };
            }
        } else { self.productos.form = productos::ProductoFormData::default(); }
        self.productos.editing_id = editing_id;
        self.productos.show_form = true;
    }

    fn save_producto_form(&mut self) {
        self.productos.errores = HashMap::new();
        if self.productos.form.nombre.trim().is_empty() {
            self.productos.errores.insert("nombre".to_string(), "El nombre es obligatorio".to_string());
            return;
        }
        let f = &self.productos.form;
        let nuevo = ProductoNuevo {
            codigo: Some(f.codigo.clone()).filter(|s| !s.is_empty()),
            nombre: f.nombre.clone(), descripcion: f.descripcion.clone(),
            categoria_id: 1, precio_compra: f.precio_compra.parse().unwrap_or(0.0),
            precio_venta: f.precio_venta.parse().unwrap_or(0.0), stock: f.stock.parse().unwrap_or(0),
            stock_minimo: f.stock_minimo.parse().unwrap_or(0), unidad: f.unidad.clone(),
        };
        let r = if let Some(id) = self.productos.editing_id { self.db.actualizar_producto(id, &nuevo).map(|_| ()) } else { self.db.crear_producto(&nuevo).map(|_| ()) };
        if r.is_ok() { self.productos.show_form = false; self.load_productos(); self.notificacion = Some(("Producto guardado correctamente".to_string(), COLOR_SUCCESS)); }
    }

    // ---- Ventas ----
    fn cargar_opciones_venta(&mut self) {
        self.ventas.opciones_productos = self.db.listar_productos().unwrap_or_default().iter()
            .map(|p| SelectOption { id: p.id, label: p.nombre.clone() }).collect();
        let mut clientes: Vec<SelectOption> = self.db.listar_clientes().unwrap_or_default().iter()
            .map(|c| SelectOption { id: c.id, label: c.nombre.clone() }).collect();
        clientes.push(SelectOption { id: 0, label: "\u{2014} Nuevo Cliente \u{2014}".to_string() });
        self.ventas.opciones_clientes = clientes;
    }

    fn open_venta_form(&mut self) {
        self.ventas.editing_id = None;
        self.ventas.form = ventas::VentaFormData::default();
        self.ventas.form.iva = self.db.obtener_iva().to_string();
        self.cargar_opciones_venta();
        self.ventas.show_form = true;
    }

    fn open_editar_venta(&mut self, id: i64) {
        self.cargar_opciones_venta();
        if let Some(v) = self.ventas.ventas.iter().find(|v| v.id == id) {
            let items = self.db.obtener_detalles_venta(id).unwrap_or_default().iter().map(|d| ventas::VentaItemData {
                producto_id: d.producto_id,
                producto_nombre: d.producto_nombre.clone(),
                cantidad: d.cantidad.to_string(),
                precio: d.precio_unitario.to_string(),
            }).collect();
            self.ventas.form = ventas::VentaFormData {
                cliente_id: v.cliente_id, cliente_nombre: v.cliente_nombre.clone(),
                tipo: v.tipo.clone(), notas: v.notas.clone(),
                iva: self.db.obtener_iva().to_string(), items, nuevo_cliente: false,
                nc_nombre: String::new(), nc_rfc: String::new(), nc_telefono: String::new(),
                nc_direccion: String::new(), nc_ciudad: String::new(), nc_email: String::new(),
            };
        }
        self.ventas.editing_id = Some(id);
        self.ventas.show_form = true;
    }

    fn save_venta_form(&mut self) {
        let mut cliente_id = self.ventas.form.cliente_id;
        if self.ventas.form.nuevo_cliente {
            if self.ventas.form.nc_nombre.trim().is_empty() {
                self.notificacion = Some(("El nombre del nuevo cliente es obligatorio".to_string(), COLOR_DANGER));
                return;
            }
            let nc = ClienteNuevo {
                codigo: None,
                nombre: self.ventas.form.nc_nombre.trim().to_string(),
                rfc: self.ventas.form.nc_rfc.trim().to_string(),
                email: self.ventas.form.nc_email.trim().to_string(),
                telefono: self.ventas.form.nc_telefono.trim().to_string(),
                direccion: self.ventas.form.nc_direccion.trim().to_string(),
                ciudad: self.ventas.form.nc_ciudad.trim().to_string(),
                limite_credito: 0.0,
            };
            match self.db.crear_cliente(&nc) {
                Ok(id) => cliente_id = Some(id),
                Err(e) => { self.notificacion = Some((format!("Error al crear el cliente: {}", e), COLOR_DANGER)); return; }
            }
        }
        if self.ventas.form.cliente_nombre.trim().is_empty() && !self.ventas.form.nuevo_cliente {
            self.notificacion = Some(("Seleccione un cliente de la lista".to_string(), COLOR_DANGER));
            return;
        }
        if self.ventas.form.items.is_empty() {
            self.notificacion = Some(("Agregue al menos un producto a la venta".to_string(), COLOR_DANGER));
            return;
        }
        let f = &self.ventas.form;
        let detalles: Vec<VentaDetalleNuevo> = f.items.iter().map(|i| VentaDetalleNuevo {
            producto_id: i.producto_id,
            producto_nombre: i.producto_nombre.clone(),
            cantidad: i.cantidad.parse().unwrap_or(1), precio_unitario: i.precio.parse().unwrap_or(0.0),
            descuento: 0.0,
        }).collect();
        let nuevo = VentaNueva {
            cliente_id, cliente_nombre: f.cliente_nombre.trim().to_string(),
            tipo: f.tipo.trim().to_string(), notas: f.notas.clone(), descuento: 0.0,
            metodo_pago: None, iva: f.iva.parse().unwrap_or(0.0), detalles,
        };
        let r = if let Some(id) = self.ventas.editing_id { self.db.actualizar_venta(id, &nuevo).map(|_| ()) } else { self.db.crear_venta(&nuevo).map(|_| ()) };
        match r {
            Ok(_) => { self.ventas.show_form = false; self.load_ventas(); self.notificacion = Some(("Venta guardada correctamente".to_string(), COLOR_SUCCESS)); }
            Err(e) => self.notificacion = Some((format!("Error al guardar la venta: {}", e), COLOR_DANGER)),
        }
    }

    // ---- Gastos ----
    fn open_gasto_form(&mut self) {
        self.gastos.form = gastos::GastoFormData::default();
        self.gastos.opciones_categorias = self.db.listar_categorias_gastos().unwrap_or_default().iter().map(|c| SelectOption { id: c.id, label: c.nombre.clone() }).collect();
        self.gastos.show_form = true;
    }

    fn save_gasto_form(&mut self) {
        let f = &self.gastos.form;
        let nuevo = GastoNuevo {
            numero: None, categoria_id: f.categoria_id.parse().unwrap_or(1), descripcion: f.descripcion.clone(),
            monto: f.monto.parse().unwrap_or(0.0), subtotal: 0.0, impuesto: 0.0,
            proveedor_id: None, metodo_pago: f.metodo_pago.clone(),
            referencia: f.referencia.clone(), comprobante: None,
            notas: f.notas.clone(), fecha_vencimiento: None,
        };
        let r = if let Some(id) = self.gastos.editing_id { self.db.actualizar_gasto(id, &nuevo).map(|_| ()) } else { self.db.crear_gasto(&nuevo).map(|_| ()) };
        if r.is_ok() { self.gastos.show_form = false; self.load_gastos(); self.notificacion = Some(("Gasto guardado correctamente".to_string(), COLOR_SUCCESS)); }
    }

    // ---- Ubicaciones ----
    fn open_ubicacion_form(&mut self, editing_id: Option<i64>) {
        self.ubicaciones.errores = HashMap::new();
        if let Some(id) = editing_id {
            if let Some(u) = self.ubicaciones.ubicaciones.iter().find(|u| u.id == id) {
                self.ubicaciones.form = ubicaciones::UbicacionFormData {
                    nombre: u.nombre.clone(), direccion: u.direccion.clone(), ciudad: u.ciudad.clone(),
                    encargado: u.encargado.clone().unwrap_or_default(),
                    cedula: u.cedula.clone().unwrap_or_default(), telefono: u.telefono.clone(),
                };
            }
        } else { self.ubicaciones.form = ubicaciones::UbicacionFormData::default(); }
        self.ubicaciones.editing_id = editing_id;
        self.ubicaciones.show_form = true;
    }

    fn save_ubicacion_form(&mut self) {
        self.ubicaciones.errores = HashMap::new();
        if self.ubicaciones.form.nombre.trim().is_empty() {
            self.ubicaciones.errores.insert("nombre".to_string(), "El nombre es obligatorio".to_string());
            return;
        }
        let f = &self.ubicaciones.form;
        let nuevo = UbicacionNueva {
            nombre: f.nombre.clone(), direccion: f.direccion.clone(), ciudad: f.ciudad.clone(),
            encargado: Some(f.encargado.clone()).filter(|s| !s.is_empty()),
            cedula: Some(f.cedula.clone()).filter(|s| !s.is_empty()), telefono: f.telefono.clone(),
        };
        let r = if let Some(id) = self.ubicaciones.editing_id { self.db.actualizar_ubicacion(id, &nuevo).map(|_| ()) } else { self.db.crear_ubicacion(&nuevo).map(|_| ()) };
        if r.is_ok() { self.ubicaciones.show_form = false; self.load_ubicaciones(); self.notificacion = Some(("Ubicación guardada correctamente".to_string(), COLOR_SUCCESS)); }
    }

    // ---- Maquinas ----
    fn open_maquina_form(&mut self, editing_id: Option<i64>) {
        if let Some(id) = editing_id {
            if let Some(m) = self.maquinas.maquinas.iter().find(|m| m.id == id) {
                self.maquinas.form = maquinas::MaquinaFormData {
                    codigo: m.codigo.clone().unwrap_or_default(), descripcion: m.descripcion.clone(), modelo: m.modelo.clone(),
                    numero_serie: m.numero_serie.clone(), comision: m.comision.to_string(),
                    ubicacion_texto: m.ubicacion_texto.clone(),
                    fecha_instalacion: m.fecha_instalacion.clone(),
                };
            }
        } else { self.maquinas.form = maquinas::MaquinaFormData::default(); }
        self.maquinas.editing_id = editing_id;
        self.maquinas.show_form = true;
    }

    fn save_maquina_form(&mut self) {
        let f = &self.maquinas.form;
        let fecha = f.fecha_instalacion.trim().to_string();
        let dia_cobro = fecha.split('-').nth(2).and_then(|d| d.parse::<i32>().ok()).filter(|d| (1..=31).contains(d)).unwrap_or(1);
        let nuevo = MaquinaNueva {
            ubicacion_texto: f.ubicacion_texto.clone(),
            codigo: Some(f.codigo.clone()).filter(|s| !s.is_empty()),
            descripcion: f.descripcion.clone(), modelo: f.modelo.clone(),
            numero_serie: f.numero_serie.clone(), color: None,
            comision: f.comision.parse().unwrap_or(0.0), comision_estimada: 0.0, dia_cobro,
            fecha_instalacion: if fecha.is_empty() { None } else { Some(fecha) },
        };
        let r = if let Some(id) = self.maquinas.editing_id { self.db.actualizar_maquina(id, &nuevo).map(|_| ()) } else { self.db.crear_maquina(&nuevo).map(|_| ()) };
        match r {
            Ok(()) => { self.maquinas.show_form = false; self.load_maquinas(); self.notificacion = Some(("Máquina guardada correctamente".to_string(), COLOR_SUCCESS)); }
            Err(e) => { self.notificacion = Some((format!("Error al guardar: {}", e), COLOR_DANGER)); }
        }
    }

    // ---- Garantias ----
    fn open_garantia_form(&mut self) {
        self.garantias.form = garantias::GarantiaFormData::default();
        self.garantias.opciones_productos = self.db.listar_productos().unwrap_or_default().iter().map(|p| SelectOption { id: p.id, label: p.nombre.clone() }).collect();
        self.garantias.show_form = true;
    }

    fn save_garantia_form(&mut self) {
        let f = &self.garantias.form;
        let nuevo = GarantiaNueva {
            producto_id: Some(f.producto_id.parse().unwrap_or(0)).filter(|&x| x > 0),
            venta_id: Some(f.venta_id.parse().unwrap_or(0)).filter(|&x| x > 0),
            producto: String::new(), numero_serie: None, cliente_nombre: String::new(),
            cedula: None, telefono: None, direccion: None, ciudad: None,
            monto_pago: 0.0, observacion: None,
            fecha_inicio: f.fecha_inicio.clone(), fecha_fin: f.fecha_fin.clone(), descripcion: f.descripcion.clone(),
        };
        let r = if let Some(id) = self.garantias.editing_id { self.db.actualizar_garantia(id, &nuevo).map(|_| ()) } else { self.db.crear_garantia(&nuevo).map(|_| ()) };
        if r.is_ok() { self.garantias.show_form = false; self.load_garantias(); self.notificacion = Some(("Garantía guardada correctamente".to_string(), COLOR_SUCCESS)); }
    }

    // ---- Creditos ----
    fn open_credito_form(&mut self) {
        self.creditos.form = creditos::CreditoFormData::default();
        self.creditos.opciones_clientes = self.db.listar_clientes().unwrap_or_default().iter().map(|c| SelectOption { id: c.id, label: c.nombre.clone() }).collect();
        self.creditos.show_form = true;
    }

    fn save_credito_form(&mut self) {
        let f = &self.creditos.form;
        let nuevo = CuentaCreditoNueva {
            nombre: String::new(), tipo: "cliente".to_string(),
            cliente_id: Some(f.cliente_id.parse().unwrap_or(0)).filter(|&x| x > 0),
            proveedor_id: None, limite: f.limite.parse().unwrap_or(0.0), notas: None,
        };
        let r = if let Some(id) = self.creditos.editing_id { self.db.actualizar_cuenta_credito(id, &nuevo).map(|_| ()) } else { self.db.crear_cuenta_credito(&nuevo).map(|_| ()) };
        if r.is_ok() { self.creditos.show_form = false; self.load_creditos(); self.notificacion = Some(("Cuenta de crédito guardada correctamente".to_string(), COLOR_SUCCESS)); }
    }

    // ---- Ahorros ----
    fn open_ahorro_form(&mut self) {
        self.ahorros.form = ahorros::AhorroFormData::default();
        self.ahorros.opciones_clientes = self.db.listar_clientes().unwrap_or_default().iter().map(|c| SelectOption { id: c.id, label: c.nombre.clone() }).collect();
        self.ahorros.show_form = true;
    }

    // ---- Asientos Contables ----
    fn open_asiento_form(&mut self) {
        self.asientos.errores = HashMap::new();
        self.asientos.form = asientos::AsientoFormData::default();
        self.asientos.opciones_cuentas = self.db.listar_plan_cuentas().unwrap_or_default().iter().map(|c| SelectOption { id: c.id, label: format!("{} - {}", c.codigo, c.nombre) }).collect();
        self.asientos.show_form = true;
    }

    fn save_asiento_form(&mut self) {
        self.asientos.errores = HashMap::new();
        if self.asientos.form.concepto.trim().is_empty() {
            self.asientos.errores.insert("concepto".to_string(), "El concepto es obligatorio".to_string());
            return;
        }
        let f = &self.asientos.form;
        let lineas: Vec<AsientoLineaNueva> = f.lineas.iter().map(|l| AsientoLineaNueva {
            cuenta_id: l.cuenta_id.parse().unwrap_or(0),
            descripcion: Some(l.descripcion.clone()).filter(|s| !s.is_empty()),
            debe: l.debe.parse().unwrap_or(0.0),
            haber: l.haber.parse().unwrap_or(0.0),
        }).collect();
        let nuevo = AsientoNuevo {
            numero: None, fecha: f.fecha.clone(), concepto: f.concepto.clone(),
            descripcion: Some(f.descripcion.clone()).filter(|s| !s.is_empty()),
            referencia: Some(f.referencia.clone()).filter(|s| !s.is_empty()),
            lineas,
        };
        let r = if let Some(id) = self.asientos.editing_id { self.db.actualizar_asiento(id, &nuevo).map(|_| ()) } else { self.db.crear_asiento(&nuevo).map(|_| ()) };
        if r.is_ok() { self.asientos.show_form = false; self.load_asientos(); self.notificacion = Some(("Asiento contable guardado correctamente".to_string(), COLOR_SUCCESS)); }
    }

    // ---- Pagos Recibidos ----
    fn open_pago_recibido_form(&mut self) {
        self.pagos_recibidos.form = pagos_recibidos::PagoRecibidoFormData::default();
        self.pagos_recibidos.opciones_clientes = self.db.listar_clientes().unwrap_or_default().iter().map(|c| SelectOption { id: c.id, label: c.nombre.clone() }).collect();
        self.load_ventas();
        self.pagos_recibidos.ventas = self.ventas.ventas.clone();
        self.pagos_recibidos.opciones_ventas = self.ventas.ventas.iter()
            .filter(|v| v.saldo_pendiente > 0.01)
            .map(|v| SelectOption { id: v.id, label: format!("{} · {} · debe ${:.2}", v.folio, v.cliente_nombre, v.saldo_pendiente) })
            .collect();
        self.pagos_recibidos.show_form = true;
    }

    fn abonar_venta(&mut self, venta_id: i64) {
        let venta = self.ventas.ventas.iter().find(|v| v.id == venta_id).cloned();
        self.open_pago_recibido_form();
        if let Some(v) = venta {
            let f = &mut self.pagos_recibidos.form;
            f.venta_id = Some(v.id);
            f.cliente_id = v.cliente_id;
            f.monto = format!("{:.2}", v.saldo_pendiente);
        }
        self.nav = NavItem::PagosRecibidos;
    }

    fn save_pago_recibido_form(&mut self) {
        let f = &self.pagos_recibidos.form;
        let nuevo = PagoRecibidoNuevo {
            venta_id: f.venta_id.filter(|&x| x > 0),
            cliente_id: f.cliente_id.filter(|&x| x > 0),
            monto: f.monto.parse().unwrap_or(0.0),
            metodo_pago: Some(f.metodo_pago.clone()).filter(|s| !s.is_empty()),
            referencia: f.referencia.clone(), notas: f.notas.clone(),
        };
        let r = if let Some(id) = self.pagos_recibidos.editing_id { self.db.actualizar_pago_recibido(id, &nuevo).map(|_| ()) } else { self.db.crear_pago_recibido(&nuevo).map(|_| ()) };
        if r.is_ok() {
            self.pagos_recibidos.show_form = false;
            self.load_pagos_recibidos();
            self.load_ventas();
            self.pagos_recibidos.ventas = self.ventas.ventas.clone();
            self.notificacion = Some(("Pago recibido guardado correctamente".to_string(), COLOR_SUCCESS));
        }
    }

    // ---- Pagos Realizados ----
    fn open_pago_realizado_form(&mut self) {
        self.pagos_realizados.form = pagos_realizados::PagoRealizadoFormData::default();
        self.pagos_realizados.opciones_proveedores = self.db.listar_proveedores().unwrap_or_default().iter().map(|p| SelectOption { id: p.id, label: p.nombre.clone() }).collect();
        self.load_gastos();
        self.pagos_realizados.gastos = self.gastos.gastos.clone();
        let mut opciones = vec![SelectOption { id: 0, label: "— Sin gasto —".to_string() }];
        let pendientes: Vec<SelectOption> = self.gastos.gastos.iter()
            .filter(|g| g.estado == "pendiente" || g.fecha_vencimiento.is_some())
            .map(|g| SelectOption { id: g.id, label: format!("{} · {} · debe ${:.2}", g.numero.clone().unwrap_or_default(), g.descripcion, g.total) })
            .collect();
        opciones.extend(pendientes);
        self.pagos_realizados.opciones_gastos = opciones;
        self.pagos_realizados.show_form = true;
    }

    fn save_pago_realizado_form(&mut self) {
        let f = &self.pagos_realizados.form;
        let nuevo = PagoRealizadoNuevo {
            gasto_id: f.gasto_id.filter(|&x| x > 0),
            proveedor_id: f.proveedor_id.filter(|&x| x > 0),
            monto: f.monto.parse().unwrap_or(0.0),
            metodo_pago: Some(f.metodo_pago.clone()).filter(|s| !s.is_empty()),
            referencia: f.referencia.clone(), notas: f.notas.clone(),
        };
        let r = if let Some(id) = self.pagos_realizados.editing_id { self.db.actualizar_pago_realizado(id, &nuevo).map(|_| ()) } else { self.db.crear_pago_realizado(&nuevo).map(|_| ()) };
        if r.is_ok() {
            self.pagos_realizados.show_form = false;
            self.load_pagos_realizados();
            self.load_gastos();
            self.pagos_realizados.gastos = self.gastos.gastos.clone();
            self.notificacion = Some(("Pago realizado guardado correctamente".to_string(), COLOR_SUCCESS));
        }
    }

    // ---- Deudas de la Empresa ----
    fn open_deuda_form(&mut self, editing_id: Option<i64>) {
        self.deudas.opciones_proveedores = self.db.listar_proveedores().unwrap_or_default().iter().map(|p| SelectOption { id: p.id, label: p.nombre.clone() }).collect();
        self.deudas.opciones_categorias = self.db.listar_categorias_gastos().unwrap_or_default().iter().map(|c| SelectOption { id: c.id, label: c.nombre.clone() }).collect();
        if let Some(id) = editing_id {
            if let Some(d) = self.deudas.deudas.iter().find(|d| d.id == id) {
                self.deudas.form = deudas::DeudaFormData {
                    proveedor_id: d.proveedor_id, proveedor_nombre: d.proveedor_nombre.clone(),
                    concepto: d.concepto.clone(), descripcion: d.descripcion.clone().unwrap_or_default(),
                    categoria_id: d.categoria_id, fecha_deuda: d.fecha_deuda.clone(),
                    fecha_vencimiento: d.fecha_vencimiento.clone().unwrap_or_default(),
                    monto_total: d.monto_total.to_string(), referencia: d.referencia.clone(),
                    notas: d.notas.clone(),
                };
            }
        } else { self.deudas.form = deudas::DeudaFormData::default(); }
        self.deudas.editing_id = editing_id;
        self.deudas.show_form = true;
    }

    fn save_deuda_form(&mut self) {
        let f = &self.deudas.form;
        let categoria_nombre = f.categoria_id
            .and_then(|cid| self.deudas.opciones_categorias.iter().find(|o| o.id == cid).map(|o| o.label.clone()))
            .unwrap_or_default();
        let nuevo = DeudaEmpresaNueva {
            proveedor_id: f.proveedor_id.filter(|&x| x > 0),
            proveedor_nombre: if f.proveedor_nombre.trim().is_empty() { "— Sin nombre —".to_string() } else { f.proveedor_nombre.trim().to_string() },
            concepto: f.concepto.trim().to_string(),
            descripcion: Some(f.descripcion.clone()).filter(|s| !s.trim().is_empty()),
            categoria_id: f.categoria_id.filter(|&x| x > 0),
            categoria_nombre,
            fecha_deuda: if f.fecha_deuda.trim().is_empty() { chrono::Local::now().format("%Y-%m-%d").to_string() } else { f.fecha_deuda.clone() },
            fecha_vencimiento: Some(f.fecha_vencimiento.clone()).filter(|s| !s.trim().is_empty()),
            monto_total: f.monto_total.parse().unwrap_or(0.0),
            referencia: f.referencia.clone(), notas: f.notas.clone(),
        };
        let r = if let Some(id) = self.deudas.editing_id {
            self.db.actualizar_deuda_empresa(id, &nuevo).map(|_| ())
        } else { self.db.crear_deuda_empresa(&nuevo).map(|_| ()) };
        if r.is_ok() { self.deudas.show_form = false; self.load_deudas(); self.notificacion = Some(("Deuda guardada correctamente".to_string(), COLOR_SUCCESS)); }
    }

    fn ver_detalle_deuda(&mut self, id: i64) {
        self.deudas.deuda_seleccionada = Some(id);
        self.deudas.pagos_deuda = self.db.listar_deuda_pagos(id).unwrap_or_default();
        self.deudas.show_detalle = true;
    }

    fn open_deuda_pago_form(&mut self) {
        self.deudas.form_pago = deudas::DeudaPagoFormData::default();
        self.deudas.show_form_pago = true;
    }

    fn save_deuda_pago_form(&mut self) {
        let f = &self.deudas.form_pago;
        let deuda_id = self.deudas.deuda_seleccionada.unwrap_or(0);
        if deuda_id == 0 { return; }
        let nuevo = DeudaPagoNuevo {
            deuda_id,
            monto: f.monto.parse().unwrap_or(0.0),
            metodo_pago: Some(f.metodo_pago.clone()).filter(|s| !s.is_empty()),
            referencia: f.referencia.clone(), notas: f.notas.clone(),
        };
        let r = self.db.crear_deuda_pago(&nuevo);
        if r.is_ok() {
            self.deudas.show_form_pago = false;
            self.load_deudas();
            self.ver_detalle_deuda(deuda_id);
            self.notificacion = Some(("Pago registrado y descontado de la deuda".to_string(), COLOR_SUCCESS));
        }
    }

    fn save_ahorro_form(&mut self) {
        let f = &self.ahorros.form;
        let nuevo = Ahorro {
            id: 0, cliente_id: Some(f.cliente_id.parse().unwrap_or(0)).filter(|&x| x > 0),
            cliente_nombre: f.cliente_nombre.clone(),
            saldo: f.saldo_inicial.parse().unwrap_or(0.0), activo: true, fecha_apertura: String::new(),
        };
        let r = if let Some(id) = self.ahorros.editing_id { self.db.actualizar_ahorro(id, &nuevo).map(|_| ()) } else { self.db.crear_ahorro(&nuevo).map(|_| ()) };
        if r.is_ok() { self.ahorros.show_form = false; self.load_ahorros(); self.notificacion = Some(("Cuenta de ahorro guardada correctamente".to_string(), COLOR_SUCCESS)); }
    }

    // ---- Documentos PDF (Factura y Garantia) ----
    fn buscar_cliente(&self, id: Option<i64>) -> Option<crate::models::Cliente> {
        id.and_then(|cid| self.db.listar_clientes().unwrap_or_default().into_iter().find(|c| c.id == cid))
    }

    fn emitir_factura(&mut self, id: i64) {
        let venta = self.ventas.ventas.iter().find(|v| v.id == id).cloned();
        let lineas = self.db.obtener_detalles_venta(id).unwrap_or_default();
        match venta {
            Some(v) => {
                let cliente = self.buscar_cliente(v.cliente_id);
                let empresa = self.empresa.clone();
                match crate::pdf::generar_factura(&v, &lineas, cliente.as_ref(), &empresa) {
                    Ok(ruta) => {
                        let nombre = ruta.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                        crate::pdf::abrir_pdf(&ruta);
                        self.notificacion = Some((format!("Factura generada: {}", nombre), COLOR_SUCCESS));
                    }
                    Err(e) => self.notificacion = Some((format!("Error al generar factura: {}", e), COLOR_DANGER)),
                }
            }
            None => self.notificacion = Some(("Venta no encontrada".to_string(), COLOR_DANGER)),
        }
    }

    fn emitir_xml(&mut self, id: i64) {
        let venta = self.ventas.ventas.iter().find(|v| v.id == id).cloned();
        let lineas = self.db.obtener_detalles_venta(id).unwrap_or_default();
        match venta {
            Some(v) => {
                let cliente = self.buscar_cliente(v.cliente_id);
                let empresa = self.empresa.clone();
                match crate::pdf::generar_xml_factura(&v, &lineas, cliente.as_ref(), &empresa) {
                    Ok(ruta) => {
                        let nombre = ruta.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                        crate::pdf::abrir_pdf(&ruta);
                        self.notificacion = Some((format!("XML generado: {}", nombre), COLOR_SUCCESS));
                    }
                    Err(e) => self.notificacion = Some((format!("Error al generar XML: {}", e), COLOR_DANGER)),
                }
            }
            None => self.notificacion = Some(("Venta no encontrada".to_string(), COLOR_DANGER)),
        }
    }

    fn emitir_garantia(&mut self, id: i64) {
        let venta = self.ventas.ventas.iter().find(|v| v.id == id).cloned();
        let lineas = self.db.obtener_detalles_venta(id).unwrap_or_default();
        let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
        let fin = (chrono::Local::now() + chrono::Duration::days(365)).format("%Y-%m-%d").to_string();
        match venta {
            Some(v) => {
                let cliente = self.buscar_cliente(v.cliente_id);
                let (fi, ff) = match self.db.listar_garantias().unwrap_or_default().into_iter().find(|g| g.venta_id == Some(id)) {
                    Some(g) => (g.fecha_inicio.clone(), g.fecha_fin.clone()),
                    None => {
                        let nuevo = GarantiaNueva {
                            producto_id: lineas.first().and_then(|l| l.producto_id),
                            venta_id: Some(id),
                            producto: String::new(),
                            numero_serie: None,
                            cliente_nombre: v.cliente_nombre.clone(),
                            cedula: cliente.as_ref().map(|c| c.rfc.clone()),
                            telefono: cliente.as_ref().map(|c| c.telefono.clone()),
                            direccion: cliente.as_ref().map(|c| c.direccion.clone()),
                            ciudad: cliente.as_ref().map(|c| c.ciudad.clone()),
                            monto_pago: v.total,
                            observacion: None,
                            fecha_inicio: hoy.clone(),
                            fecha_fin: fin.clone(),
                            descripcion: format!("Garantia generada desde la venta {}", v.folio),
                        };
                        let _ = self.db.crear_garantia(&nuevo);
                        self.load_garantias();
                        (hoy, fin)
                    }
                };
                match crate::pdf::generar_garantia(&v, &lineas, &fi, &ff, cliente.as_ref(), &self.empresa) {
                    Ok(ruta) => {
                        let nombre = ruta.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                        crate::pdf::abrir_pdf(&ruta);
                        self.notificacion = Some((format!("Garantia generada: {}", nombre), COLOR_SUCCESS));
                    }
                    Err(e) => self.notificacion = Some((format!("Error al generar garantia: {}", e), COLOR_DANGER)),
                }
            }
            None => self.notificacion = Some(("Venta no encontrada".to_string(), COLOR_DANGER)),
        }
    }

    // ---- Compras / Almacén ----
    fn open_compra_form(&mut self) {
        self.compras.form = compras::CompraFormData::default();
        self.compras.opciones_productos = self.db.listar_productos().unwrap_or_default().iter()
            .filter(|p| p.activo)
            .map(|p| SelectOption { id: p.id, label: format!("{} ({} disp.)", p.nombre, p.stock) })
            .collect();
        self.compras.opciones_proveedores = self.db.listar_proveedores().unwrap_or_default().iter()
            .map(|p| SelectOption { id: p.id, label: p.nombre.clone() })
            .collect();
        self.compras.show_form = true;
    }

    fn save_compra_form(&mut self) {
        let f = &self.compras.form;
        if f.items.is_empty() {
            self.notificacion = Some(("Agregue al menos un producto a la compra".to_string(), COLOR_DANGER));
            return;
        }
        let mut proveedor_id = f.proveedor_id.filter(|&id| id > 0);
        if proveedor_id.is_none() && !f.proveedor_nombre.trim().is_empty() {
            let nombre = f.proveedor_nombre.trim().to_string();
            let encontrado = self.db.listar_proveedores().unwrap_or_default().iter()
                .find(|p| p.nombre.eq_ignore_ascii_case(&nombre)).map(|p| p.id);
            proveedor_id = match encontrado {
                Some(id) => Some(id),
                None => match self.db.crear_proveedor(&ProveedorNuevo {
                    codigo: None, nombre, contacto: String::new(), rfc: String::new(), email: String::new(),
                    telefono: String::new(), direccion: String::new(), ciudad: String::new(),
                }) {
                    Ok(id) => Some(id),
                    Err(e) => { self.notificacion = Some((format!("Error al crear el proveedor: {}", e), COLOR_DANGER)); return; }
                }
            };
        }
        let detalles: Vec<CompraDetalleNuevo> = f.items.iter().map(|i| CompraDetalleNuevo {
            producto_id: i.producto_id,
            producto_nombre: i.producto_nombre.clone(),
            cantidad: i.cantidad.parse().unwrap_or(1),
            precio_unitario: i.precio.parse().unwrap_or(0.0),
            descuento: 0.0,
        }).collect();
        let proveedor_nombre = f.proveedor_nombre.trim().to_string();
        let nuevo = CompraNueva {
            proveedor_id,
            proveedor_nombre,
            notas: f.notas.clone(),
            descuento: 0.0,
            metodo_pago: Some(f.metodo_pago.clone()),
            iva: self.db.obtener_iva(),
            detalles,
        };
        match self.db.crear_compra(&nuevo) {
            Ok(_) => { self.compras.show_form = false; self.load_compras(); self.notificacion = Some(("Compra registrada, inventario actualizado".to_string(), COLOR_SUCCESS)); }
            Err(e) => self.notificacion = Some((format!("Error al guardar la compra: {}", e), COLOR_DANGER)),
        }
    }

    fn ver_detalle_compra(&mut self, id: i64) {
        self.compras.detail_lineas = self.db.obtener_detalles_compra(id).unwrap_or_default();
        self.compras.show_detail = true;
    }

    // ---- Cotizaciones ----
    fn open_cotizacion_form(&mut self) {
        self.cotizaciones.form = cotizaciones::CotizacionFormData::default();
        self.cotizaciones.opciones_productos = self.db.listar_productos().unwrap_or_default().iter()
            .filter(|p| p.activo)
            .map(|p| SelectOption { id: p.id, label: format!("{} (${:.2})", p.nombre, p.precio_venta) })
            .collect();
        self.cotizaciones.opciones_clientes = self.db.listar_clientes().unwrap_or_default().iter()
            .map(|c| SelectOption { id: c.id, label: c.nombre.clone() })
            .collect();
        self.cotizaciones.show_form = true;
    }

    fn save_cotizacion_form(&mut self) {
        let f = &self.cotizaciones.form;
        if f.items.is_empty() {
            self.notificacion = Some(("Agregue al menos un producto a la cotización".to_string(), COLOR_DANGER));
            return;
        }
        let cliente_id = f.cliente_id.filter(|&id| id > 0);
        if cliente_id.is_none() && f.cliente_nombre.trim().is_empty() {
            self.notificacion = Some(("Seleccione un cliente o escriba el nombre".to_string(), COLOR_DANGER));
            return;
        }
        let detalles: Vec<CotizacionDetalleNuevo> = f.items.iter().map(|i| CotizacionDetalleNuevo {
            producto_id: i.producto_id,
            producto_nombre: i.producto_nombre.clone(),
            cantidad: i.cantidad.parse().unwrap_or(1),
            precio_unitario: i.precio.parse().unwrap_or(0.0),
            descuento: 0.0,
        }).collect();
        let nuevo = CotizacionNueva {
            cliente_id,
            cliente_nombre: f.cliente_nombre.trim().to_string(),
            validez_dias: f.validez_dias.parse().unwrap_or(7),
            notas: f.notas.clone(),
            descuento: 0.0,
            iva: self.db.obtener_iva(),
            detalles,
        };
        match self.db.crear_cotizacion(&nuevo) {
            Ok(_) => { self.cotizaciones.show_form = false; self.load_cotizaciones(); self.notificacion = Some(("Cotización guardada correctamente".to_string(), COLOR_SUCCESS)); }
            Err(e) => self.notificacion = Some((format!("Error al guardar la cotización: {}", e), COLOR_DANGER)),
        }
    }

    fn convertir_cotizacion(&mut self, id: i64) {
        match self.db.convertir_cotizacion_en_venta(id) {
            Ok(r) => {
                self.load_cotizaciones();
                self.load_ventas();
                self.load_compras();
                self.notificacion = Some((format!("Venta {} creada desde la cotización", r.folio), COLOR_SUCCESS));
            }
            Err(e) => self.notificacion = Some((format!("No se pudo convertir: {}", e), COLOR_DANGER)),
        }
    }

    // ---- Configuración ----
    fn save_configuracion(&mut self) {
        let f = &self.configuracion.form;
        if f.empresa_nombre.trim().is_empty() {
            self.configuracion.mensaje = "El nombre de la empresa es obligatorio".to_string();
            self.configuracion.es_error = true;
            return;
        }
        let iva: f64 = f.iva.parse().unwrap_or(15.0);
        if !(0.0..=100.0).contains(&iva) {
            self.configuracion.mensaje = "El IVA debe estar entre 0 y 100".to_string();
            self.configuracion.es_error = true;
            return;
        }
        let nueva = Configuracion {
            empresa_nombre: f.empresa_nombre.trim().to_string(),
            ruc: f.ruc.trim().to_string(),
            direccion: f.direccion.trim().to_string(),
            telefono: f.telefono.trim().to_string(),
            email: f.email.trim().to_string(),
            ciudad: f.ciudad.trim().to_string(),
            iva,
        };
        match self.db.guardar_configuracion(&nueva) {
            Ok(_) => {
                self.empresa = nueva;
                self.configuracion.mensaje = format!("Configuración guardada. Los documentos usarán el IVA del {}%.", iva);
                self.configuracion.es_error = false;
            }
            Err(e) => { self.configuracion.mensaje = format!("Error al guardar: {}", e); self.configuracion.es_error = true; }
        }
    }

    fn hacer_respaldo(&mut self) {
        let dir = std::env::current_exe().ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        let origen = dir.join("contabilidad_rust.db");
        let ruta = dir.join("backups");
        let _ = std::fs::create_dir_all(&ruta);
        let nombre = format!("contabilidad_{}.db", chrono::Local::now().format("%Y%m%d_%H%M%S"));
        let destino = ruta.join(nombre);
        match std::fs::copy(&origen, &destino) {
            Ok(_) => { self.configuracion.mensaje = format!("Respaldo creado en backups/{}", destino.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()); self.configuracion.es_error = false; }
            Err(e) => { self.configuracion.mensaje = format!("Error al crear el respaldo: {}", e); self.configuracion.es_error = true; }
        }
    }

    pub fn hacer_respaldo_automatico(&mut self) {
        let dir = std::env::current_exe().ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        let origen = dir.join("contabilidad_rust.db");
        if !origen.exists() { return; }
        self.db.checkpoint();
        let ruta = dir.join("backups");
        let _ = std::fs::create_dir_all(&ruta);
        let hoy = chrono::Local::now().format("%Y%m%d").to_string();
        let ya_hay_hoy = std::fs::read_dir(&ruta).map(|entries| {
            entries.filter_map(|e| e.ok()).any(|e| {
                e.file_name().to_string_lossy().contains(&hoy)
            })
        }).unwrap_or(false);
        if ya_hay_hoy { return; }
        let nombre = format!("contabilidad_{}_{}.db", hoy, chrono::Local::now().format("%H%M%S"));
        let _ = std::fs::copy(&origen, ruta.join(nombre));
    }

    fn abrir_documentos(&mut self) {
        let dir = std::env::current_exe().ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("Documentos");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", ""])
            .arg(&dir)
            .spawn();
        self.configuracion.mensaje = "Carpeta de documentos abierta".to_string();
        self.configuracion.es_error = false;
    }

    fn notificar_cobros_comisiones(&mut self, usuario: String) {
        let ahora = chrono::Local::now();
        let periodo = ahora.format("%Y-%m").to_string();
        let (n, monto) = self.db.resumen_cobros_vencidos_hoy(&periodo, ahora.day() as i32).unwrap_or((0, 0.0));
        if n > 0 {
            self.notificacion = Some((
                format!("\u{1F514} {} cobro(s) de comisión vencido(s) hoy (${:.2}). ¡Bienvenido, {}!", n, monto, usuario),
                COLOR_WARNING,
            ));
        } else {
            self.notificacion = Some((format!("Bienvenido, {}", usuario), COLOR_SUCCESS));
        }
    }
}

pub fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Navigate(item) => { state.nav = item; state.notificacion = None; state.load_current_section(); }
        Message::RefreshDashboard => state.load_dashboard(),
        Message::CerrarSolicitado => return cerrar_app(state),

        // Primera instalación
        Message::SetupMsg(msg) => match msg {
            primera_vez::SetupMessage::EmpresaNombre(v) => state.setup.form.empresa_nombre = v,
            primera_vez::SetupMessage::Ruc(v) => state.setup.form.ruc = v,
            primera_vez::SetupMessage::Direccion(v) => state.setup.form.direccion = v,
            primera_vez::SetupMessage::Telefono(v) => state.setup.form.telefono = v,
            primera_vez::SetupMessage::Email(v) => state.setup.form.email = v,
            primera_vez::SetupMessage::Ciudad(v) => state.setup.form.ciudad = v,
            primera_vez::SetupMessage::Iva(v) => state.setup.form.iva = v,
            primera_vez::SetupMessage::Usuario(v) => state.setup.form.usuario = v,
            primera_vez::SetupMessage::Contrasena(v) => state.setup.form.contrasena = v,
            primera_vez::SetupMessage::Confirmar(v) => state.setup.form.confirmar = v,
            primera_vez::SetupMessage::Guardar => state.save_setup(),
        },

        // Login
        Message::LoginMsg(msg) => match msg {
            login::LoginMessage::Usuario(v) => state.login.usuario = v,
            login::LoginMessage::Contrasena(v) => state.login.contrasena = v,
            login::LoginMessage::Ingresar => state.do_login(),
        },
        Message::CerrarSesion => {
            state.fase = Fase::Login;
            state.usuario_actual = String::new();
            state.login.contrasena = String::new();
            state.login.mensaje = String::new();
            state.login.es_error = false;
            state.notificacion = None;
        },
        Message::ConectarCelular => return state.conectar_celular(),
        Message::CelularListo(url, payload) => {
            state.celular.activo = false;
            state.celular.mensaje = String::new();
            if let Ok(code) = qrcode::QrCode::new(payload.as_bytes()) {
                state.celular.qr_size = code.width();
                state.celular.qr = Some(code.to_vec());
            } else {
                state.celular.qr = None;
                state.celular.qr_size = 0;
            }
            state.celular.url = Some(url);
        }
        Message::CelularError(e) => {
            state.celular.activo = false;
            state.celular.mensaje = e;
            state.celular.url = None;
            state.celular.qr = None;
            state.celular.qr_size = 0;
        }
        Message::CerrarDialogoCelular => state.celular = celular::CelularState::default(),
        Message::FinPresentacion => state.fase = Fase::Instalacion,

        // Clientes
        Message::CrearCliente => state.open_cliente_form(None),
        Message::EditarCliente(id) => state.open_cliente_form(Some(id)),
        Message::EliminarCliente(id) => state.confirmar = Some((id, ConfirmTarget::Cliente)),
        Message::BuscarClientes(q) => state.clientes.busqueda = q,
        Message::ClienteFormMsg(msg) => match msg {
            clientes::ClienteFormMessage::Codigo(v) => state.clientes.form.codigo = v,
            clientes::ClienteFormMessage::Nombre(v) => state.clientes.form.nombre = v,
            clientes::ClienteFormMessage::Rfc(v) => state.clientes.form.rfc = v,
            clientes::ClienteFormMessage::Email(v) => state.clientes.form.email = v,
            clientes::ClienteFormMessage::Telefono(v) => state.clientes.form.telefono = v,
            clientes::ClienteFormMessage::Direccion(v) => state.clientes.form.direccion = v,
            clientes::ClienteFormMessage::Ciudad(v) => state.clientes.form.ciudad = v,
            clientes::ClienteFormMessage::LimiteCredito(v) => state.clientes.form.limite_credito = v,
            clientes::ClienteFormMessage::Guardar => state.save_cliente_form(),
            clientes::ClienteFormMessage::Cancelar => state.clientes.show_form = false,
        },

        // Proveedores
        Message::CrearProveedor => state.open_proveedor_form(None),
        Message::EditarProveedor(id) => state.open_proveedor_form(Some(id)),
        Message::EliminarProveedor(id) => state.confirmar = Some((id, ConfirmTarget::Proveedor)),
        Message::BuscarProveedores(q) => state.proveedores.busqueda = q,
        Message::ProveedorFormMsg(msg) => match msg {
            proveedores::ProveedorFormMessage::Codigo(v) => state.proveedores.form.codigo = v,
            proveedores::ProveedorFormMessage::Nombre(v) => state.proveedores.form.nombre = v,
            proveedores::ProveedorFormMessage::Contacto(v) => state.proveedores.form.contacto = v,
            proveedores::ProveedorFormMessage::Rfc(v) => state.proveedores.form.rfc = v,
            proveedores::ProveedorFormMessage::Email(v) => state.proveedores.form.email = v,
            proveedores::ProveedorFormMessage::Telefono(v) => state.proveedores.form.telefono = v,
            proveedores::ProveedorFormMessage::Direccion(v) => state.proveedores.form.direccion = v,
            proveedores::ProveedorFormMessage::Guardar => state.save_proveedor_form(),
            proveedores::ProveedorFormMessage::Cancelar => state.proveedores.show_form = false,
        },

        // Productos
        Message::CrearProducto => state.open_producto_form(None),
        Message::EditarProducto(id) => state.open_producto_form(Some(id)),
        Message::EliminarProducto(id) => state.confirmar = Some((id, ConfirmTarget::Producto)),
        Message::BuscarProductos(q) => state.productos.busqueda = q,
        Message::ProductoFormMsg(msg) => match msg {
            productos::ProductoFormMessage::Codigo(v) => state.productos.form.codigo = v,
            productos::ProductoFormMessage::Nombre(v) => state.productos.form.nombre = v,
            productos::ProductoFormMessage::Descripcion(v) => state.productos.form.descripcion = v,
            productos::ProductoFormMessage::PrecioCompra(v) => state.productos.form.precio_compra = v,
            productos::ProductoFormMessage::PrecioVenta(v) => state.productos.form.precio_venta = v,
            productos::ProductoFormMessage::Stock(v) => state.productos.form.stock = v,
            productos::ProductoFormMessage::StockMinimo(v) => state.productos.form.stock_minimo = v,
            productos::ProductoFormMessage::Unidad(v) => state.productos.form.unidad = v,
            productos::ProductoFormMessage::Guardar => state.save_producto_form(),
            productos::ProductoFormMessage::Cancelar => state.productos.show_form = false,
            productos::ProductoFormMessage::AbrirAjuste(id) => {
                state.productos.ajuste_producto_id = Some(id);
                state.productos.ajuste_stock = state.productos.productos.iter().find(|p| p.id == id).map(|p| p.stock.to_string()).unwrap_or_default();
                state.productos.ajuste_motivo = String::new();
                state.productos.show_ajuste = true;
            }
            productos::ProductoFormMessage::AjusteStock(v) => state.productos.ajuste_stock = v,
            productos::ProductoFormMessage::AjusteMotivo(v) => state.productos.ajuste_motivo = v,
            productos::ProductoFormMessage::GuardarAjuste => {
                if let Some(pid) = state.productos.ajuste_producto_id {
                    let nuevo: i32 = state.productos.ajuste_stock.parse().unwrap_or(-1);
                    if nuevo < 0 {
                        state.notificacion = Some(("Ingrese un stock válido (0 o mayor)".to_string(), COLOR_DANGER));
                    } else {
                        let motivo = if state.productos.ajuste_motivo.trim().is_empty() { "ajuste manual".to_string() } else { state.productos.ajuste_motivo.trim().to_string() };
                        match state.db.ajustar_stock(pid, nuevo, &motivo) {
                            Ok(_) => { state.productos.show_ajuste = false; state.load_productos(); state.notificacion = Some(("Stock ajustado correctamente".to_string(), COLOR_SUCCESS)); }
                            Err(e) => state.notificacion = Some((format!("Error al ajustar stock: {}", e), COLOR_DANGER)),
                        }
                    }
                }
            }
            productos::ProductoFormMessage::CerrarAjuste => state.productos.show_ajuste = false,
            productos::ProductoFormMessage::AbrirMovimientos(id) => {
                state.productos.mov_producto_id = Some(id);
                state.productos.movimientos = state.db.listar_movimientos_producto(id).unwrap_or_default();
                state.productos.show_movimientos = true;
            }
            productos::ProductoFormMessage::CerrarMovimientos => state.productos.show_movimientos = false,
        },

        // Ventas
        Message::NuevaVenta => state.open_venta_form(),
        Message::EditarVenta(id) => state.open_editar_venta(id),
        Message::FiltrarVentaDesde(v) => state.ventas.desde = v,
        Message::FiltrarVentaHasta(v) => state.ventas.hasta = v,
        Message::EliminarVenta(id) => state.confirmar = Some((id, ConfirmTarget::Venta)),
        Message::BuscarVenta(q) => state.ventas.busqueda = q,
        Message::VerDetalleVenta(id) => { state.ventas.detail_lineas = state.db.obtener_detalles_venta(id).unwrap_or_default(); state.ventas.show_detail = true; }
        Message::EmitirFactura(id) => state.emitir_factura(id),
        Message::EmitirXml(id) => state.emitir_xml(id),
        Message::EmitirGarantia(id) => state.emitir_garantia(id),
        Message::AbonarVenta(id) => state.abonar_venta(id),
        Message::VentaFormMsg(msg) => match msg {
            ventas::VentaFormMessage::ClienteSeleccionado(id) => {
                if id == 0 {
                    state.ventas.form.nuevo_cliente = true;
                    state.ventas.form.cliente_id = None;
                    state.ventas.form.cliente_nombre = String::new();
                } else {
                    state.ventas.form.nuevo_cliente = false;
                    state.ventas.form.cliente_id = Some(id);
                    if let Some(c) = state.ventas.opciones_clientes.iter().find(|o| o.id == id) {
                        state.ventas.form.cliente_nombre = c.label.clone();
                    }
                }
            }
            ventas::VentaFormMessage::NuevoClienteNombre(v) => state.ventas.form.nc_nombre = v,
            ventas::VentaFormMessage::NuevoClienteRfc(v) => state.ventas.form.nc_rfc = v,
            ventas::VentaFormMessage::NuevoClienteTelefono(v) => state.ventas.form.nc_telefono = v,
            ventas::VentaFormMessage::NuevoClienteDireccion(v) => state.ventas.form.nc_direccion = v,
            ventas::VentaFormMessage::NuevoClienteCiudad(v) => state.ventas.form.nc_ciudad = v,
            ventas::VentaFormMessage::NuevoClienteEmail(v) => state.ventas.form.nc_email = v,
            ventas::VentaFormMessage::Tipo(v) => state.ventas.form.tipo = v,
            ventas::VentaFormMessage::Notas(v) => state.ventas.form.notas = v,
            ventas::VentaFormMessage::Iva(v) => state.ventas.form.iva = v,
            ventas::VentaFormMessage::ItemProducto(i, id) => {
                if let Some(item) = state.ventas.form.items.get_mut(i) {
                    item.producto_id = Some(id);
                    if let Some(p) = state.db.listar_productos().unwrap_or_default().iter().find(|p| p.id == id) {
                        item.producto_nombre = p.nombre.clone();
                        item.precio = p.precio_venta.to_string();
                        if item.cantidad.trim().is_empty() { item.cantidad = "1".to_string(); }
                    }
                }
            }
            ventas::VentaFormMessage::ItemCantidad(i, v) => { if let Some(item) = state.ventas.form.items.get_mut(i) { item.cantidad = v; } }
            ventas::VentaFormMessage::ItemPrecio(i, v) => { if let Some(item) = state.ventas.form.items.get_mut(i) { item.precio = v; } }
            ventas::VentaFormMessage::AgregarItem => state.ventas.form.items.push(ventas::VentaItemData {
                producto_id: None, producto_nombre: String::new(), cantidad: "1".to_string(), precio: "0".to_string(),
            }),
            ventas::VentaFormMessage::QuitarItem(i) => { if i < state.ventas.form.items.len() { state.ventas.form.items.remove(i); } }
            ventas::VentaFormMessage::Guardar => state.save_venta_form(),
            ventas::VentaFormMessage::Cancelar => state.ventas.show_form = false,
            ventas::VentaFormMessage::CerrarDetalle => state.ventas.show_detail = false,
        },

        // Gastos
        Message::NuevoGasto => state.open_gasto_form(),
        Message::FiltrarGastoDesde(v) => state.gastos.desde = v,
        Message::FiltrarGastoHasta(v) => state.gastos.hasta = v,
        Message::EditarGasto(id) => { if let Some(g) = state.gastos.gastos.iter().find(|g| g.id == id) { state.gastos.form = gastos::GastoFormData { categoria_id: g.categoria_id.to_string(), descripcion: g.descripcion.clone(), monto: g.monto.to_string(), proveedor: g.proveedor_nombre.clone(), metodo_pago: g.metodo_pago.clone(), referencia: g.referencia.clone(), notas: g.notas.clone() }; } state.gastos.editing_id = Some(id); state.gastos.show_form = true; }
        Message::EliminarGasto(id) => state.confirmar = Some((id, ConfirmTarget::Gasto)),
        Message::BuscarGasto(q) => state.gastos.busqueda = q,
        Message::GastoFormMsg(msg) => match msg {
            gastos::GastoFormMessage::CategoriaId(v) => state.gastos.form.categoria_id = v,
            gastos::GastoFormMessage::Descripcion(v) => state.gastos.form.descripcion = v,
            gastos::GastoFormMessage::Monto(v) => state.gastos.form.monto = v,
            gastos::GastoFormMessage::Proveedor(v) => state.gastos.form.proveedor = v,
            gastos::GastoFormMessage::MetodoPago(v) => state.gastos.form.metodo_pago = v,
            gastos::GastoFormMessage::Referencia(v) => state.gastos.form.referencia = v,
            gastos::GastoFormMessage::Notas(v) => state.gastos.form.notas = v,
            gastos::GastoFormMessage::Guardar => state.save_gasto_form(),
            gastos::GastoFormMessage::Cancelar => state.gastos.show_form = false,
        },

        // Ubicaciones
        Message::NuevaUbicacion => state.open_ubicacion_form(None),
        Message::EditarUbicacion(id) => state.open_ubicacion_form(Some(id)),
        Message::EliminarUbicacion(id) => state.confirmar = Some((id, ConfirmTarget::Ubicacion)),
        Message::UbicacionFormMsg(msg) => match msg {
            ubicaciones::UbicacionFormMessage::Nombre(v) => state.ubicaciones.form.nombre = v,
            ubicaciones::UbicacionFormMessage::Direccion(v) => state.ubicaciones.form.direccion = v,
            ubicaciones::UbicacionFormMessage::Ciudad(v) => state.ubicaciones.form.ciudad = v,
            ubicaciones::UbicacionFormMessage::Encargado(v) => state.ubicaciones.form.encargado = v,
            ubicaciones::UbicacionFormMessage::Cedula(v) => state.ubicaciones.form.cedula = v,
            ubicaciones::UbicacionFormMessage::Telefono(v) => state.ubicaciones.form.telefono = v,
            ubicaciones::UbicacionFormMessage::Guardar => state.save_ubicacion_form(),
            ubicaciones::UbicacionFormMessage::Cancelar => state.ubicaciones.show_form = false,
        },

        // Maquinas
        Message::NuevaMaquina => state.open_maquina_form(None),
        Message::EditarMaquina(id) => state.open_maquina_form(Some(id)),
        Message::EliminarMaquina(id) => state.confirmar = Some((id, ConfirmTarget::Maquina)),
        Message::MaquinaFormMsg(msg) => match msg {
            maquinas::MaquinaFormMessage::Codigo(v) => state.maquinas.form.codigo = v,
            maquinas::MaquinaFormMessage::Descripcion(v) => state.maquinas.form.descripcion = v,
            maquinas::MaquinaFormMessage::Modelo(v) => state.maquinas.form.modelo = v,
            maquinas::MaquinaFormMessage::NumeroSerie(v) => state.maquinas.form.numero_serie = v,
            maquinas::MaquinaFormMessage::Comision(v) => state.maquinas.form.comision = v,
            maquinas::MaquinaFormMessage::UbicacionTexto(v) => state.maquinas.form.ubicacion_texto = v,
            maquinas::MaquinaFormMessage::FechaInstalacion(v) => state.maquinas.form.fecha_instalacion = v,
            maquinas::MaquinaFormMessage::Guardar => state.save_maquina_form(),
            maquinas::MaquinaFormMessage::Cancelar => state.maquinas.show_form = false,
        },

        // Plan de Cuentas
        Message::NuevaCuenta => state.open_cuenta_form(None),
        Message::EditarCuenta(id) => state.open_cuenta_form(Some(id)),
        Message::EliminarCuenta(id) => state.confirmar = Some((id, ConfirmTarget::Cuenta)),
        Message::BuscarCuenta(q) => state.plan_cuentas.busqueda = q,
        Message::CuentaFormMsg(msg) => match msg {
            plan_cuentas::PlanCuentasFormMessage::Codigo(v) => state.plan_cuentas.form.codigo = v,
            plan_cuentas::PlanCuentasFormMessage::Nombre(v) => state.plan_cuentas.form.nombre = v,
            plan_cuentas::PlanCuentasFormMessage::Tipo(v) => state.plan_cuentas.form.tipo = v,
            plan_cuentas::PlanCuentasFormMessage::Naturaleza(v) => state.plan_cuentas.form.naturaleza = v,
            plan_cuentas::PlanCuentasFormMessage::Nivel(v) => state.plan_cuentas.form.nivel = v,
            plan_cuentas::PlanCuentasFormMessage::PadreId(v) => state.plan_cuentas.form.padre_id = v,
            plan_cuentas::PlanCuentasFormMessage::Activo(v) => state.plan_cuentas.form.activo = v,
            plan_cuentas::PlanCuentasFormMessage::Guardar => state.save_cuenta_form(),
            plan_cuentas::PlanCuentasFormMessage::Cancelar => state.plan_cuentas.show_form = false,
        },

        // Garantias
        Message::NuevaGarantia => state.open_garantia_form(),
        Message::EditarGarantia(id) => { if let Some(g) = state.garantias.garantias.iter().find(|g| g.id == id) { state.garantias.form = garantias::GarantiaFormData { producto_id: g.producto_id.map(|x| x.to_string()).unwrap_or_default(), venta_id: g.venta_id.map(|x| x.to_string()).unwrap_or_default(), fecha_inicio: g.fecha_inicio.clone(), fecha_fin: g.fecha_fin.clone(), descripcion: g.descripcion.clone() }; } state.garantias.editing_id = Some(id); state.garantias.show_form = true; }
        Message::EliminarGarantia(id) => state.confirmar = Some((id, ConfirmTarget::Garantia)),
        Message::BuscarGarantia(q) => state.garantias.busqueda = q,
        Message::GarantiaFormMsg(msg) => match msg {
            garantias::GarantiaFormMessage::ProductoId(v) => state.garantias.form.producto_id = v,
            garantias::GarantiaFormMessage::VentaId(v) => state.garantias.form.venta_id = v,
            garantias::GarantiaFormMessage::FechaInicio(v) => state.garantias.form.fecha_inicio = v,
            garantias::GarantiaFormMessage::FechaFin(v) => state.garantias.form.fecha_fin = v,
            garantias::GarantiaFormMessage::Descripcion(v) => state.garantias.form.descripcion = v,
            garantias::GarantiaFormMessage::Guardar => state.save_garantia_form(),
            garantias::GarantiaFormMessage::Cancelar => state.garantias.show_form = false,
        },

        // Creditos
        Message::NuevoCredito => state.open_credito_form(),
        Message::EditarCredito(id) => { if let Some(c) = state.creditos.cuentas.iter().find(|c| c.id == id) { state.creditos.form = creditos::CreditoFormData { cliente_id: c.cliente_id.map(|x| x.to_string()).unwrap_or_default(), cliente_nombre: c.cliente_nombre.clone(), limite: c.limite.to_string() }; } state.creditos.editing_id = Some(id); state.creditos.show_form = true; }
        Message::EliminarCredito(id) => state.confirmar = Some((id, ConfirmTarget::Credito)),
        Message::BuscarCredito(q) => state.creditos.busqueda = q,
        Message::VerMovimientosCredito(id) => { state.creditos.movimientos = state.db.listar_credito_movimientos(id).unwrap_or_default(); state.creditos.cuenta_seleccionada = Some(id); state.creditos.show_movimientos = true; }
        Message::CerrarMovimientosCredito => { state.creditos.show_movimientos = false; state.creditos.show_form_movimiento = false; state.creditos.cuenta_seleccionada = None; }
        Message::NuevoMovimientoCredito => { state.creditos.form_movimiento = creditos::CreditoMovFormData::default(); state.creditos.show_form_movimiento = true; }
        Message::CreditoMovFormMsg(msg) => match msg {
            creditos::CreditoMovFormMessage::Tipo(v) => state.creditos.form_movimiento.tipo = v,
            creditos::CreditoMovFormMessage::Monto(v) => state.creditos.form_movimiento.monto = v,
            creditos::CreditoMovFormMessage::Descripcion(v) => state.creditos.form_movimiento.descripcion = v,
            creditos::CreditoMovFormMessage::Cantidad(v) => state.creditos.form_movimiento.cantidad = v,
            creditos::CreditoMovFormMessage::PrecioUnit(v) => state.creditos.form_movimiento.precio_unit = v,
            creditos::CreditoMovFormMessage::Guardar => {
                let f = &state.creditos.form_movimiento;
                if let Some(cuenta_id) = state.creditos.cuenta_seleccionada {
                    let m = CreditoMovimientoNuevo {
                        cuenta_id, tipo: f.tipo.clone(), monto: f.monto.parse().unwrap_or(0.0),
                        cantidad: f.cantidad.parse().unwrap_or(0.0), precio_unit: f.precio_unit.parse().unwrap_or(0.0),
                        descripcion: f.descripcion.clone(), referencia_id: None,
                    };
                    if state.db.crear_credito_movimiento(&m).is_ok() {
                        state.creditos.show_form_movimiento = false;
                        state.creditos.movimientos = state.db.listar_credito_movimientos(cuenta_id).unwrap_or_default();
                        state.load_creditos();
                        state.notificacion = Some(("Movimiento registrado correctamente".to_string(), COLOR_SUCCESS));
                    }
                }
            }
            creditos::CreditoMovFormMessage::Cancelar => state.creditos.show_form_movimiento = false,
        },
        Message::CreditoFormMsg(msg) => match msg {
            creditos::CreditoFormMessage::ClienteId(v) => state.creditos.form.cliente_id = v,
            creditos::CreditoFormMessage::ClienteNombre(v) => state.creditos.form.cliente_nombre = v,
            creditos::CreditoFormMessage::Limite(v) => state.creditos.form.limite = v,
            creditos::CreditoFormMessage::Guardar => state.save_credito_form(),
            creditos::CreditoFormMessage::Cancelar => state.creditos.show_form = false,
        },

        // Ahorros
        Message::NuevoAhorro => state.open_ahorro_form(),
        Message::EditarAhorro(id) => { if let Some(a) = state.ahorros.ahorros.iter().find(|a| a.id == id) { state.ahorros.form = ahorros::AhorroFormData { cliente_id: a.cliente_id.map(|x| x.to_string()).unwrap_or_default(), cliente_nombre: a.cliente_nombre.clone(), saldo_inicial: a.saldo.to_string() }; } state.ahorros.editing_id = Some(id); state.ahorros.show_form = true; }
        Message::EliminarAhorro(id) => state.confirmar = Some((id, ConfirmTarget::Ahorro)),
        Message::BuscarAhorro(q) => state.ahorros.busqueda = q,
        Message::VerMovimientosAhorro(id) => { state.ahorros.movimientos = state.db.listar_ahorro_movimientos(id).unwrap_or_default(); state.ahorros.cuenta_seleccionada = Some(id); state.ahorros.show_movimientos = true; }
        Message::CerrarMovimientosAhorro => { state.ahorros.show_movimientos = false; state.ahorros.show_form_movimiento = false; state.ahorros.cuenta_seleccionada = None; }
        Message::NuevoMovimientoAhorro => { state.ahorros.form_movimiento = ahorros::AhorroMovFormData::default(); state.ahorros.show_form_movimiento = true; }
        Message::AhorroMovFormMsg(msg) => match msg {
            ahorros::AhorroMovFormMessage::Tipo(v) => state.ahorros.form_movimiento.tipo = v,
            ahorros::AhorroMovFormMessage::Monto(v) => state.ahorros.form_movimiento.monto = v,
            ahorros::AhorroMovFormMessage::Descripcion(v) => state.ahorros.form_movimiento.descripcion = v,
            ahorros::AhorroMovFormMessage::Guardar => {
                let f = &state.ahorros.form_movimiento;
                if let Some(ahorro_id) = state.ahorros.cuenta_seleccionada {
                    let m = AhorroMovimientoNuevo {
                        ahorro_id, tipo: f.tipo.clone(), monto: f.monto.parse().unwrap_or(0.0),
                        cobro_id: None, descripcion: f.descripcion.clone(),
                    };
                    if state.db.crear_ahorro_movimiento(&m).is_ok() {
                        state.ahorros.show_form_movimiento = false;
                        state.ahorros.movimientos = state.db.listar_ahorro_movimientos(ahorro_id).unwrap_or_default();
                        state.load_ahorros();
                        state.notificacion = Some(("Movimiento registrado correctamente".to_string(), COLOR_SUCCESS));
                    }
                }
            }
            ahorros::AhorroMovFormMessage::Cancelar => state.ahorros.show_form_movimiento = false,
        },
        Message::AhorroFormMsg(msg) => match msg {
            ahorros::AhorroFormMessage::ClienteId(v) => state.ahorros.form.cliente_id = v,
            ahorros::AhorroFormMessage::ClienteNombre(v) => state.ahorros.form.cliente_nombre = v,
            ahorros::AhorroFormMessage::SaldoInicial(v) => state.ahorros.form.saldo_inicial = v,
            ahorros::AhorroFormMessage::Guardar => state.save_ahorro_form(),
            ahorros::AhorroFormMessage::Cancelar => state.ahorros.show_form = false,
        },

        // Asientos
        Message::NuevoAsiento => state.open_asiento_form(),
        Message::FiltrarAsientoDesde(v) => state.asientos.desde = v,
        Message::FiltrarAsientoHasta(v) => state.asientos.hasta = v,
        Message::EditarAsiento(id) => {
            if let Some(a) = state.asientos.asientos.iter().find(|a| a.id == id) {
                state.asientos.form = asientos::AsientoFormData {
                    fecha: a.fecha.clone(), concepto: a.concepto.clone(),
                    descripcion: a.descripcion.clone().unwrap_or_default(),
                    referencia: a.referencia.clone().unwrap_or_default(), lineas: vec![],
                };
            }
            state.asientos.editing_id = Some(id);
            state.asientos.show_form = true;
        }
        Message::EliminarAsiento(id) => state.confirmar = Some((id, ConfirmTarget::Asiento)),
        Message::BuscarAsiento(q) => state.asientos.busqueda = q,
        Message::VerDetalleAsiento(id) => { state.asientos.detail_lineas = state.db.obtener_lineas_asiento(id).unwrap_or_default(); state.asientos.show_detail = true; }
        Message::CerrarDetalleAsiento => state.asientos.show_detail = false,
        Message::AsientoFormMsg(msg) => match msg {
            asientos::AsientoFormMessage::Fecha(v) => state.asientos.form.fecha = v,
            asientos::AsientoFormMessage::Concepto(v) => state.asientos.form.concepto = v,
            asientos::AsientoFormMessage::Descripcion(v) => state.asientos.form.descripcion = v,
            asientos::AsientoFormMessage::Referencia(v) => state.asientos.form.referencia = v,
            asientos::AsientoFormMessage::LineaCuenta(i, v) => { if let Some(item) = state.asientos.form.lineas.get_mut(i) { item.cuenta_id = v; } }
            asientos::AsientoFormMessage::LineaDescripcion(i, v) => { if let Some(item) = state.asientos.form.lineas.get_mut(i) { item.descripcion = v; } }
            asientos::AsientoFormMessage::LineaDebe(i, v) => { if let Some(item) = state.asientos.form.lineas.get_mut(i) { item.debe = v; } }
            asientos::AsientoFormMessage::LineaHaber(i, v) => { if let Some(item) = state.asientos.form.lineas.get_mut(i) { item.haber = v; } }
            asientos::AsientoFormMessage::AgregarLinea => state.asientos.form.lineas.push(asientos::AsientoLineaData {
                cuenta_id: String::new(), cuenta_nombre: String::new(), descripcion: String::new(), debe: "0".to_string(), haber: "0".to_string(),
            }),
            asientos::AsientoFormMessage::QuitarLinea(i) => { if i < state.asientos.form.lineas.len() { state.asientos.form.lineas.remove(i); } }
            asientos::AsientoFormMessage::Guardar => state.save_asiento_form(),
            asientos::AsientoFormMessage::Cancelar => state.asientos.show_form = false,
        },

        // Pagos Recibidos
        Message::NuevoPagoRecibido => state.open_pago_recibido_form(),
        Message::FiltrarPagoRecibidoDesde(v) => state.pagos_recibidos.desde = v,
        Message::FiltrarPagoRecibidoHasta(v) => state.pagos_recibidos.hasta = v,
        Message::EliminarPagoRecibido(id) => state.confirmar = Some((id, ConfirmTarget::PagoRecibido)),
        Message::BuscarPagoRecibido(q) => state.pagos_recibidos.busqueda = q,
        Message::PagoRecibidoFormMsg(msg) => match msg {
            pagos_recibidos::PagoRecibidoFormMessage::VentaSeleccionada(v) => {
                let form = &mut state.pagos_recibidos.form;
                form.venta_id = Some(v);
                if let Some(venta) = state.pagos_recibidos.ventas.iter().find(|x| x.id == v) {
                    form.cliente_id = venta.cliente_id;
                    form.monto = format!("{:.2}", venta.saldo_pendiente);
                }
            }
            pagos_recibidos::PagoRecibidoFormMessage::ClienteId(v) => state.pagos_recibidos.form.cliente_id = Some(v),
            pagos_recibidos::PagoRecibidoFormMessage::Monto(v) => state.pagos_recibidos.form.monto = v,
            pagos_recibidos::PagoRecibidoFormMessage::MetodoPago(v) => state.pagos_recibidos.form.metodo_pago = v,
            pagos_recibidos::PagoRecibidoFormMessage::Referencia(v) => state.pagos_recibidos.form.referencia = v,
            pagos_recibidos::PagoRecibidoFormMessage::Notas(v) => state.pagos_recibidos.form.notas = v,
            pagos_recibidos::PagoRecibidoFormMessage::Guardar => state.save_pago_recibido_form(),
            pagos_recibidos::PagoRecibidoFormMessage::Cancelar => state.pagos_recibidos.show_form = false,
        },

        // Pagos Realizados
        Message::NuevoPagoRealizado => state.open_pago_realizado_form(),
        Message::FiltrarPagoRealizadoDesde(v) => state.pagos_realizados.desde = v,
        Message::FiltrarPagoRealizadoHasta(v) => state.pagos_realizados.hasta = v,
        Message::EliminarPagoRealizado(id) => state.confirmar = Some((id, ConfirmTarget::PagoRealizado)),
        Message::BuscarPagoRealizado(q) => state.pagos_realizados.busqueda = q,
        Message::PagoRealizadoFormMsg(msg) => match msg {
            pagos_realizados::PagoRealizadoFormMessage::GastoSeleccionado(g) => {
                let form = &mut state.pagos_realizados.form;
                form.gasto_id = if g > 0 { Some(g) } else { None };
                if let Some(gasto) = state.pagos_realizados.gastos.iter().find(|x| x.id == g) {
                    form.proveedor_id = gasto.proveedor_id;
                    let ya: f64 = state.pagos_realizados.pagos.iter().filter(|p| p.gasto_id == Some(g)).map(|p| p.monto).sum();
                    form.monto = format!("{:.2}", (gasto.total - ya).max(0.0));
                }
            }
            pagos_realizados::PagoRealizadoFormMessage::ProveedorId(v) => state.pagos_realizados.form.proveedor_id = Some(v),
            pagos_realizados::PagoRealizadoFormMessage::Monto(v) => state.pagos_realizados.form.monto = v,
            pagos_realizados::PagoRealizadoFormMessage::MetodoPago(v) => state.pagos_realizados.form.metodo_pago = v,
            pagos_realizados::PagoRealizadoFormMessage::Referencia(v) => state.pagos_realizados.form.referencia = v,
            pagos_realizados::PagoRealizadoFormMessage::Notas(v) => state.pagos_realizados.form.notas = v,
            pagos_realizados::PagoRealizadoFormMessage::Guardar => state.save_pago_realizado_form(),
            pagos_realizados::PagoRealizadoFormMessage::Cancelar => state.pagos_realizados.show_form = false,
        },

        // Deudas de la Empresa
        Message::NuevaDeuda => state.open_deuda_form(None),
        Message::EditarDeuda(id) => state.open_deuda_form(Some(id)),
        Message::EliminarDeuda(id) => state.confirmar = Some((id, ConfirmTarget::Deuda)),
        Message::BuscarDeuda(q) => state.deudas.busqueda = q,
        Message::FiltrarDeudaEstado(e) => state.deudas.filtro_estado = e,
        Message::DeudaFormMsg(msg) => match msg {
            deudas::DeudaFormMessage::ProveedorSeleccionado(v) => {
                let form = &mut state.deudas.form;
                form.proveedor_id = if v > 0 { Some(v) } else { None };
                if let Some(prov) = state.db.listar_proveedores().unwrap_or_default().iter().find(|p| p.id == v) {
                    form.proveedor_nombre = prov.nombre.clone();
                }
            }
            deudas::DeudaFormMessage::ProveedorNombre(v) => state.deudas.form.proveedor_nombre = v,
            deudas::DeudaFormMessage::Concepto(v) => state.deudas.form.concepto = v,
            deudas::DeudaFormMessage::Descripcion(v) => state.deudas.form.descripcion = v,
            deudas::DeudaFormMessage::CategoriaSeleccionada(v) => state.deudas.form.categoria_id = if v > 0 { Some(v) } else { None },
            deudas::DeudaFormMessage::FechaDeuda(v) => state.deudas.form.fecha_deuda = v,
            deudas::DeudaFormMessage::FechaVencimiento(v) => state.deudas.form.fecha_vencimiento = v,
            deudas::DeudaFormMessage::MontoTotal(v) => state.deudas.form.monto_total = v,
            deudas::DeudaFormMessage::Referencia(v) => state.deudas.form.referencia = v,
            deudas::DeudaFormMessage::Notas(v) => state.deudas.form.notas = v,
            deudas::DeudaFormMessage::Guardar => state.save_deuda_form(),
            deudas::DeudaFormMessage::Cancelar => state.deudas.show_form = false,
        },
        Message::VerDetalleDeuda(id) => state.ver_detalle_deuda(id),
        Message::CerrarDetalleDeuda => {
            state.deudas.show_detalle = false;
            state.deudas.show_form_pago = false;
            state.deudas.deuda_seleccionada = None;
        }
        Message::NuevoPagoDeuda => state.open_deuda_pago_form(),
        Message::EliminarPagoDeuda(id) => state.confirmar = Some((id, ConfirmTarget::DeudaPago)),
        Message::DeudaPagoFormMsg(msg) => match msg {
            deudas::DeudaPagoFormMessage::Monto(v) => state.deudas.form_pago.monto = v,
            deudas::DeudaPagoFormMessage::MetodoPago(v) => state.deudas.form_pago.metodo_pago = v,
            deudas::DeudaPagoFormMessage::Referencia(v) => state.deudas.form_pago.referencia = v,
            deudas::DeudaPagoFormMessage::Notas(v) => state.deudas.form_pago.notas = v,
            deudas::DeudaPagoFormMessage::Guardar => state.save_deuda_pago_form(),
            deudas::DeudaPagoFormMessage::Cancelar => state.deudas.show_form_pago = false,
        },

        Message::NuevoCobroComision => { state.cobro_comisiones.form = cobro_comisiones::CobroComisionFormData::default(); state.cobro_comisiones.opciones_maquinas = state.db.listar_maquinas().unwrap_or_default().iter().map(|m| SelectOption { id: m.id, label: format!("{} - {}", m.codigo.clone().unwrap_or_default(), m.descripcion) }).collect(); state.cobro_comisiones.show_form = true; }
        Message::EliminarCobroComision(id) => state.confirmar = Some((id, ConfirmTarget::Comision)),
        Message::BuscarCobroComision(q) => state.cobro_comisiones.busqueda = q,
        Message::CobroComisionFormMsg(msg) => match msg {
            cobro_comisiones::CobroComisionFormMessage::MaquinaId(v) => state.cobro_comisiones.form.maquina_id = v,
            cobro_comisiones::CobroComisionFormMessage::Monto(v) => state.cobro_comisiones.form.monto = v,
            cobro_comisiones::CobroComisionFormMessage::Mes(v) => state.cobro_comisiones.form.mes = v,
            cobro_comisiones::CobroComisionFormMessage::Periodo(v) => state.cobro_comisiones.form.periodo = v,
            cobro_comisiones::CobroComisionFormMessage::Observacion(v) => state.cobro_comisiones.form.observacion = v,
            cobro_comisiones::CobroComisionFormMessage::Notas(v) => state.cobro_comisiones.form.notas = v,
            cobro_comisiones::CobroComisionFormMessage::Guardar => {
                let f = &state.cobro_comisiones.form;
                let nuevo = CobroComisionNuevo {
                    maquina_id: f.maquina_id.parse().unwrap_or(0), monto: f.monto.parse().unwrap_or(0.0),
                    mes: Some(f.mes.clone()).filter(|s| !s.is_empty()), periodo: f.periodo.clone(),
                    observacion: Some(f.observacion.clone()).filter(|s| !s.is_empty()), notas: f.notas.clone(),
                };
                if state.db.crear_cobro_comision(&nuevo).is_ok() { state.cobro_comisiones.show_form = false; state.load_cobro_comisiones(); state.notificacion = Some(("Comisión registrada".to_string(), COLOR_SUCCESS)); }
            }
            cobro_comisiones::CobroComisionFormMessage::Cancelar => state.cobro_comisiones.show_form = false,
        },
        // Compras / Almacén
        Message::NuevaCompra => state.open_compra_form(),
        Message::EliminarCompra(id) => state.confirmar = Some((id, ConfirmTarget::Compra)),
        Message::BuscarCompra(q) => state.compras.busqueda = q,
        Message::VerDetalleCompra(id) => state.ver_detalle_compra(id),
        Message::CompraFormMsg(msg) => match msg {
            compras::CompraFormMessage::ProveedorSeleccionado(v) => {
                let form = &mut state.compras.form;
                form.proveedor_id = if v > 0 { Some(v) } else { None };
                if let Some(p) = state.db.listar_proveedores().unwrap_or_default().iter().find(|p| p.id == v) {
                    form.proveedor_nombre = p.nombre.clone();
                }
            }
            compras::CompraFormMessage::ProveedorNombre(v) => state.compras.form.proveedor_nombre = v,
            compras::CompraFormMessage::MetodoPago(v) => state.compras.form.metodo_pago = v,
            compras::CompraFormMessage::Notas(v) => state.compras.form.notas = v,
            compras::CompraFormMessage::ItemProducto(i, id) => {
                if let Some(item) = state.compras.form.items.get_mut(i) {
                    item.producto_id = Some(id);
                    if let Some(p) = state.db.listar_productos().unwrap_or_default().iter().find(|p| p.id == id) {
                        item.producto_nombre = p.nombre.clone();
                        item.precio = p.precio_compra.to_string();
                        if item.cantidad.trim().is_empty() { item.cantidad = "1".to_string(); }
                    }
                }
            }
            compras::CompraFormMessage::ItemCantidad(i, v) => { if let Some(item) = state.compras.form.items.get_mut(i) { item.cantidad = v; } }
            compras::CompraFormMessage::ItemPrecio(i, v) => { if let Some(item) = state.compras.form.items.get_mut(i) { item.precio = v; } }
            compras::CompraFormMessage::AgregarItem => state.compras.form.items.push(compras::CompraItemData {
                producto_id: None, producto_nombre: String::new(), cantidad: "1".to_string(), precio: "0".to_string(),
            }),
            compras::CompraFormMessage::QuitarItem(i) => { if i < state.compras.form.items.len() { state.compras.form.items.remove(i); } }
            compras::CompraFormMessage::Guardar => state.save_compra_form(),
            compras::CompraFormMessage::Cancelar => state.compras.show_form = false,
            compras::CompraFormMessage::CerrarDetalle => state.compras.show_detail = false,
            compras::CompraFormMessage::AbrirMovimientos => { state.compras.show_movimientos = true; }
            compras::CompraFormMessage::CerrarMovimientos => { state.compras.show_movimientos = false; state.compras.show_detail = false; }
            compras::CompraFormMessage::FiltroMov(v) => state.compras.filtro_mov = v,
        },

        // Cotizaciones
        Message::NuevaCotizacion => state.open_cotizacion_form(),
        Message::EliminarCotizacion(id) => state.confirmar = Some((id, ConfirmTarget::Cotizacion)),
        Message::BuscarCotizacion(q) => state.cotizaciones.busqueda = q,
        Message::FiltrarCotizacionEstado(e) => state.cotizaciones.filtro_estado = e,
        Message::VerDetalleCotizacion(id) => { state.cotizaciones.detail_lineas = state.db.obtener_detalles_cotizacion(id).unwrap_or_default(); state.cotizaciones.show_detail = true; }
        Message::ConvertirCotizacion(id) => { state.cotizaciones.detail_lineas = state.db.obtener_detalles_cotizacion(id).unwrap_or_default(); state.cotizaciones.show_convertir = true; }
        Message::CotizacionFormMsg(msg) => match msg {
            cotizaciones::CotizacionFormMessage::ClienteSeleccionado(v) => {
                let form = &mut state.cotizaciones.form;
                form.cliente_id = if v > 0 { Some(v) } else { None };
                if let Some(c) = state.cotizaciones.opciones_clientes.iter().find(|o| o.id == v) {
                    form.cliente_nombre = c.label.clone();
                }
            }
            cotizaciones::CotizacionFormMessage::ClienteNombre(v) => state.cotizaciones.form.cliente_nombre = v,
            cotizaciones::CotizacionFormMessage::Validez(v) => state.cotizaciones.form.validez_dias = v,
            cotizaciones::CotizacionFormMessage::Notas(v) => state.cotizaciones.form.notas = v,
            cotizaciones::CotizacionFormMessage::ItemProducto(i, id) => {
                if let Some(item) = state.cotizaciones.form.items.get_mut(i) {
                    item.producto_id = Some(id);
                    if let Some(p) = state.db.listar_productos().unwrap_or_default().iter().find(|p| p.id == id) {
                        item.producto_nombre = p.nombre.clone();
                        item.precio = p.precio_venta.to_string();
                        if item.cantidad.trim().is_empty() { item.cantidad = "1".to_string(); }
                    }
                }
            }
            cotizaciones::CotizacionFormMessage::ItemCantidad(i, v) => { if let Some(item) = state.cotizaciones.form.items.get_mut(i) { item.cantidad = v; } }
            cotizaciones::CotizacionFormMessage::ItemPrecio(i, v) => { if let Some(item) = state.cotizaciones.form.items.get_mut(i) { item.precio = v; } }
            cotizaciones::CotizacionFormMessage::AgregarItem => state.cotizaciones.form.items.push(cotizaciones::CotizacionItemData {
                producto_id: None, producto_nombre: String::new(), cantidad: "1".to_string(), precio: "0".to_string(),
            }),
            cotizaciones::CotizacionFormMessage::QuitarItem(i) => { if i < state.cotizaciones.form.items.len() { state.cotizaciones.form.items.remove(i); } }
            cotizaciones::CotizacionFormMessage::Guardar => {
                if state.cotizaciones.show_convertir {
                    if let Some(id) = state.cotizaciones.detail_lineas.first().map(|d| d.cotizacion_id) {
                        state.cotizaciones.show_convertir = false;
                        state.cotizaciones.show_detail = false;
                        state.convertir_cotizacion(id);
                    }
                } else { state.save_cotizacion_form(); }
            }
            cotizaciones::CotizacionFormMessage::Cancelar => state.cotizaciones.show_form = false,
            cotizaciones::CotizacionFormMessage::CerrarDetalle => state.cotizaciones.show_detail = false,
            cotizaciones::CotizacionFormMessage::CerrarConvertir => state.cotizaciones.show_convertir = false,
        },

        // Retenciones
        Message::NuevaRetencion => {
            state.retenciones.form = retenciones::RetencionFormData::default();
            state.retenciones.numero = state.db.proximo_numero_retencion().unwrap_or_else(|_| "R-0001".into());
            state.retenciones.show_form = true;
        }
        Message::EliminarRetencion(id) => state.confirmar = Some((id, ConfirmTarget::Retencion)),
        Message::BuscarRetencion(q) => state.retenciones.busqueda = q,
        Message::ImprimirRetencion(id) => {
            if let Some(r) = state.retenciones.retenciones.iter().find(|r| r.id == id) {
                match crate::pdf::generar_retencion_pdf(r, &state.empresa) {
                    Ok(ruta) => { crate::pdf::abrir_pdf(&ruta); state.notificacion = Some(("Comprobante de retención generado".to_string(), COLOR_SUCCESS)); }
                    Err(e) => state.notificacion = Some((format!("Error al generar el PDF: {}", e), COLOR_DANGER)),
                }
            }
        }
        Message::RetencionFormMsg(msg) => match msg {
            retenciones::RetencionFormMessage::ProveedorNombre(v) => state.retenciones.form.proveedor_nombre = v,
            retenciones::RetencionFormMessage::Cedula(v) => state.retenciones.form.cedula = v,
            retenciones::RetencionFormMessage::Fecha(v) => state.retenciones.form.fecha = v,
            retenciones::RetencionFormMessage::BaseImpRenta(v) => state.retenciones.form.base_imp_renta = v,
            retenciones::RetencionFormMessage::PorcentajeRenta(v) => state.retenciones.form.porcentaje_renta = v,
            retenciones::RetencionFormMessage::BaseImpIva(v) => state.retenciones.form.base_imp_iva = v,
            retenciones::RetencionFormMessage::PorcentajeIva(v) => state.retenciones.form.porcentaje_iva = v,
            retenciones::RetencionFormMessage::TipoComprobante(v) => state.retenciones.form.tipo_comprobante = v,
            retenciones::RetencionFormMessage::NumeroComprobante(v) => state.retenciones.form.numero_comprobante = v,
            retenciones::RetencionFormMessage::Referencia(v) => state.retenciones.form.referencia = v,
            retenciones::RetencionFormMessage::Guardar => {
                let f = &state.retenciones.form;
                let base_renta = f.base_imp_renta.parse().unwrap_or(0.0);
                let porc_renta = f.porcentaje_renta.parse().unwrap_or(0.0);
                let base_iva = f.base_imp_iva.parse().unwrap_or(0.0);
                let porc_iva = f.porcentaje_iva.parse().unwrap_or(0.0);
                let nuevo = RetencionNueva {
                    numero: state.retenciones.numero.clone(),
                    proveedor_id: None,
                    proveedor_nombre: f.proveedor_nombre.clone(),
                    cedula: f.cedula.clone(),
                    fecha: f.fecha.clone(),
                    base_imp_renta: base_renta, porcentaje_renta: porc_renta,
                    valor_renta: base_renta * porc_renta / 100.0,
                    base_imp_iva: base_iva, porcentaje_iva: porc_iva,
                    valor_iva: base_iva * porc_iva / 100.0,
                    tipo_comprobante: f.tipo_comprobante.clone(),
                    numero_comprobante: f.numero_comprobante.clone(),
                    referencia: f.referencia.clone(),
                    estado: "emitida".to_string(),
                };
                if state.db.crear_retencion(&nuevo).is_ok() {
                    state.retenciones.show_form = false;
                    state.load_retenciones();
                    state.notificacion = Some(("Retención registrada".to_string(), COLOR_SUCCESS));
                }
            }
            retenciones::RetencionFormMessage::Cancelar => state.retenciones.show_form = false,
        },

        // Nómina
        Message::NominaTab(t) => state.nomina.tab = t,
        Message::NuevoEmpleado => {
            state.nomina.form_empleado = nomina::EmpleadoFormData::default();
            state.nomina.editing_empleado_id = None;
            state.nomina.show_form_empleado = true;
        }
        Message::EliminarEmpleado(id) => state.confirmar = Some((id, ConfirmTarget::Empleado)),
        Message::EmpleadoFormMsg(msg) => match msg {
            nomina::EmpleadoFormMessage::Cedula(v) => state.nomina.form_empleado.cedula = v,
            nomina::EmpleadoFormMessage::Nombre(v) => state.nomina.form_empleado.nombre = v,
            nomina::EmpleadoFormMessage::Cargo(v) => state.nomina.form_empleado.cargo = v,
            nomina::EmpleadoFormMessage::Telefono(v) => state.nomina.form_empleado.telefono = v,
            nomina::EmpleadoFormMessage::SueldoBase(v) => state.nomina.form_empleado.sueldo_base = v,
            nomina::EmpleadoFormMessage::FechaIngreso(v) => state.nomina.form_empleado.fecha_ingreso = v,
            nomina::EmpleadoFormMessage::Guardar => {
                let f = &state.nomina.form_empleado;
                let nuevo = EmpleadoNuevo {
                    cedula: f.cedula.clone(), nombre: f.nombre.clone(), cargo: f.cargo.clone(),
                    telefono: f.telefono.clone(), sueldo_base: f.sueldo_base.parse().unwrap_or(0.0),
                    fecha_ingreso: f.fecha_ingreso.clone(),
                };
                let r = if let Some(id) = state.nomina.editing_empleado_id {
                    state.db.actualizar_empleado(id, &nuevo).map(|_| ())
                } else { state.db.crear_empleado(&nuevo).map(|_| ()) };
                if r.is_ok() {
                    state.nomina.show_form_empleado = false;
                    state.load_nomina();
                    state.notificacion = Some(("Empleado guardado".to_string(), COLOR_SUCCESS));
                }
            }
            nomina::EmpleadoFormMessage::Cancelar => state.nomina.show_form_empleado = false,
        },
        Message::NuevoRol => {
            state.nomina.form_rol = nomina::RolFormData::default();
            state.nomina.opciones_empleados = state.db.listar_empleados().unwrap_or_default().iter()
                .filter(|e| e.activo)
                .map(|e| SelectOption { id: e.id, label: e.nombre.clone() })
                .collect();
            state.nomina.editing_rol_id = None;
            state.nomina.show_form_rol = true;
        }
        Message::EliminarRol(id) => state.confirmar = Some((id, ConfirmTarget::Rol)),
        Message::MarcarRolPagado(id) => {
            if state.db.marcar_rol_pagado(id).is_ok() {
                state.load_nomina();
                state.notificacion = Some(("Rol marcado como pagado".to_string(), COLOR_SUCCESS));
            }
        }
        Message::RolFormMsg(msg) => match msg {
            nomina::RolFormMessage::EmpleadoId(v) => state.nomina.form_rol.empleado_id = v,
            nomina::RolFormMessage::Periodo(v) => state.nomina.form_rol.periodo = v,
            nomina::RolFormMessage::Dias(v) => state.nomina.form_rol.dias = v,
            nomina::RolFormMessage::HorasExtra(v) => state.nomina.form_rol.horas_extra = v,
            nomina::RolFormMessage::Comisiones(v) => state.nomina.form_rol.comisiones = v,
            nomina::RolFormMessage::Iess(v) => state.nomina.form_rol.iess = v,
            nomina::RolFormMessage::Prestamos(v) => state.nomina.form_rol.prestamos = v,
            nomina::RolFormMessage::OtrasRetenciones(v) => state.nomina.form_rol.otras_retenciones = v,
            nomina::RolFormMessage::Notas(v) => state.nomina.form_rol.notas = v,
            nomina::RolFormMessage::Guardar => {
                let f = &state.nomina.form_rol;
                let emp_id = f.empleado_id.parse().unwrap_or(0);
                let sueldo_base = state.db.listar_empleados().unwrap_or_default().iter()
                    .find(|e| e.id == emp_id).map(|e| e.sueldo_base).unwrap_or(0.0);
                let dias = f.dias.parse::<i32>().unwrap_or(30).max(1);
                let sueldo_bruto = sueldo_base * dias as f64 / 30.0;
                let horas_extra = f.horas_extra.parse().unwrap_or(0.0);
                let comisiones = f.comisiones.parse().unwrap_or(0.0);
                let aporte = sueldo_bruto + horas_extra + comisiones;
                let iess = if f.iess.trim().is_empty() { aporte * 0.0945 } else { f.iess.parse().unwrap_or(0.0) };
                let nuevo = RolPagoNuevo {
                    empleado_id: emp_id, periodo: f.periodo.clone(), dias,
                    sueldo_bruto, horas_extra, comisiones, iess,
                    prestamos: f.prestamos.parse().unwrap_or(0.0),
                    otras_retenciones: f.otras_retenciones.parse().unwrap_or(0.0),
                    notas: f.notas.clone(),
                };
                let r = if let Some(id) = state.nomina.editing_rol_id {
                    state.db.actualizar_rol_pago(id, &nuevo).map(|_| ())
                } else { state.db.crear_rol_pago(&nuevo).map(|_| ()) };
                if r.is_ok() {
                    state.nomina.show_form_rol = false;
                    state.load_nomina();
                    state.notificacion = Some(("Rol de pago guardado".to_string(), COLOR_SUCCESS));
                }
            }
            nomina::RolFormMessage::Cancelar => state.nomina.show_form_rol = false,
        },

        // Depreciación de activos
        Message::DepreciacionTab(t) => state.depreciacion.tab = t,
        Message::NuevoActivo => {
            state.depreciacion.form = depreciacion::ActivoFormData::default();
            state.depreciacion.editing_id = None;
            state.depreciacion.show_form = true;
        }
        Message::EliminarActivo(id) => state.confirmar = Some((id, ConfirmTarget::Activo)),
        Message::EliminarDepreciacion(id) => state.confirmar = Some((id, ConfirmTarget::Depreciacion)),
        Message::DepreciarActivo(id) => {
            if state.db.registrar_depreciacion_mensual(id, &state.depreciacion.periodo).is_ok() {
                state.load_depreciacion();
                state.notificacion = Some(("Depreciación registrada".to_string(), COLOR_SUCCESS));
            }
        }
        Message::DepreciacionPeriodo(p) => state.depreciacion.periodo = p,
        Message::ActivoFormMsg(msg) => match msg {
            depreciacion::ActivoFormMessage::Descripcion(v) => state.depreciacion.form.descripcion = v,
            depreciacion::ActivoFormMessage::Categoria(v) => state.depreciacion.form.categoria = v,
            depreciacion::ActivoFormMessage::Fecha(v) => state.depreciacion.form.fecha_adquisicion = v,
            depreciacion::ActivoFormMessage::ValorAdquisicion(v) => state.depreciacion.form.valor_adquisicion = v,
            depreciacion::ActivoFormMessage::ValorResidual(v) => state.depreciacion.form.valor_residual = v,
            depreciacion::ActivoFormMessage::VidaUtil(v) => state.depreciacion.form.vida_util_anios = v,
            depreciacion::ActivoFormMessage::Guardar => {
                let f = &state.depreciacion.form;
                let nuevo = ActivoFijoNuevo {
                    descripcion: f.descripcion.clone(), categoria: f.categoria.clone(),
                    fecha_adquisicion: f.fecha_adquisicion.clone(),
                    valor_adquisicion: f.valor_adquisicion.parse().unwrap_or(0.0),
                    valor_residual: f.valor_residual.parse().unwrap_or(0.0),
                    vida_util_anios: f.vida_util_anios.parse().unwrap_or(5.0),
                };
                let r = if let Some(id) = state.depreciacion.editing_id {
                    state.db.actualizar_activo_fijo(id, &nuevo).map(|_| ())
                } else { state.db.crear_activo_fijo(&nuevo).map(|_| ()) };
                if r.is_ok() {
                    state.depreciacion.show_form = false;
                    state.load_depreciacion();
                    state.notificacion = Some(("Activo fijo guardado".to_string(), COLOR_SUCCESS));
                }
            }
            depreciacion::ActivoFormMessage::Cancelar => state.depreciacion.show_form = false,
        },

        // Cierre contable
        Message::NuevoCierre => {
            state.cierre_contable.form = cierre_contable::CierreFormData::default();
            state.cierre_contable.show_form = true;
        }
        Message::EliminarCierre(id) => state.confirmar = Some((id, ConfirmTarget::Cierre)),
        Message::CierreFormMsg(msg) => match msg {
            cierre_contable::CierreFormMessage::Anio(v) => state.cierre_contable.form.anio = v,
            cierre_contable::CierreFormMessage::Notas(v) => state.cierre_contable.form.notas = v,
            cierre_contable::CierreFormMessage::Guardar => {
                let f = &state.cierre_contable.form;
                let anio_default: i32 = chrono::Local::now().format("%Y").to_string().parse().unwrap_or(2026);
                let nuevo = CierreContableNuevo {
                    anio: f.anio.parse().unwrap_or(anio_default),
                    notas: f.notas.clone(),
                };
                if state.db.crear_cierre(&nuevo).is_ok() {
                    state.cierre_contable.show_form = false;
                    state.load_cierres();
                    state.notificacion = Some(("Cierre contable generado".to_string(), COLOR_SUCCESS));
                }
            }
            cierre_contable::CierreFormMessage::Cancelar => state.cierre_contable.show_form = false,
        },

        // Conciliación bancaria
        Message::ConciliacionTab(t) => state.conciliacion.tab = t,
        Message::NuevaCuentaBancaria => {
            state.conciliacion.form_cuenta = conciliacion::CuentaFormData::default();
            state.conciliacion.editing_cuenta_id = None;
            state.conciliacion.show_form_cuenta = true;
        }
        Message::EliminarCuentaBancaria(id) => state.confirmar = Some((id, ConfirmTarget::CuentaBancaria)),
        Message::CuentaBancariaFormMsg(msg) => match msg {
            conciliacion::CuentaFormMessage::Nombre(v) => state.conciliacion.form_cuenta.nombre = v,
            conciliacion::CuentaFormMessage::Banco(v) => state.conciliacion.form_cuenta.banco = v,
            conciliacion::CuentaFormMessage::NumeroCuenta(v) => state.conciliacion.form_cuenta.numero_cuenta = v,
            conciliacion::CuentaFormMessage::Tipo(v) => state.conciliacion.form_cuenta.tipo = v,
            conciliacion::CuentaFormMessage::SaldoInicial(v) => state.conciliacion.form_cuenta.saldo_inicial = v,
            conciliacion::CuentaFormMessage::Guardar => {
                let f = &state.conciliacion.form_cuenta;
                let nuevo = CuentaBancariaNueva {
                    nombre: f.nombre.clone(), banco: f.banco.clone(),
                    numero_cuenta: f.numero_cuenta.clone(), tipo: f.tipo.clone(),
                    saldo_inicial: f.saldo_inicial.parse().unwrap_or(0.0),
                };
                let r = if let Some(id) = state.conciliacion.editing_cuenta_id {
                    state.db.actualizar_cuenta_bancaria(id, &nuevo).map(|_| ())
                } else { state.db.crear_cuenta_bancaria(&nuevo).map(|_| ()) };
                if r.is_ok() {
                    state.conciliacion.show_form_cuenta = false;
                    state.load_conciliacion();
                    state.notificacion = Some(("Cuenta bancaria guardada".to_string(), COLOR_SUCCESS));
                }
            }
            conciliacion::CuentaFormMessage::Cancelar => state.conciliacion.show_form_cuenta = false,
        },
        Message::SeleccionarCuentaBancaria(id) => {
            state.conciliacion.cuenta_seleccionada = id;
            state.conciliacion.tab = conciliacion::ConciliacionTab::Movimientos;
            state.load_conciliacion();
        }
        Message::NuevoMovimientoBancario => {
            state.conciliacion.form_movimiento = conciliacion::MovimientoFormData::default();
            state.conciliacion.editing_movimiento_id = None;
            state.conciliacion.show_form_movimiento = true;
        }
        Message::EliminarMovimientoBancario(id) => state.confirmar = Some((id, ConfirmTarget::MovimientoBancario)),
        Message::ToggleConciliado(id) => {
            if let Some(m) = state.conciliacion.movimientos.iter().find(|m| m.id == id) {
                let nuevo = MovimientoBancarioNuevo {
                    cuenta_id: m.cuenta_id, fecha: m.fecha.clone(), descripcion: m.descripcion.clone(),
                    tipo: m.tipo.clone(), monto: m.monto, conciliado: !m.conciliado, referencia: m.referencia.clone(),
                };
                if state.db.actualizar_movimiento_bancario(id, &nuevo).is_ok() {
                    state.load_conciliacion();
                }
            }
        }
        Message::MovimientoBancarioFormMsg(msg) => match msg {
            conciliacion::MovimientoFormMessage::Fecha(v) => state.conciliacion.form_movimiento.fecha = v,
            conciliacion::MovimientoFormMessage::Descripcion(v) => state.conciliacion.form_movimiento.descripcion = v,
            conciliacion::MovimientoFormMessage::Tipo(v) => state.conciliacion.form_movimiento.tipo = v,
            conciliacion::MovimientoFormMessage::Monto(v) => state.conciliacion.form_movimiento.monto = v,
            conciliacion::MovimientoFormMessage::Referencia(v) => state.conciliacion.form_movimiento.referencia = v,
            conciliacion::MovimientoFormMessage::Guardar => {
                let f = &state.conciliacion.form_movimiento;
                let nuevo = MovimientoBancarioNuevo {
                    cuenta_id: state.conciliacion.cuenta_seleccionada,
                    fecha: f.fecha.clone(), descripcion: f.descripcion.clone(),
                    tipo: f.tipo.clone(), monto: f.monto.parse().unwrap_or(0.0),
                    conciliado: false, referencia: f.referencia.clone(),
                };
                if state.db.crear_movimiento_bancario(&nuevo).is_ok() {
                    state.conciliacion.show_form_movimiento = false;
                    state.load_conciliacion();
                    state.notificacion = Some(("Movimiento registrado".to_string(), COLOR_SUCCESS));
                }
            }
            conciliacion::MovimientoFormMessage::Cancelar => state.conciliacion.show_form_movimiento = false,
        },

        // Arqueo de caja
        Message::NuevoArqueo => {
            state.caja_chica.form = caja_chica::ArqueoFormData::default();
            state.caja_chica.show_form = true;
        }
        Message::EliminarArqueo(id) => state.confirmar = Some((id, ConfirmTarget::Arqueo)),
        Message::BuscarArqueo(q) => state.caja_chica.busqueda = q,
        Message::ArqueoFormMsg(msg) => match msg {
            caja_chica::ArqueoFormMessage::Fecha(v) => state.caja_chica.form.fecha = v,
            caja_chica::ArqueoFormMessage::Responsable(v) => state.caja_chica.form.responsable = v,
            caja_chica::ArqueoFormMessage::MontoEsperado(v) => state.caja_chica.form.monto_esperado = v,
            caja_chica::ArqueoFormMessage::MontoReal(v) => state.caja_chica.form.monto_real = v,
            caja_chica::ArqueoFormMessage::Observacion(v) => state.caja_chica.form.observacion = v,
            caja_chica::ArqueoFormMessage::Guardar => {
                let f = &state.caja_chica.form;
                let nuevo = ArqueoCajaNuevo {
                    fecha: f.fecha.clone(), responsable: f.responsable.clone(),
                    monto_esperado: f.monto_esperado.parse().unwrap_or(0.0),
                    monto_real: f.monto_real.parse().unwrap_or(0.0),
                    observacion: f.observacion.clone(),
                };
                if state.db.crear_arqueo(&nuevo).is_ok() {
                    state.caja_chica.show_form = false;
                    state.load_arqueos();
                    state.notificacion = Some(("Arqueo registrado".to_string(), COLOR_SUCCESS));
                }
            }
            caja_chica::ArqueoFormMessage::Cancelar => state.caja_chica.show_form = false,
        },

        // Configuración
        Message::ConfiguracionMsg(msg) => match msg {
            configuracion::ConfiguracionMessage::EmpresaNombre(v) => state.configuracion.form.empresa_nombre = v,
            configuracion::ConfiguracionMessage::Ruc(v) => state.configuracion.form.ruc = v,
            configuracion::ConfiguracionMessage::Direccion(v) => state.configuracion.form.direccion = v,
            configuracion::ConfiguracionMessage::Telefono(v) => state.configuracion.form.telefono = v,
            configuracion::ConfiguracionMessage::Email(v) => state.configuracion.form.email = v,
            configuracion::ConfiguracionMessage::Ciudad(v) => state.configuracion.form.ciudad = v,
            configuracion::ConfiguracionMessage::Iva(v) => state.configuracion.form.iva = v,
            configuracion::ConfiguracionMessage::Guardar => state.save_configuracion(),
            configuracion::ConfiguracionMessage::Respaldo => state.hacer_respaldo(),
            configuracion::ConfiguracionMessage::AbrirDocumentos => state.abrir_documentos(),
        },

        Message::ReportesMsg(msg) => match msg {
            reportes::ReportesMessage::Tab(t) => { state.reportes.tab = t; match t {
                reportes::ReporteTab::LibroDiario => state.load_reportes_libro_diario(),
                reportes::ReporteTab::BalanceGeneral => state.load_reportes_balance_resumen(),
                reportes::ReporteTab::Comprobacion => state.load_reportes_balance(),
                reportes::ReporteTab::EstadoResultados => state.load_reportes_resultados(),
                reportes::ReporteTab::LibroMayor => state.load_reportes_mayor(),
                reportes::ReporteTab::Antiguedad => state.load_reportes_antiguedad(),
                reportes::ReporteTab::LibroCompras => state.load_reportes_libro_compras(),
                reportes::ReporteTab::LibroVentas => state.load_reportes_libro_ventas(),
                reportes::ReporteTab::Ats => state.load_reportes_ats(),
            } }
            reportes::ReportesMessage::Desde(v) => state.reportes.desde = v,
            reportes::ReportesMessage::Hasta(v) => state.reportes.hasta = v,
            reportes::ReportesMessage::MayorCuenta(id) => { state.reportes.mayor_cuenta = id; state.load_reportes_mayor(); }
            reportes::ReportesMessage::Generar => match state.reportes.tab {
                reportes::ReporteTab::LibroDiario => state.load_reportes_libro_diario(),
                reportes::ReporteTab::BalanceGeneral => state.load_reportes_balance_resumen(),
                reportes::ReporteTab::Comprobacion => state.load_reportes_balance(),
                reportes::ReporteTab::EstadoResultados => state.load_reportes_resultados(),
                reportes::ReporteTab::LibroMayor => state.load_reportes_mayor(),
                reportes::ReporteTab::Antiguedad => state.load_reportes_antiguedad(),
                reportes::ReporteTab::LibroCompras => state.load_reportes_libro_compras(),
                reportes::ReporteTab::LibroVentas => state.load_reportes_libro_ventas(),
                reportes::ReporteTab::Ats => state.load_reportes_ats(),
            },
            reportes::ReportesMessage::ExportarAts => state.exportar_ats_csv(),
        },
        Message::ConfirmarSi => {
            if let Some((id, target)) = state.confirmar.take() {
                let r = match target {
                    ConfirmTarget::Cliente => state.db.eliminar_cliente(id).map(|_| ()),
                    ConfirmTarget::Proveedor => state.db.eliminar_proveedor(id).map(|_| ()),
                    ConfirmTarget::Producto => state.db.eliminar_producto(id).map(|_| ()),
                    ConfirmTarget::Venta => state.db.eliminar_venta(id).map(|_| ()),
                    ConfirmTarget::Gasto => state.db.eliminar_gasto(id).map(|_| ()),
                    ConfirmTarget::Ubicacion => state.db.eliminar_ubicacion(id).map(|_| ()),
                    ConfirmTarget::Maquina => state.db.eliminar_maquina(id).map(|_| ()),
                    ConfirmTarget::Cuenta => state.db.eliminar_cuenta(id).map(|_| ()),
                    ConfirmTarget::Garantia => state.db.eliminar_garantia(id).map(|_| ()),
                    ConfirmTarget::Credito => state.db.eliminar_cuenta_credito(id).map(|_| ()),
                    ConfirmTarget::Ahorro => state.db.eliminar_ahorro(id).map(|_| ()),
                    ConfirmTarget::Asiento => state.db.eliminar_asiento(id).map(|_| ()),
                    ConfirmTarget::PagoRecibido => state.db.eliminar_pago_recibido(id).map(|_| ()),
                    ConfirmTarget::PagoRealizado => state.db.eliminar_pago_realizado(id).map(|_| ()),
                    ConfirmTarget::Deuda => state.db.eliminar_deuda_empresa(id).map(|_| ()),
                    ConfirmTarget::DeudaPago => state.db.eliminar_deuda_pago(id).map(|_| ()),
                    ConfirmTarget::Comision => state.db.eliminar_cobro_comision(id).map(|_| ()),
                    ConfirmTarget::Compra => state.db.eliminar_compra(id).map(|_| ()),
                    ConfirmTarget::Cotizacion => state.db.eliminar_cotizacion(id).map(|_| ()),
                    ConfirmTarget::Retencion => state.db.eliminar_retencion(id).map(|_| ()),
                    ConfirmTarget::Empleado => state.db.eliminar_empleado(id).map(|_| ()),
                    ConfirmTarget::Rol => state.db.eliminar_rol_pago(id).map(|_| ()),
                    ConfirmTarget::Activo => state.db.eliminar_activo_fijo(id).map(|_| ()),
                    ConfirmTarget::Depreciacion => state.db.eliminar_depreciacion(id).map(|_| ()),
                    ConfirmTarget::Cierre => state.db.eliminar_cierre(id).map(|_| ()),
                    ConfirmTarget::CuentaBancaria => state.db.eliminar_cuenta_bancaria(id).map(|_| ()),
                    ConfirmTarget::MovimientoBancario => state.db.eliminar_movimiento_bancario(id).map(|_| ()),
                    ConfirmTarget::Arqueo => state.db.eliminar_arqueo(id).map(|_| ()),
                };
                if r.is_ok() {
                    state.load_current_section();
                    if matches!(target, ConfirmTarget::DeudaPago) {
                        if let Some(did) = state.deudas.deuda_seleccionada { state.ver_detalle_deuda(did); }
                    }
                    state.notificacion = Some(("Registro eliminado correctamente".to_string(), COLOR_DANGER));
                }
            }
        }
        Message::ConfirmarNo => state.confirmar = None,
        Message::LimpiarNotificacion => state.notificacion = None,
    }
    Task::none()
}

fn cerrar_app(state: &mut App) -> Task<Message> {
    state.hacer_respaldo_automatico();
    iced::exit()
}

pub fn view(state: &App) -> Element<Message> {
    if state.fase == Fase::Presentacion {
        return presentacion::presentacion_view(Message::FinPresentacion);
    }
    if state.fase == Fase::Instalacion {
        return primera_vez::setup_view(&state.setup, |msg| Message::SetupMsg(msg));
    }
    if state.fase == Fase::Login {
        return login::login_view(&state.login, state.empresa.nombre_corto(), |msg| Message::LoginMsg(msg));
    }

    let content: Element<Message> = match state.nav {
        NavItem::Dashboard => dashboard_view(&state.dashboard, Message::RefreshDashboard),
        NavItem::Clientes => clientes_view(
            &state.clientes, Message::CrearCliente,
            |id| Message::EditarCliente(id), |id| Message::EliminarCliente(id),
            |q| Message::BuscarClientes(q), |msg| Message::ClienteFormMsg(msg),
        ),
        NavItem::Proveedores => proveedores_view(
            &state.proveedores, Message::CrearProveedor,
            |id| Message::EditarProveedor(id), |id| Message::EliminarProveedor(id),
            |q| Message::BuscarProveedores(q), |msg| Message::ProveedorFormMsg(msg),
        ),
        NavItem::Productos => productos_view(
            &state.productos, Message::CrearProducto,
            |id| Message::EditarProducto(id), |id| Message::EliminarProducto(id),
            |q| Message::BuscarProductos(q), |msg| Message::ProductoFormMsg(msg),
        ),
        NavItem::Ventas => ventas_view(&state.ventas, Message::NuevaVenta, |id| Message::EditarVenta(id), |id| Message::EliminarVenta(id), |q| Message::BuscarVenta(q), |id| Message::VerDetalleVenta(id), |id| Message::EmitirFactura(id), |id| Message::EmitirXml(id), |id| Message::EmitirGarantia(id), |id| Message::AbonarVenta(id), |msg| Message::VentaFormMsg(msg), |s| Message::FiltrarVentaDesde(s), |s| Message::FiltrarVentaHasta(s)),
        NavItem::Gastos => gastos_view(&state.gastos, Message::NuevoGasto, |msg| Message::GastoFormMsg(msg), |id| Message::EditarGasto(id), |id| Message::EliminarGasto(id), |q| Message::BuscarGasto(q), |s| Message::FiltrarGastoDesde(s), |s| Message::FiltrarGastoHasta(s)),
        NavItem::PlanCuentas => plan_cuentas_view(&state.plan_cuentas, Message::NuevaCuenta, |id| Message::EditarCuenta(id), |id| Message::EliminarCuenta(id), |q| Message::BuscarCuenta(q), |msg| Message::CuentaFormMsg(msg)),
        NavItem::Ubicaciones => ubicaciones_view(
            &state.ubicaciones, Message::NuevaUbicacion,
            |id| Message::EditarUbicacion(id), |id| Message::EliminarUbicacion(id),
            |msg| Message::UbicacionFormMsg(msg),
        ),
        NavItem::Maquinas => maquinas_view(
            &state.maquinas, Message::NuevaMaquina,
            |id| Message::EditarMaquina(id), |id| Message::EliminarMaquina(id),
            |msg| Message::MaquinaFormMsg(msg),
        ),
        NavItem::Garantias => garantias_view(&state.garantias, Message::NuevaGarantia, |id| Message::EditarGarantia(id), |id| Message::EliminarGarantia(id), |q| Message::BuscarGarantia(q), |msg| Message::GarantiaFormMsg(msg)),
        NavItem::Creditos => creditos_view(&state.creditos, Message::NuevoCredito, |id| Message::EditarCredito(id), |id| Message::EliminarCredito(id), |q| Message::BuscarCredito(q), |id| Message::VerMovimientosCredito(id), Message::CerrarMovimientosCredito, |msg| Message::CreditoFormMsg(msg), Message::NuevoMovimientoCredito, |msg| Message::CreditoMovFormMsg(msg)),
        NavItem::Ahorros => ahorros_view(&state.ahorros, Message::NuevoAhorro, |msg| Message::AhorroFormMsg(msg), |id| Message::EditarAhorro(id), |id| Message::EliminarAhorro(id), |q| Message::BuscarAhorro(q), |id| Message::VerMovimientosAhorro(id), Message::NuevoMovimientoAhorro, |msg| Message::AhorroMovFormMsg(msg), Message::CerrarMovimientosAhorro),
        NavItem::Asientos => asientos_view(&state.asientos, Message::NuevoAsiento, |id| Message::EditarAsiento(id), |id| Message::EliminarAsiento(id), |q| Message::BuscarAsiento(q), |id| Message::VerDetalleAsiento(id), Message::CerrarDetalleAsiento, |msg| Message::AsientoFormMsg(msg), |s| Message::FiltrarAsientoDesde(s), |s| Message::FiltrarAsientoHasta(s)),
        NavItem::PagosRecibidos => pagos_recibidos_view(&state.pagos_recibidos, Message::NuevoPagoRecibido, |msg| Message::PagoRecibidoFormMsg(msg), |id| Message::EliminarPagoRecibido(id), |s| Message::BuscarPagoRecibido(s), |s| Message::FiltrarPagoRecibidoDesde(s), |s| Message::FiltrarPagoRecibidoHasta(s)),
        NavItem::PagosRealizados => pagos_realizados_view(&state.pagos_realizados, Message::NuevoPagoRealizado, |msg| Message::PagoRealizadoFormMsg(msg), |id| Message::EliminarPagoRealizado(id), |s| Message::BuscarPagoRealizado(s), |s| Message::FiltrarPagoRealizadoDesde(s), |s| Message::FiltrarPagoRealizadoHasta(s)),
        NavItem::DeudasEmpresa => deudas_view(
            &state.deudas, Message::NuevaDeuda,
            |id| Message::EditarDeuda(id), |id| Message::EliminarDeuda(id),
            |q| Message::BuscarDeuda(q), |e| Message::FiltrarDeudaEstado(e),
            |id| Message::VerDetalleDeuda(id), Message::CerrarDetalleDeuda,
            |msg| Message::DeudaFormMsg(msg), Message::NuevoPagoDeuda,
            |id| Message::EliminarPagoDeuda(id), |msg| Message::DeudaPagoFormMsg(msg),
        ),
        NavItem::Reportes => reportes_view(&state.reportes, |msg| Message::ReportesMsg(msg)),
        NavItem::CobroComisiones => cobro_comisiones_view(&state.cobro_comisiones, Message::NuevoCobroComision, |msg| Message::CobroComisionFormMsg(msg), |id| Message::EliminarCobroComision(id), |s| Message::BuscarCobroComision(s)),
        NavItem::Compras => compras_view(&state.compras, Message::NuevaCompra, |id| Message::EliminarCompra(id), |q| Message::BuscarCompra(q), |id| Message::VerDetalleCompra(id), |msg| Message::CompraFormMsg(msg)),
        NavItem::Cotizaciones => cotizaciones_view(&state.cotizaciones, Message::NuevaCotizacion, |id| Message::EliminarCotizacion(id), |q| Message::BuscarCotizacion(q), |e| Message::FiltrarCotizacionEstado(e), |id| Message::VerDetalleCotizacion(id), |id| Message::ConvertirCotizacion(id), |msg| Message::CotizacionFormMsg(msg)),
        NavItem::Retenciones => retenciones_view(&state.retenciones, Message::NuevaRetencion, |msg| Message::RetencionFormMsg(msg), |id| Message::EliminarRetencion(id), |s| Message::BuscarRetencion(s), |id| Message::ImprimirRetencion(id)),
        NavItem::Nomina => nomina_view(&state.nomina, |t| Message::NominaTab(t), |msg| Message::EmpleadoFormMsg(msg), |msg| Message::RolFormMsg(msg), Message::NuevoEmpleado, Message::NuevoRol, |id| Message::EliminarEmpleado(id), |id| Message::EliminarRol(id), |id| Message::MarcarRolPagado(id)),
        NavItem::Depreciacion => depreciacion_view(&state.depreciacion, |t| Message::DepreciacionTab(t), |msg| Message::ActivoFormMsg(msg), Message::NuevoActivo, |id| Message::EliminarActivo(id), |id| Message::DepreciarActivo(id), |s| Message::DepreciacionPeriodo(s)),
        NavItem::CierreContable => cierre_contable_view(&state.cierre_contable, Message::NuevoCierre, |msg| Message::CierreFormMsg(msg), |id| Message::EliminarCierre(id)),
        NavItem::Conciliacion => conciliacion_view(&state.conciliacion, |t| Message::ConciliacionTab(t), |msg| Message::CuentaBancariaFormMsg(msg), |msg| Message::MovimientoBancarioFormMsg(msg), Message::NuevaCuentaBancaria, Message::NuevoMovimientoBancario, |id| Message::EliminarCuentaBancaria(id), |id| Message::EliminarMovimientoBancario(id), |id| Message::ToggleConciliado(id), |id| Message::SeleccionarCuentaBancaria(id)),
        NavItem::CajaChica => caja_chica_view(&state.caja_chica, Message::NuevoArqueo, |msg| Message::ArqueoFormMsg(msg), |id| Message::EliminarArqueo(id), |s| Message::BuscarArqueo(s)),
        NavItem::Configuracion => configuracion_view(&state.configuracion, |msg| Message::ConfiguracionMsg(msg)),
    };

    let page: Element<Message> = if let Some((nota, color)) = &state.notificacion {
        let banner = container(
            row![
                text(nota).size(13).color(COLOR_TEXT_PRIMARY),
                Space::new().width(Length::Fill),
                text("\u{2715}").size(11).color(COLOR_TEXT_MUTED),
            ].align_y(Alignment::Center),
        )
        .style(move |_| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color { a: 0.15, ..*color })),
            border: iced::Border { radius: RADIUS_MD.into(), width: 1.0, color: iced::Color { a: 0.4, ..*color } },
            text_color: Some(COLOR_TEXT_PRIMARY),
            snap: false,
            shadow: SHADOW_SMALL,
        })
        .padding([10.0, SPACING_MD])
        .width(Length::Fill);
        column![
            button(banner).style(|_, _| iced::widget::button::Style {
                background: None, text_color: COLOR_TEXT_PRIMARY,
                border: iced::Border::default(), shadow: iced::Shadow::default(), snap: false,
            }).on_press(Message::LimpiarNotificacion),
            content,
        ].into()
    } else { content };

    let base: Element<Message> = row![sidebar_view(&state.nav, &state.empresa, &state.usuario_actual, |item| Message::Navigate(item), || Message::ConectarCelular, || Message::CerrarSesion), page]
        .spacing(0)
        .height(iced::Length::Fill)
        .into();

    if state.celular.activo || state.celular.url.is_some() {
        celular::celular_dialog(&state.celular, Message::CerrarDialogoCelular)
    } else if state.confirmar.is_some() {
        confirm_dialog(
            "¿Está seguro de eliminar este registro?",
            Message::ConfirmarSi,
            Message::ConfirmarNo,
        )
    } else {
        base
    }
}
