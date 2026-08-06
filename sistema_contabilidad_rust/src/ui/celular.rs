use iced::widget::canvas::{self, Canvas, Frame, Geometry, Program};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Color, Element, Length, Point, Rectangle, Size};
use crate::theme::*;

#[derive(Debug, Clone, Default)]
pub struct CelularState {
    pub activo: bool,
    pub mensaje: String,
    pub url: Option<String>,
    pub qr: Option<Vec<bool>>,
    pub qr_size: usize,
}

/// Extrae la URL pública HTTPS del log de cloudflared.
pub fn extraer_url_cloudflared(log: &str) -> Option<String> {
    let idx = log.find("https://")?;
    let rest = &log[idx..];
    let end = rest
        .find(|c: char| c.is_whitespace() || c == '|' || c == '"')
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

struct QrProgram {
    data: Vec<bool>,
    size: usize,
}

impl<Message> Program<Message> for QrProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry<iced::Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());
        if self.size > 0 {
            let cell = (bounds.width.min(bounds.height)) / self.size as f32;
            let offset_x = (bounds.width - cell * self.size as f32) / 2.0;
            let offset_y = (bounds.height - cell * self.size as f32) / 2.0;
            for i in 0..self.size {
                for j in 0..self.size {
                    if self.data[i * self.size + j] {
                        frame.fill_rectangle(
                            Point::new(offset_x + j as f32 * cell, offset_y + i as f32 * cell),
                            Size::new(cell, cell),
                            Color::BLACK,
                        );
                    }
                }
            }
        }
        vec![frame.into_geometry()]
    }
}

pub fn qr_widget<Message: 'static>(data: &[bool], size: usize) -> Element<'static, Message> {
    let program = QrProgram { data: data.to_vec(), size };
    Canvas::new(program)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn celular_dialog<Message: 'static + Clone>(
    state: &CelularState,
    on_cerrar: Message,
) -> Element<'static, Message> {
    let inner: Element<'static, Message> = if state.activo && state.url.is_none() {
        column![
            text("Conectando celular...").size(16).color(COLOR_TEXT_PRIMARY),
            Space::new().height(SPACING_SM),
            text(
                if state.mensaje.is_empty() {
                    "Iniciando el servidor y el túnel de internet. Esto puede tomar unos segundos.".to_string()
                } else {
                    state.mensaje.clone()
                }
            )
            .size(13)
            .color(COLOR_TEXT_MUTED),
            Space::new().height(SPACING_MD),
            button(text("Cancelar").size(13))
                .style(|_, _| secondary_button_style())
                .on_press(on_cerrar),
        ]
        .spacing(SPACING_SM)
        .padding([SPACING_LG, SPACING_LG])
        .into()
    } else if let (Some(url), Some(qr)) = (&state.url, &state.qr) {
        column![
            text("Conectar celular").size(17).color(COLOR_TEXT_PRIMARY),
            Space::new().height(SPACING_SM),
            container(qr_widget(qr, state.qr_size))
                .width(260)
                .height(260)
                .padding(12)
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(Color::WHITE)),
                    border: iced::Border { radius: RADIUS_MD.into(), width: 1.0, color: COLOR_BORDER },
                    text_color: Some(Color::BLACK),
                    snap: false,
                    shadow: iced::Shadow::default(),
                }),
            Space::new().height(SPACING_SM),
            text("En el celular: abra la app Contabilidad y pulse \"Escanear QR\".".to_string())
                .size(12)
                .color(COLOR_TEXT_MUTED),
            Space::new().height(SPACING_SM),
            text(url.clone()).size(11).color(COLOR_ACCENT),
            Space::new().height(SPACING_MD),
            button(text("Cerrar").size(13))
                .style(|_, _| secondary_button_style())
                .on_press(on_cerrar),
        ]
        .spacing(SPACING_XS)
        .padding([SPACING_LG, SPACING_LG])
        .into()
    } else {
        column![
            text("No se pudo conectar").size(16).color(COLOR_DANGER),
            text(state.mensaje.clone()).size(13).color(COLOR_TEXT_MUTED),
            Space::new().height(SPACING_MD),
            button(text("Cerrar").size(13))
                .style(|_, _| secondary_button_style())
                .on_press(on_cerrar),
        ]
        .spacing(SPACING_SM)
        .padding([SPACING_LG, SPACING_LG])
        .into()
    };

    container(
        container(inner)
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Background::Color(COLOR_CARD)),
                border: iced::Border { radius: RADIUS_XL.into(), width: 1.0, color: COLOR_BORDER },
                text_color: Some(COLOR_TEXT_PRIMARY),
                snap: false,
                shadow: SHADOW_CARD,
            })
            .width(500),
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
