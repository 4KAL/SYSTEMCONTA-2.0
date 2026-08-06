#[path = "../db/mod.rs"]
mod db;
#[path = "../models/mod.rs"]
mod models;

use axum::{
    Router, Json, extract::State, http::{Method, StatusCode, header},
    middleware, response::{IntoResponse, Response}, routing::{get, post, put, delete},
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;
use serde::Deserialize;

use db::DatabaseManager;
use models::*;
use chrono::Datelike;

struct AppState {
    db: DatabaseManager,
    tokens: Mutex<Vec<String>>,
}

type AppStateRef = Arc<AppState>;

// ---------------------------------------------------------------------------
// Autenticación
// ---------------------------------------------------------------------------
async fn require_auth(
    State(state): State<AppStateRef>,
    req: axum::http::Request<axum::body::Body>,
    next: middleware::Next,
) -> Response {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string());
    let valido = token
        .map(|t| state.tokens.lock().unwrap().iter().any(|x| *x == t))
        .unwrap_or(false);
    if valido {
        next.run(req).await
    } else {
        (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "No autorizado"}))).into_response()
    }
}

#[derive(Deserialize)]
struct LoginData { usuario: String, contrasena: String }

async fn login(State(state): State<AppStateRef>, Json(data): Json<LoginData>) -> Response {
    match state.db.verificar_usuario(&data.usuario, &data.contrasena) {
        Ok(Some(u)) => {
            let token = format!("{:016x}{:016x}", rand::random::<u64>(), rand::random::<u64>());
            state.tokens.lock().unwrap().push(token.clone());
            (StatusCode::OK, Json(serde_json::json!({"token": token, "usuario": u.nombre_usuario}))).into_response()
        }
        _ => (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Usuario o contraseña incorrectos"}))).into_response(),
    }
}

#[derive(Deserialize)]
struct QrLoginData { token: String }

async fn login_qr(State(state): State<AppStateRef>, Json(data): Json<QrLoginData>) -> Response {
    match state.db.usar_qr_token(&data.token) {
        Ok(Some(usuario)) => {
            let token = format!("{:016x}{:016x}", rand::random::<u64>(), rand::random::<u64>());
            state.tokens.lock().unwrap().push(token.clone());
            (StatusCode::OK, Json(serde_json::json!({"token": token, "usuario": usuario}))).into_response()
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Token inválido o expirado. Vuelva a generar el QR en la PC."}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": format!("{}", e)}))).into_response(),
    }
}

// ---------------------------------------------------------------------------
// Dashboard
// ---------------------------------------------------------------------------
async fn dashboard(State(state): State<AppStateRef>) -> Json<serde_json::Value> {
    let d = &state.db;
    let ahora = chrono::Local::now();
    let periodo = ahora.format("%Y-%m").to_string();
    let dia = ahora.day() as i32;
    let pendientes = d.cobros_pendientes_mes(&periodo, dia).unwrap_or_default();
    Json(serde_json::json!({
        "ventas_hoy": d.kpi_ventas_hoy().unwrap_or(0.0),
        "gastos_hoy": d.kpi_gastos_hoy().unwrap_or(0.0),
        "cxc": d.kpi_cxc().unwrap_or(0.0),
        "cxp": d.kpi_cxp().unwrap_or(0.0),
        "utilidad_mes": d.kpi_utilidad_mes().unwrap_or(0.0),
        "clientes": d.listar_clientes().unwrap_or_default().len(),
        "ventas": d.listar_ventas().unwrap_or_default().len(),
        "cobros_pendientes": pendientes.len(),
    }))
}

// ---------------------------------------------------------------------------
// Migración de datos del sistema anterior
// ---------------------------------------------------------------------------
#[derive(Deserialize)]
struct MigrarData { origen: Option<String> }

