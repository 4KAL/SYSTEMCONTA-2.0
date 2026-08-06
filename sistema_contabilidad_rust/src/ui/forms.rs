use iced::widget::{button, column, container, pick_list, row, text, text_input, Space};
use iced::{Element, Length, Alignment};
use crate::theme::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    pub id: i64,
    pub label: String,
}

impl std::fmt::Display for SelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

pub fn form_card<'a, Message: 'a + Clone>(
    title: &'a str,
    fields: impl IntoIterator<Item = Element<'a, Message>>,
    on_save: Option<Message>,
    on_cancel: Message,
    save_label: &'a str,
) -> Element<'a, Message> {
    let cancel_close = on_cancel.clone();
    let mut col: iced::widget::Column<'a, Message> = column![
        row![
            text(title).size(20).color(COLOR_TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            button(text("\u{2715}").size(14).color(COLOR_TEXT_MUTED))
                .style(|_, _| ghost_button_style())
                .on_press(cancel_close)
                .padding([4, 8]),
        ]
        .align_y(Alignment::Center),
        Space::new().height(Length::Fixed(SPACING_SM)),
    ];

    for f in fields {
        col = col.push(f);
    }

    col = col.push(Space::new().height(Length::Fixed(SPACING_MD)));

    let mut btn_row = row![].spacing(SPACING_SM);

    if let Some(save) = on_save {
        btn_row = btn_row.push(
            button(text(save_label).size(13).color(COLOR_TEXT_PRIMARY))
                .style(|_, _| primary_button_style())
                .on_press(save)
                .padding([SPACING_SM, SPACING_MD]),
        );
    }

    btn_row = btn_row.push(
        button(text("Cancelar").size(13).color(COLOR_TEXT_SECONDARY))
            .style(|_, _| secondary_button_style())
            .on_press(on_cancel)
            .padding([SPACING_SM, SPACING_MD]),
    );

    col = col.push(row![Space::new().width(Length::Fill), btn_row].align_y(Alignment::Center));

    container(
        container(col.spacing(SPACING_MD).padding([SPACING_LG, SPACING_LG]))
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Background::Color(COLOR_CARD)),
                border: iced::Border { radius: RADIUS_XL.into(), width: 1.0, color: COLOR_BORDER },
                text_color: Some(COLOR_TEXT_PRIMARY),
                snap: false,
                shadow: SHADOW_CARD,
            })
            .width(540)
            .max_width(600),
    )
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::Color { a: 0.7, ..COLOR_BG })),
        border: iced::Border::default(),
        text_color: Some(COLOR_TEXT_PRIMARY),
        snap: false,
        shadow: iced::Shadow::default(),
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .center(Length::Fill)
    .into()
}

pub fn labeled_input<'a, Message: 'a + Clone>(
    label: &'a str,
    value: &str,
    placeholder: &str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    column![
        text(label).size(10).color(COLOR_TEXT_MUTED),
        Space::new().height(Length::Fixed(2.0)),
        text_input(placeholder, value)
            .on_input(on_input)
            .style(|_, _| input_style())
            .padding([8, 12])
            .width(Length::Fill),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

pub fn labeled_input_f64<'a, Message: 'a + Clone>(
    label: &'a str,
    value: &str,
    placeholder: &str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    column![
        text(label).size(10).color(COLOR_TEXT_MUTED),
        Space::new().height(Length::Fixed(2.0)),
        text_input(placeholder, value)
            .on_input(on_input)
            .style(|_, _| input_style())
            .padding([8, 12])
            .width(Length::Fill),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

pub fn labeled_input_i32<'a, Message: 'a + Clone>(
    label: &'a str,
    value: &str,
    placeholder: &str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    column![
        text(label).size(10).color(COLOR_TEXT_MUTED),
        Space::new().height(Length::Fixed(2.0)),
        text_input(placeholder, value)
            .on_input(on_input)
            .style(|_, _| input_style())
            .padding([8, 12])
            .width(Length::Fill),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

pub fn pick_list_field<'a, Message: 'a + Clone>(
    label: &'a str,
    options: &'a [SelectOption],
    selected_id: i64,
    on_selected: impl Fn(i64) -> Message + 'a,
) -> Element<'a, Message> {
    let selected = options.iter().find(|o| o.id == selected_id);
    column![
        text(label).size(10).color(COLOR_TEXT_MUTED),
        Space::new().height(Length::Fixed(2.0)),
        pick_list(options, selected, move |opt: SelectOption| on_selected(opt.id))
            .style(|_, _| pick_list_style())
            .menu_style(|_| menu_style())
            .padding([8, 12])
            .width(Length::Fill),
    ]
    .spacing(0)
    .width(Length::Fill)
    .into()
}

pub fn form_two_columns<'a, Message: 'a + Clone>(
    left: Element<'a, Message>,
    right: Element<'a, Message>,
) -> Element<'a, Message> {
    row![left, right]
        .spacing(SPACING_MD)
        .width(Length::Fill)
        .into()
}

pub fn texto_error<'a, Message: 'a>(
    key: &str,
    errores: &'a HashMap<String, String>,
) -> Option<Element<'a, Message>> {
    errores.get(key).map(|msg| {
        text(msg.as_str()).size(11).color(COLOR_DANGER).into()
    })
}

pub fn confirm_dialog<'a, Message: 'a + Clone>(
    mensaje: &'a str,
    on_confirm: Message,
    on_cancel: Message,
) -> Element<'a, Message> {
    container(
        container(
            column![
                text(mensaje).size(14).color(COLOR_TEXT_PRIMARY),
                Space::new().height(SPACING_MD),
                row![
                    button(text("Cancelar").size(13)).style(|_, _| secondary_button_style()).on_press(on_cancel),
                    button(text("Eliminar").size(13)).style(|_, _| danger_button_style()).on_press(on_confirm),
                ].spacing(SPACING_SM).align_y(Alignment::Center),
            ].spacing(SPACING_SM).padding([SPACING_LG, SPACING_LG])
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(COLOR_CARD)),
            border: iced::Border { radius: RADIUS_XL.into(), width: 1.0, color: COLOR_BORDER },
            text_color: Some(COLOR_TEXT_PRIMARY),
            snap: false,
            shadow: SHADOW_CARD,
        })
        .width(360),
    )
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::Color { a: 0.7, ..COLOR_BG })),
        border: iced::Border::default(),
        text_color: Some(COLOR_TEXT_PRIMARY),
        snap: false,
        shadow: iced::Shadow::default(),
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .center(Length::Fill)
    .into()
}

