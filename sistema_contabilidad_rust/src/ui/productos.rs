use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{Producto, MovimientoInventario};
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, labeled_input_i32, form_two_columns, texto_error};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProductoFormData {
    pub codigo: String, pub nombre: String, pub descripcion: String,
    pub precio_compra: String, pub precio_venta: String,
    pub stock: String, pub stock_minimo: String, pub unidad: String,
}

impl Default for ProductoFormData {
    fn default() -> Self {
        Self { codigo: String::new(), nombre: String::new(), descripcion: String::new(),
            precio_compra: String::new(), precio_venta: String::new(),
            stock: String::new(), stock_minimo: String::new(), unidad: String::new() }
    }
}

#[derive(Debug, Clone)]
pub struct ProductosState {
    pub productos: Vec<Producto>, pub busqueda: String,
    pub show_form: bool, pub editing_id: Option<i64>, pub form: ProductoFormData,
    pub errores: HashMap<String, String>,
    pub show_ajuste: bool,
    pub ajuste_stock: String,
    pub ajuste_motivo: String,
    pub ajuste_producto_id: Option<i64>,
    pub show_movimientos: bool,
    pub movimientos: Vec<MovimientoInventario>,
    pub mov_producto_id: Option<i64>,
}

impl Default for ProductosState {
    fn default() -> Self {
        Self {
            productos: Vec::new(), busqueda: String::new(), show_form: false, editing_id: None,
            form: ProductoFormData::default(), errores: HashMap::new(),
            show_ajuste: false, ajuste_stock: String::new(), ajuste_motivo: String::new(),
            ajuste_producto_id: None, show_movimientos: false, movimientos: Vec::new(),
            mov_producto_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProductoFormMessage {
    Codigo(String), Nombre(String), Descripcion(String),
    PrecioCompra(String), PrecioVenta(String), Stock(String),
    StockMinimo(String), Unidad(String), Guardar, Cancelar,
    AbrirAjuste(i64), AjusteStock(String), AjusteMotivo(String), GuardarAjuste, CerrarAjuste,
    AbrirMovimientos(i64), CerrarMovimientos,
}

pub fn productos_view<'a, Message: 'a + Clone>(
    state: &'a ProductosState,
    on_crear: Message, on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone, on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_form_msg: impl Fn(ProductoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_ajuste { return render_ajuste(state, on_form_msg); }
    if state.show_movimientos { return render_movimientos(state, on_form_msg); }
    if state.show_form { return render_form(state, on_form_msg); }

    let filtrados: Vec<&Producto> = if state.busqueda.is_empty() {
        state.productos.iter().filter(|p| p.activo).collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.productos.iter().filter(|p| p.activo && (
            p.nombre.to_lowercase().contains(&q) || p.codigo.as_deref().unwrap_or("").to_lowercase().contains(&q)
        )).collect()
    };

    let stock_bajo = filtrados.iter().filter(|p| p.stock <= p.stock_minimo).count();
    let valor_inv: f64 = filtrados.iter().map(|p| p.stock as f64 * p.precio_compra).sum();

    let kpis = row![
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Productos activos".to_string(),
            valor: filtrados.len().to_string(),
            subtitulo: "en el catalogo".to_string(),
            color: COLOR_ACCENT, icono: '\u{2603}',
        }),
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Stock bajo o agotado".to_string(),
            valor: stock_bajo.to_string(),
            subtitulo: "reponer inventario".to_string(),
            color: if stock_bajo > 0 { COLOR_DANGER } else { COLOR_SUCCESS }, icono: '\u{26A0}',
        }),
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Valor del inventario".to_string(),
            valor: format!("${:.2}", valor_inv),
            subtitulo: "a precio de compra".to_string(),
            color: COLOR_VENTAS, icono: '\u{25BC}',
        }),
    ].spacing(SPACING_MD).width(Length::Fill);

    let header = row![
        text("Productos").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        text_input("Buscar productos...", &state.busqueda)
            .on_input(on_buscar).style(|_, _| input_style()).width(220),
        button(text("+ Nuevo").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style()).on_press(on_crear).padding([SPACING_SM, SPACING_MD]),
    ].spacing(SPACING_MD).align_y(Alignment::Center);

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|p| {
        let id = p.id;
        let stock_color = if p.stock <= p.stock_minimo { COLOR_DANGER } else { COLOR_SUCCESS };
        row![
            text(p.codigo.as_deref().unwrap_or("")).size(12).color(COLOR_ACCENT).width(Length::FillPortion(1)),
            text(&p.nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(format!("${:.0}", p.precio_venta)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            text(format!("{}", p.stock)).size(12).color(stock_color).width(Length::FillPortion(1)),
            text(&p.unidad).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(1)),
            row![
                button(text("\u{25C6} Ajustar").size(11).color(COLOR_TEXT_PRIMARY)).style(|_, _| secondary_button_style()).on_press(on_form_msg(ProductoFormMessage::AbrirAjuste(id))).padding([4, 6]),
                button(text("\u{2263}").size(12)).style(|_, _| ghost_button_style()).on_press(on_form_msg(ProductoFormMessage::AbrirMovimientos(id))).padding([4, 6]),
                button(text("\u{270E}").size(12)).style(|_, _| ghost_button_style()).on_press(on_editar(id)).padding([4, 6]),
                button(text("\u{2715}").size(11)).style(|_, _| ghost_button_style()).on_press(on_eliminar(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(2)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    column![
        kpis,
        Space::new().height(Length::Fixed(SPACING_MD)),
        header, Space::new().height(Length::Fixed(SPACING_SM)),
        scrollable(column(rows).spacing(2.0).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].padding(SPACING_LG).spacing(SPACING_SM).into()
}

fn render_ajuste<'a, Message: 'a + Clone>(
    state: &'a ProductosState,
    on_form_msg: impl Fn(ProductoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let producto = state.productos.iter().find(|p| Some(p.id) == state.ajuste_producto_id);
    let mut fields: Vec<Element<'a, ProductoFormMessage>> = Vec::new();
    if let Some(p) = producto {
        fields.push(container(column![
            text(&p.nombre).size(15).color(COLOR_TEXT_PRIMARY),
            text(format!("Stock actual: {}", p.stock)).size(12).color(COLOR_TEXT_SECONDARY),
        ].spacing(SPACING_XS).padding(SPACING_MD))
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color { a: 0.06, ..COLOR_BG })),
            border: iced::Border { radius: RADIUS_MD.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
        }).into());
    }
    fields.push(labeled_input_i32("Nuevo stock", &state.ajuste_stock, "Ej: 15", ProductoFormMessage::AjusteStock));
    fields.push(labeled_input("Motivo del ajuste", &state.ajuste_motivo, "Ej: inventario fisico, merma, error", ProductoFormMessage::AjusteMotivo));
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, ProductoFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card("Ajuste de Stock", fields.into_iter().map(map_fn), Some(guardar(ProductoFormMessage::GuardarAjuste)), cancelar(ProductoFormMessage::CerrarAjuste), "Aplicar Ajuste")
}

fn render_movimientos<'a, Message: 'a + Clone>(
    state: &'a ProductosState,
    on_form_msg: impl Fn(ProductoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let producto = state.productos.iter().find(|p| Some(p.id) == state.mov_producto_id);
    let mut fields: Vec<Element<'a, ProductoFormMessage>> = vec![
        row![
            text(format!("Movimientos de {}", producto.map(|p| p.nombre.as_str()).unwrap_or("producto"))).size(16).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
        ].into(),
        Space::new().height(Length::Fixed(SPACING_SM)).into(),
        row![
            text("Fecha").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text("Tipo").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            text("Cant.").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            text("Motivo / Referencia").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(4)),
        ].spacing(SPACING_SM).padding([SPACING_SM, SPACING_MD]).into(),
    ];
    for m in &state.movimientos {
        let (tipo_txt, color) = match m.tipo.as_str() {
            "entrada" => ("ENTRADA", COLOR_SUCCESS),
            "salida" => ("SALIDA", COLOR_DANGER),
            _ => ("AJUSTE", COLOR_ACCENT),
        };
        let motivo = format!("{} · {}", m.motivo.clone().unwrap_or_default(), m.referencia.clone().unwrap_or_default());
        fields.push(row![
            text(&m.fecha).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(tipo_txt).size(11).color(color).width(Length::FillPortion(1)),
            text(format!("{}", m.cantidad)).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(1)),
            text(motivo).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(4)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into());
    }
    let cerrar = on_form_msg.clone();
    let map_fn = move |f: Element<'a, ProductoFormMessage>| { let cb = cerrar.clone(); f.map(move |msg| cb(msg)) };
    let cancel = on_form_msg;
    form_card("Historial de Inventario", fields.into_iter().map(map_fn), None, cancel(ProductoFormMessage::CerrarMovimientos), "")
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a ProductosState,
    on_form_msg: impl Fn(ProductoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Producto" } else { "Nuevo Producto" };
    let mut fields: Vec<Element<'a, ProductoFormMessage>> = vec![
        form_two_columns(labeled_input("Código", &state.form.codigo, "PROD-001", ProductoFormMessage::Codigo), labeled_input("Nombre", &state.form.nombre, "Nombre del producto", ProductoFormMessage::Nombre)),
    ];
    if let Some(err) = texto_error("nombre", &state.errores) { fields.push(err); }
    fields.extend(vec![
        labeled_input("Descripción", &state.form.descripcion, "Descripción del producto", ProductoFormMessage::Descripcion),
        form_two_columns(labeled_input_f64("Precio Compra", &state.form.precio_compra, "0.00", ProductoFormMessage::PrecioCompra), labeled_input_f64("Precio Venta", &state.form.precio_venta, "0.00", ProductoFormMessage::PrecioVenta)),
        form_two_columns(labeled_input_i32("Stock", &state.form.stock, "0", ProductoFormMessage::Stock), labeled_input_i32("Stock Mínimo", &state.form.stock_minimo, "0", ProductoFormMessage::StockMinimo)),
        labeled_input("Unidad", &state.form.unidad, "pza, kg, m, etc.", ProductoFormMessage::Unidad),
    ]);
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, ProductoFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(ProductoFormMessage::Guardar)), cancelar(ProductoFormMessage::Cancelar), "Guardar")
}