async fn migrar(State(state): State<AppStateRef>, Json(data): Json<MigrarData>) -> Response {
    let origen = data.origen.unwrap_or_else(|| {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("contabilidad.db")))
            .unwrap_or_else(|| std::path::PathBuf::from("contabilidad.db"))
            .to_string_lossy()
            .to_string()
    });
    match state.db.migrar_desde_archivo(&origen) {
        Ok(r) => (StatusCode::CREATED, Json(serde_json::json!({"ok": true, "filas": r.filas_migradas, "mensaje": r.mensaje}))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

// ---------------------------------------------------------------------------
// Máquinas
// ---------------------------------------------------------------------------
async fn list_maquinas(State(state): State<AppStateRef>) -> Json<Vec<MaquinaUbicada>> {
    Json(state.db.listar_maquinas().unwrap_or_default())
}

// ---------------------------------------------------------------------------
// Cobros de comisiones
// ---------------------------------------------------------------------------
async fn cobros_pendientes(State(state): State<AppStateRef>) -> Json<serde_json::Value> {
    let d = &state.db;
    let ahora = chrono::Local::now();
    let periodo = ahora.format("%Y-%m").to_string();
    let dia = ahora.day() as i32;
    let pendientes = d.cobros_pendientes_mes(&periodo, dia).unwrap_or_default();
    let items: Vec<serde_json::Value> = pendientes.iter().map(|m| serde_json::json!({
        "id": m.id,
        "codigo": m.codigo,
        "descripcion": m.descripcion,
        "ubicacion": m.ubicacion_texto,
        "comision_estimada": m.comision_estimada,
        "dia_cobro": m.dia_cobro,
        "vencido": m.dia_cobro < dia,
    })).collect();
    Json(serde_json::json!({"periodo": periodo, "items": items}))
}

#[derive(Deserialize)]
struct CobroData {
    maquina_id: i64,
    monto: f64,
    observacion: Option<String>,
    notas: Option<String>,
}

async fn crear_cobro(State(state): State<AppStateRef>, Json(data): Json<CobroData>) -> Response {
    let periodo = chrono::Local::now().format("%Y-%m").to_string();
    let c = CobroComisionNuevo {
        maquina_id: data.maquina_id,
        monto: data.monto,
        mes: Some(periodo.clone()),
        periodo,
        observacion: data.observacion,
        notas: data.notas.unwrap_or_default(),
    };
    match state.db.crear_cobro_comision(&c) {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({"id": id, "ok": true}))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": format!("{}", e)}))).into_response(),
    }
}

async fn list_cobros(State(state): State<AppStateRef>) -> Json<Vec<CobroComision>> {
    Json(state.db.listar_todas_comisiones().unwrap_or_default())
}

// ---------------------------------------------------------------------------
// Pagos recibidos
// ---------------------------------------------------------------------------
async fn list_pagos_recibidos(State(state): State<AppStateRef>) -> Json<Vec<PagoRecibido>> {
    Json(state.db.listar_pagos_recibidos().unwrap_or_default())
}

#[derive(Deserialize)]
struct PagoRecibidoData {
    venta_id: Option<i64>,
    cliente_id: Option<i64>,
    monto: f64,
    metodo_pago: Option<String>,
    referencia: Option<String>,
    notas: Option<String>,
}

async fn crear_pago_recibido(State(state): State<AppStateRef>, Json(data): Json<PagoRecibidoData>) -> Response {
    let p = PagoRecibidoNuevo {
        venta_id: data.venta_id,
        cliente_id: data.cliente_id,
        monto: data.monto,
        metodo_pago: data.metodo_pago,
        referencia: data.referencia.unwrap_or_default(),
        notas: data.notas.unwrap_or_default(),
    };
    match state.db.crear_pago_recibido(&p) {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({"id": id, "ok": true}))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": format!("{}", e)}))).into_response(),
    }
}

// ---------------------------------------------------------------------------
// Alertas (para notificaciones)
// ---------------------------------------------------------------------------
async fn alertas(State(state): State<AppStateRef>) -> Json<serde_json::Value> {
    let d = &state.db;
    let ahora = chrono::Local::now();
    let periodo = ahora.format("%Y-%m").to_string();
    let dia = ahora.day() as i32;
    let pendientes = d.cobros_pendientes_mes(&periodo, dia).unwrap_or_default();
    let vencidos = pendientes.iter().filter(|m| m.dia_cobro < dia).count();
    let creditos = d.alertas_creditos_vencidos().unwrap_or_default();
    let stock = d.alertas_stock().unwrap_or_default();
    let mut msgs: Vec<String> = Vec::new();
    if !pendientes.is_empty() {
        msgs.push(format!("{} cobros de comisiones pendientes de cobrar", pendientes.len()));
    }
    if vencidos > 0 {
        msgs.push(format!("{} cobros ya vencidos, ¡cóbralos!", vencidos));
    }
    if !creditos.is_empty() {
        msgs.push(format!("{} créditos vencidos", creditos.len()));
    }
    if !stock.is_empty() {
        msgs.push(format!("{} productos con stock bajo", stock.len()));
    }
    Json(serde_json::json!({
        "cobros_pendientes": pendientes.len(),
        "cobros_vencidos": vencidos,
        "creditos_vencidos": creditos.len(),
        "stock_bajo": stock.len(),
        "mensajes": msgs,
    }))
}

