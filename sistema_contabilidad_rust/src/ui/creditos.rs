use std::sync::LazyLock;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{CuentaCredito, CreditoMovimiento};
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, pick_list_field, SelectOption, form_two_columns};

#[derive(Debug, Clone)]
pub struct CreditoFormData { pub cliente_id: String, pub cliente_nombre: String, pub limite: String }
impl Default for CreditoFormData { fn default() -> Self { Self { cliente_id: String::new(), cliente_nombre: String::new(), limite: String::new() } } }

#[derive(Debug, Clone)]
pub struct CreditoMovFormData {
    pub tipo: String, pub monto: String, pub descripcion: String,
    pub cantidad: String, pub precio_unit: String,
}
impl Default for CreditoMovFormData {
    fn default() -> Self {
        Self { tipo: "cargo".into(), monto: String::new(), descripcion: String::new(), cantidad: String::new(), precio_unit: String::new() }
    }
}

#[derive(Debug, Clone)]
pub struct CreditosState {
    pub cuentas: Vec<CuentaCredito>, pub show_form: bool, pub editing_id: Option<i64>,
    pub busqueda: String, pub form: CreditoFormData, pub opciones_clientes: Vec<SelectOption>,
    pub show_movimientos: bool, pub movimientos: Vec<CreditoMovimiento>,
    pub cuenta_seleccionada: Option<i64>,
    pub show_form_movimiento: bool, pub form_movimiento: CreditoMovFormData,
}
impl Default for CreditosState {
    fn default() -> Self {
        Self {
            cuentas: Vec::new(), show_form: false, editing_id: None, busqueda: String::new(),
            form: CreditoFormData::default(), opciones_clientes: Vec::new(),
            show_movimientos: false, movimientos: Vec::new(), cuenta_seleccionada: None,
            show_form_movimiento: false, form_movimiento: CreditoMovFormData::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CreditoFormMessage { ClienteId(String), ClienteNombre(String), Limite(String), Guardar, Cancelar }

#[derive(Debug, Clone)]
pub enum CreditoMovFormMessage { Tipo(String), Monto(String), Descripcion(String), Cantidad(String), PrecioUnit(String), Guardar, Cancelar }

static OPCIONES_TIPO_MOV: LazyLock<Vec<SelectOption>> = LazyLock::new(|| vec![
    SelectOption { id: 1, label: "cargo".to_string() },
    SelectOption { id: 2, label: "abono".to_string() },
]);

pub fn creditos_view<'a, Message: 'a + Clone>(
    state: &'a CreditosState, on_nueva: Message,
    on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_ver_movimientos: impl Fn(i64) -> Message + 'a + Clone,
    on_cerrar_movimientos: Message,
    on_form_msg: impl Fn(CreditoFormMessage) -> Message + 'a + Clone,
    on_nuevo_movimiento: Message,
    on_mov_form_msg: impl Fn(CreditoMovFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }
    if state.show_form_movimiento { return render_movimiento_form(state, on_mov_form_msg); }
    if state.show_movimientos { return render_movimientos_view(state, on_cerrar_movimientos, on_nuevo_movimiento); }
    render_creditos_list(state, on_nueva, on_editar, on_eliminar, on_buscar, on_ver_movimientos)
}

fn render_creditos_list<'a, Message: 'a + Clone>(
    state: &'a CreditosState, on_nueva: Message,
    on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_ver_movimientos: impl Fn(i64) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let filtrados: Vec<&CuentaCredito> = if state.busqueda.is_empty() {
        state.cuentas.iter().filter(|c| c.activa).collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.cuentas.iter().filter(|c| c.activa && (
            c.cliente_nombre.to_lowercase().contains(&q) || c.nombre.to_lowercase().contains(&q)
        )).collect()
    };

    let header = row![
        text("Cuentas de Crédito").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        text_input("Buscar cuentas...", &state.busqueda)
            .on_input(on_buscar)
            .style(|_, _| input_style())
            .width(220),
        button(text("+ Nueva").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(on_nueva)
            .padding([SPACING_SM, SPACING_MD]),
    ]
    .spacing(SPACING_MD)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|c| {
        let id = c.id;
        let pct = if c.limite > 0.0 { (c.saldo_actual / c.limite * 100.0) as i32 } else { 0 };
        let sc = if pct > 80 { COLOR_DANGER } else if pct > 50 { COLOR_CXP } else { COLOR_SUCCESS };
        row![
            text(&c.cliente_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(format!("${:.0}", c.limite)).size(12).color(COLOR_ACCENT).width(Length::FillPortion(2)),
            text(format!("${:.0}", c.saldo_actual)).size(12).color(sc).width(Length::FillPortion(2)),
            text(format!("{}%", pct)).size(11).color(sc).width(Length::FillPortion(1)),
            row![
                button(text("\u{2630}").size(12))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_ver_movimientos(id)).padding([4, 6]),
                button(text("\u{270E}").size(12))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_editar(id)).padding([4, 6]),
                button(text("\u{2715}").size(11))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_eliminar(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(2)),
        ]
        .spacing(SPACING_SM)
        .align_y(Alignment::Center)
        .padding([SPACING_SM, SPACING_MD])
        .into()
    }).collect();

    let body: Element<'a, Message> = if filtrados.is_empty() {
        container(column![
            text("No hay cuentas de crédito").size(16).color(COLOR_TEXT_SECONDARY),
            text("Crea tu primera cuenta para comenzar").size(12).color(COLOR_TEXT_MUTED),
        ].spacing(SPACING_SM).align_x(Alignment::Center))
        .center(Length::Fill).width(Length::Fill).height(300).into()
    } else {
        scrollable(column(rows).spacing(2.0).width(Length::Fill))
            .style(|_, _| scrollable_style()).width(Length::Fill).height(Length::Fill).into()
    };

    column![
        header, Space::new().height(Length::Fixed(SPACING_MD)), body,
    ]
    .padding(SPACING_LG)
    .spacing(SPACING_SM)
    .into()
}

fn render_movimientos_view<'a, Message: 'a + Clone>(
    state: &'a CreditosState, on_cerrar_movimientos: Message,
    on_nuevo_movimiento: Message,
) -> Element<'a, Message> {
    let cuenta_nombre = state.cuentas.iter()
        .find(|c| Some(c.id) == state.cuenta_seleccionada)
        .map(|c| &c.cliente_nombre)
        .map(|s| s.as_str())
        .unwrap_or("Movimientos");

    let header = row![
        button(text("\u{2190} Volver").size(13).color(COLOR_ACCENT))
            .style(|_, _| ghost_button_style())
            .on_press(on_cerrar_movimientos).padding([SPACING_SM, SPACING_MD]),
        Space::new().width(Length::Fill),
        text(cuenta_nombre).size(20).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        button(text("+ Nuevo Movimiento").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(on_nuevo_movimiento)
            .padding([SPACING_SM, SPACING_MD]),
    ]
    .spacing(SPACING_MD)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let col_header = row![
        text("Fecha").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
        text("Tipo").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
        text("Monto").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
        text("Descripción").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(3)),
        text("Saldo Acum.").size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
    ]
    .spacing(SPACING_SM)
    .padding([SPACING_SM, SPACING_MD]);

    let rows: Vec<Element<'a, Message>> = state.movimientos.iter().map(|m| {
        let monto_color = if m.tipo == "abono" { COLOR_SUCCESS } else { COLOR_DANGER };
        row![
            text(&m.fecha).size(12).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&m.tipo).size(12).color(COLOR_ACCENT).width(Length::FillPortion(1)),
            text(format!("${:.2}", m.monto)).size(12).color(monto_color).width(Length::FillPortion(2)),
            text(&m.descripcion).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(format!("${:.2}", m.saldo_acumulado)).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(2)),
        ]
        .spacing(SPACING_SM)
        .align_y(Alignment::Center)
        .padding([SPACING_SM, SPACING_MD])
        .into()
    }).collect();

    let body: Element<'a, Message> = if state.movimientos.is_empty() {
        container(column![
            text("No hay movimientos registrados").size(16).color(COLOR_TEXT_SECONDARY),
            text("Agrega un nuevo movimiento para comenzar").size(12).color(COLOR_TEXT_MUTED),
        ].spacing(SPACING_SM).align_x(Alignment::Center))
        .center(Length::Fill).width(Length::Fill).height(300).into()
    } else {
        scrollable(column(
            std::iter::once(col_header.into()).chain(rows)
        ).spacing(2.0).width(Length::Fill))
            .style(|_, _| scrollable_style()).width(Length::Fill).height(Length::Fill).into()
    };

    column![
        header, Space::new().height(Length::Fixed(SPACING_MD)), body,
    ]
    .padding(SPACING_LG)
    .spacing(SPACING_SM)
    .into()
}

