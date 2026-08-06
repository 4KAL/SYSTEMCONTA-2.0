use iced::widget::{button, column, row, scrollable, text, Space};
use iced::{Element, Length, Alignment};
use crate::models::MaquinaUbicada;
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, form_two_columns};

#[derive(Debug, Clone)]
pub struct MaquinaFormData { pub codigo: String, pub descripcion: String, pub modelo: String, pub numero_serie: String, pub comision: String, pub ubicacion_texto: String, pub fecha_instalacion: String }
impl Default for MaquinaFormData { fn default() -> Self { Self { codigo: String::new(), descripcion: String::new(), modelo: String::new(), numero_serie: String::new(), comision: String::new(), ubicacion_texto: String::new(), fecha_instalacion: String::new() } } }
#[derive(Debug, Clone)]
pub struct MaquinasState { pub maquinas: Vec<MaquinaUbicada>, pub show_form: bool, pub editing_id: Option<i64>, pub form: MaquinaFormData }
impl Default for MaquinasState { fn default() -> Self { Self { maquinas: Vec::new(), show_form: false, editing_id: None, form: MaquinaFormData::default() } } }

#[derive(Debug, Clone)]
pub enum MaquinaFormMessage { Codigo(String), Descripcion(String), Modelo(String), NumeroSerie(String), Comision(String), UbicacionTexto(String), FechaInstalacion(String), Guardar, Cancelar }

pub fn maquinas_view<'a, Message: 'a + Clone>(
    state: &'a MaquinasState, on_nueva: Message,
    on_editar: impl Fn(i64) -> Message + 'a + Clone, on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_form_msg: impl Fn(MaquinaFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form { return render_form(state, on_form_msg); }
    let rows: Vec<Element<'a, Message>> = state.maquinas.iter().filter(|m| m.activo).map(|m| {
        let id = m.id;
        row![
            text(m.codigo.as_deref().unwrap_or("")).size(12).color(COLOR_ACCENT).width(Length::FillPortion(2)),
            text(&m.descripcion).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
            text(&m.modelo).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(&m.fecha_instalacion).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text(format!("{:.0}%", m.comision)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            text(&m.ubicacion_texto).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            row![
                button(text("\u{270E}").size(12)).style(|_, _| ghost_button_style()).on_press(on_editar(id)).padding([4, 6]),
                button(text("\u{2715}").size(11)).style(|_, _| ghost_button_style()).on_press(on_eliminar(id)).padding([4, 6]),
            ].spacing(SPACING_XS).width(Length::FillPortion(1)),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();
    column![
        row![text("Máquinas").size(24).color(COLOR_TEXT_PRIMARY), Space::new().width(Length::Fill),
            button(text("+ Nueva").size(13).color(COLOR_TEXT_PRIMARY)).style(|_, _| primary_button_style()).on_press(on_nueva).padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}

fn render_form<'a, Message: 'a + Clone>(
    state: &'a MaquinasState, on_form_msg: impl Fn(MaquinaFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let title = if state.editing_id.is_some() { "Editar Máquina" } else { "Nueva Máquina" };
    let fields: Vec<Element<'a, MaquinaFormMessage>> = vec![
        form_two_columns(labeled_input("Código", &state.form.codigo, "MAQ-001", MaquinaFormMessage::Codigo), labeled_input("Modelo", &state.form.modelo, "Modelo", MaquinaFormMessage::Modelo)),
        labeled_input("Descripción", &state.form.descripcion, "Descripción", MaquinaFormMessage::Descripcion),
        form_two_columns(labeled_input("Número de Serie", &state.form.numero_serie, "SN-12345", MaquinaFormMessage::NumeroSerie), labeled_input_f64("Comisión (%)", &state.form.comision, "0.00", MaquinaFormMessage::Comision)),
        form_two_columns(
            labeled_input("Ubicación", &state.form.ubicacion_texto, "Quito - Ej. Conocoto", MaquinaFormMessage::UbicacionTexto),
            labeled_input("Fecha que se dejó (AAAA-MM-DD)", &state.form.fecha_instalacion, "2026-08-09", MaquinaFormMessage::FechaInstalacion),
        ),
    ];
    let fm_clone = on_form_msg.clone();
    let map_fn = move |f: Element<'a, MaquinaFormMessage>| { let cb = fm_clone.clone(); f.map(move |msg| cb(msg)) };
    let guardar = on_form_msg.clone(); let cancelar = on_form_msg;
    form_card(title, fields.into_iter().map(map_fn), Some(guardar(MaquinaFormMessage::Guardar)), cancelar(MaquinaFormMessage::Cancelar), "Guardar")
}
