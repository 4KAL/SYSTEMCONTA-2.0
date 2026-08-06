use iced::widget::{button, column, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::maquina::CobroComision;
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, pick_list_field, SelectOption, form_two_columns};

#[derive(Debug, Clone)]
pub struct CobroComisionFormData {
    pub maquina_id: String,
    pub monto: String,
    pub mes: String,
    pub periodo: String,
    pub observacion: String,
    pub notas: String,
}

impl Default for CobroComisionFormData {
    fn default() -> Self { Self { maquina_id: String::new(), monto: String::new(), mes: String::new(), periodo: String::new(), observacion: String::new(), notas: String::new() } }
}

#[derive(Debug, Clone)]
pub enum CobroComisionFormMessage {
    MaquinaId(String), Monto(String), Mes(String), Periodo(String), Observacion(String), Notas(String), Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub struct CobroComisionesState {
    pub comisiones: Vec<CobroComision>,
    pub show_form: bool,
    pub editing_id: Option<i64>,
    pub busqueda: String,
    pub form: CobroComisionFormData,
    pub opciones_maquinas: Vec<SelectOption>,
}

impl Default for CobroComisionesState {
    fn default() -> Self { Self { comisiones: Vec::new(), show_form: false, editing_id: None, busqueda: String::new(), form: CobroComisionFormData::default(), opciones_maquinas: Vec::new() } }
}

pub fn cobro_comisiones_view<'a, Message: 'a + Clone>(
    state: &'a CobroComisionesState,
    on_nuevo: Message,
    on_form_msg: impl Fn(CobroComisionFormMessage) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form {
        let title = if state.editing_id.is_some() { "Editar Cobro Comisión" } else { "Nuevo Cobro Comisión" };
        let guardar = on_form_msg(CobroComisionFormMessage::Guardar);
        let cancelar = on_form_msg(CobroComisionFormMessage::Cancelar);
        let f_maq = on_form_msg.clone();
        let f_monto = on_form_msg.clone();
        let f_mes = on_form_msg.clone();
        let f_per = on_form_msg.clone();
        let f_obs = on_form_msg.clone();
        let f_not = on_form_msg.clone();
        let maq_id: i64 = state.form.maquina_id.parse().unwrap_or(0);
        return form_card(
            title,
            vec![
                pick_list_field("Máquina", &state.opciones_maquinas, maq_id, move |id| f_maq(CobroComisionFormMessage::MaquinaId(id.to_string()))),
                labeled_input_f64("Monto", &state.form.monto, "0.00", move |v| f_monto(CobroComisionFormMessage::Monto(v))),
                form_two_columns(
                    labeled_input("Mes", &state.form.mes, "Mes", move |v| f_mes(CobroComisionFormMessage::Mes(v))),
                    labeled_input("Periodo", &state.form.periodo, "Periodo", move |v| f_per(CobroComisionFormMessage::Periodo(v))),
                ),
                labeled_input("Observación", &state.form.observacion, "Observación", move |v| f_obs(CobroComisionFormMessage::Observacion(v))),
                labeled_input("Notas", &state.form.notas, "Notas", move |v| f_not(CobroComisionFormMessage::Notas(v))),
            ],
            Some(guardar), cancelar, "Guardar",
        );
    }

    let filtrados: Vec<&CobroComision> = if state.busqueda.is_empty() {
        state.comisiones.iter().collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.comisiones.iter().filter(|c|
            c.periodo.to_lowercase().contains(&q) || c.notas.to_lowercase().contains(&q)
        ).collect()
    };

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|c| {
        let id = c.id;
        row![
            text(format!("Máquina #{}", c.maquina_id)).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(2)),
            text(format!("${:.2}", c.monto)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(1)),
            text(c.mes.as_deref().unwrap_or("")).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(1)),
            text(&c.periodo).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(1)),
            text(&c.fecha).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            button(text("\u{2715}").size(12).color(COLOR_DANGER))
                .style(|_, _| ghost_button_style())
                .on_press(on_eliminar(id)).padding([4, 6]),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    column![
        row![
            text("Cobro Comisiones").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text_input("Buscar...", &state.busqueda)
                .on_input(on_buscar)
                .style(|_, _| input_style())
                .width(220),
            button(text("+ Nueva Comisión").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style())
                .on_press(on_nuevo)
                .padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}
