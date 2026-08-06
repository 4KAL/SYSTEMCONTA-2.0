use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{Cotizacion, CotizacionDetalle};
use crate::theme::*;
use super::forms::{form_card, labeled_input, pick_list_field, SelectOption, form_two_columns};

static OPCIONES_ESTADO: std::sync::LazyLock<Vec<SelectOption>> = std::sync::LazyLock::new(|| vec![
    SelectOption { id: 0, label: "todas".to_string() },
    SelectOption { id: 1, label: "vigente".to_string() },
    SelectOption { id: 2, label: "convertida".to_string() },
    SelectOption { id: 3, label: "vencida".to_string() },
    SelectOption { id: 4, label: "cancelada".to_string() },
]);

#[derive(Debug, Clone)]
pub struct CotizacionItemData { pub producto_id: Option<i64>, pub producto_nombre: String, pub cantidad: String, pub precio: String }

#[derive(Debug, Clone)]
pub struct CotizacionFormData {
    pub cliente_id: Option<i64>,
    pub cliente_nombre: String,
    pub validez_dias: String,
    pub notas: String,
    pub items: Vec<CotizacionItemData>,
}

impl Default for CotizacionFormData {
    fn default() -> Self {
        Self { cliente_id: None, cliente_nombre: String::new(), validez_dias: "7".to_string(), notas: String::new(), items: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct CotizacionesState {
    pub cotizaciones: Vec<Cotizacion>,
    pub busqueda: String,
    pub filtro_estado: String,
    pub show_form: bool,
    pub form: CotizacionFormData,
    pub opciones_productos: Vec<SelectOption>,
    pub opciones_clientes: Vec<SelectOption>,
    pub show_detail: bool,
    pub detail_lineas: Vec<CotizacionDetalle>,
    pub show_convertir: bool,
}

impl Default for CotizacionesState {
    fn default() -> Self {
        Self {
            cotizaciones: Vec::new(), busqueda: String::new(), filtro_estado: "todas".to_string(),
            show_form: false, form: CotizacionFormData::default(),
            opciones_productos: Vec::new(), opciones_clientes: Vec::new(),
            show_detail: false, detail_lineas: Vec::new(), show_convertir: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CotizacionFormMessage {
    ClienteSeleccionado(i64),
    ClienteNombre(String),
    Validez(String),
    Notas(String),
    ItemProducto(usize, i64),
    ItemCantidad(usize, String),
    ItemPrecio(usize, String),
    AgregarItem,
    QuitarItem(usize),
    Guardar,
    Cancelar,
    CerrarDetalle,
    CerrarConvertir,
}

pub fn cotizaciones_view<'a, Message: 'a + Clone>(
    state: &'a CotizacionesState,
    on_nueva: Message,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_filtro_estado: impl Fn(String) -> Message + 'a + Clone,
    on_ver_detalle: impl Fn(i64) -> Message + 'a + Clone,
    on_convertir: impl Fn(i64) -> Message + 'a + Clone,
    on_form_msg: impl Fn(CotizacionFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }
    if state.show_detail { return render_detalle(state, on_form_msg); }
    if state.show_convertir { return render_convertir(state, on_form_msg); }
    render_list(state, on_nueva, on_eliminar, on_buscar, on_filtro_estado, on_ver_detalle, on_convertir)
}

fn render_list<'a, Message: 'a + Clone>(
    state: &'a CotizacionesState,
    on_nueva: Message,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_filtro_estado: impl Fn(String) -> Message + 'a + Clone,
    on_ver_detalle: impl Fn(i64) -> Message + 'a + Clone,
    on_convertir: impl Fn(i64) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let total_vigente: f64 = state.cotizaciones.iter()
        .filter(|c| c.estado == "vigente")
        .map(|c| c.total).sum();
    let vencidas: usize = state.cotizaciones.iter()
        .filter(|c| c.estado == "vigente" && es_vencida(c, &hoy))
        .count();

    let kpis = row![
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Cotizaciones vigentes".to_string(),
            valor: state.cotizaciones.iter().filter(|c| c.estado == "vigente").count().to_string(),
            subtitulo: format!("total ${:.2}", total_vigente),
            color: COLOR_ACCENT, icono: '\u{2709}',
        }),
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Convertidas en venta".to_string(),
            valor: state.cotizaciones.iter().filter(|c| c.estado == "convertida").count().to_string(),
            subtitulo: "aceptadas por el cliente".to_string(),
            color: COLOR_SUCCESS, icono: '\u{21B4}',
        }),
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Vencidas".to_string(),
            valor: vencidas.to_string(),
            subtitulo: "sin aceptar".to_string(),
            color: if vencidas > 0 { COLOR_DANGER } else { COLOR_SUCCESS }, icono: '\u{23F0}',
        }),
        crate::widgets::kpi_card_view(crate::widgets::KpiCard {
            titulo: "Valor promedio".to_string(),
            valor: format!("${:.2}", if state.cotizaciones.is_empty() { 0.0 } else { total_vigente / state.cotizaciones.len() as f64 }),
            subtitulo: "por cotizacion".to_string(),
            color: COLOR_VENTAS, icono: '\u{26A0}',
        }),
    ].spacing(SPACING_MD).width(Length::Fill);

