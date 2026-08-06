use std::sync::LazyLock;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{DeudaEmpresa, DeudaPago};
use crate::theme::*;
use crate::widgets::{kpi_card_view, KpiCard};
use super::forms::{form_card, labeled_input, labeled_input_f64, pick_list_field, SelectOption, form_two_columns};

#[derive(Debug, Clone)]
pub struct DeudaFormData {
    pub proveedor_id: Option<i64>,
    pub proveedor_nombre: String,
    pub concepto: String,
    pub descripcion: String,
    pub categoria_id: Option<i64>,
    pub fecha_deuda: String,
    pub fecha_vencimiento: String,
    pub monto_total: String,
    pub referencia: String,
    pub notas: String,
}
impl Default for DeudaFormData {
    fn default() -> Self {
        let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
        Self { proveedor_id: None, proveedor_nombre: String::new(), concepto: String::new(),
            descripcion: String::new(), categoria_id: None, fecha_deuda: hoy,
            fecha_vencimiento: String::new(), monto_total: String::new(),
            referencia: String::new(), notas: String::new() }
    }
}

#[derive(Debug, Clone)]
pub struct DeudaPagoFormData {
    pub monto: String, pub metodo_pago: String, pub referencia: String, pub notas: String,
}
impl Default for DeudaPagoFormData {
    fn default() -> Self { Self { monto: String::new(), metodo_pago: "efectivo".to_string(), referencia: String::new(), notas: String::new() } }
}

