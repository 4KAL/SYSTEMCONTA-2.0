use iced::{Element, Length, Alignment};
use iced::widget::{column, row, text, progress_bar};
use crate::theme::*;

pub fn bar_chart_view<'a, Message: 'a>(
    titulo: &'a str,
    datos: &[(String, f64)],
    color: iced::Color,
) -> Element<'a, Message> {
    let max = datos.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max).max(1.0);

    let bars: Vec<Element<'a, Message>> = datos.iter().map(|(label, valor)| {
        let ratio = (valor / max) as f32;
        row![
            text(label.clone())
                .size(11)
                .color(COLOR_TEXT_SECONDARY)
                .width(Length::FillPortion(3)),
            progress_bar(0.0..=1.0, ratio)
                .style(move |_| progress_bar_style(color)),
            text(format!("${:.0}", valor))
                .size(11)
                .color(COLOR_TEXT_PRIMARY)
                .width(Length::FillPortion(2)),
        ]
        .align_y(Alignment::Center)
        .spacing(SPACING_SM)
        .width(Length::Fill)
        .into()
    }).collect();

    let mut children: Vec<Element<'a, Message>> = vec![
        text(titulo)
            .size(14)
            .color(COLOR_TEXT_PRIMARY)
            .width(Length::Fill)
            .into(),
    ];
    children.extend(bars);

    column(children)
        .spacing(SPACING_SM)
        .padding(SPACING_MD)
        .into()
}
