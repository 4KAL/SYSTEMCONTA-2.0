use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length};
use crate::models::PlanCuentas;
use crate::theme::*;
use crate::ui::forms::{form_card, labeled_input, texto_error};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PlanCuentasFormData {
    pub codigo: String,
    pub nombre: String,
    pub tipo: String,
    pub naturaleza: String,
    pub nivel: String,
    pub padre_id: String,
    pub activo: bool,
}

impl Default for PlanCuentasFormData {
    fn default() -> Self { Self { codigo: String::new(), nombre: String::new(), tipo: "activo".to_string(), naturaleza: "deudora".to_string(), nivel: "1".to_string(), padre_id: String::new(), activo: true } }
}

#[derive(Debug, Clone)]
pub enum PlanCuentasFormMessage {
    Codigo(String),
    Nombre(String),
    Tipo(String),
    Naturaleza(String),
    Nivel(String),
    PadreId(String),
    Activo(bool),
    Guardar,
    Cancelar,
}

#[derive(Debug, Clone)]
pub struct PlanCuentasState {
    pub cuentas: Vec<PlanCuentas>,
    pub show_form: bool,
    pub editing_id: Option<i64>,
    pub busqueda: String,
    pub form: PlanCuentasFormData,
    pub errores: HashMap<String, String>,
}

impl Default for PlanCuentasState {
    fn default() -> Self { Self { cuentas: Vec::new(), show_form: false, editing_id: None, busqueda: String::new(), form: PlanCuentasFormData::default(), errores: HashMap::new() } }
}

pub fn plan_cuentas_view<'a, Message: 'a + Clone>(
    state: &'a PlanCuentasState,
    on_nuevo: Message,
    on_editar: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_form_msg: impl Fn(PlanCuentasFormMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let tipo_color = |t: &str| -> iced::Color {
        match t {
            "activo" => COLOR_INFO,
            "pasivo" => COLOR_CXP,
            "capital" => COLOR_UTILIDAD,
            "ingreso" => COLOR_VENTAS,
            "gasto" => COLOR_GASTOS,
            _ => COLOR_TEXT_SECONDARY,
        }
    };

    let filtered: Vec<&PlanCuentas> = if state.busqueda.is_empty() {
        state.cuentas.iter().collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.cuentas.iter().filter(|c| c.codigo.to_lowercase().contains(&q) || c.nombre.to_lowercase().contains(&q)).collect()
    };

    let rows: Vec<Element<'a, Message>> = filtered.iter().map(|c| {
        let ident = "  ".repeat(c.nivel as usize - 1);
        let row_bg = if c.activo { COLOR_CARD } else { COLOR_BG };
        container(
            row![
                text(format!("{}{}", ident, c.codigo)).size(12).color(COLOR_ACCENT).width(Length::FillPortion(2)),
                text(format!("{}{}", ident, c.nombre)).size(12).color(if c.activo { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_MUTED }).width(Length::FillPortion(3)),
                text(&c.tipo).size(11).color(tipo_color(&c.tipo)).width(Length::FillPortion(1)),
                text(&c.naturaleza).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(1)),
                row![
                    button(text("editar").size(11)).style(|_, _| ghost_button_style()).on_press((on_editar)(c.id)),
                    button(text("eliminar").size(11)).style(|_, _| danger_button_style()).on_press((on_eliminar)(c.id)),
                ].spacing(SPACING_XS).width(Length::FillPortion(2)),
            ]
            .spacing(SPACING_SM)
            .align_y(iced::Alignment::Center)
            .padding([SPACING_SM, SPACING_MD])
        )
        .style(move |_| iced::widget::container::Style {
            background: Some(iced::Background::Color(row_bg)),
            border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY),
            snap: false,
            shadow: iced::Shadow::default(),
        })
        .into()
    }).collect();

    let header = row![
        text("Plan de Cuentas").size(22).color(COLOR_TEXT_PRIMARY).width(Length::Fill),
        text_input("Buscar...", &state.busqueda)
            .on_input(on_buscar).size(13)
            .style(|_, _| input_style()).width(Length::Fixed(200.0)),
        button(text("+ Nueva Cuenta").size(13)).style(|_, _| primary_button_style()).on_press(on_nuevo),
    ].spacing(SPACING_MD).align_y(iced::Alignment::Center);

    let list = scrollable(column(rows).spacing(SPACING_XS).width(Length::Fill))
        .style(|_, _| scrollable_style());

    let mut content: Element<'a, Message> = column![header, list]
        .padding(SPACING_LG)
        .spacing(SPACING_MD)
        .width(Length::Fill)
        .into();

    if state.show_form {
        let titulo = if state.editing_id.is_some() { "Editar Cuenta" } else { "Nueva Cuenta" };
        let guardar = on_form_msg(PlanCuentasFormMessage::Guardar);
        let cancelar = on_form_msg(PlanCuentasFormMessage::Cancelar);
        let f_codigo = on_form_msg.clone();
        let f_nivel = on_form_msg.clone();
        let f_padre = on_form_msg.clone();
        let mut fields: Vec<Element<'a, Message>> = vec![
            labeled_input("Código", &state.form.codigo, "Código", move |v| f_codigo(PlanCuentasFormMessage::Codigo(v))),
        ];
        if let Some(err) = texto_error("codigo", &state.errores) { fields.push(err); }
        let f_nombre2 = on_form_msg.clone();
        fields.push(labeled_input("Nombre", &state.form.nombre, "Nombre", move |v| f_nombre2(PlanCuentasFormMessage::Nombre(v))));
        if let Some(err) = texto_error("nombre", &state.errores) { fields.push(err); }
        fields.extend(vec![
            labeled_input("Nivel", &state.form.nivel, "Nivel", move |v| f_nivel(PlanCuentasFormMessage::Nivel(v))),
            labeled_input("Padre ID", &state.form.padre_id, "Padre ID", move |v| f_padre(PlanCuentasFormMessage::PadreId(v))),
        ]);
        let form = form_card(titulo, fields, Some(guardar), cancelar, "Guardar");
        content = column![content, form].into();
    }

    content
}
