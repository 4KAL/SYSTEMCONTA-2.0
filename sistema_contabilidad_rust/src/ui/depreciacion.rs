use iced::widget::{button, column, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::{ActivoFijo, Depreciacion};
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, form_two_columns};

#[derive(Debug, Clone)]
pub struct ActivoFormData {
    pub descripcion: String,
    pub categoria: String,
    pub fecha_adquisicion: String,
    pub valor_adquisicion: String,
    pub valor_residual: String,
    pub vida_util_anios: String,
}

impl Default for ActivoFormData {
    fn default() -> Self {
        Self {
            descripcion: String::new(), categoria: "equipo".to_string(),
            fecha_adquisicion: chrono::Local::now().format("%Y-%m-%d").to_string(),
            valor_adquisicion: String::new(), valor_residual: String::new(), vida_util_anios: "5".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ActivoFormMessage {
    Descripcion(String), Categoria(String), Fecha(String),
    ValorAdquisicion(String), ValorResidual(String), VidaUtil(String),
    Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub enum DepreciacionTab { Activos, Historial }

#[derive(Debug, Clone)]
pub struct DepreciacionState {
    pub tab: DepreciacionTab,
    pub activos: Vec<ActivoFijo>,
    pub historial: Vec<Depreciacion>,
    pub show_form: bool,
    pub editing_id: Option<i64>,
    pub form: ActivoFormData,
    pub periodo: String,
    pub busqueda: String,
}

impl Default for DepreciacionState {
    fn default() -> Self {
        Self {
            tab: DepreciacionTab::Activos, activos: Vec::new(), historial: Vec::new(),
            show_form: false, editing_id: None, form: ActivoFormData::default(),
            periodo: chrono::Local::now().format("%Y-%m").to_string(),
            busqueda: String::new(),
        }
    }
}

pub fn depreciacion_view<'a, Message: 'a + Clone>(
    state: &'a DepreciacionState,
    on_tab: impl Fn(DepreciacionTab) -> Message + 'a + Clone,
    on_form_msg: impl Fn(ActivoFormMessage) -> Message + 'a + Clone,
    on_nuevo: Message,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_depreciar: impl Fn(i64) -> Message + 'a + Clone,
    on_periodo: impl Fn(String) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form {
        let title = if state.editing_id.is_some() { "Editar Activo Fijo" } else { "Nuevo Activo Fijo" };
        let guardar = on_form_msg(ActivoFormMessage::Guardar);
        let cancelar = on_form_msg(ActivoFormMessage::Cancelar);
        let f_des = on_form_msg.clone();
        let f_cat = on_form_msg.clone();
        let f_fec = on_form_msg.clone();
        let f_val = on_form_msg.clone();
        let f_res = on_form_msg.clone();
        let f_vid = on_form_msg.clone();
        return form_card(
            title,
            vec![
                labeled_input("Descripción", &state.form.descripcion, "Descripción del activo", move |v| f_des(ActivoFormMessage::Descripcion(v))),
                form_two_columns(
                    labeled_input("Categoría", &state.form.categoria, "equipo", move |v| f_cat(ActivoFormMessage::Categoria(v))),
                    labeled_input("Fecha de adquisición", &state.form.fecha_adquisicion, "YYYY-MM-DD", move |v| f_fec(ActivoFormMessage::Fecha(v))),
                ),
                form_two_columns(
                    labeled_input_f64("Valor de adquisición", &state.form.valor_adquisicion, "0.00", move |v| f_val(ActivoFormMessage::ValorAdquisicion(v))),
                    labeled_input_f64("Valor residual", &state.form.valor_residual, "0.00", move |v| f_res(ActivoFormMessage::ValorResidual(v))),
                ),
                labeled_input_f64("Vida útil (años)", &state.form.vida_util_anios, "5", move |v| f_vid(ActivoFormMessage::VidaUtil(v))),
            ],
            Some(guardar), cancelar, "Guardar",
        );
    }

    let tabs = row![
        button(text("Activos Fijos").size(13).color(if matches!(state.tab, DepreciacionTab::Activos) { COLOR_ACCENT } else { COLOR_TEXT_MUTED }))
            .style(|_, _| ghost_button_style())
            .on_press(on_tab(DepreciacionTab::Activos)),
        button(text("Historial de Depreciación").size(13).color(if matches!(state.tab, DepreciacionTab::Historial) { COLOR_ACCENT } else { COLOR_TEXT_MUTED }))
            .style(|_, _| ghost_button_style())
            .on_press(on_tab(DepreciacionTab::Historial)),
    ].spacing(SPACING_SM).align_y(Alignment::Center);

    let mut rows: Vec<Element<'a, Message>> = Vec::new();
    match state.tab {
        DepreciacionTab::Activos => {
            for a in state.activos.iter().filter(|a| a.activo) {
                let id = a.id;
                let valor_libro = a.valor_adquisicion - a.depreciacion_acumulada;
                rows.push(row![
                    text(&a.descripcion).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                    text(&a.categoria).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
                    text(format!("${:.2}", a.valor_adquisicion)).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                    text(format!("Dep/mes: {:.2}", a.depreciacion_mensual)).size(11).color(COLOR_GASTOS).width(Length::FillPortion(2)),
                    text(format!("Valor libro: {:.2}", valor_libro)).size(11).color(COLOR_VENTAS).width(Length::FillPortion(2)),
                    button(text("Depreciar").size(11).color(COLOR_ACCENT))
                        .style(|_, _| ghost_button_style())
                        .on_press(on_depreciar(id)).padding([4, 6]),
                    button(text("\u{2715}").size(12).color(COLOR_DANGER))
                        .style(|_, _| ghost_button_style())
                        .on_press(on_eliminar(id)).padding([4, 6]),
                ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into());
            }
        }
        DepreciacionTab::Historial => {
            for d in state.historial.iter() {
                let id = d.id;
                rows.push(row![
                    text(&d.activo_descripcion).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                    text(&d.periodo).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
                    text(format!("Monto: {:.2}", d.monto)).size(11).color(COLOR_GASTOS).width(Length::FillPortion(2)),
                    text(format!("Acumulado: {:.2}", d.acumulado)).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                    text(&d.fecha).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
                    button(text("\u{2715}").size(12).color(COLOR_DANGER))
                        .style(|_, _| ghost_button_style())
                        .on_press(on_eliminar(id)).padding([4, 6]),
                ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into());
            }
        }
    }

    column![
        row![
            text("Depreciación de Activos").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text_input("Periodo (YYYY-MM)", &state.periodo)
                .on_input(on_periodo)
                .style(|_, _| input_style())
                .width(150),
            button(text("+ Nuevo Activo").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style())
                .on_press(on_nuevo)
                .padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        row![tabs].padding([0.0, SPACING_LG]),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}
