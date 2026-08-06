use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use std::collections::HashMap;
use crate::theme::*;
use super::forms::{labeled_input, labeled_input_f64, form_two_columns};

#[derive(Debug, Clone)]
pub struct SetupFormData {
    pub empresa_nombre: String,
    pub ruc: String,
    pub direccion: String,
    pub telefono: String,
    pub email: String,
    pub ciudad: String,
    pub iva: String,
    pub usuario: String,
    pub contrasena: String,
    pub confirmar: String,
}

impl Default for SetupFormData {
    fn default() -> Self {
        Self {
            empresa_nombre: String::new(),
            ruc: String::new(),
            direccion: String::new(),
            telefono: String::new(),
            email: String::new(),
            ciudad: String::new(),
            iva: "15".into(),
            usuario: String::new(),
            contrasena: String::new(),
            confirmar: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetupState {
    pub form: SetupFormData,
    pub errores: HashMap<String, String>,
    pub mensaje: String,
    pub es_error: bool,
}

impl Default for SetupState {
    fn default() -> Self {
        Self { form: SetupFormData::default(), errores: HashMap::new(), mensaje: String::new(), es_error: false }
    }
}

#[derive(Debug, Clone)]
pub enum SetupMessage {
    EmpresaNombre(String), Ruc(String), Direccion(String), Telefono(String),
    Email(String), Ciudad(String), Iva(String),
    Usuario(String), Contrasena(String), Confirmar(String),
    Guardar,
}

pub fn setup_view<'a, Message: 'a + Clone>(
    state: &'a SetupState,
    on_msg: impl Fn(SetupMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let f1 = on_msg.clone();
    let f2 = on_msg.clone();
    let f3 = on_msg.clone();
    let f4 = on_msg.clone();
    let f5 = on_msg.clone();
    let f6 = on_msg.clone();
    let f7 = on_msg.clone();
    let f8 = on_msg.clone();
    let f9 = on_msg.clone();
    let f10 = on_msg.clone();

    let pass1 = text_input("Contraseña", &state.form.contrasena)
        .on_input(move |v| f9(SetupMessage::Contrasena(v)))
        .secure(true)
        .style(|_, _| input_style())
        .padding([8, 12]);
    let pass2 = text_input("Confirmar contraseña", &state.form.confirmar)
        .on_input(move |v| f10(SetupMessage::Confirmar(v)))
        .secure(true)
        .style(|_, _| input_style())
        .padding([8, 12]);

    let campos_empresa = vec![
        labeled_input("Nombre de la empresa *", &state.form.empresa_nombre, "MI EMPRESA S.A.", move |v| f1(SetupMessage::EmpresaNombre(v))),
        form_two_columns(
            labeled_input("RUC *", &state.form.ruc, "1700000000001", move |v| f2(SetupMessage::Ruc(v))),
            labeled_input("Teléfono *", &state.form.telefono, "02-0000000", move |v| f4(SetupMessage::Telefono(v))),
        ),
        form_two_columns(
            labeled_input("Ciudad", &state.form.ciudad, "Quito", move |v| f5(SetupMessage::Ciudad(v))),
            labeled_input_f64("IVA (%)", &state.form.iva, "15", move |v| f7(SetupMessage::Iva(v))),
        ),
        labeled_input("Dirección", &state.form.direccion, "Av. Principal, Edif. Central", move |v| f3(SetupMessage::Direccion(v))),
        labeled_input("Email", &state.form.email, "contacto@miempresa.com", move |v| f6(SetupMessage::Email(v))),
    ];
    let campos_usuario = vec![
        labeled_input("Usuario *", &state.form.usuario, "admin", move |v| f8(SetupMessage::Usuario(v))),
        column![
            text("Contraseña *").size(10).color(COLOR_TEXT_MUTED),
            Space::new().height(Length::Fixed(2.0)),
            pass1,
        ].spacing(0).into(),
        column![
            text("Confirmar contraseña *").size(10).color(COLOR_TEXT_MUTED),
            Space::new().height(Length::Fixed(2.0)),
            pass2,
        ].spacing(0).into(),
    ];

    let g = on_msg.clone();
    let contenido = column![
        column![
            container(text("SC").size(30).color(COLOR_ACCENT))
                .padding([SPACING_MD, SPACING_LG])
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(COLOR_ACCENT_GLOW)),
                    border: iced::Border { radius: RADIUS_LG.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                    text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
                }),
            Space::new().height(SPACING_SM),
            text("Bienvenido al Sistema de Contabilidad").size(24).color(COLOR_TEXT_PRIMARY),
            text("Configure los datos de su empresa y cree el usuario administrador.").size(13).color(COLOR_TEXT_SECONDARY),
        ].spacing(SPACING_XS).align_x(Alignment::Center),
        Space::new().height(SPACING_LG),
        column![
            row![
                text("Datos de la empresa").size(16).color(COLOR_TEXT_PRIMARY),
                Space::new().width(Length::Fill),
            ],
            column(campos_empresa).spacing(SPACING_MD),
        ].spacing(SPACING_SM),
        Space::new().height(SPACING_MD),
        column![
            row![
                text("Usuario administrador").size(16).color(COLOR_TEXT_PRIMARY),
                Space::new().width(Length::Fill),
            ],
            column(campos_usuario).spacing(SPACING_MD),
        ].spacing(SPACING_SM),
    ]
    .padding([SPACING_LG, SPACING_LG]);

    let mut cuerpo = column![
        container(contenido)
            .style(|_| card_style())
            .width(560)
            .max_width(620),
        Space::new().height(SPACING_MD),
        button(text("Guardar y continuar").size(14).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(g(SetupMessage::Guardar))
            .padding([SPACING_MD, SPACING_LG]),
    ]
    .align_x(Alignment::Center)
    .spacing(0)
    .width(Length::Fill)
    .padding([SPACING_XL, 0.0]);

    for msg in state.errores.values() {
        cuerpo = cuerpo.push(text(msg.as_str()).size(12).color(COLOR_DANGER));
    }
    if !state.mensaje.is_empty() {
        cuerpo = cuerpo.push(Space::new().height(SPACING_SM));
        cuerpo = cuerpo.push(
            text(&state.mensaje).size(13).color(if state.es_error { COLOR_DANGER } else { COLOR_SUCCESS }),
        );
    }

    container(
        scrollable(cuerpo)
            .style(|_, _| scrollable_style())
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .style(|_| page_style())
    .width(Length::Fill)
    .height(Length::Fill)
    .center(Length::Fill)
    .into()
}