// ---------------------------------------------------------------------------
// Clientes
// ---------------------------------------------------------------------------
async fn list_clientes(State(state): State<AppStateRef>) -> Json<Vec<Cliente>> {
    Json(state.db.listar_clientes().unwrap_or_default())
}
async fn create_cliente(State(state): State<AppStateRef>, Json(data): Json<ClienteNuevo>) -> Json<&'static str> {
    match state.db.crear_cliente(&data) { Ok(_) => Json("ok"), Err(_) => Json("error") }
}
async fn update_cliente(State(state): State<AppStateRef>, axum::extract::Path(id): axum::extract::Path<i64>, Json(data): Json<ClienteNuevo>) -> Json<&'static str> {
    match state.db.actualizar_cliente(id, &data) { Ok(_) => Json("ok"), Err(_) => Json("error") }
}
async fn delete_cliente(State(state): State<AppStateRef>, axum::extract::Path(id): axum::extract::Path<i64>) -> Json<&'static str> {
    match state.db.eliminar_cliente(id) { Ok(_) => Json("ok"), Err(_) => Json("error") }
}

// ---------------------------------------------------------------------------
// Proveedores
// ---------------------------------------------------------------------------
async fn list_proveedores(State(state): State<AppStateRef>) -> Json<Vec<Proveedor>> {
    Json(state.db.listar_proveedores().unwrap_or_default())
}
async fn create_proveedor(State(state): State<AppStateRef>, Json(data): Json<ProveedorNuevo>) -> Json<&'static str> {
    match state.db.crear_proveedor(&data) { Ok(_) => Json("ok"), Err(_) => Json("error") }
}

// ---------------------------------------------------------------------------
// Productos
// ---------------------------------------------------------------------------
async fn list_productos(State(state): State<AppStateRef>) -> Json<Vec<Producto>> {
    Json(state.db.listar_productos().unwrap_or_default())
}

// ---------------------------------------------------------------------------
// Categorías de gastos (para el móvil)
// ---------------------------------------------------------------------------
async fn list_categorias_gastos(State(state): State<AppStateRef>) -> Json<Vec<CategoriaGasto>> {
    Json(state.db.listar_categorias_gastos().unwrap_or_default())
}

// ---------------------------------------------------------------------------
// Categorías de productos (para el móvil)
// ---------------------------------------------------------------------------
async fn list_categorias_productos(State(state): State<AppStateRef>) -> Json<Vec<CategoriaProducto>> {
    Json(state.db.listar_categorias_productos().unwrap_or_default())
}
async fn create_producto(State(state): State<AppStateRef>, Json(data): Json<ProductoNuevo>) -> Json<&'static str> {
    match state.db.crear_producto(&data) { Ok(_) => Json("ok"), Err(_) => Json("error") }
}

// ---------------------------------------------------------------------------
// Ventas
// ---------------------------------------------------------------------------
async fn list_ventas(State(state): State<AppStateRef>) -> Json<Vec<Venta>> {
    Json(state.db.listar_ventas().unwrap_or_default())
}
async fn create_venta(State(state): State<AppStateRef>, Json(data): Json<VentaNueva>) -> Json<&'static str> {
    match state.db.crear_venta(&data) { Ok(_) => Json("ok"), Err(_) => Json("error") }
}
async fn delete_venta(State(state): State<AppStateRef>, axum::extract::Path(id): axum::extract::Path<i64>) -> Json<&'static str> {
    match state.db.eliminar_venta(id) { Ok(_) => Json("ok"), Err(_) => Json("error") }
}
async fn venta_detalles(State(state): State<AppStateRef>, axum::extract::Path(id): axum::extract::Path<i64>) -> Json<Vec<VentaDetalle>> {
    Json(state.db.obtener_detalles_venta(id).unwrap_or_default())
}

