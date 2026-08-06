use iced::widget::{button, column, container, progress_bar, text, Space};
use iced::{Element, Length, Alignment};
use crate::theme::*;

pub fn presentacion_view<Message: 'static + Clone>(
    on_continuar: Message,
) -> Element<'static, Message> {
    let continuar = on_continuar.clone();
    container(
        column![
            container(text("SC").size(44).color(COLOR_ACCENT))
                .padding([SPACING_LG, SPACING_XL])
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(COLOR_ACCENT_GLOW)),
                    border: iced::Border { radius: RADIUS_LG.into(), width: 0.0, color: iced::Color::TRANSPARENT },
                    text_color: Some(COLOR_TEXT_PRIMARY), snap: false, shadow: SHADOW_CARD,
                }),
            Space::new().height(SPACING_MD),
            text("Sistema de Contabilidad").size(28).color(COLOR_TEXT_PRIMARY),
            text("Bienvenido, vamos a preparar su sistema").size(14).color(COLOR_TEXT_SECONDARY),
            Space::new().height(SPACING_LG),
            container(
                progress_bar(0.0..=1.0, 0.55)
                    .style(|_| progress_bar_style(COLOR_ACCENT))
            )
            .width(240),
            Space::new().height(SPACING_XS),
            text("Primera instalación detectada, un momento...").size(11).color(COLOR_TEXT_MUTED),
            Space::new().height(SPACING_LG),
            button(text("Comenzar").size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style())
                .on_press(continuar)
                .padding([SPACING_SM, SPACING_LG]),
        ]
        .align_x(Alignment::Center)
        .spacing(0)
        .width(Length::Fill),
    )
    .style(|_| page_style())
    .width(Length::Fill)
    .height(Length::Fill)
    .center(Length::Fill)
    .into()
}
