use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{Ahorro, AhorroMovimiento};
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, pick_list_field, SelectOption, form_two_columns};

#[derive(Debug, Clone)]
pub struct AhorroFormData { pub cliente_id: String, pub cliente_nombre: String, pub saldo_inicial: String }
impl Default for AhorroFormData { fn default() -> Self { Self { cliente_id: String::new(), cliente_nombre: String::new(), saldo_inicial: String::new() } } }

#[derive(Debug, Clone)]
pub struct AhorroMovFormData {
    pub tipo: String, pub monto: String, pub descripcion: String,
}
impl Default for AhorroMovFormData {
    fn default() -> Self { Self { tipo: String::new(), monto: String::new(), descripcion: String::new() } }
}

#[derive(Debug, Clone)]
pub struct AhorrosState {
    pub ahorros: Vec<Ahorro>, pub show_form: bool, pub editing_id: Option<i64>, pub busqueda: String,
    pub form: AhorroFormData, pub opciones_clientes: Vec<SelectOption>,
    pub show_movimientos: bool, pub movimientos: Vec<AhorroMovimiento>, pub cuenta_seleccionada: Option<i64>,
    pub show_form_movimiento: bool, pub form_movimiento: AhorroMovFormData,
}
impl Default for AhorrosState {
    fn default() -> Self {
        Self {
            ahorros: Vec::new(), show_form: false, editing_id: None, busqueda: String::new(),
            form: AhorroFormData::default(), opciones_clientes: Vec::new(),
            show_movimientos: false, movimientos: Vec::new(), cuenta_seleccionada: None,
            show_form_movimiento: false, form_movimiento: AhorroMovFormData::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AhorroFormMessage { ClienteId(String), ClienteNombre(String), SaldoInicial(String), Guardar, Cancelar }

#[derive(Debug, Clone)]
pub enum AhorroMovFormMessage { Tipo(String), Monto(String), Descripcion(String), Guardar, Cancelar }

#[derive(Debug, Clone)]
pub enum AhorroMessage {
    Editar(i64), Eliminar(i64), Buscar(String),
    VerMovimientos(i64), CerrarMovimientos, NuevoMovimiento,
    MovFormMsg(AhorroMovFormMessage),
}

pub fn ahorros_view<'a, Message: 'a + Clone>(
    state: &'a AhorrosState, on_nueva: Message,
    on_form_msg: impl Fn(AhorroFormMessage) -> Message + 'a + Clone,
    on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_ver_movimientos: impl Fn(i64) -> Message + 'a + Clone,
    on_nuevo_movimiento: Message,
    on_mov_form_msg: impl Fn(AhorroMovFormMessage) -> Message + 'a + Clone,
    on_cerrar_movimientos: Message,
) -> Element<'a, Message> {
    if state.show_form {
        return render_form(state, on_form_msg);
    }
    if state.show_form_movimiento {
        return render_movimiento_form(state, on_mov_form_msg);
    }
    if state.show_movimientos {
        return render_movimientos_view(state, on_cerrar_movimientos, on_nuevo_movimiento);
    }

    let filtrados: Vec<&Ahorro> = if state.busqueda.is_empty() {
        state.ahorros.iter().filter(|a| a.activo).collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.ahorros.iter().filter(|a| a.activo && a.cliente_nombre.to_lowercase().contains(&q)).collect()
    };

    let header = row![
        text("Cuentas de Ahorro").size(24).color(COLOR_TEXT_PRIMARY),
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

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|a| {
        let id = a.id;
        row![
            text(&a.cliente_nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(format!("${:.2}", a.saldo)).size(14).color(COLOR_VENTAS).width(Length::FillPortion(2)),
            text(if a.activo { "Activo" } else { "Inactivo" }).size(11).color(if a.activo { COLOR_SUCCESS } else { COLOR_TEXT_MUTED }).width(Length::FillPortion(1)),
            row![
                button(text("\u{270E}").size(12))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_editar(id)).padding([4, 6]),
                button(text("\u{2715}").size(11))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_eliminar(id)).padding([4, 6]),
                button(text("Mov.").size(11))
                    .style(|_, _| ghost_button_style())
                    .on_press(on_ver_movimientos(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(2)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    let body: Element<'a, Message> = if filtrados.is_empty() {
        container(column![
            text("No hay cuentas de ahorro").size(16).color(COLOR_TEXT_SECONDARY),
            text("Crea una nueva cuenta para comenzar").size(12).color(COLOR_TEXT_MUTED),
        ].spacing(SPACING_SM).align_x(Alignment::Center))
        .center(Length::Fill).width(Length::Fill).height(300).into()
    } else {
        scrollable(column(rows).spacing(2.0).width(Length::Fill))
            .style(|_, _| scrollable_style()).width(Length::Fill).height(Length::Fill).into()
    };

    column![header, Space::new().height(Length::Fixed(SPACING_MD)), body]
        .padding(SPACING_LG).spacing(SPACING_SM).into()
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a AhorrosState, on_form_msg: impl Fn(AhorroFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Cuenta de Ahorro" } else { "Nueva Cuenta de Ahorro" };
    let cli_id: i64 = state.form.cliente_id.parse().unwrap_or(0);
    let fields: Vec<Element<'a, AhorroFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Cliente", &state.opciones_clientes, cli_id, |id| AhorroFormMessage::ClienteId(id.to_string())),
            labeled_input_f64("Saldo Inicial", &state.form.saldo_inicial, "0.00", AhorroFormMessage::SaldoInicial),
        ),
    ];
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, AhorroFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone(); let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(AhorroFormMessage::Guardar)), cancelar(AhorroFormMessage::Cancelar), "Guardar")
}

fn render_movimientos_view<'a, Message: 'a + Clone>(
    state: &'a AhorrosState, on_cerrar: Message,
    on_nuevo_movimiento: Message,
) -> Element<'a, Message> {
    let header = row![
        button(text("\u{2190} Volver").size(13).color(COLOR_ACCENT))
            .style(|_, _| ghost_button_style())
            .on_press(on_cerrar).padding([SPACING_SM, SPACING_MD]),
        Space::new().width(Length::Fill),
        text("Movimientos de Ahorro").size(20).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        button(text("+ Nuevo").size(13).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(on_nuevo_movimiento)
            .padding([SPACING_SM, SPACING_MD]),
    ].spacing(SPACING_MD).align_y(Alignment::Center).width(Length::Fill);

    let rows: Vec<Element<'a, Message>> = state.movimientos.iter().map(|m| {
        row![
            text(&m.fecha).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&m.tipo).size(11).color(if m.tipo == "deposito" { COLOR_SUCCESS } else { COLOR_DANGER }).width(Length::FillPortion(1)),
            text(format!("${:.2}", m.monto)).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(1)),
            text(format!("${:.2}", m.saldo_acumulado)).size(12).color(COLOR_ACCENT).width(Length::FillPortion(1)),
            text(&m.descripcion).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(3)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    let body: Element<'a, Message> = if rows.is_empty() {
        container(column![
            text("No hay movimientos registrados").size(16).color(COLOR_TEXT_SECONDARY),
        ].spacing(SPACING_SM).align_x(Alignment::Center))
        .center(Length::Fill).width(Length::Fill).height(300).into()
    } else {
        scrollable(column(rows).spacing(2.0).width(Length::Fill))
            .style(|_, _| scrollable_style()).width(Length::Fill).height(Length::Fill).into()
    };

    column![header, Space::new().height(Length::Fixed(SPACING_MD)), body]
        .padding(SPACING_LG).spacing(SPACING_SM).into()
}

fn render_movimiento_form<'a, Message: 'a + Clone>(
    state: &'a AhorrosState, on_form_msg: impl Fn(AhorroMovFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let fields: Vec<Element<'a, AhorroMovFormMessage>> = vec![
        labeled_input("Tipo", &state.form_movimiento.tipo, "deposito/retiro", AhorroMovFormMessage::Tipo),
        labeled_input_f64("Monto", &state.form_movimiento.monto, "0.00", AhorroMovFormMessage::Monto),
        labeled_input("Descripción", &state.form_movimiento.descripcion, "Descripción del movimiento", AhorroMovFormMessage::Descripcion),
    ];
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, AhorroMovFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone(); let cancelar = on_form_msg;
    form_card("Nuevo Movimiento", fields.into_iter().map(map_fn), Some(guardar(AhorroMovFormMessage::Guardar)), cancelar(AhorroMovFormMessage::Cancelar), "Guardar")
}
