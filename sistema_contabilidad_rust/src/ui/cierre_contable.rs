use iced::widget::{button, column, row, scrollable, text, Space};
use iced::{Element, Length, Alignment};
use crate::models::CierreContable;
use crate::theme::*;
use super::forms::{form_card, labeled_input};

#[derive(Debug, Clone)]
pub struct CierreFormData {
    pub anio: String,
    pub notas: String,
}

impl Default for CierreFormData {
    fn default() -> Self {
        Self { anio: chrono::Local::now().format("%Y").to_string(), notas: String::new() }
    }
}

#[derive(Debug, Clone)]
pub enum CierreFormMessage {
    Anio(String), Notas(String), Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub struct CierreState {
    pub cierres: Vec<CierreContable>,
    pub show_form: bool,
    pub form: CierreFormData,
}

impl Default for CierreState {
    fn default() -> Self {
        Self { cierres: Vec::new(), show_form: false, form: CierreFormData::default() }
    }
}

pub fn cierre_contable_view<'a, Message: 'a + Clone>(
    state: &'a CierreState,
    on_nuevo: Message,
    on_form_msg: impl Fn(CierreFormMessage) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form {
        let guardar = on_form_msg(CierreFormMessage::Guardar);
        let cancelar = on_form_msg(CierreFormMessage::Cancelar);
        let f_anio = on_form_msg.clone();
        let f_not = on_form_msg.clone();
        return form_card(
            "Nuevo Cierre Contable",
            vec![
                labeled_input("Año del ejercicio", &state.form.anio, "2026", move |v| f_anio(CierreFormMessage::Anio(v))),
                labeled_input("Notas", &state.form.notas, "Notas del cierre", move |v| f_not(CierreFormMessage::Notas(v))),
            ],
            Some(guardar), cancelar, "Generar Cierre",
        );
    }

    let rows: Vec<Element<'a, Message>> = state.cierres.iter().map(|c| {
        let id = c.id;
        row![
            text(c.anio.to_string()).size(12).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(1)),
            text(&c.fecha).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
            text(format!("Ingresos: {:.2}", c.ingresos)).size(11).color(COLOR_VENTAS).width(Length::FillPortion(2)),
            text(format!("Gastos: {:.2}", c.gastos)).size(11).color(COLOR_GASTOS).width(Length::FillPortion(2)),
            text(format!("Utilidad: {:.2}", c.utilidad)).size(12)
                .color(if c.utilidad >= 0.0 { COLOR_SUCCESS } else { COLOR_DANGER }).width(Length::FillPortion(2)),
            text(&c.estado).size(10)
                .color(if c.estado == "cerrado" { COLOR_SUCCESS } else { COLOR_TEXT_MUTED }).width(Length::FillPortion(1)),
            button(text("\u{2715}").size(12).color(COLOR_DANGER))
                .style(|_, _| ghost_button_style())
                .on_press(on_eliminar(id)).padding([4, 6]),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    column![
        row![
            text("Cierre Contable").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            button(text("+ Generar Cierre").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style())
                .on_press(on_nuevo)
                .padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        row![text("El cierre calcula ingresos (ventas) y gastos del año para determinar la utilidad del ejercicio.")
            .size(11).color(COLOR_TEXT_MUTED)].padding([0.0, SPACING_LG]),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}
