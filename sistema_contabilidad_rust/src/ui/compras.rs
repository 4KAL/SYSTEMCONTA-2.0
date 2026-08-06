use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{Compra, CompraDetalle, MovimientoInventario};
use crate::theme::*;
use super::forms::{form_card, labeled_input, pick_list_field, SelectOption, form_two_columns};

static OPCIONES_METODO: std::sync::LazyLock<Vec<SelectOption>> = std::sync::LazyLock::new(|| vec![
    SelectOption { id: 1, label: "efectivo".to_string() },
    SelectOption { id: 2, label: "transferencia".to_string() },
    SelectOption { id: 3, label: "tarjeta".to_string() },
    SelectOption { id: 4, label: "cheque".to_string() },
    SelectOption { id: 5, label: "credito".to_string() },
    SelectOption { id: 6, label: "otro".to_string() },
]);

#[derive(Debug, Clone)]
pub struct CompraItemData { pub producto_id: Option<i64>, pub producto_nombre: String, pub cantidad: String, pub precio: String }

#[derive(Debug, Clone)]
pub struct CompraFormData {
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub metodo_pago: String,
    pub notas: String,
    pub items: Vec<CompraItemData>,
}

impl Default for CompraFormData {
    fn default() -> Self {
        Self { proveedor_id: None, proveedor_nombre: String::new(), metodo_pago: "efectivo".to_string(), notas: String::new(), items: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct ComprasState {
    pub compras: Vec<Compra>,
    pub movimientos: Vec<MovimientoInventario>,
    pub busqueda: String,
    pub show_form: bool,
    pub form: CompraFormData,
    pub opciones_productos: Vec<SelectOption>,
    pub opciones_proveedores: Vec<SelectOption>,
    pub show_detail: bool,
    pub detail_lineas: Vec<CompraDetalle>,
    pub show_movimientos: bool,
    pub filtro_mov: String,
}

impl Default for ComprasState {
    fn default() -> Self {
        Self {
            compras: Vec::new(), movimientos: Vec::new(), busqueda: String::new(),
            show_form: false, form: CompraFormData::default(),
            opciones_productos: Vec::new(), opciones_proveedores: Vec::new(),
            show_detail: false, detail_lineas: Vec::new(), show_movimientos: false,
            filtro_mov: "todos".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompraFormMessage {
    ProveedorSeleccionado(i64),
    ProveedorNombre(String),
    MetodoPago(String),
    Notas(String),
    ItemProducto(usize, i64),
    ItemCantidad(usize, String),
    ItemPrecio(usize, String),
    AgregarItem,
    QuitarItem(usize),
    Guardar,
    Cancelar,
    CerrarDetalle,
    AbrirMovimientos,
    CerrarMovimientos,
    FiltroMov(String),
}

static OPCIONES_TIPO_MOV: std::sync::LazyLock<Vec<SelectOption>> = std::sync::LazyLock::new(|| vec![
    SelectOption { id: 0, label: "todos".to_string() },
    SelectOption { id: 1, label: "entrada".to_string() },
    SelectOption { id: 2, label: "salida".to_string() },
    SelectOption { id: 3, label: "ajuste".to_string() },
]);

pub fn compras_view<'a, Message: 'a + Clone>(
    state: &'a ComprasState,
    on_nueva: Message,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_ver_detalle: impl Fn(i64) -> Message + 'a + Clone,
    on_form_msg: impl Fn(CompraFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }
    if state.show_movimientos { return render_movimientos(state, on_form_msg); }
    if state.show_detail { return render_detalle(state, on_form_msg); }
    render_list(state, on_nueva, on_eliminar, on_buscar, on_ver_detalle, on_form_msg)
}

fn render_list<'a, Message: 'a + Clone>(
    state: &'a ComprasState,
    on_nueva: Message,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_ver_detalle: impl Fn(i64) -> Message + 'a + Clone,
    on_form_msg: impl Fn(CompraFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let total_compras: f64 = state.compras.iter().map(|c| c.total).sum();
    let mov = &state.movimientos;
    let entradas: i32 = mov.iter().filter(|m| m.tipo == "entrada").map(|m| m.cantidad).sum();
    let salidas: i32 = mov.iter().filter(|m| m.tipo == "salida").map(|m| m.cantidad).sum();
    let ajustes: i32 = mov.iter().filter(|m| m.tipo == "ajuste").count() as i32;

    let kpis = row![
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Total en compras".to_string(),
            valor: format!("${:.2}", total_compras),
            subtitulo: format!("{} compra(s)", state.compras.len()),
            color: COLOR_CXC, icono: '\u{2190}',
        }),
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Entradas a inventario".to_string(),
            valor: format!("{}", entradas),
            subtitulo: "unidades recibidas".to_string(),
            color: COLOR_SUCCESS, icono: '\u{2193}',
        }),
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Salidas de inventario".to_string(),
            valor: format!("{}", salidas),
            subtitulo: "unidades vendidas".to_string(),
            color: COLOR_VENTAS, icono: '\u{2191}',
        }),
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Ajustes manuales".to_string(),
            valor: format!("{}", ajustes),
            subtitulo: "correcciones de stock".to_string(),
            color: COLOR_ACCENT, icono: '\u{270E}',
        }),
    ].spacing(SPACING_MD).width(Length::Fill);

    let abrir_mov = on_form_msg.clone();
    let header = row![
        text("Compras y Almacén").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        text_input("Buscar compras...", &state.busqueda)
            .on_input(on_buscar).style(|_, _| input_style()).width(200),
        button(text("\u{2263} Movimientos").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| secondary_button_style())
            .on_press(abrir_mov(CompraFormMessage::AbrirMovimientos))
            .padding([SPACING_SM, SPACING_MD]),
        button(text("+ Nueva Compra").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(on_nueva)
            .padding([SPACING_SM, SPACING_MD]),
    ]
    .spacing(SPACING_MD)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let filtrados: Vec<&Compra> = state.compras.iter().filter(|c| {
        if state.busqueda.is_empty() { return true; }
        let q = state.busqueda.to_lowercase();
        c.numero.to_lowercase().contains(&q) || c.proveedor_nombre.to_lowercase().contains(&q)
    }).collect();

    let col_header = row![
        text("Compra").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
        text("Proveedor").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        text("Fecha").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
        text("Subtotal").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        text("Total").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        text("Pago").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
    ]
    .spacing(SPACING_SM)
    .padding([SPACING_SM, SPACING_MD]);

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|c| {
        let id = c.id;
        let el = on_eliminar.clone();
        let det = on_ver_detalle.clone();
        row![
            text(&c.numero).size(11).color(COLOR_ACCENT).width(Length::FillPortion(2)),
            text(&c.proveedor_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&c.fecha).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(format!("${:.2}", c.subtotal)).size(12).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(1)),
            text(format!("${:.2}", c.total)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            text(c.metodo_pago.as_deref().unwrap_or("")).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(1)),
            row![
                button(text("\u{2630}").size(12)).style(|_, _| ghost_button_style()).on_press(det(id)).padding([4, 6]),
                button(text("\u{2715}").size(11)).style(|_, _| ghost_button_style()).on_press(el(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(1)),
        ]
        .spacing(SPACING_SM)
        .align_y(Alignment::Center)
        .padding([SPACING_SM, SPACING_MD])
        .into()
    }).collect();

    let body: Element<'a, Message> = if filtrados.is_empty() {
        container(column![
            text("No hay compras registradas").size(16).color(COLOR_TEXT_SECONDARY),
            text("Registra la primera compra para dar entrada al inventario").size(12).color(COLOR_TEXT_MUTED),
        ].spacing(SPACING_SM).align_x(Alignment::Center))
        .center(Length::Fill).width(Length::Fill).height(300).into()
    } else {
        scrollable(column(
            std::iter::once(col_header.into()).chain(rows)
        ).spacing(2.0).width(Length::Fill))
            .style(|_, _| scrollable_style()).width(Length::Fill).height(Length::Fill).into()
    };

    column![
        kpis,
        Space::new().height(SPACING_MD),
        header,
        Space::new().height(SPACING_SM),
        body,
    ]
    .padding(SPACING_LG)
    .spacing(SPACING_SM)
    .into()
}