    let estado_id: i64 = match state.filtro_estado.as_str() {
        "vigente" => 1, "convertida" => 2, "vencida" => 3, "cancelada" => 4, _ => 0,
    };

    let header = row![
        text("Cotizaciones").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        text_input("Buscar cotizaciones...", &state.busqueda)
            .on_input(on_buscar).style(|_, _| input_style()).width(200),
        pick_list(&OPCIONES_ESTADO[..], OPCIONES_ESTADO.iter().find(|o| o.id == estado_id), move |o| {
            let cb = on_filtro_estado.clone();
            cb(o.label.clone())
        })
        .style(|_, _| pick_list_style())
        .menu_style(|_| menu_style())
        .padding([8, 12]),
        button(text("+ Nueva Cotización").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(on_nueva)
            .padding([SPACING_SM, SPACING_MD]),
    ]
    .spacing(SPACING_MD)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let filtrados: Vec<&Cotizacion> = state.cotizaciones.iter().filter(|c| {
        if state.filtro_estado != "todas" {
            let real = if c.estado == "vigente" && es_vencida(c, &hoy) { "vencida" } else { &c.estado };
            if real != state.filtro_estado { return false; }
        }
        if !state.busqueda.is_empty() {
            let q = state.busqueda.to_lowercase();
            if !c.numero.to_lowercase().contains(&q) && !c.cliente_nombre.to_lowercase().contains(&q) { return false; }
        }
        true
    }).collect();

    let col_header = row![
        text("No.").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
        text("Cliente").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        text("Fecha").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
        text("Total").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        text("Validez").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        text("Estado").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
    ]
    .spacing(SPACING_SM)
    .padding([SPACING_SM, SPACING_MD]);

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|c| {
        let id = c.id;
        let el = on_eliminar.clone();
        let det = on_ver_detalle.clone();
        let conv = on_convertir.clone();
        let vencida = c.estado == "vigente" && es_vencida(c, &hoy);
        let (estado_txt, estado_color) = if vencida { ("VENCIDA", COLOR_DANGER) }
            else if c.estado == "convertida" { ("convertida", COLOR_SUCCESS) }
            else if c.estado == "cancelada" { ("cancelada", COLOR_TEXT_MUTED) }
            else { ("vigente", COLOR_ACCENT) };
        row![
            text(&c.numero).size(11).color(COLOR_ACCENT).width(Length::FillPortion(2)),
            text(&c.cliente_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&c.fecha).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(format!("${:.2}", c.total)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            text(format!("{} días", c.validez_dias)).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(1)),
            text(estado_txt).size(11).color(estado_color).width(Length::FillPortion(1)),
            row![
                button(text("\u{21B4} Venta").size(11).color(COLOR_TEXT_PRIMARY)).style(|_, _| secondary_button_style()).on_press(conv(id)).padding([4, 6]),
                button(text("\u{2630}").size(12)).style(|_, _| ghost_button_style()).on_press(det(id)).padding([4, 6]),
                button(text("\u{2715}").size(11)).style(|_, _| ghost_button_style()).on_press(el(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(2)),
        ]
        .spacing(SPACING_SM)
        .align_y(Alignment::Center)
        .padding([SPACING_SM, SPACING_MD])
        .into()
    }).collect();

    let body: Element<'a, Message> = if filtrados.is_empty() {
        container(column![
            text("No hay cotizaciones registradas").size(16).color(COLOR_TEXT_SECONDARY),
            text("Crea una cotización para ofrecer tus productos").size(12).color(COLOR_TEXT_MUTED),
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

fn es_vencida(c: &Cotizacion, hoy: &str) -> bool {
    let fecha = chrono::NaiveDate::parse_from_str(&c.fecha, "%Y-%m-%d")
        .unwrap_or(chrono::Local::now().date_naive());
    let vence = fecha + chrono::Duration::days(c.validez_dias as i64);
    let hoy_d = chrono::NaiveDate::parse_from_str(hoy, "%Y-%m-%d")
        .unwrap_or(chrono::Local::now().date_naive());
    hoy_d > vence
}

fn render_detalle<'a, Message: 'a + Clone>(
    state: &'a CotizacionesState,
    on_form_msg: impl Fn(CotizacionFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let cot = state.cotizaciones.iter().find(|c| state.detail_lineas.first().map(|d| d.cotizacion_id == c.id).unwrap_or(false));
    let mut fields: Vec<Element<'a, CotizacionFormMessage>> = Vec::new();
    if let Some(c) = cot {
        fields.push(row![
            column![text("No.").size(10).color(COLOR_TEXT_MUTED), text(&c.numero).size(14).color(COLOR_ACCENT),].spacing(2).width(Length::FillPortion(1)),
            column![text("Cliente").size(10).color(COLOR_TEXT_MUTED), text(&c.cliente_nombre).size(14).color(COLOR_TEXT_PRIMARY),].spacing(2).width(Length::FillPortion(2)),
            column![text("Fecha").size(10).color(COLOR_TEXT_MUTED), text(&c.fecha).size(14).color(COLOR_TEXT_PRIMARY),].spacing(2).width(Length::FillPortion(1)),
            column![text("Total").size(10).color(COLOR_TEXT_MUTED), text(format!("${:.2}", c.total)).size(14).color(COLOR_VENTAS),].spacing(2).width(Length::FillPortion(1)),
        ].spacing(SPACING_MD).align_y(Alignment::Center).into());
        fields.push(Space::new().height(Length::Fixed(SPACING_SM)).into());
        fields.push(row![
            text("Producto").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
            text("Cantidad").size(10).color(COLOR_TEXT_MUTED).width(Length::Fixed(60.0)),
            text("Precio").size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
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
    let map_fn = move |f: Element<'a, CotizacionFormMessage>| { let cb = cerrar.clone(); f.map(move |_| cb(CotizacionFormMessage::CerrarDetalle)) };
    let cancel = on_form_msg;
    form_card("Detalle de Cotización", fields.into_iter().map(map_fn), None, cancel(CotizacionFormMessage::CerrarDetalle), "")
}

fn render_convertir<'a, Message: 'a + Clone>(
    state: &'a CotizacionesState,
    on_form_msg: impl Fn(CotizacionFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let cot = state.cotizaciones.iter().find(|c| state.detail_lineas.first().map(|d| d.cotizacion_id == c.id).unwrap_or(false));
    let mut fields: Vec<Element<'a, CotizacionFormMessage>> = Vec::new();
    if let Some(c) = cot {
        fields.push(container(column![
            text("Convertir en venta").size(16).color(COLOR_ACCENT),
            Space::new().height(SPACING_XS),
            text(format!("La cotización {} por ${:.2} se registrará como venta de contado y descontará el inventario.", c.numero, c.total))
                .size(13).color(COLOR_TEXT_SECONDARY),
        ].spacing(SPACING_SM).padding(SPACING_MD))
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color { a: 0.08, ..COLOR_ACCENT })),
            border: iced::Border { radius: RADIUS_MD.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
        }).width(Length::Fill).into());
    }
    let cerrar = on_form_msg.clone();
    let map_fn = move |f: Element<'a, CotizacionFormMessage>| { let cb = cerrar.clone(); f.map(move |_| cb(CotizacionFormMessage::CerrarConvertir)) };
    let guardar = on_form_msg.clone();
    let cancel = on_form_msg;
    form_card("Convertir Cotización", fields.into_iter().map(map_fn), Some(guardar(CotizacionFormMessage::Guardar)), cancel(CotizacionFormMessage::CerrarConvertir), "Convertir en Venta")
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a CotizacionesState,
    on_form_msg: impl Fn(CotizacionFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let mut fields: Vec<Element<'a, CotizacionFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Cliente", &state.opciones_clientes, state.form.cliente_id.unwrap_or(0), move |id| CotizacionFormMessage::ClienteSeleccionado(id)),
            labeled_input("Validez (días)", &state.form.validez_dias, "7", CotizacionFormMessage::Validez),
        ),
    ];
    if state.form.cliente_id.is_none() || state.form.cliente_id == Some(0) {
        fields.push(labeled_input("Nombre del cliente (nuevo)", &state.form.cliente_nombre, "Nombre del cliente", CotizacionFormMessage::ClienteNombre));
    }
    fields.push(labeled_input("Notas", &state.form.notas, "Condiciones, tiempo de entrega, etc.", CotizacionFormMessage::Notas));
    for (i, item) in state.form.items.iter().enumerate() {
        let campo_producto: Element<'a, CotizacionFormMessage> =
            container(pick_list_field("Producto", &state.opciones_productos, item.producto_id.unwrap_or(0), move |id| CotizacionFormMessage::ItemProducto(i, id)))
                .width(Length::FillPortion(3)).into();
        fields.push(
            row![
                text(format!("{}.", i+1)).size(11).color(COLOR_TEXT_MUTED).width(Length::Fixed(20.0)),
                campo_producto,
                text_input("Cant", &item.cantidad).on_input(move |v| CotizacionFormMessage::ItemCantidad(i, v)).style(|_, _| input_style()).width(Length::FillPortion(1)),
                text_input("Precio", &item.precio).on_input(move |v| CotizacionFormMessage::ItemPrecio(i, v)).style(|_, _| input_style()).width(Length::FillPortion(1)),
                button(text("\u{2715}").size(10).color(COLOR_DANGER)).style(|_, _| ghost_button_style()).on_press(CotizacionFormMessage::QuitarItem(i)).padding([4, 6]),
            ].spacing(SPACING_XS).align_y(Alignment::Center).into()
        );
    }
    fields.push(button(text("+ Agregar Producto").size(12).color(COLOR_ACCENT)).style(|_, _| secondary_button_style()).on_press(CotizacionFormMessage::AgregarItem).padding(SPACING_SM).into());
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, CotizacionFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card("Nueva Cotización", fields.into_iter().map(map_fn), Some(guardar(CotizacionFormMessage::Guardar)), cancelar(CotizacionFormMessage::Cancelar), "Guardar Cotización")
}
