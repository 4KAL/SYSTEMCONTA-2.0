use iced::widget::{button, column, container, row, text, Space};
use iced::{Element, Length};
use crate::models::Configuracion;
use crate::theme::*;
use super::forms::{labeled_input, labeled_input_f64, form_two_columns};

#[derive(Debug, Clone)]
pub struct ConfiguracionFormData {
    pub empresa_nombre: String,
    pub ruc: String,
    pub direccion: String,
    pub telefono: String,
    pub email: String,
    pub ciudad: String,
    pub iva: String,
}
impl Default for ConfiguracionFormData {
    fn default() -> Self {
        let c = Configuracion::default();
        Self {
            empresa_nombre: c.empresa_nombre,
            ruc: c.ruc,
            direccion: c.direccion,
            telefono: c.telefono,
            email: c.email,
            ciudad: c.ciudad,
            iva: c.iva.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfiguracionState {
    pub form: ConfiguracionFormData,
    pub guardado: bool,
    pub mensaje: String,
    pub es_error: bool,
}

impl Default for ConfiguracionState {
    fn default() -> Self {
        Self { form: ConfiguracionFormData::default(), guardado: false, mensaje: String::new(), es_error: false }
    }
}

#[derive(Debug, Clone)]
pub enum ConfiguracionMessage {
    EmpresaNombre(String), Ruc(String), Direccion(String), Telefono(String),
    Email(String), Ciudad(String), Iva(String),
    Guardar, Respaldo, AbrirDocumentos,
}

pub fn configuracion_view<'a, Message: 'a + Clone>(
    state: &'a ConfiguracionState,
    on_msg: impl Fn(ConfiguracionMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let f1 = on_msg.clone();
    let f2 = on_msg.clone();
    let f3 = on_msg.clone();
    let f4 = on_msg.clone();
    let f5 = on_msg.clone();
    let f6 = on_msg.clone();
    let f7 = on_msg.clone();
    let fields: Vec<Element<'a, Message>> = vec![
        form_two_columns(
            labeled_input("Nombre de la empresa", &state.form.empresa_nombre, "MI NEGOCIO CIA. LTDA.", move |v| f1(ConfiguracionMessage::EmpresaNombre(v))),
            labeled_input("RUC", &state.form.ruc, "1700000000001", move |v| f2(ConfiguracionMessage::Ruc(v))),
        ),
        form_two_columns(
            labeled_input("Ciudad", &state.form.ciudad, "Quito", move |v| f3(ConfiguracionMessage::Ciudad(v))),
            labeled_input("Teléfono", &state.form.telefono, "02-0000000", move |v| f4(ConfiguracionMessage::Telefono(v))),
        ),
        labeled_input("Dirección", &state.form.direccion, "Av. Principal, Edif. Central", move |v| f5(ConfiguracionMessage::Direccion(v))),
        labeled_input("Email", &state.form.email, "contacto@minegocio.com", move |v| f6(ConfiguracionMessage::Email(v))),
        labeled_input_f64("IVA (%)", &state.form.iva, "15", move |v| f7(ConfiguracionMessage::Iva(v))),
    ];

    let g = on_msg.clone();
    let r = on_msg.clone();
    let d = on_msg.clone();
    let acciones: Vec<Element<'a, Message>> = vec![
        row![
            button(text("Guardar configuración").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style())
                .on_press(g(ConfiguracionMessage::Guardar))
                .padding([SPACING_SM, SPACING_MD]),
            button(text("Respaldo de base de datos").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| secondary_button_style())
                .on_press(r(ConfiguracionMessage::Respaldo))
                .padding([SPACING_SM, SPACING_MD]),
            button(text("Abrir carpeta de documentos").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| secondary_button_style())
                .on_press(d(ConfiguracionMessage::AbrirDocumentos))
                .padding([SPACING_SM, SPACING_MD]),
        ].spacing(SPACING_SM).into(),
    ];
    let mut cuerpo = column![
        row![
            text("Configuración de la Empresa").size(24).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
        ],
        Space::new().height(SPACING_SM),
        container(
            column(fields.into_iter().chain(acciones)).spacing(SPACING_MD).padding([SPACING_LG, SPACING_LG])
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_CARD)),
            border: iced::Border { radius: RADIUS_XL.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: SHADOW_CARD,
        })
        .width(560),
    ];

    if !state.mensaje.is_empty() {
        cuerpo = cuerpo.push(Space::new().height(SPACING_SM));
        cuerpo = cuerpo.push(
            container(text(&state.mensaje).size(13).color(if state.es_error { COLOR_DANGER } else { COLOR_SUCCESS }))
                .padding([SPACING_SM, SPACING_MD])
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(COLOR_CARD)),
                    border: iced::Border { radius: RADIUS_MD.into(), width: 1.0, color: if state.es_error { COLOR_DANGER } else { COLOR_BORDER } },
                    text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
                })
                .width(560),
        );
    }

    cuerpo.padding(SPACING_LG).spacing(SPACING_SM).into()
}
