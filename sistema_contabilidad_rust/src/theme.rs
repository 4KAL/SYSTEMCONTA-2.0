use iced::color;

// ── Backgrounds ──────────────────────────────────────────
pub const COLOR_BG: iced::Color = color!(0x09, 0x09, 0x0e);
pub const COLOR_SURFACE: iced::Color = color!(0x11, 0x11, 0x18);
pub const COLOR_SIDEBAR: iced::Color = color!(0x0b, 0x0b, 0x12);
pub const COLOR_CARD: iced::Color = color!(0x18, 0x18, 0x22);
pub const COLOR_CARD_HOVER: iced::Color = color!(0x1e, 0x1e, 0x2a);
pub const COLOR_BORDER: iced::Color = color!(0x25, 0x25, 0x32);
pub const COLOR_BORDER_FOCUS: iced::Color = color!(0x4a, 0x4a, 0x60);

// ── Text ─────────────────────────────────────────────────
pub const COLOR_TEXT_PRIMARY: iced::Color = color!(0xee, 0xee, 0xf0);
pub const COLOR_TEXT_SECONDARY: iced::Color = color!(0x94, 0xa3, 0xb8);
pub const COLOR_TEXT_MUTED: iced::Color = color!(0x64, 0x74, 0x8b);

// ── Accents ──────────────────────────────────────────────
pub const COLOR_ACCENT: iced::Color = color!(0x63, 0x66, 0xf1);
pub const COLOR_ACCENT_HOVER: iced::Color = color!(0x81, 0x8c, 0xf8);
pub const COLOR_ACCENT_GLOW: iced::Color = color!(0x63, 0x66, 0xf1, 0.15);

// ── Semantic ─────────────────────────────────────────────
pub const COLOR_SUCCESS: iced::Color = color!(0x22, 0xc5, 0x5e);
pub const COLOR_WARNING: iced::Color = color!(0xf5, 0x9e, 0x0b);
pub const COLOR_DANGER: iced::Color = color!(0xef, 0x44, 0x44);
pub const COLOR_INFO: iced::Color = color!(0x3b, 0x82, 0xf6);

// ── Module colors ────────────────────────────────────────
pub const COLOR_VENTAS: iced::Color = color!(0x22, 0xc5, 0x5e);
pub const COLOR_GASTOS: iced::Color = color!(0xef, 0x44, 0x44);
pub const COLOR_CXC: iced::Color = color!(0x3b, 0x82, 0xf6);
pub const COLOR_CXP: iced::Color = color!(0xf5, 0x9e, 0x0b);
pub const COLOR_UTILIDAD: iced::Color = color!(0x8b, 0x5c, 0xf6);

// ── Spacing ──────────────────────────────────────────────
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 16.0;
pub const SPACING_LG: f32 = 24.0;
pub const SPACING_XL: f32 = 32.0;

// ── Radius ───────────────────────────────────────────────
pub const RADIUS_SM: f32 = 6.0;
pub const RADIUS_MD: f32 = 10.0;
pub const RADIUS_LG: f32 = 14.0;
pub const RADIUS_XL: f32 = 20.0;

// ── Shadows ──────────────────────────────────────────────
pub const SHADOW_CARD: iced::Shadow = iced::Shadow {
    color: iced::Color { a: 0.4, ..iced::Color::BLACK },
    offset: iced::Vector { x: 0.0, y: 4.0 },
    blur_radius: 16.0,
};
pub const SHADOW_SMALL: iced::Shadow = iced::Shadow {
    color: iced::Color { a: 0.2, ..iced::Color::BLACK },
    offset: iced::Vector { x: 0.0, y: 2.0 },
    blur_radius: 8.0,
};

// ── Container Styles ─────────────────────────────────────
pub fn container_base() -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(COLOR_SURFACE)),
        border: iced::Border { radius: RADIUS_MD.into(), width: 1.0, color: COLOR_BORDER },
        text_color: Some(COLOR_TEXT_PRIMARY),
        snap: false,
        shadow: SHADOW_SMALL,
    }
}

pub fn card_style() -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(COLOR_CARD)),
        border: iced::Border { radius: RADIUS_LG.into(), width: 1.0, color: COLOR_BORDER },
        text_color: Some(COLOR_TEXT_PRIMARY),
        snap: false,
        shadow: SHADOW_CARD,
    }
}

pub fn page_style() -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(COLOR_BG)),
        border: iced::Border::default(),
        text_color: Some(COLOR_TEXT_PRIMARY),
        snap: false,
        shadow: iced::Shadow::default(),
    }
}