#[derive(Debug, Clone)]
pub struct DeudasState {
    pub deudas: Vec<DeudaEmpresa>,
    pub pagos: Vec<DeudaPago>,
    pub show_form: bool,
    pub editing_id: Option<i64>,
    pub busqueda: String,
    pub filtro_estado: String,
    pub form: DeudaFormData,
    pub opciones_proveedores: Vec<SelectOption>,
    pub opciones_categorias: Vec<SelectOption>,
    pub show_detalle: bool,
    pub pagos_deuda: Vec<DeudaPago>,
    pub deuda_seleccionada: Option<i64>,
    pub show_form_pago: bool,
    pub form_pago: DeudaPagoFormData,
}
impl Default for DeudasState {
    fn default() -> Self {
        Self {
            deudas: Vec::new(), pagos: Vec::new(), show_form: false, editing_id: None,
            busqueda: String::new(), filtro_estado: "todas".to_string(),
            form: DeudaFormData::default(), opciones_proveedores: Vec::new(),
            opciones_categorias: Vec::new(), show_detalle: false, pagos_deuda: Vec::new(),
            deuda_seleccionada: None, show_form_pago: false, form_pago: DeudaPagoFormData::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeudaFormMessage {
    ProveedorSeleccionado(i64),
    ProveedorNombre(String),
    Concepto(String),
    Descripcion(String),
    CategoriaSeleccionada(i64),
    FechaDeuda(String),
    FechaVencimiento(String),
    MontoTotal(String),
    Referencia(String),
    Notas(String),
    Guardar,
    Cancelar,
}

#[derive(Debug, Clone)]
pub enum DeudaPagoFormMessage {
    Monto(String), MetodoPago(String), Referencia(String), Notas(String), Guardar, Cancelar,
}

static OPCIONES_ESTADO: LazyLock<Vec<SelectOption>> = LazyLock::new(|| vec![
    SelectOption { id: 0, label: "todas".to_string() },
    SelectOption { id: 1, label: "pendiente".to_string() },
    SelectOption { id: 2, label: "pagada".to_string() },
]);

static OPCIONES_METODO: LazyLock<Vec<SelectOption>> = LazyLock::new(|| vec![
    SelectOption { id: 1, label: "efectivo".to_string() },
    SelectOption { id: 2, label: "transferencia".to_string() },
    SelectOption { id: 3, label: "tarjeta".to_string() },
    SelectOption { id: 4, label: "cheque".to_string() },
    SelectOption { id: 5, label: "otro".to_string() },
]);

fn hoy() -> String { chrono::Local::now().format("%Y-%m-%d").to_string() }

fn pagos_de(state: &DeudasState, deuda_id: i64) -> usize {
    state.pagos.iter().filter(|p| p.deuda_id == deuda_id).count()
}

pub fn deudas_view<'a, Message: 'a + Clone>(
    state: &'a DeudasState,
    on_nueva: Message,
    on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_filtro_estado: impl Fn(String) -> Message + 'a + Clone,
    on_ver_detalle: impl Fn(i64) -> Message + 'a + Clone,
    on_cerrar_detalle: Message,
    on_form_msg: impl Fn(DeudaFormMessage) -> Message + 'a + Clone,
    on_nuevo_pago: Message,
    on_eliminar_pago: impl Fn(i64) -> Message + 'a + Clone,
    on_pago_form_msg: impl Fn(DeudaPagoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }
    if state.show_form_pago { return render_pago_form(state, on_pago_form_msg); }
    if state.show_detalle { return render_detalle(state, on_cerrar_detalle, on_nuevo_pago, on_eliminar_pago); }
    render_list(state, on_nueva, on_editar, on_eliminar, on_buscar, on_filtro_estado, on_ver_detalle)
}

fn render_list<'a, Message: 'a + Clone>(
    state: &'a DeudasState,
    on_nueva: Message,
    on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_filtro_estado: impl Fn(String) -> Message + 'a + Clone,
    on_ver_detalle: impl Fn(i64) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let hoy_s = hoy();
    let total_por_pagar: f64 = state.deudas.iter().filter(|d| d.estado != "pagada").map(|d| d.saldo_pendiente).sum();
    let total_pagado: f64 = state.pagos.iter().map(|p| p.monto).sum();
    let vencidas: usize = state.deudas.iter()
        .filter(|d| d.estado != "pagada" && d.fecha_vencimiento.as_deref().map(|f| f < hoy_s.as_str()).unwrap_or(false))
        .count();

    let kpis = row![
        kpi_card_view(KpiCard { titulo: "Deudas por pagar".to_string(), valor: format!("${:.2}", total_por_pagar), subtitulo: format!("{} deuda(s) activa(s)", state.deudas.iter().filter(|d| d.estado != "pagada").count()), color: COLOR_DANGER, icono: '\u{25C6}' }),
        kpi_card_view(KpiCard { titulo: "Total deudas registradas".to_string(), valor: state.deudas.len().to_string(), subtitulo: "historico".to_string(), color: COLOR_ACCENT, icono: '\u{2261}' }),
        kpi_card_view(KpiCard { titulo: "Total pagado".to_string(), valor: format!("${:.2}", total_pagado), subtitulo: format!("{} pago(s) registrado(s)", state.pagos.len()), color: COLOR_SUCCESS, icono: '\u{2714}' }),
        kpi_card_view(KpiCard { titulo: "Deudas vencidas".to_string(), valor: vencidas.to_string(), subtitulo: "sin pagar al dia de hoy".to_string(), color: if vencidas > 0 { COLOR_DANGER } else { COLOR_SUCCESS }, icono: '\u{23F0}' }),
    ].spacing(SPACING_MD).width(Length::Fill);

    let estado_id: i64 = match state.filtro_estado.as_str() { "pendiente" => 1, "pagada" => 2, _ => 0 };

    let header = row![
        text("Deudas de la Empresa").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        text_input("Buscar proveedor o concepto...", &state.busqueda)
            .on_input(on_buscar).style(|_, _| input_style()).width(200),
        pick_list(&OPCIONES_ESTADO[..], OPCIONES_ESTADO.iter().find(|o| o.id == estado_id), move |o| {
            let cb = on_filtro_estado.clone();
            cb(o.label.clone())
        })
        .style(|_, _| pick_list_style())
        .menu_style(|_| menu_style())
        .padding([8, 12]),
        button(text("+ Nueva Deuda").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(on_nueva)
            .padding([SPACING_SM, SPACING_MD]),
    ]
    .spacing(SPACING_MD)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let filtrados: Vec<&DeudaEmpresa> = state.deudas.iter().filter(|d| {
        if state.filtro_estado != "todas" && d.estado != state.filtro_estado { return false; }
        if !state.busqueda.is_empty() {
            let q = state.busqueda.to_lowercase();
            if !d.proveedor_nombre.to_lowercase().contains(&q)
                && !d.concepto.to_lowercase().contains(&q)
                && !d.numero.to_lowercase().contains(&q) { return false; }
        }
        true
    }).collect();

    let col_header = row![
        text("Deuda").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
        text("A quien se debe").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        text("Concepto").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        text("Desde").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
        text("Total").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        text("Saldo").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        text("Pagos").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        text("Estado").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
    ]
    .spacing(SPACING_SM)
    .padding([SPACING_SM, SPACING_MD]);

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|d| {
        let id = d.id;
        let ed = on_editar.clone();
        let el = on_eliminar.clone();
        let det = on_ver_detalle.clone();
        let n_pagos = pagos_de(state, id);
        let vencida = d.estado != "pagada" && d.fecha_vencimiento.as_deref().map(|f| f < hoy_s.as_str()).unwrap_or(false);
        let (estado_txt, estado_color) = if vencida { ("VENCIDA", COLOR_DANGER) }
            else if d.estado == "pagada" { ("pagada", COLOR_SUCCESS) }
            else { ("pendiente", COLOR_CXP) };
        row![
            text(&d.numero).size(11).color(COLOR_ACCENT).width(Length::FillPortion(2)),
            text(&d.proveedor_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&d.concepto).size(12).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(3)),
            text(&d.fecha_deuda).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(format!("${:.2}", d.monto_total)).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(1)),
            text(format!("${:.2}", d.saldo_pendiente)).size(12).color(if d.saldo_pendiente > 0.01 { COLOR_DANGER } else { COLOR_SUCCESS }).width(Length::FillPortion(1)),
            text(n_pagos.to_string()).size(12).color(COLOR_ACCENT).width(Length::FillPortion(1)),
            text(estado_txt).size(11).color(estado_color).width(Length::FillPortion(1)),
            row![
                button(text("\u{2630}").size(12)).style(|_, _| ghost_button_style()).on_press(det(id)).padding([4, 6]),
                button(text("\u{270E}").size(12)).style(|_, _| ghost_button_style()).on_press(ed(id)).padding([4, 6]),
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
            text("No hay deudas registradas").size(16).color(COLOR_TEXT_SECONDARY),
            text("Registra la primera deuda de la empresa para comenzar").size(12).color(COLOR_TEXT_MUTED),
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
    state: &'a DeudasState,
    on_cerrar: Message,
    on_nuevo_pago: Message,
    on_eliminar_pago: impl Fn(i64) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let deuda = state.deudas.iter().find(|d| Some(d.id) == state.deuda_seleccionada);
    let header = row![
        button(text("\u{2190} Volver").size(13).color(COLOR_ACCENT))
            .style(|_, _| ghost_button_style())
            .on_press(on_cerrar).padding([SPACING_SM, SPACING_MD]),
        Space::new().width(Length::Fill),
        text(deuda.map(|d| d.numero.as_str()).unwrap_or("Deuda")).size(20).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        button(text("+ Nuevo Pago").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(on_nuevo_pago)
            .padding([SPACING_SM, SPACING_MD]),
    ]
    .spacing(SPACING_MD)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let mut info: Vec<Element<'a, Message>> = Vec::new();
    if let Some(d) = deuda {
        let pagado: f64 = d.monto_total - d.saldo_pendiente;
        let vencida = d.estado != "pagada" && d.fecha_vencimiento.as_deref().map(|f| f < hoy().as_str()).unwrap_or(false);
        let (estado_txt, estado_color) = if vencida { ("VENCIDA", COLOR_DANGER) }
            else if d.estado == "pagada" { ("pagada", COLOR_SUCCESS) }
            else { ("pendiente", COLOR_CXP) };
        let descripcion_el: Element<'a, Message> = if d.descripcion.as_deref().map(|s| !s.is_empty()).unwrap_or(false) {
            text(d.descripcion.clone().unwrap_or_default()).size(11).color(COLOR_TEXT_MUTED).into()
        } else { Space::new().height(0).into() };
        info.push(container(
            column![
                row![
                    column![
                        text("A quien se debe").size(10).color(COLOR_TEXT_MUTED),
                        text(&d.proveedor_nombre).size(16).color(COLOR_TEXT_PRIMARY),
                        Space::new().height(SPACING_XS),
                        text("Concepto / Qué se compró").size(10).color(COLOR_TEXT_MUTED),
                        text(&d.concepto).size(13).color(COLOR_TEXT_SECONDARY),
                        descripcion_el,
                    ].spacing(SPACING_XS).width(Length::FillPortion(1)),
                    column![
                        text("Fecha de la deuda").size(10).color(COLOR_TEXT_MUTED),
                        text(&d.fecha_deuda).size(13).color(COLOR_TEXT_PRIMARY),
                        Space::new().height(SPACING_XS),
                        text("Vence").size(10).color(COLOR_TEXT_MUTED),
                        text(d.fecha_vencimiento.as_deref().unwrap_or("—")).size(13).color(if vencida { COLOR_DANGER } else { COLOR_TEXT_PRIMARY }),
                        Space::new().height(SPACING_XS),
                        text("Categoría").size(10).color(COLOR_TEXT_MUTED),
                        text(&d.categoria_nombre).size(13).color(COLOR_TEXT_SECONDARY),
                    ].spacing(SPACING_XS).width(Length::FillPortion(1)),
                    column![
                        text("Total de la deuda").size(10).color(COLOR_TEXT_MUTED),
                        text(format!("${:.2}", d.monto_total)).size(16).color(COLOR_TEXT_PRIMARY),
                        Space::new().height(SPACING_XS),
                        text("Pagado").size(10).color(COLOR_TEXT_MUTED),
                        text(format!("${:.2}", pagado)).size(13).color(COLOR_SUCCESS),
                        Space::new().height(SPACING_XS),
                        text("Saldo pendiente").size(10).color(COLOR_TEXT_MUTED),
                        text(format!("${:.2}", d.saldo_pendiente)).size(16).color(if d.saldo_pendiente > 0.01 { COLOR_DANGER } else { COLOR_SUCCESS }),
                    ].spacing(SPACING_XS).width(Length::FillPortion(1)),
                    column![
                        text("Estado").size(10).color(COLOR_TEXT_MUTED),
                        text(estado_txt).size(14).color(estado_color),
                        Space::new().height(SPACING_XS),
                        text("Referencia").size(10).color(COLOR_TEXT_MUTED),
                        text(if d.referencia.is_empty() { "—".to_string() } else { d.referencia.clone() }).size(12).color(COLOR_TEXT_SECONDARY),
                        Space::new().height(SPACING_XS),
                        text("Notas").size(10).color(COLOR_TEXT_MUTED),
                        text(if d.notas.is_empty() { "—".to_string() } else { d.notas.clone() }).size(12).color(COLOR_TEXT_SECONDARY),
                    ].spacing(SPACING_XS).width(Length::FillPortion(1)),
                ].spacing(SPACING_LG).width(Length::Fill),
            ].spacing(SPACING_SM)
        )
        .padding(SPACING_LG)
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_CARD)),
            border: iced::Border { radius: RADIUS_LG.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
        })
        .width(Length::Fill).into());
        info.push(Space::new().height(SPACING_SM).into());
        info.push(row![
            text("Historial de pagos").size(16).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text(format!("{} pago(s)", state.pagos_deuda.len())).size(12).color(COLOR_TEXT_MUTED),
        ].width(Length::Fill).into());
        info.push(Space::new().height(SPACING_XS).into());

        let col_header = row![
            text("Fecha").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text("Monto").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text("Método").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text("Referencia").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text("Notas").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        ]
        .spacing(SPACING_SM)
        .padding([SPACING_SM, SPACING_MD]);

        let rows: Vec<Element<'a, Message>> = state.pagos_deuda.iter().map(|p| {
            let pid = p.id;
            row![
                text(&p.fecha).size(12).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                text(format!("${:.2}", p.monto)).size(12).color(COLOR_SUCCESS).width(Length::FillPortion(2)),
                text(p.metodo_pago.as_deref().unwrap_or("")).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                text(&p.referencia).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                text(&p.notas).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
                button(text("\u{2715}").size(11).color(COLOR_DANGER))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_eliminar_pago(pid)).padding([4, 6]),
            ]
            .spacing(SPACING_SM)
            .align_y(Alignment::Center)
            .padding([SPACING_SM, SPACING_MD])
            .into()
        }).collect();

