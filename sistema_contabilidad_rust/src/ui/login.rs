use iced::widget::{button, column, container, scrollable, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::theme::*;
use super::forms::labeled_input;

#[derive(Debug, Clone)]
pub struct LoginState {
    pub usuario: String,
    pub contrasena: String,
    pub mensaje: String,
    pub es_error: bool,
}

impl Default for LoginState {
    fn default() -> Self {
        Self { usuario: String::new(), contrasena: String::new(), mensaje: String::new(), es_error: false }
    }
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    Usuario(String),
    Contrasena(String),
    Ingresar,
}

pub fn login_view<'a, Message: 'a + Clone>(
    state: &'a LoginState,
    empresa_nombre: String,
    on_msg: impl Fn(LoginMessage) -> Message + 'a + Clone,
) -> Element<'a, Message> {
    let f1 = on_msg.clone();
    let f2 = on_msg.clone();
    let g = on_msg.clone();

    let contrasena_input = text_input("Contraseña", &state.contrasena)
        .on_input(move |v| f2(LoginMessage::Contrasena(v)))
        .secure(true)
        .style(|_, _| input_style())
        .padding([10, 14]);

    let mut card = column![
        container(text("SC").size(34).color(COLOR_ACCENT))
            .padding([SPACING_MD, SPACING_XL])
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Background::Color(COLOR_ACCENT_GLOW)),
                border: iced::Border { radius: RADIUS_LG.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: iced::Shadow::default(),
            }),
        Space::new().height(SPACING_MD),
        text(empresa_nombre).size(20).color(COLOR_TEXT_PRIMARY),
        text("Sistema de Contabilidad").size(12).color(COLOR_TEXT_SECONDARY),        Space::new().height(SPACING_LG),
        labeled_input("Usuario", &state.usuario, "admin", move |v| f1(LoginMessage::Usuario(v))),
        Space::new().height(SPACING_SM),
        column![
            text("Contraseña").size(10).color(COLOR_TEXT_MUTED),
            Space::new().height(Length::Fixed(2.0)),
            contrasena_input,
        ].spacing(0),
        Space::new().height(SPACING_LG),
        button(text("Iniciar sesión").size(14).color(COLOR_TEXT_PRIMARY))
            .style(|_, _| primary_button_style())
            .on_press(g(LoginMessage::Ingresar))
            .width(Length::Fill)
            .padding([SPACING_SM, SPACING_MD]),
    ]
    .align_x(Alignment::Center)
    .spacing(0)
    .padding([SPACING_XL, SPACING_XL]);

    if !state.mensaje.is_empty() {
        card = card.push(Space::new().height(SPACING_MD));
        card = card.push(
            text(&state.mensaje).size(13).color(if state.es_error { COLOR_DANGER } else { COLOR_SUCCESS }),
        );
    }

    let cuerpo = column![
        container(card)
            .style(|_| card_style())
            .width(400)
            .max_width(440),
    ]
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .padding([SPACING_XL, 0.0]);

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