// ── Button Styles ────────────────────────────────────────
pub fn sidebar_button_style(selected: bool) -> iced::widget::button::Style {
    if selected {
        iced::widget::button::Style {
            background: Some(iced::Background::Color(COLOR_ACCENT_GLOW)),
            text_color: COLOR_TEXT_PRIMARY,
            border: iced::Border {
                radius: RADIUS_MD.into(),
                width: 1.0,
                color: iced::Color { a: 0.3, ..COLOR_ACCENT },
            },
            shadow: iced::Shadow::default(),
            snap: false,
        }
    } else {
        iced::widget::button::Style {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            text_color: COLOR_TEXT_SECONDARY,
            border: iced::Border { radius: RADIUS_MD.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            shadow: iced::Shadow::default(),
            snap: false,
        }
    }
}

pub fn primary_button_style() -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: Some(iced::Background::Color(COLOR_ACCENT)),
        text_color: COLOR_TEXT_PRIMARY,
        border: iced::Border { radius: RADIUS_SM.into(), width: 0.0, color: iced::Color::TRANSPARENT },
        shadow: iced::Shadow::default(),
        snap: false,
    }
}

pub fn secondary_button_style() -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: Some(iced::Background::Color(COLOR_CARD)),
        text_color: COLOR_TEXT_SECONDARY,
        border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
        shadow: iced::Shadow::default(),
        snap: false,
    }
}

pub fn ghost_button_style() -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: None,
        text_color: COLOR_TEXT_MUTED,
        border: iced::Border { radius: RADIUS_SM.into(), width: 0.0, color: iced::Color::TRANSPARENT },
        shadow: iced::Shadow::default(),
        snap: false,
    }
}

pub fn danger_button_style() -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: Some(iced::Background::Color(COLOR_DANGER)),
        text_color: COLOR_TEXT_PRIMARY,
        border: iced::Border { radius: RADIUS_SM.into(), width: 0.0, color: iced::Color::TRANSPARENT },
        shadow: iced::Shadow::default(),
        snap: false,
    }
}

// ── Input Style ──────────────────────────────────────────
pub fn input_style() -> iced::widget::text_input::Style {
    iced::widget::text_input::Style {
        background: iced::Background::Color(COLOR_SURFACE),
        border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
        icon: COLOR_ACCENT,
        placeholder: COLOR_TEXT_MUTED,
        value: COLOR_TEXT_PRIMARY,
        selection: COLOR_ACCENT,
    }
}

// ── Scrollable Style ─────────────────────────────────────
pub fn scrollable_style() -> iced::widget::scrollable::Style {
    iced::widget::scrollable::Style {
        container: iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            text_color: None,
            snap: false,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        },
        vertical_rail: iced::widget::scrollable::Rail {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: iced::Background::Color(iced::Color { a: 0.3, ..COLOR_TEXT_MUTED }),
                border: iced::Border { radius: 4.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            },
        },
        horizontal_rail: iced::widget::scrollable::Rail {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: iced::Background::Color(iced::Color { a: 0.3, ..COLOR_TEXT_MUTED }),
                border: iced::Border { radius: 4.0.into(), width: 0.0, color: iced::Color::TRANSPARENT },
            },
        },
        gap: None,
        auto_scroll: iced::widget::scrollable::AutoScroll {
            background: iced::Background::Color(COLOR_ACCENT),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            icon: COLOR_TEXT_PRIMARY,
        },
    }
}

// ── PickList Style ───────────────────────────────────────
pub fn pick_list_style() -> iced::widget::pick_list::Style {
    iced::widget::pick_list::Style {
        text_color: COLOR_TEXT_PRIMARY,
        placeholder_color: COLOR_TEXT_MUTED,
        handle_color: COLOR_ACCENT,
        background: iced::Background::Color(COLOR_SURFACE),
        border: iced::Border { radius: RADIUS_SM.into(), width: 1.0, color: COLOR_BORDER },
    }
}

pub fn menu_style() -> iced::widget::overlay::menu::Style {
    iced::widget::overlay::menu::Style {
        text_color: COLOR_TEXT_PRIMARY,
        background: iced::Background::Color(COLOR_CARD),
        border: iced::Border { radius: RADIUS_MD.into(), width: 1.0, color: COLOR_BORDER },
        selected_text_color: COLOR_TEXT_PRIMARY,
        selected_background: iced::Background::Color(COLOR_ACCENT_GLOW),
        shadow: SHADOW_CARD,
    }
}

// ── Progress Bar Style ───────────────────────────────────
pub fn progress_bar_style(color: iced::Color) -> iced::widget::progress_bar::Style {
    iced::widget::progress_bar::Style {
        background: iced::Background::Color(iced::Color { a: 0.2, ..color }),
        bar: iced::Background::Color(color),
        border: iced::Border { radius: RADIUS_SM.into(), width: 0.0, color: iced::Color::TRANSPARENT },
    }
}
