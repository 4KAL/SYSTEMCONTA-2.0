use iced::widget::{button, column, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{PagoRealizado, Gasto};
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, pick_list_field, SelectOption, form_two_columns};

#[derive(Debug, Clone)]
pub struct PagoRealizadoFormData { pub gasto_id: Option<i64>, pub proveedor_id: Option<i64>, pub monto: String, pub metodo_pago: String, pub referencia: String, pub notas: String }
impl Default for PagoRealizadoFormData {
    fn default() -> Self { Self { gasto_id: None, proveedor_id: None, monto: String::new(), metodo_pago: "efectivo".to_string(), referencia: String::new(), notas: String::new() } }
}
#[derive(Debug, Clone)]
pub struct PagosRealizadosState {
    pub pagos: Vec<PagoRealizado>,
    pub gastos: Vec<Gasto>,
    pub show_form: bool,
    pub form: PagoRealizadoFormData,
    pub opciones_proveedores: Vec<SelectOption>,
    pub opciones_gastos: Vec<SelectOption>,
    pub opciones_metodo: Vec<SelectOption>,
    pub editing_id: Option<i64>,
    pub busqueda: String,
    pub desde: String,
    pub hasta: String,
}
impl Default for PagosRealizadosState {
    fn default() -> Self {
        Self {
            pagos: Vec::new(), gastos: Vec::new(), show_form: false,
            form: PagoRealizadoFormData::default(),
            opciones_proveedores: Vec::new(), opciones_gastos: Vec::new(),
            opciones_metodo: vec![
                SelectOption { id: 1, label: "efectivo".to_string() },
                SelectOption { id: 2, label: "transferencia".to_string() },
                SelectOption { id: 3, label: "tarjeta".to_string() },
                SelectOption { id: 4, label: "cheque".to_string() },
                SelectOption { id: 5, label: "otro".to_string() },
            ],
            editing_id: None, busqueda: String::new(), desde: String::new(), hasta: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PagoRealizadoFormMessage {
    GastoSeleccionado(i64),
    ProveedorId(i64),
    Monto(String),
    MetodoPago(String),
    Referencia(String),
    Notas(String),
    Guardar,
    Cancelar,
}

fn nombre_proveedor<'a>(state: &'a PagosRealizadosState, id: Option<i64>) -> String {
    id.and_then(|pid| state.opciones_proveedores.iter().find(|o| o.id == pid))
        .map(|o| o.label.clone())
        .unwrap_or_else(|| "—".to_string())
}

fn info_gasto<'a>(state: &'a PagosRealizadosState, id: Option<i64>) -> Option<&'a Gasto> {
    id.and_then(|gid| state.gastos.iter().find(|g| g.id == gid))
}

fn pagado_por_gasto(state: &PagosRealizadosState, gasto_id: Option<i64>) -> f64 {
    gasto_id.map(|gid| state.pagos.iter().filter(|p| p.gasto_id == Some(gid)).map(|p| p.monto).sum()).unwrap_or(0.0)
}