// ---------------------------------------------------------------------------
// Gastos
// ---------------------------------------------------------------------------
async fn list_gastos(State(state): State<AppStateRef>) -> Json<Vec<Gasto>> {
    Json(state.db.listar_gastos().unwrap_or_default())
}
async fn create_gasto(State(state): State<AppStateRef>, Json(data): Json<GastoNuevo>) -> Json<&'static str> {
    match state.db.crear_gasto(&data) { Ok(_) => Json("ok"), Err(_) => Json("error") }
}
async fn delete_gasto(State(state): State<AppStateRef>, axum::extract::Path(id): axum::extract::Path<i64>) -> Json<&'static str> {
    match state.db.eliminar_gasto(id) { Ok(_) => Json("ok"), Err(_) => Json("error") }
}

// ---------------------------------------------------------------------------
// KPIs
// ---------------------------------------------------------------------------
async fn kpi_ventas_hoy(State(state): State<AppStateRef>) -> Json<f64> { Json(state.db.kpi_ventas_hoy().unwrap_or(0.0)) }
async fn kpi_gastos_hoy(State(state): State<AppStateRef>) -> Json<f64> { Json(state.db.kpi_gastos_hoy().unwrap_or(0.0)) }
async fn kpi_cxc(State(state): State<AppStateRef>) -> Json<f64> { Json(state.db.kpi_cxc().unwrap_or(0.0)) }
async fn kpi_cxp(State(state): State<AppStateRef>) -> Json<f64> { Json(state.db.kpi_cxp().unwrap_or(0.0)) }
async fn kpi_utilidad_mes(State(state): State<AppStateRef>) -> Json<f64> { Json(state.db.kpi_utilidad_mes().unwrap_or(0.0)) }

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let db_path = args.get(1).cloned().unwrap_or_else(|| "contabilidad_rust.db".to_string());
    let port: u16 = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(8080);

    let state = Arc::new(AppState {
        db: DatabaseManager::new(&db_path).expect("Error al abrir la base de datos"),
        tokens: Mutex::new(Vec::new()),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let api = Router::new()
        .route("/api/clientes", get(list_clientes).post(create_cliente))
        .route("/api/clientes/{id}", put(update_cliente).delete(delete_cliente))
        .route("/api/proveedores", get(list_proveedores).post(create_proveedor))
        .route("/api/productos", get(list_productos).post(create_producto))
        .route("/api/categorias-gasto", get(list_categorias_gastos))
        .route("/api/categorias-producto", get(list_categorias_productos))
        .route("/api/migrar", post(migrar))
        .route("/api/ventas", get(list_ventas).post(create_venta))
        .route("/api/ventas/{id}", delete(delete_venta))
        .route("/api/ventas/{id}/detalles", get(venta_detalles))
        .route("/api/gastos", get(list_gastos).post(create_gasto))
        .route("/api/gastos/{id}", delete(delete_gasto))
        .route("/api/dashboard", get(dashboard))
        .route("/api/maquinas", get(list_maquinas))
        .route("/api/cobros/pendientes", get(cobros_pendientes))
        .route("/api/cobros", get(list_cobros).post(crear_cobro))
        .route("/api/pagos/recibidos", get(list_pagos_recibidos).post(crear_pago_recibido))
        .route("/api/alertas", get(alertas))
        .route("/api/kpi/ventas-hoy", get(kpi_ventas_hoy))
        .route("/api/kpi/gastos-hoy", get(kpi_gastos_hoy))
        .route("/api/kpi/cxc", get(kpi_cxc))
        .route("/api/kpi/cxp", get(kpi_cxp))
        .route("/api/kpi/utilidad-mes", get(kpi_utilidad_mes))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let public = Router::new()
        .route("/api/login", post(login))
        .route("/api/login/qr", post(login_qr));

    let pwa_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("pwa")))
        .filter(|p| p.exists())
        .unwrap_or_else(|| std::path::PathBuf::from("pwa"));

    let app = Router::new()
        .merge(public)
        .merge(api)
        .fallback_service(ServeDir::new(&pwa_dir).append_index_html_on_directories(true))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("=== Servidor Contabilidad (PC) ===");
    println!("Base de datos: {}", db_path);
    println!("API + App:    http://localhost:{}", port);
    println!("PWA estática: {}", pwa_dir.display());
    println!("Para que el celular acceda desde internet, expón este puerto con un túnel");
    println!("(ej: bore local {} --to bore.pub  o  cloudflared tunnel --url http://localhost:{})", port, port);
    println!("================================");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
