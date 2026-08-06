use iced::widget::{button, column, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::Gasto;
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, pick_list_field, SelectOption, form_two_columns};

#[derive(Debug, Clone)]
pub struct GastoFormData { pub categoria_id: String, pub descripcion: String, pub monto: String, pub proveedor: String, pub metodo_pago: String, pub referencia: String, pub notas: String }
impl Default for GastoFormData {
    fn default() -> Self { Self { categoria_id: String::new(), descripcion: String::new(), monto: String::new(), proveedor: String::new(), metodo_pago: "efectivo".to_string(), referencia: String::new(), notas: String::new() } }
}
#[derive(Debug, Clone)]
pub struct GastosState {
    pub gastos: Vec<Gasto>,
    pub show_form: bool,
    pub form: GastoFormData,
    pub opciones_categorias: Vec<SelectOption>,
    pub editing_id: Option<i64>,
    pub busqueda: String,
    pub desde: String,
    pub hasta: String,
}
impl Default for GastosState {
    fn default() -> Self {
        Self {
            gastos: Vec::new(),
            show_form: false,
            form: GastoFormData::default(),
            opciones_categorias: Vec::new(),
            editing_id: None,
            busqueda: String::new(),
            desde: String::new(),
            hasta: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GastoFormMessage {
    CategoriaId(String), Descripcion(String), Monto(String), Proveedor(String),
    MetodoPago(String), Referencia(String), Notas(String), Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub enum GastoMessage {
    Editar(i64),
    Eliminar(i64),
    Buscar(String),
}

pub fn gastos_view<'a, Message: 'a + Clone>(
    state: &'a GastosState, on_nuevo: Message,
    on_form_msg: impl Fn(GastoFormMessage) -> Message + 'a + Clone,
    on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_desde: impl Fn(String) -> Message + 'a,
    on_hasta: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }
    let busqueda_lower = state.busqueda.to_lowercase();
    let rows: Vec<Element<'a, Message>> = state.gastos.iter().filter(|g| {
        if !busqueda_lower.is_empty() && !g.descripcion.to_lowercase().contains(&busqueda_lower) { return false; }
        if !state.desde.is_empty() && g.fecha.as_str() < state.desde.as_str() { return false; }
        if !state.hasta.is_empty() && g.fecha.as_str() > state.hasta.as_str() { return false; }
        true
    }).map(|g| {
        let editar = on_editar(g.id);
        let eliminar = on_eliminar(g.id);
        row![
            text(&g.categoria_nombre).size(12).color(COLOR_ACCENT).width(Length::FillPortion(2)),
            text(&g.descripcion).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&g.fecha).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(format!("${:.2}", g.monto)).size(12).color(COLOR_GASTOS).width(Length::FillPortion(1)),
            text(&g.metodo_pago).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(1)),
            row![
                button(text("\u{270E}").size(11)).style(|_, _| ghost_button_style()).on_press(editar),
                button(text("\u{2715}").size(11)).style(|_, _| ghost_button_style()).on_press(eliminar),
            ].spacing(SPACING_SM).width(Length::FillPortion(1)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();
    column![
        row![text("Gastos").size(24).color(COLOR_TEXT_PRIMARY), Space::new().width(Length::Fill),
            text_input("Buscar...", &state.busqueda).on_input(on_buscar).style(|_, _| input_style()).width(140),
            text_input("Desde", &state.desde).on_input(on_desde).style(|_, _| input_style()).width(120),
            text_input("Hasta", &state.hasta).on_input(on_hasta).style(|_, _| input_style()).width(120),
            button(text("+ Nuevo Gasto").size(13).color(COLOR_TEXT_PRIMARY)).style(|_, _| primary_button_style()).on_press(on_nuevo).padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a GastosState,
    on_form_msg: impl Fn(GastoFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let cat_id: i64 = state.form.categoria_id.parse().unwrap_or(0);
    let fields: Vec<Element<'a, GastoFormMessage>> = vec![
        form_two_columns(
            pick_list_field("Categoría", &state.opciones_categorias, cat_id, |id| GastoFormMessage::CategoriaId(id.to_string())),
            labeled_input_f64("Monto", &state.form.monto, "0.00", GastoFormMessage::Monto),
        ),
        labeled_input("Descripción", &state.form.descripcion, "Descripción del gasto", GastoFormMessage::Descripcion),
        form_two_columns(labeled_input("Método de Pago", &state.form.metodo_pago, "efectivo / tarjeta", GastoFormMessage::MetodoPago), labeled_input("Referencia", &state.form.referencia, "Folio o referencia", GastoFormMessage::Referencia)),
        labeled_input("Proveedor", &state.form.proveedor, "Nombre del proveedor (opcional)", GastoFormMessage::Proveedor),
        labeled_input("Notas", &state.form.notas, "Notas adicionales", GastoFormMessage::Notas),
    ];
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, GastoFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone();
    let cancelar = on_form_msg;
    let title = if state.editing_id.is_some() { "Editar Gasto" } else { "Nuevo Gasto" };
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(GastoFormMessage::Guardar)), cancelar(GastoFormMessage::Cancelar), "Guardar")
}
