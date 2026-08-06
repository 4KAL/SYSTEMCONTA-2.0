use iced::widget::{button, column, row, scrollable, text, Space};
use iced::{Element, Length, Alignment};
use crate::models::{CuentaBancaria, MovimientoBancario};
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, form_two_columns, SelectOption};

#[derive(Debug, Clone)]
pub struct CuentaFormData {
    pub nombre: String,
    pub banco: String,
    pub numero_cuenta: String,
    pub tipo: String,
    pub saldo_inicial: String,
}

impl Default for CuentaFormData {
    fn default() -> Self {
        Self {
            nombre: String::new(), banco: String::new(), numero_cuenta: String::new(),
            tipo: "ahorros".to_string(), saldo_inicial: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CuentaFormMessage {
    Nombre(String), Banco(String), NumeroCuenta(String), Tipo(String),
    SaldoInicial(String), Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub struct MovimientoFormData {
    pub fecha: String,
    pub descripcion: String,
    pub tipo: String,
    pub monto: String,
    pub referencia: String,
}

impl Default for MovimientoFormData {
    fn default() -> Self {
        Self {
            fecha: chrono::Local::now().format("%Y-%m-%d").to_string(),
            descripcion: String::new(), tipo: "ingreso".to_string(),
            monto: String::new(), referencia: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MovimientoFormMessage {
    Fecha(String), Descripcion(String), Tipo(String), Monto(String), Referencia(String), Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub enum ConciliacionTab { Cuentas, Movimientos }

#[derive(Debug, Clone)]
pub struct ConciliacionState {
    pub tab: ConciliacionTab,
    pub cuentas: Vec<CuentaBancaria>,
    pub movimientos: Vec<MovimientoBancario>,
    pub cuenta_seleccionada: i64,
    pub saldo_actual: f64,
    pub show_form_cuenta: bool,
    pub editing_cuenta_id: Option<i64>,
    pub form_cuenta: CuentaFormData,
    pub show_form_movimiento: bool,
    pub editing_movimiento_id: Option<i64>,
    pub form_movimiento: MovimientoFormData,
    pub opciones_cuentas: Vec<SelectOption>,
    pub busqueda: String,
}

impl Default for ConciliacionState {
    fn default() -> Self {
        Self {
            tab: ConciliacionTab::Cuentas, cuentas: Vec::new(), movimientos: Vec::new(),
            cuenta_seleccionada: 0, saldo_actual: 0.0,
            show_form_cuenta: false, editing_cuenta_id: None, form_cuenta: CuentaFormData::default(),
            show_form_movimiento: false, editing_movimiento_id: None, form_movimiento: MovimientoFormData::default(),
            opciones_cuentas: Vec::new(), busqueda: String::new(),
        }
    }
}

pub fn conciliacion_view<'a, Message: 'a + Clone>(
    state: &'a ConciliacionState,
    on_tab: impl Fn(ConciliacionTab) -> Message + 'a + Clone,
    on_form_cuenta: impl Fn(CuentaFormMessage) -> Message + 'a + Clone,
    on_form_mov: impl Fn(MovimientoFormMessage) -> Message + 'a + Clone,
    on_nueva_cuenta: Message,
    on_nuevo_movimiento: Message,
    on_eliminar_cuenta: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar_mov: impl Fn(i64) -> Message + 'a + Clone,
    on_toggle_conciliado: impl Fn(i64) -> Message + 'a + Clone,
    on_seleccionar_cuenta: impl Fn(i64) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form_cuenta {
        let title = if state.editing_cuenta_id.is_some() { "Editar Cuenta" } else { "Nueva Cuenta Bancaria" };
        let guardar = on_form_cuenta(CuentaFormMessage::Guardar);
        let cancelar = on_form_cuenta(CuentaFormMessage::Cancelar);
        let f_nom = on_form_cuenta.clone();
        let f_ban = on_form_cuenta.clone();
        let f_num = on_form_cuenta.clone();
        let f_tip = on_form_cuenta.clone();
        let f_sal = on_form_cuenta.clone();
        return form_card(
            title,
            vec![
                labeled_input("Nombre de la cuenta", &state.form_cuenta.nombre, "Caja Principal", move |v| f_nom(CuentaFormMessage::Nombre(v))),
                form_two_columns(
                    labeled_input("Banco", &state.form_cuenta.banco, "Banco", move |v| f_ban(CuentaFormMessage::Banco(v))),
                    labeled_input("Tipo", &state.form_cuenta.tipo, "ahorros", move |v| f_tip(CuentaFormMessage::Tipo(v))),
                ),
                form_two_columns(
                    labeled_input("Número de cuenta", &state.form_cuenta.numero_cuenta, "Número", move |v| f_num(CuentaFormMessage::NumeroCuenta(v))),
                    labeled_input_f64("Saldo inicial", &state.form_cuenta.saldo_inicial, "0.00", move |v| f_sal(CuentaFormMessage::SaldoInicial(v))),
                ),
            ],
            Some(guardar), cancelar, "Guardar",
        );
    }

    if state.show_form_movimiento {
        let title = if state.editing_movimiento_id.is_some() { "Editar Movimiento" } else { "Nuevo Movimiento" };
        let guardar = on_form_mov(MovimientoFormMessage::Guardar);
        let cancelar = on_form_mov(MovimientoFormMessage::Cancelar);
        let f_fec = on_form_mov.clone();
        let f_des = on_form_mov.clone();
        let f_tip = on_form_mov.clone();
        let f_mon = on_form_mov.clone();
        let f_ref = on_form_mov.clone();
        return form_card(
            title,
            vec![
                labeled_input("Fecha", &state.form_movimiento.fecha, "YYYY-MM-DD", move |v| f_fec(MovimientoFormMessage::Fecha(v))),
                labeled_input("Descripción", &state.form_movimiento.descripcion, "Descripción", move |v| f_des(MovimientoFormMessage::Descripcion(v))),
                form_two_columns(
                    labeled_input("Tipo (ingreso/egreso)", &state.form_movimiento.tipo, "ingreso", move |v| f_tip(MovimientoFormMessage::Tipo(v))),
                    labeled_input_f64("Monto", &state.form_movimiento.monto, "0.00", move |v| f_mon(MovimientoFormMessage::Monto(v))),
                ),
                labeled_input("Referencia", &state.form_movimiento.referencia, "Referencia", move |v| f_ref(MovimientoFormMessage::Referencia(v))),
            ],
            Some(guardar), cancelar, "Guardar",
        );
    }

    let tabs = row![
        button(text("Cuentas Bancarias").size(13).color(if matches!(state.tab, ConciliacionTab::Cuentas) { COLOR_ACCENT } else { COLOR_TEXT_MUTED }))
            .style(|_, _| ghost_button_style())
            .on_press(on_tab(ConciliacionTab::Cuentas)),
        button(text("Movimientos / Conciliación").size(13).color(if matches!(state.tab, ConciliacionTab::Movimientos) { COLOR_ACCENT } else { COLOR_TEXT_MUTED }))
            .style(|_, _| ghost_button_style())
            .on_press(on_tab(ConciliacionTab::Movimientos)),
    ].spacing(SPACING_SM).align_y(Alignment::Center);

    let header_btn = match state.tab {
        ConciliacionTab::Cuentas =>
            button(text("+ Nueva Cuenta").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style()).on_press(on_nueva_cuenta).padding([SPACING_SM, SPACING_MD]),
        ConciliacionTab::Movimientos =>
            button(text("+ Nuevo Movimiento").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style()).on_press(on_nuevo_movimiento).padding([SPACING_SM, SPACING_MD]),
    };

    let mut rows: Vec<Element<'a, Message>> = Vec::new();
    match state.tab {
        ConciliacionTab::Cuentas => {
            for c in state.cuentas.iter().filter(|c| c.activo) {
                let id = c.id;
                rows.push(row![
                    text(&c.nombre).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                    text(&c.banco).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
                    text(&c.numero_cuenta).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
                    text(format!("Saldo inicial: {:.2}", c.saldo_inicial)).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                    button(text("Movimientos").size(11).color(COLOR_ACCENT))
                        .style(|_, _| ghost_button_style())
                        .on_press(on_seleccionar_cuenta(id)).padding([4, 6]),
                    button(text("\u{2715}").size(12).color(COLOR_DANGER))
                        .style(|_, _| ghost_button_style())
                        .on_press(on_eliminar_cuenta(id)).padding([4, 6]),
                ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into());
            }
        }
        ConciliacionTab::Movimientos => {
            for m in state.movimientos.iter() {
                let id = m.id;
                rows.push(row![
                    text(&m.fecha).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
                    text(&m.descripcion).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                    text(if m.tipo == "ingreso" { "Ingreso" } else { "Egreso" }).size(10)
                        .color(if m.tipo == "ingreso" { COLOR_VENTAS } else { COLOR_GASTOS }).width(Length::FillPortion(1)),
                    text(format!("${:.2}", m.monto)).size(12)
                        .color(if m.tipo == "ingreso" { COLOR_VENTAS } else { COLOR_GASTOS }).width(Length::FillPortion(2)),
                    text(if m.conciliado { "Conciliado" } else { "No conciliado" }).size(10)
                        .color(if m.conciliado { COLOR_SUCCESS } else { COLOR_DANGER }).width(Length::FillPortion(1)),
                    button(text("Conciliar").size(11).color(COLOR_ACCENT))
                        .style(|_, _| ghost_button_style())
                        .on_press(on_toggle_conciliado(id)).padding([4, 6]),
                    button(text("\u{2715}").size(12).color(COLOR_DANGER))
                        .style(|_, _| ghost_button_style())
                        .on_press(on_eliminar_mov(id)).padding([4, 6]),
                ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into());
            }
        }
    }

    let saldo_info: Element<'a, Message> = match state.tab {
        ConciliacionTab::Movimientos => row![
            text("Saldo actual de la cuenta:").size(12).color(COLOR_TEXT_MUTED),
            text(format!("${:.2}", state.saldo_actual)).size(14).color(COLOR_ACCENT),
        ].spacing(SPACING_SM).align_y(Alignment::Center).into(),
        ConciliacionTab::Cuentas => Space::new().height(Length::Fixed(0.0)).into(),
    };

    column![
        row![
            text("Conciliación Bancaria").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            header_btn,
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        row![tabs, Space::new().width(Length::Fill), saldo_info].spacing(SPACING_MD).align_y(Alignment::Center).padding([0.0, SPACING_LG]),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}