pub fn pagos_realizados_view<'a, Message: 'a + Clone>(
    state: &'a PagosRealizadosState, on_nuevo: Message,
    on_form_msg: impl Fn(PagoRealizadoFormMessage) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_desde: impl Fn(String) -> Message + 'a,
    on_hasta: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }

    let filtrados: Vec<&PagoRealizado> = state.pagos.iter().filter(|p| {
        if !state.busqueda.is_empty() {
            let q = state.busqueda.to_lowercase();
            if !p.notas.to_lowercase().contains(&q) && !p.referencia.to_lowercase().contains(&q)
                && !nombre_proveedor(state, p.proveedor_id).to_lowercase().contains(&q) { return false; }
        }
        if !state.desde.is_empty() && p.fecha.as_str() < state.desde.as_str() { return false; }
        if !state.hasta.is_empty() && p.fecha.as_str() > state.hasta.as_str() { return false; }
        true
    }).collect();

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|p| {
        let id = p.id;
        let prov = nombre_proveedor(state, p.proveedor_id);
        let (numero, resta): (String, Option<f64>) = match info_gasto(state, p.gasto_id) {
            Some(g) => (g.numero.clone().unwrap_or_default(), Some((g.total - pagado_por_gasto(state, p.gasto_id)).max(0.0))),
            None => (String::new(), None),
        };
        row![
            text(&p.fecha).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(2)),
            text(prov).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(format!("${:.2}", p.monto)).size(12).color(COLOR_CXP).width(Length::FillPortion(1)),
            text(p.metodo_pago.as_deref().unwrap_or("")).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&p.referencia).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(if numero.is_empty() { "—".to_string() } else { numero }).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            text(match resta {
                Some(r) if r > 0.01 => format!("queda ${:.2}", r),
                Some(_) => "pagada".to_string(),
                None => "—".to_string(),
            }).size(11).color(if resta.unwrap_or(0.0) > 0.01 { COLOR_DANGER } else { COLOR_SUCCESS }).width(Length::FillPortion(2)),
            button(text("\u{2715}").size(12).color(COLOR_DANGER))
                .style(|_, _| ghost_button_style())
                .on_press(on_eliminar(id)).padding([4, 6]),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    let total_pagado: f64 = filtrados.iter().map(|p| p.monto).sum();

    column![
        row![
            text("Pagos Realizados").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text(format!("Total pagado: ${:.2}", total_pagado)).size(13).color(COLOR_CXP),
            Space::new().width(20),
            text_input("Buscar...", &state.busqueda).on_input(on_buscar).style(|_, _| input_style()).width(140),
            text_input("Desde", &state.desde).on_input(on_desde).style(|_, _| input_style()).width(110),
            text_input("Hasta", &state.hasta).on_input(on_hasta).style(|_, _| input_style()).width(110),
            button(text("+ Nuevo Pago").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style())
                .on_press(on_nuevo)
                .padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a PagosRealizadosState,
    on_form_msg: impl Fn(PagoRealizadoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Pago Realizado" } else { "Nuevo Pago Realizado (Abono)" };
    let met_id: i64 = match state.form.metodo_pago.as_str() {
        "transferencia" => 2, "tarjeta" => 3, "cheque" => 4, "otro" => 5, _ => 1,
    };
    let gasto_info = info_gasto(state, state.form.gasto_id);
    let fields: Vec<Element<'a, PagoRealizadoFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Gasto a pagar", &state.opciones_gastos, state.form.gasto_id.unwrap_or(0), PagoRealizadoFormMessage::GastoSeleccionado),
            pick_list_field("Proveedor", &state.opciones_proveedores, state.form.proveedor_id.unwrap_or(0), PagoRealizadoFormMessage::ProveedorId),
        ),
        form_two_columns(
            labeled_input_f64("Monto del pago", &state.form.monto, "0.00", PagoRealizadoFormMessage::Monto),
            pick_list_field("Método de Pago", &state.opciones_metodo, met_id, move |id| PagoRealizadoFormMessage::MetodoPago(state.opciones_metodo.iter().find(|o| o.id == id).map(|o| o.label.clone()).unwrap_or_else(|| "efectivo".to_string()))),
        ),
        form_two_columns(labeled_input("Referencia", &state.form.referencia, "Folio o referencia", PagoRealizadoFormMessage::Referencia), labeled_input("Notas", &state.form.notas, "Notas", PagoRealizadoFormMessage::Notas)),
    ];
    let mut footer: Vec<Element<'a, PagoRealizadoFormMessage>> = Vec::new();
    if let Some(g) = gasto_info {
        let ya = pagado_por_gasto(state, state.form.gasto_id);
        footer.push(
            row![
                text(format!("Total gasto: ${:.2} · Ya pagado: ${:.2} · Falta: ${:.2}", g.total, ya, (g.total - ya).max(0.0))).size(13).color(COLOR_DANGER),
            ].spacing(0).into()
        );
    }
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, PagoRealizadoFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card(title, fields.into_iter().chain(footer).map(map_fn), Some(guardar(PagoRealizadoFormMessage::Guardar)), cancelar(PagoRealizadoFormMessage::Cancelar), "Guardar")
}