fn render_detalle<'a, Message: 'a + Clone>(
    state: &'a ComprasState,
    on_form_msg: impl Fn(CompraFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let compra = state.compras.iter().find(|c| state.detail_lineas.first().map(|d| d.compra_id == c.id).unwrap_or(false));
    let mut fields: Vec<Element<'a, CompraFormMessage>> = Vec::new();
    if let Some(c) = compra {
        fields.push(row![
            column![text("Compra").size(10).color(COLOR_TEXT_MUTED), text(&c.numero).size(14).color(COLOR_ACCENT),].spacing(2).width(Length::FillPortion(1)),
            column![text("Proveedor").size(10).color(COLOR_TEXT_MUTED), text(&c.proveedor_nombre).size(14).color(COLOR_TEXT_PRIMARY),].spacing(2).width(Length::FillPortion(2)),
            column![text("Fecha").size(10).color(COLOR_TEXT_MUTED), text(&c.fecha).size(14).color(COLOR_TEXT_PRIMARY),].spacing(2).width(Length::FillPortion(1)),
            column![text("Total").size(10).color(COLOR_TEXT_MUTED), text(format!("${:.2}", c.total)).size(14).color(COLOR_VENTAS),].spacing(2).width(Length::FillPortion(1)),
        ].spacing(SPACING_MD).align_y(Alignment::Center).into());
        fields.push(Space::new().height(Length::Fixed(SPACING_SM)).into());
        fields.push(row![
            text("Producto").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
            text("Cantidad").size(10).color(COLOR_TEXT_MUTED).width(Length::Fixed(60.0)),
            text("P. Compra").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            text("Descuento").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            text("Importe").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        ].spacing(SPACING_SM).padding([SPACING_SM, SPACING_MD]).into());
        for det in &state.detail_lineas {
            fields.push(row![
                text(&det.producto_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                text(format!("{}", det.cantidad)).size(12).color(COLOR_TEXT_PRIMARY).width(Length::Fixed(60.0)),
                text(format!("${:.2}", det.precio_unitario)).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(1)),
                text(format!("${:.2}", det.descuento)).size(12).color(COLOR_DANGER).width(Length::FillPortion(1)),
                text(format!("${:.2}", det.importe)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            ].spacing(SPACING_SM).padding([SPACING_SM, SPACING_MD]).into());
        }
    }
    let cerrar = on_form_msg.clone();
    let map_fn = move |f: Element<'a, CompraFormMessage>| { let cb = cerrar.clone(); f.map(move |_| cb(CompraFormMessage::CerrarDetalle)) };
    let cancel = on_form_msg;
    form_card("Detalle de Compra", fields.into_iter().map(map_fn), None, cancel(CompraFormMessage::CerrarDetalle), "")
}

fn render_movimientos<'a, Message: 'a + Clone>(
    state: &'a ComprasState,
    on_form_msg: impl Fn(CompraFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let filtro_id: i64 = match state.filtro_mov.as_str() {
        "entrada" => 1, "salida" => 2, "ajuste" => 3, _ => 0,
    };
    let filtrados: Vec<&MovimientoInventario> = state.movimientos.iter()
        .filter(|m| state.filtro_mov == "todos" || m.tipo == state.filtro_mov)
        .collect();
    let mut fields: Vec<Element<'a, CompraFormMessage>> = vec![
        row![
            text("Historial de movimientos de inventario").size(16).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            pick_list(&OPCIONES_TIPO_MOV[..], OPCIONES_TIPO_MOV.iter().find(|o| o.id == filtro_id), move |o| CompraFormMessage::FiltroMov(o.label.clone()))
                .style(|_, _| pick_list_style())
                .menu_style(|_| menu_style())
                .padding([8, 12]),
        ].align_y(Alignment::Center).into(),
        Space::new().height(Length::Fixed(SPACING_SM)).into(),
        row![
            text("Fecha").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text("Producto").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
            text("Tipo").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            text("Cant.").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            text("Motivo / Referencia").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        ].spacing(SPACING_SM).padding([SPACING_SM, SPACING_MD]).into(),
    ];
    for m in &filtrados {
        let (tipo_txt, color) = match m.tipo.as_str() {
            "entrada" => ("ENTRADA", COLOR_SUCCESS),
            "salida" => ("SALIDA", COLOR_DANGER),
            _ => ("AJUSTE", COLOR_ACCENT),
        };
        let motivo = format!("{} · {}", m.motivo.clone().unwrap_or_default(), m.referencia.clone().unwrap_or_default());
        fields.push(row![
            text(&m.fecha).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&m.producto_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(tipo_txt).size(11).color(color).width(Length::FillPortion(1)),
            text(format!("{}", m.cantidad)).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(1)),
            text(motivo).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into());
    }
    let cerrar = on_form_msg.clone();
    let map_fn = move |f: Element<'a, CompraFormMessage>| { let cb = cerrar.clone(); f.map(move |msg| cb(msg)) };
    let cancel = on_form_msg;
    form_card("Movimientos de Inventario", fields.into_iter().map(map_fn), None, cancel(CompraFormMessage::CerrarMovimientos), "")
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a ComprasState,
    on_form_msg: impl Fn(CompraFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let met_id: i64 = match state.form.metodo_pago.as_str() {
        "transferencia" => 2, "tarjeta" => 3, "cheque" => 4, "credito" => 5, "otro" => 6, _ => 1,
    };
    let mut fields: Vec<Element<'a, CompraFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Proveedor", &state.opciones_proveedores, state.form.proveedor_id.unwrap_or(0), move |id| CompraFormMessage::ProveedorSeleccionado(id)),
            pick_list_field("Método de pago", &*OPCIONES_METODO, met_id, move |id| {
                let val = OPCIONES_METODO.iter().find(|o| o.id == id).map(|o| o.label.clone()).unwrap_or_else(|| "efectivo".to_string());
                CompraFormMessage::MetodoPago(val)
            }),
        ),
    ];
    if state.form.proveedor_id.is_none() || state.form.proveedor_id == Some(0) {
        fields.push(labeled_input("Nombre del proveedor (nuevo)", &state.form.proveedor_nombre, "Nombre del proveedor", CompraFormMessage::ProveedorNombre));
    }
    fields.push(labeled_input("Notas", &state.form.notas, "Notas de la compra", CompraFormMessage::Notas));
    for (i, item) in state.form.items.iter().enumerate() {
        let campo_producto: Element<'a, CompraFormMessage> =
            container(pick_list_field("Producto", &state.opciones_productos, item.producto_id.unwrap_or(0), move |id| CompraFormMessage::ItemProducto(i, id)))
                .width(Length::FillPortion(3)).into();
        fields.push(
            row![
                text(format!("{}.", i+1)).size(11).color(COLOR_TEXT_MUTED).width(Length::Fixed(20.0)),
                campo_producto,
                text_input("Cant", &item.cantidad).on_input(move |v| CompraFormMessage::ItemCantidad(i, v)).style(|_, _| input_style()).width(Length::FillPortion(1)),
                text_input("P. Compra", &item.precio).on_input(move |v| CompraFormMessage::ItemPrecio(i, v)).style(|_, _| input_style()).width(Length::FillPortion(1)),
                button(text("\u{2715}").size(10).color(COLOR_DANGER)).style(|_, _| ghost_button_style()).on_press(CompraFormMessage::QuitarItem(i)).padding([4, 6]),
            ].spacing(SPACING_XS).align_y(Alignment::Center).into()
        );
    }
    fields.push(button(text("+ Agregar Producto").size(12).color(COLOR_ACCENT)).style(|_, _| secondary_button_style()).on_press(CompraFormMessage::AgregarItem).padding(SPACING_SM).into());
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, CompraFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card("Nueva Compra", fields.into_iter().map(map_fn), Some(guardar(CompraFormMessage::Guardar)), cancelar(CompraFormMessage::Cancelar), "Guardar Compra")
}
