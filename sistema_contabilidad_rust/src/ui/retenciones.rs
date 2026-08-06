use iced::widget::{button, column, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::models::Retencion;
use crate::theme::*;
use super::forms::{form_card, labeled_input, labeled_input_f64, form_two_columns};

#[derive(Debug, Clone)]
pub struct RetencionFormData {
    pub proveedor_id: String,
    pub proveedor_nombre: String,
    pub cedula: String,
    pub fecha: String,
    pub base_imp_renta: String,
    pub porcentaje_renta: String,
    pub base_imp_iva: String,
    pub porcentaje_iva: String,
    pub tipo_comprobante: String,
    pub numero_comprobante: String,
    pub referencia: String,
}

impl Default for RetencionFormData {
    fn default() -> Self {
        Self {
            proveedor_id: String::new(), proveedor_nombre: String::new(), cedula: String::new(),
            fecha: chrono::Local::now().format("%Y-%m-%d").to_string(),
            base_imp_renta: String::new(), porcentaje_renta: String::new(),
            base_imp_iva: String::new(), porcentaje_iva: String::new(),
            tipo_comprobante: "factura".to_string(), numero_comprobante: String::new(), referencia: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RetencionFormMessage {
    ProveedorNombre(String), Cedula(String), Fecha(String),
    BaseImpRenta(String), PorcentajeRenta(String),
    BaseImpIva(String), PorcentajeIva(String),
    TipoComprobante(String), NumeroComprobante(String), Referencia(String),
    Guardar, Cancelar,
}

#[derive(Debug, Clone)]
pub struct RetencionesState {
    pub retenciones: Vec<Retencion>,
    pub show_form: bool,
    pub editing_id: Option<i64>,
    pub numero: String,
    pub busqueda: String,
    pub form: RetencionFormData,
}

impl Default for RetencionesState {
    fn default() -> Self {
        Self {
            retenciones: Vec::new(), show_form: false, editing_id: None,
            numero: String::new(), busqueda: String::new(), form: RetencionFormData::default(),
        }
    }
}

pub fn retenciones_view<'a, Message: 'a + Clone>(
    state: &'a RetencionesState,
    on_nuevo: Message,
    on_form_msg: impl Fn(RetencionFormMessage) -> Message + 'a + Clone,
    on_eliminar: impl Fn(i64) -> Message + 'a + Clone,
    on_buscar: impl Fn(String) -> Message + 'a + Clone,
    on_imprimir: impl Fn(i64) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    if state.show_form {
        let title = if state.editing_id.is_some() { "Editar Retención" } else { "Nueva Retención" };
        let guardar = on_form_msg(RetencionFormMessage::Guardar);
        let cancelar = on_form_msg(RetencionFormMessage::Cancelar);
        let f_pn = on_form_msg.clone();
        let f_ced = on_form_msg.clone();
        let f_fec = on_form_msg.clone();
        let f_bir = on_form_msg.clone();
        let f_pr = on_form_msg.clone();
        let f_bii = on_form_msg.clone();
        let f_pi = on_form_msg.clone();
        let f_tc = on_form_msg.clone();
        let f_nc = on_form_msg.clone();
        let f_ref = on_form_msg.clone();
        return form_card(
            title,
            vec![
                labeled_input("Proveedor", &state.form.proveedor_nombre, "Nombre del proveedor", move |v| f_pn(RetencionFormMessage::ProveedorNombre(v))),
                form_two_columns(
                    labeled_input("RUC / Cédula", &state.form.cedula, "Cédula o RUC", move |v| f_ced(RetencionFormMessage::Cedula(v))),
                    labeled_input("Fecha", &state.form.fecha, "YYYY-MM-DD", move |v| f_fec(RetencionFormMessage::Fecha(v))),
                ),
                form_two_columns(
                    labeled_input_f64("Base Imponible Renta", &state.form.base_imp_renta, "0.00", move |v| f_bir(RetencionFormMessage::BaseImpRenta(v))),
                    labeled_input_f64("% Renta", &state.form.porcentaje_renta, "1.00", move |v| f_pr(RetencionFormMessage::PorcentajeRenta(v))),
                ),
                form_two_columns(
                    labeled_input_f64("Base Imponible IVA", &state.form.base_imp_iva, "0.00", move |v| f_bii(RetencionFormMessage::BaseImpIva(v))),
                    labeled_input_f64("% IVA", &state.form.porcentaje_iva, "30.00", move |v| f_pi(RetencionFormMessage::PorcentajeIva(v))),
                ),
                form_two_columns(
                    labeled_input("Comprobante", &state.form.tipo_comprobante, "factura", move |v| f_tc(RetencionFormMessage::TipoComprobante(v))),
                    labeled_input("No. Comprobante", &state.form.numero_comprobante, "001-002-000000123", move |v| f_nc(RetencionFormMessage::NumeroComprobante(v))),
                ),
                labeled_input("Referencia", &state.form.referencia, "Concepto de la retención", move |v| f_ref(RetencionFormMessage::Referencia(v))),
            ],
            Some(guardar), cancelar, "Guardar",
        );
    }

    let filtrados: Vec<&Retencion> = if state.busqueda.is_empty() {
        state.retenciones.iter().collect()
    } else {
        let q = state.busqueda.to_lowercase();
        state.retenciones.iter().filter(|r|
            r.proveedor_nombre.to_lowercase().contains(&q) ||
            r.numero.to_lowercase().contains(&q) ||
            r.cedula.to_lowercase().contains(&q)
        ).collect()
    };

    let rows: Vec<Element<'a, Message>> = filtrados.iter().map(|r| {
        let id = r.id;
        row![
            text(&r.numero).size(11).color(COLOR_TEXT_PRIMARY).width(Length::FillPortion(1)),
            text(&r.proveedor_nombre).size(11).color(COLOR_TEXT_SECONDARY).width(Length::FillPortion(3)),
            text(&r.fecha).size(10).color(COLOR_TEXT_MUTED).width(Length::FillPortion(1)),
            text(format!("Renta: {:.2}", r.valor_renta)).size(11).color(COLOR_CXC).width(Length::FillPortion(2)),
            text(format!("IVA: {:.2}", r.valor_iva)).size(11).color(COLOR_GASTOS).width(Length::FillPortion(2)),
            button(text("\u{1F5A8}").size(11).color(COLOR_ACCENT))
                .style(|_, _| ghost_button_style())
                .on_press(on_imprimir(id)).padding([4, 6]),
            button(text("\u{2715}").size(12).color(COLOR_DANGER))
                .style(|_, _| ghost_button_style())
                .on_press(on_eliminar(id)).padding([4, 6]),
        ].spacing(SPACING_SM).align_y(Alignment::Center).padding([SPACING_SM, SPACING_MD]).into()
    }).collect();

    column![
        row![
            text("Retenciones").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            text_input("Buscar...", &state.busqueda)
                .on_input(on_buscar)
                .style(|_, _| input_style())
                .width(220),
            button(text("+ Nueva Retención").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style())
                .on_press(on_nuevo)
                .padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_MD).align_y(Alignment::Center).padding(SPACING_LG),
        scrollable(column(rows).spacing(2.0).padding([0.0, SPACING_LG]).width(Length::Fill)).style(|_, _| scrollable_style()),
    ].into()
}
