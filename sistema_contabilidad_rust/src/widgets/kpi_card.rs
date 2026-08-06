use iced::{Element, Length, Alignment};
use iced::widget::{column, container, row, text};
use crate::theme::*;

#[derive(Debug, Clone)]
pub struct KpiCard {
    pub titulo: String,
    pub valor: String,
    pub subtitulo: String,
    pub color: iced::Color,
    pub icono: char,
}

pub fn kpi_card_view<'a, Message: 'a>(kpi: KpiCard) -> Element<'a, Message> {
    let color = kpi.color;
    let glow = iced::Color { a: 0.08, ..color };
    let titulo = kpi.titulo.clone();
    let valor = kpi.valor.clone();
    let subtitulo = kpi.subtitulo.clone();

    container(
        column![
            row![
                text(kpi.icono.to_string()).size(20).color(color),
                text(titulo).size(11).color(COLOR_TEXT_SECONDARY),
            ].spacing(SPACING_SM).align_y(Alignment::Center),
            text(valor).size(26).color(COLOR_TEXT_PRIMARY),
            text(subtitulo).size(10).color(COLOR_TEXT_MUTED),
        ].spacing(SPACING_SM).width(Length::Fill)
    )
    .padding([SPACING_MD, SPACING_LG])
    .style(move |_| iced::widget::container::Style {
        background: Some(iced::Background::Color(COLOR_CARD)),
        border: iced::Border {
            radius: RADIUS_LG.into(),
            width: 1.0,
            color: iced::Color { a: 0.2, ..color },
        },
        text_color: Some(COLOR_TEXT_PRIMARY),
        snap: false,
        shadow: iced::Shadow { color: glow, offset: iced::Vector { x: 0.0, y: 4.0 }, blur_radius: 12.0 },
    })
    .width(Length::Fill)
    .into()
}