fn render_movimiento_form<'a, Message: 'a + Clone>(
    state: &'a CreditosState,
    on_mov_form_msg: impl Fn(CreditoMovFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let tipo_id = if state.form_movimiento.tipo == "abono" { 2 } else { 1 };
    let fields: Vec<Element<'a, CreditoMovFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Tipo", &OPCIONES_TIPO_MOV, tipo_id, |id| {
                let val = if id == 2 { "abono".to_string() } else { "cargo".to_string() };
                CreditoMovFormMessage::Tipo(val)
            }),
            labeled_input_f64("Monto", &state.form_movimiento.monto, "0.00", CreditoMovFormMessage::Monto),
        ),
        labeled_input("Descripción", &state.form_movimiento.descripcion, "Descripción del movimiento", CreditoMovFormMessage::Descripcion),
    ];

    let fm_clone = on_mov_form_msg.clone();
    let map_fn = move |f: Element<'a, CreditoMovFormMessage>| {
        let cb = fm_clone.clone();
        f.map(move |msg| cb(msg))
    };
    let guardar = on_mov_form_msg.clone();
    let cancelar = on_mov_form_msg;
    form_card("Nuevo Movimiento", fields.into_iter().map(map_fn),
        Some(guardar(CreditoMovFormMessage::Guardar)),
        cancelar(CreditoMovFormMessage::Cancelar), "Guardar")
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a CreditosState, on_form_msg: impl Fn(CreditoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Cuenta de Crédito" } else { "Nueva Cuenta de Crédito" };
    let cli_id: i64 = state.form.cliente_id.parse().unwrap_or(0);
    let fields: Vec<Element<'a, CreditoFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Cliente", &state.opciones_clientes, cli_id, |id| CreditoFormMessage::ClienteId(id.to_string())),
            labeled_input_f64("Límite de Crédito", &state.form.limite, "0.00", CreditoFormMessage::Limite),
        ),
    ];
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, CreditoFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone(); let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(CreditoFormMessage::Guardar)), cancelar(CreditoFormMessage::Cancelar), "Guardar")
}
