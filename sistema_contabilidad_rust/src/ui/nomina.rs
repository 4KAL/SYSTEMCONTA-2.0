use iced::widget::{button, column, row, scrollable, text, Space};
use iced::{Element, Length, Alignment};
use crate::models::{Empleado, RolPago};
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, pick_list_field, form_two_columns, SelectOption};

#[derive(Debug, Clone)]
pub struct EmpleadoFormData {
    pub cedula: String,
    pub nombre: String,
    pub cargo: String,
    pub telefono: String,
    pub sueldo_base: String,
    pub fecha_ingreso: String,
}

impl Default for EmpleadoFormData {
    fn default() -> Self {
        Self {
            cedula: String::new(), nombre: String::new(), cargo: String::new(), telefono: String::new(),
            sueldo_base: String::new(), fecha_ingreso: chrono::Local::now().format("%Y-%m-%d").to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EmpleadoFormMessage {
    Cedula(String), Nombre(String), Cargo(String), Telefono(String),
    SueldoBase(String), FechaIngreso(String), Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub struct RolFormData {
    pub empleado_id: String,
    pub periodo: String,
    pub dias: String,
    pub horas_extra: String,
    pub comisiones: String,
    pub iess: String,
    pub prestamos: String,
    pub otras_retenciones: String,
    pub notas: String,
}

impl Default for RolFormData {
    fn default() -> Self {
        Self {
            empleado_id: String::new(),
            periodo: chrono::Local::now().format("%Y-%m").to_string(),
            dias: "30".to_string(),
            horas_extra: String::new(), comisiones: String::new(),
            iess: String::new(), prestamos: String::new(), otras_retenciones: String::new(),
            notas: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RolFormMessage {
    EmpleadoId(String), Periodo(String), Dias(String), HorasExtra(String), Comisiones(String),
    Iess(String), Prestamos(String), OtrasRetenciones(String), Notas(String), Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub enum NominaTab { Empleados, Roles }

#[derive(Debug, Clone)]
pub struct NominaState {
    pub tab: NominaTab,
    pub empleados: Vec<Empleado>,
    pub roles: Vec<RolPago>,
    pub show_form_empleado: bool,
    pub editing_empleado_id: Option<i64>,
    pub form_empleado: EmpleadoFormData,
    pub show_form_rol: bool,
    pub editing_rol_id: Option<i64>,
    pub form_rol: RolFormData,
    pub opciones_empleados: Vec<SelectOption>,
}

impl Default for NominaState {
    fn default() -> Self {
        Self {
            tab: NominaTab::Empleados,
            empleados: Vec::new(), roles: Vec::new(),
            show_form_empleado: false, editing_empleado_id: None,
            form_empleado: EmpleadoFormData::default(),
            show_form_rol: false, editing_rol_id: None,
            form_rol: RolFormData::default(),
            opciones_empleados: Vec::new(),
        }
    }
}

pub fn nomina_view<'a, Message: 'a + Clone>(
    state: &'a NominaState,
    on_tab: impl Fn(NominaTab) -> Message + 'a + Clone,
    on_form_emp: impl Fn(EmpleadoFormMessage) -> Message + 'a + Clone,
    on_form_rol: impl Fn(RolFormMessage) -> Message + 'a + Clone,
    on_nuevo_empleado: Message,
    on_nuevo_rol: Message,
    on_eliminar_empleado: impl Fn(i64) -> Message + 'a + Clone,
    on_eliminar_rol: impl Fn(i64) -> Message + 'a + Clone,
    on_marcar_pagado: impl Fn(i64) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form_empleado {
        let title = if state.editing_empleado_id.is_some() { "Editar Empleado" } else { "Nuevo Empleado" };
        let guardar = on_form_emp(EmpleadoFormMessage::Guardar);
        let cancelar = on_form_emp(EmpleadoFormMessage::Cancelar);
        let f_ced = on_form_emp.clone();
        let f_nom = on_form_emp.clone();
        let f_car = on_form_emp.clone();
        let f_tel = on_form_emp.clone();
        let f_sue = on_form_emp.clone();
        let f_fec = on_form_emp.clone();
        return form_card(
            title,
            vec![
                labeled_input("Nombre completo", &state.form_empleado.nombre, "Nombre", move |v| f_nom(EmpleadoFormMessage::Nombre(v))),
                form_two_columns(
                    labeled_input("Cédula", &state.form_empleado.cedula, "Cédula", move |v| f_ced(EmpleadoFormMessage::Cedula(v))),
                    labeled_input("Cargo", &state.form_empleado.cargo, "Cargo", move |v| f_car(EmpleadoFormMessage::Cargo(v))),
                ),
                form_two_columns(
                    labeled_input("Teléfono", &state.form_empleado.telefono, "Teléfono", move |v| f_tel(EmpleadoFormMessage::Telefono(v))),
                    labeled_input_f64("Sueldo base", &state.form_empleado.sueldo_base, "0.00", move |v| f_sue(EmpleadoFormMessage::SueldoBase(v))),
                ),
                labeled_input("Fecha de ingreso", &state.form_empleado.fecha_ingreso, "YYYY-MM-DD", move |v| f_fec(EmpleadoFormMessage::FechaIngreso(v))),
            ],
            Some(guardar), cancelar, "Guardar",
        );
    }

    if state.show_form_rol {
        let title = if state.editing_rol_id.is_some() { "Editar Rol de Pago" } else { "Nuevo Rol de Pago" };
        let guardar = on_form_rol(RolFormMessage::Guardar);
        let cancelar = on_form_rol(RolFormMessage::Cancelar);
        let f_emp = on_form_rol.clone();
        let f_per = on_form_rol.clone();
        let f_dia = on_form_rol.clone();
        let f_he = on_form_rol.clone();
        let f_com = on_form_rol.clone();
        let f_iess = on_form_rol.clone();
        let f_pres = on_form_rol.clone();
        let f_otr = on_form_rol.clone();
        let f_not = on_form_rol.clone();
        let emp_id: i64 = state.form_rol.empleado_id.parse().unwrap_or(0);
        return form_card(
            title,
            vec![
                pick_list_field("Empleado", &state.opciones_empleados, emp_id, move |id| f_emp(RolFormMessage::EmpleadoId(id.to_string()))),
                form_two_columns(
                    labeled_input("Periodo (YYYY-MM)", &state.form_rol.periodo, "2026-08", move |v| f_per(RolFormMessage::Periodo(v))),
                    labeled_input("Días", &state.form_rol.dias, "30", move |v| f_dia(RolFormMessage::Dias(v))),
                ),
                form_two_columns(
                    labeled_input_f64("Horas extra", &state.form_rol.horas_extra, "0.00", move |v| f_he(RolFormMessage::HorasExtra(v))),
                    labeled_input_f64("Comisiones", &state.form_rol.comisiones, "0.00", move |v| f_com(RolFormMessage::Comisiones(v))),
                ),
                form_two_columns(
                    labeled_input_f64("IESS (9.45%)", &state.form_rol.iess, "9.45%", move |v| f_iess(RolFormMessage::Iess(v))),
                    labeled_input_f64("Préstamos", &state.form_rol.prestamos, "0.00", move |v| f_pres(RolFormMessage::Prestamos(v))),
                ),
                form_two_columns(
                    labeled_input_f64("Otras retenciones", &state.form_rol.otras_retenciones, "0.00", move |v| f_otr(RolFormMessage::OtrasRetenciones(v))),
                    labeled_input("Notas", &state.form_rol.notas, "Notas", move |v| f_not(RolFormMessage::Notas(v))),
                ),
            ],
            Some(guardar), cancelar, "Guardar",
        );
    }

    let tabs = row![
        button(text("Empleados").size(13).color(if matches!(state.tab, NominaTab::Empleados) { COLOR_ACCENT } else { COLOR_TEXT_MUTED }))
            .style(|_, _| ghost_button_style())
            .on_press(on_tab(NominaTab::Empleados)),
        button(text("Roles de Pago").size(13).color(if matches!(state.tab, NominaTab::Roles) { COLOR_ACCENT } else { COLOR_TEXT_MUTED }))
            .style(|_, _| ghost_button_style())
            .on_press(on_tab(NominaTab::Roles)),
    ].spacing(SPACING_SM).align_y(Alignment::Center);

    let header = row![
        text("Nómina").size(24).color(COLOR_TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        match state.tab {
            NominaTab::Empleados =>
                button(text("+ Nuevo Empleado").size(13).color(COLOR_TEXT_PRIMARY))
                    .style(|_, _| primary_button_style()).on_press(on_nuevo_empleado).padding([SPACING_SM, SPACING_MD]),
            NominaTab::Roles =>
                button(text("+ Nuevo Rol").size(13).color(COLOR_TEXT_PRIMARY))
                    .style(|_, _| primary_button_style()).on_press(on_nuevo_rol).padding([SPACING_SM, SPACING_MD]),
        },
    ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG);

    let mut rows: Vec<Element<'a, Message>> = Vec::new();
    match state.tab {
        NominaTab::Empleados => {
            for e in state.empleados.iter().filter(|e| e.activo) {
                let id = e.id;
                rows.push(row![
                    text(&e.nombre).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                    text(&e.cargo).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                    text(&e.cedula).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(2)),
                    text(format!("${:.2}", e.sueldo_base)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(2)),
                    button(text("\u{2715}").size(12).color(COLOR_DANGER))
                        .style(|_, _| ghost_button_style())
                        .on_press(on_eliminar_empleado(id)).padding([4, 6]),
                ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into());
            }
        }
        NominaTab::Roles => {
            for r in state.roles.iter() {
                let id = r.id;
                let pagado = r.estado == "pagado";
                rows.push(row![
                    text(&r.empleado_nombre).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(3)),
                    text(&r.periodo).size(11).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
                    text(format!("Bruto: {:.2}", r.total_ingresos)).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(2)),
                    text(format!("IESS: {:.2}", r.iess)).size(11).color(COLOR_GASTOS).width(Length::FillPortion(2)),
                    text(format!("Neto: {:.2}", r.total_neto)).size(12).color(COLOR_VENTAS).width(Length::FillPortion(2)),
                    text(if pagado { "Pagado" } else { "Pendiente" }).size(10)
                        .color(if pagado { COLOR_SUCCESS } else { COLOR_DANGER }).width(Length::FillPortion(1)),
                    if pagado {
                        button(text("\u{1F5A8}").size(11).color(COLOR_ACCENT)).style(|_, _| ghost_button_style()).padding([4, 6])
                    } else {
                        button(text("Marcar pagado").size(11).color(COLOR_TEXT_PRIMARY))
                            .style(|_, _| ghost_button_style())
                            .on_press(on_marcar_pagado(id)).padding([4, 6])
                    },
                    button(text("\u{2715}").size(12).color(COLOR_DANGER))
                        .style(|_, _| ghost_button_style())
                        .on_press(on_eliminar_rol(id)).padding([4, 6]),
                ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into());
            }
        }
    }

    column![
        header,
        row![tabs].padding([0.0, SPACING_LG]),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}