        let body: Element<'a, Message> = if state.pagos_deuda.is_empty() {
            container(column![
                text("Aún no hay pagos registrados").size(14).color(COLOR_TEXT_SECONDARY),
                text("Registra un abono con el botón \"+ Nuevo Pago\"").size(12).color(COLOR_TEXT_MUTED),
            ].spacing(SPACING_SM).align_x(Alignment::Center))
            .center(Length::Fill).width(Length::Fill).height(200).into()
        } else {
            scrollable(column(
                std::iter::once(col_header.into()).chain(rows)
            ).spacing(2.0).width(Length::Fill))
                .style(|_, _| scrollable_style()).width(Length::Fill).height(Length::Fill).into()
        };
        info.push(body);
    }

    column![
        header, Space::new().height(SPACING_MD), column(info).spacing(0).width(Length::Fill),
    ]
    .padding(SPACING_LG)
    .spacing(SPACING_SM)
    .into()
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a DeudasState,
    on_form_msg: impl Fn(DeudaFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Deuda de la Empresa" } else { "Nueva Deuda de la Empresa" };
    let fields: Vec<Element<'a, DeudaFormMessage>> = vec![
        form_two_columns(
            pick_list_field("A quién se le debe (proveedor)", &state.opciones_proveedores, state.form.proveedor_id.unwrap_or(0), DeudaFormMessage::ProveedorSeleccionado),
            pick_list_field("Categoría", &state.opciones_categorias, state.form.categoria_id.unwrap_or(0), DeudaFormMessage::CategoriaSeleccionada),
        ),
        form_two_columns(
            labeled_input("Nombre / Negocio", &state.form.proveedor_nombre, "Nombre de quien se le debe", DeudaFormMessage::ProveedorNombre),
            labeled_input_f64("Monto total de la deuda", &state.form.monto_total, "0.00", DeudaFormMessage::MontoTotal),
        ),
        form_two_columns(
            labeled_input("Concepto / Qué se compró", &state.form.concepto, "Ej: 10 teclados USB, repuestos, etc.", DeudaFormMessage::Concepto),
            labeled_input("Descripción", &state.form.descripcion, "Detalle adicional (opcional)", DeudaFormMessage::Descripcion),
        ),
        form_two_columns(
            labeled_input("Fecha de la deuda", &state.form.fecha_deuda, "AAAA-MM-DD", DeudaFormMessage::FechaDeuda),
            labeled_input("Fecha de vencimiento", &state.form.fecha_vencimiento, "AAAA-MM-DD (opcional)", DeudaFormMessage::FechaVencimiento),
        ),
        form_two_columns(
            labeled_input("Referencia", &state.form.referencia, "Factura / nota de remisión", DeudaFormMessage::Referencia),
            labeled_input("Notas", &state.form.notas, "Notas", DeudaFormMessage::Notas),
        ),
    ];
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, DeudaFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(DeudaFormMessage::Guardar)), cancelar(DeudaFormMessage::Cancelar), "Guardar")
}

