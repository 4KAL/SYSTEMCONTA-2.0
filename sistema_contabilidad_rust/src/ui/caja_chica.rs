use iced::widget::{button, column, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::ArqueoCaja;
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, form_two_columns};

#[derive(Debug, Clone)]
pub struct ArqueoFormData {
    pub fecha: String,
    pub responsable: String,
    pub monto_esperado: String,
    pub monto_real: String,
    pub observacion: String,
}

impl Default for ArqueoFormData {
    fn default() -> Self {
        Self {
            fecha: chrono::Local::now().format("%Y-%m-%d").to_string(),
            responsable: String::new(), monto_esperado: String::new(),
            monto_real: String::new(), observacion: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ArqueoFormMessage {
    Fecha(String), Responsable(String), MontoEsperado(String), MontoReal(String),
    Observacion(String), Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub struct CajaChicaState {
    pub arqueos: Vec<ArqueoCaja>,
    pub show_form: bool,
    pub form: ArqueoFormData,
    pub busqueda: String,
}

impl Default for CajaChicaState {
    fn default() -> Self {
        Self { arqueos: Vec::new(), show_form: false, form: ArqueoFormData::default(), busqueda: String::new() }
    }
}

pub fn caja_chica_view<'a, Message: 'a + Clone>(
    state: &'a CajaChicaState,
    on_nuevo: Message,
    on_form_msg: impl Fn(ArqueoFormMessage) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form {
        let guardar = on_form_msg(ArqueoFormMessage::Guardar);
        let cancelar = on_form_msg(ArqueoFormMessage::Cancelar);
        let f_fec = on_form_msg.clone();
        let f_res = on_form_msg.clone();
        let f_esp = on_form_msg.clone();
        let f_real = on_form_msg.clone();
        let f_obs = on_form_msg.clone();
        return form_card(
            "Nuevo Arqueo de Caja",
            vec![
                labeled_input("Responsable", &state.form.responsable, "Nombre del responsable", move |v| f_res(ArqueoFormMessage::Responsable(v))),
                labeled_input("Fecha", &state.form.fecha, "YYYY-MM-DD", move |v| f_fec(ArqueoFormMessage::Fecha(v))),
                form_two_columns(
                    labeled_input_f64("Monto esperado", &state.form.monto_esperado, "0.00", move |v| f_esp(ArqueoFormMessage::MontoEsperado(v))),
                    labeled_input_f64("Monto real", &state.form.monto_real, "0.00", move |v| f_real(ArqueoFormMessage::MontoReal(v))),
                ),
                labeled_input("Observación", &state.form.observacion, "Observación", move |v| f_obs(ArqueoFormMessage::Observacion(v))),
            ],
            Some(guardar), cancelar, "Guardar",
        );
    }

    let filtrados: Vec<&ArqueoCaja> = if state.busqueda.is_empty() {
        state.arqueos.iter().collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.arqueos.iter().filter(|a|
            a.responsable.to_lowercase().contains(&q) || a.observacion.to_lowercase().contains(&q)
        ).collect()
    };

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|a| {
        let id = a.id;
        row![
            text(&a.fecha).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text(&a.responsable).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(2)),
            text(format!("Esperado: {:.2}", a.monto_esperado)).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(format!("Real: {:.2}", a.monto_real)).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
            text(format!("Dif: {:.2}", a.diferencia)).size(12)
                .color(if a.diferencia.abs() < 0.01 { COLOR_SUCCESS } else { COLOR_DANGER }).width(Length::FillPortion(2)),
            button(text("\u{2715}").size(12).color(COLOR_DANGER))
                .style(|_, _| ghost_button_style())
                .on_press(on_eliminar(id)).padding([4, 6]),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    column![
        row![
            text("Arqueo de Caja / Caja Chica").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text_input("Buscar...", &state.busqueda)
                .on_input(on_buscar)
                .style(|_, _| input_style())
                .width(220),
            button(text("+ Nuevo Arqueo").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style())
                .on_press(on_nuevo)
                .padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}
