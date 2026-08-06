use iced::widget::{button, column, row, scrollable, text, Space};
use iced::{Element, Length, Alignment};
use crate::models::Ubicacion;
use crate::theme::*;
use super::forms::{form_card, labeled_input, form_two_columns, texto_error};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UbicacionFormData { pub nombre: String, pub direccion: String, pub ciudad: String, pub encargado: String, pub cedula: String, pub telefono: String }
impl Default for UbicacionFormData { fn default() -> Self { Self { nombre: String::new(), direccion: String::new(), ciudad: String::new(), encargado: String::new(), cedula: String::new(), telefono: String::new() } } }
#[derive(Debug, Clone)]
pub struct UbicacionesState { pub ubicaciones: Vec<Ubicacion>, pub show_form: bool, pub editing_id: Option<i64>, pub form: UbicacionFormData, pub errores: HashMap<String, String> }
impl Default for UbicacionesState { fn default() -> Self { Self { ubicaciones: Vec::new(), show_form: false, editing_id: None, form: UbicacionFormData::default(), errores: HashMap::new() } } }

#[derive(Debug, Clone)]
pub enum UbicacionFormMessage { Nombre(String), Direccion(String), Ciudad(String), Encargado(String), Cedula(String), Telefono(String), Guardar, Cancelar }

pub fn ubicaciones_view<'a, Message: 'a + Clone>(
    state: &'a UbicacionesState, on_nueva: Message,
    on_editar: impl Fn(i64) -> Message + 'a + Clone, on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_form_msg: impl Fn(UbicacionFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }
    let rows: Vec<Element<'a, Message>> = state.ubicaciones.iter().filter(|u| u.activo).map(|u| {
        let id = u.id;
        row![
            text(&u.nombre).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&u.ciudad).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(u.encargado.as_deref().unwrap_or("")).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&u.telefono).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            row![
                button(text("\u{270E}").size(12)).style(|_, _| ghost_button_style()).on_press(on_editar(id)).padding([4, 6]),
                button(text("\u{2715}").size(11)).style(|_, _| ghost_button_style()).on_press(on_eliminar(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(1)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();
    column![
        row![text("Ubicaciones").size(24).color(COLOR_TEXT_PRIMARY), Space::new().width(Length::Fill),
            button(text("+ Nueva").size(13).color(COLOR_TEXT_PRIMARY)).style(|_, _| primary_button_style()).on_press(on_nueva).padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a UbicacionesState, on_form_msg: impl Fn(UbicacionFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Ubicación" } else { "Nueva Ubicación" };
    let mut fields: Vec<Element<'a, UbicacionFormMessage>> = vec![
        form_two_columns(labeled_input("Nombre", &state.form.nombre, "Nombre", UbicacionFormMessage::Nombre), labeled_input("Teléfono", &state.form.telefono, "555-123-4567", UbicacionFormMessage::Telefono)),
    ];
    if let Some(err) = texto_error("nombre", &state.errores) { fields.push(err); }
    fields.extend(vec![
        form_two_columns(labeled_input("Encargado", &state.form.encargado, "Encargado", UbicacionFormMessage::Encargado), labeled_input("Cédula", &state.form.cedula, "000-0000000-0", UbicacionFormMessage::Cedula)),
        labeled_input("Dirección", &state.form.direccion, "Calle y número", UbicacionFormMessage::Direccion),
    ]);
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, UbicacionFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone(); let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(UbicacionFormMessage::Guardar)), cancelar(UbicacionFormMessage::Cancelar), "Guardar")
}
