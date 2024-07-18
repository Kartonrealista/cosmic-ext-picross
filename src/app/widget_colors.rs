use crate::app::{theme, widget, Theme};
use cosmic::{
    iced::{Border, Color},
    iced_core::Shadow,
};

const GREY1RGB: Color = Color {
    r: 238.0 / 255.0,
    g: 228.0 / 255.0,
    b: 218.0 / 255.0,
    a: 1.0,
};

const GREY2RGB: Color = Color {
    r: 55.0 / 255.0,
    g: 57.0 / 255.0,
    b: 58.0 / 255.0,
    a: 1.0,
};

const ORANGE1RGB: Color = Color {
    r: 242.0 / 255.0,
    g: 177.0 / 255.0,
    b: 121.0 / 255.0,
    a: 1.0,
};

pub fn blacktheme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = orange1theme(theme);
    appearance.icon_color = Some(Color::BLACK);
    appearance.background = Some(cosmic::iced::Background::Color(Color::BLACK));
    appearance
}

pub fn whitetheme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = orange1theme(theme);
    appearance.icon_color = Some(Color::WHITE);
    appearance.text_color = Some(Color::BLACK);
    appearance.background = Some(cosmic::iced::Background::Color(Color::WHITE));
    appearance
}

pub fn orange1theme(theme: &Theme) -> widget::container::Appearance {
    let cosmic = theme.cosmic();
    widget::container::Appearance {
        icon_color: Some(ORANGE1RGB),
        text_color: Some(Color::WHITE),
        background: Some(cosmic::iced::Background::Color(ORANGE1RGB)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: cosmic.corner_radii.radius_xs.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: cosmic::iced::Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
    }
}

pub fn gray1theme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = orange1theme(theme);
    appearance.background = Some(cosmic::iced::Background::Color(GREY1RGB));
    appearance.text_color = Some(Color {
        r: 119.0 / 255.0,
        g: 110.0 / 255.0,
        b: 101.0 / 255.0,
        a: 1.0,
    });
    appearance
}

pub fn gray2theme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = orange1theme(theme);
    appearance.background = Some(cosmic::iced::Background::Color(GREY2RGB));
    appearance
}
