use iced::widget::{button, column, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{PagoRecibido, Venta};
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, pick_list_field, SelectOption, form_two_columns};

#[derive(Debug, Clone)]
pub struct PagoRecibidoFormData { pub venta_id: Option<i64>, pub cliente_id: Option<i64>, pub monto: String, pub metodo_pago: String, pub referencia: String, pub notas: String }
impl Default for PagoRecibidoFormData {
    fn default() -> Self { Self { venta_id: None, cliente_id: None, monto: String::new(), metodo_pago: "efectivo".to_string(), referencia: String::new(), notas: String::new() } }
}
#[derive(Debug, Clone)]
pub struct PagosRecibidosState {
    pub pagos: Vec<PagoRecibido>,
    pub ventas: Vec<Venta>,
    pub show_form: bool,
    pub form: PagoRecibidoFormData,
    pub opciones_clientes: Vec<SelectOption>,
    pub opciones_ventas: Vec<SelectOption>,
    pub opciones_metodo: Vec<SelectOption>,
    pub editing_id: Option<i64>,
    pub busqueda: String,
    pub desde: String,
    pub hasta: String,
}
impl Default for PagosRecibidosState {
    fn default() -> Self {
        Self {
            pagos: Vec::new(), ventas: Vec::new(), show_form: false,
            form: PagoRecibidoFormData::default(),
            opciones_clientes: Vec::new(), opciones_ventas: Vec::new(),
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
pub enum PagoRecibidoFormMessage {
    VentaSeleccionada(i64),
    ClienteId(i64),
    Monto(String),
    MetodoPago(String),
    Referencia(String),
    Notas(String),
    Guardar,
    Cancelar,
}

fn nombre_cliente<'a>(state: &'a PagosRecibidosState, id: Option<i64>) -> String {
    id.and_then(|cid| state.opciones_clientes.iter().find(|o| o.id == cid))
        .map(|o| o.label.clone())
        .unwrap_or_else(|| "—".to_string())
}

fn info_venta<'a>(state: &'a PagosRecibidosState, id: Option<i64>) -> Option<&'a Venta> {
    id.and_then(|vid| state.ventas.iter().find(|v| v.id == vid))
}

pub fn pagos_recibidos_view<'a, Message: 'a + Clone>(
    state: &'a PagosRecibidosState, on_nuevo: Message,
    on_form_msg: impl Fn(PagoRecibidoFormMessage) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_desde: impl Fn(String) -> Message + 'a,
    on_hasta: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }

    let filtrados: Vec<&PagoRecibido> = state.pagos.iter().filter(|p| {
        if !state.busqueda.is_empty() {
            let q = state.busqueda.to_lowercase();
            if !p.notas.to_lowercase().contains(&q) && !p.referencia.to_lowercase().contains(&q)
                && !nombre_cliente(state, p.cliente_id).to_lowercase().contains(&q) { return false; }
        }
        if !state.desde.is_empty() && p.fecha.as_str() < state.desde.as_str() { return false; }
        if !state.hasta.is_empty() && p.fecha.as_str() > state.hasta.as_str() { return false; }
        true
    }).collect();

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|p| {
        let id = p.id;
        let cli = nombre_cliente(state, p.cliente_id);
        let (folio, resto): (String, Option<f64>) = match info_venta(state, p.venta_id) {
            Some(v) => (v.folio.clone(), Some(v.saldo_pendiente)),
            None => (String::new(), None),
        };
        row![
            text(&p.fecha).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(2)),
            text(cli).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(format!("${:.2}", p.monto)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            text(p.metodo_pago.as_deref().unwrap_or("")).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&p.referencia).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(if folio.is_empty() { "—".to_string() } else { folio }).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            text(match resto {
                Some(r) if r > 0.01 => format!("queda ${:.2}", r),
                Some(_) => "pagada".to_string(),
                None => "—".to_string(),
            }).size(11).color(if resto.unwrap_or(0.0) > 0.01 { COLOR_DANGER } else { COLOR_SUCCESS }).width(Length::FillPortion(2)),
            button(text("\u{2715}").size(12).color(COLOR_DANGER))
                .style(|_, _| ghost_button_style())
                .on_press(on_eliminar(id)).padding([4, 6]),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    let total_abonado: f64 = filtrados.iter().map(|p| p.monto).sum();

    column![
        row![
            text("Pagos Recibidos").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text(format!("Total abonado: ${:.2}", total_abonado)).size(13).color(COLOR_VENTAS),
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
    state: &'a PagosRecibidosState,
    on_form_msg: impl Fn(PagoRecibidoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Pago Recibido" } else { "Nuevo Pago Recibido (Abono)" };
    let met_id: i64 = match state.form.metodo_pago.as_str() {
        "transferencia" => 2, "tarjeta" => 3, "cheque" => 4, "otro" => 5, _ => 1,
    };
    let venta_info = info_venta(state, state.form.venta_id);
    let fields: Vec<Element<'a, PagoRecibidoFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Venta a abonar", &state.opciones_ventas, state.form.venta_id.unwrap_or(0), PagoRecibidoFormMessage::VentaSeleccionada),
            pick_list_field("Cliente", &state.opciones_clientes, state.form.cliente_id.unwrap_or(0), PagoRecibidoFormMessage::ClienteId),
        ),
        form_two_columns(
            labeled_input_f64("Monto del abono", &state.form.monto, "0.00", PagoRecibidoFormMessage::Monto),
            pick_list_field("Método de Pago", &state.opciones_metodo, met_id, move |id| PagoRecibidoFormMessage::MetodoPago(state.opciones_metodo.iter().find(|o| o.id == id).map(|o| o.label.clone()).unwrap_or_else(|| "efectivo".to_string()))),
        ),
        form_two_columns(labeled_input("Referencia", &state.form.referencia, "Folio o referencia", PagoRecibidoFormMessage::Referencia), labeled_input("Notas", &state.form.notas, "Notas", PagoRecibidoFormMessage::Notas)),
    ];
    let mut footer: Vec<Element<'a, PagoRecibidoFormMessage>> = Vec::new();
    if let Some(v) = venta_info {
        footer.push(
            row![
                text(format!("Debe de esta venta: ${:.2}", v.saldo_pendiente)).size(13).color(COLOR_DANGER),
                Space::new().width(10),
                text(format!("Total venta: ${:.2}", v.total)).size(12).color(COLOR_TEXT_SECONDARY),
            ].spacing(0).into()
        );
    }
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, PagoRecibidoFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    form_card(title, fields.into_iter().chain(footer).map(map_fn), Some(guardar(PagoRecibidoFormMessage::Guardar)), cancelar(PagoRecibidoFormMessage::Cancelar), "Guardar")
}