fn render_pago_form<'a, Message: 'a + Clone>(
    state: &'a DeudasState,
    on_form_msg: impl Fn(DeudaPagoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let met_id: i64 = match state.form_pago.metodo_pago.as_str() {
        "transferencia" => 2, "tarjeta" => 3, "cheque" => 4, "otro" => 5, _ => 1,
    };
    let deuda = state.deudas.iter().find(|d| Some(d.id) == state.deuda_seleccionada);
    let fields: Vec<Element<'a, DeudaPagoFormMessage>> = vec![
        form_two_columns(
            labeled_input_f64("Monto del pago", &state.form_pago.monto, "0.00", DeudaPagoFormMessage::Monto),
            pick_list_field("Método de Pago", &*OPCIONES_METODO, met_id, |id| {
                let val = OPCIONES_METODO.iter().find(|o| o.id == id).map(|o| o.label.clone()).unwrap_or_else(|| "efectivo".to_string());
                DeudaPagoFormMessage::MetodoPago(val)
            }),
        ),
        form_two_columns(
            labeled_input("Referencia", &state.form_pago.referencia, "Folio o referencia", DeudaPagoFormMessage::Referencia),
            labeled_input("Notas", &state.form_pago.notas, "Notas", DeudaPagoFormMessage::Notas),
        ),
    ];
    let mut footer: Vec<Element<'a, DeudaPagoFormMessage>> = Vec::new();
    if let Some(d) = deuda {
        footer.push(row![
            text(format!("Saldo actual de esta deuda: ${:.2}", d.saldo_pendiente)).size(13).color(COLOR_DANGER),
            Space::new().width(10),
            text(format!("Total: ${:.2}", d.monto_total)).size(12).color(COLOR_TEXT_SECONDARY),
        ].into());
    }
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, DeudaPagoFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card("Nuevo Pago a Deuda", fields.into_iter().chain(footer).map(map_fn),
        Some(guardar(DeudaPagoFormMessage::Guardar)), cancelar(DeudaPagoFormMessage::Cancelar), "Guardar")
}
