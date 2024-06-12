use iced::{
    border::Radius,
    widget::{
        container::Appearance,
        rule::{Appearance as RuleAppearance, FillMode},
    },
    Background, Border, Color, Length, Shadow, Theme,
};

pub fn style_main_area(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();
    Appearance {
        background: Some(Background::Color(palette.background.weak.color)),
        text_color: Some(palette.background.weak.text),
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: Radius::default(),
        },
        shadow: Shadow::default(),
    }
}

pub fn style_sidebar(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();
    Appearance {
        background: Some(Background::Color(palette.background.strong.color)),
        text_color: Some(palette.background.strong.text),
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: Radius::default(),
        },
        shadow: Shadow::default(),
    }
}

pub fn style_sidebar_item(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();
    Appearance {
        background: Some(Background::Color(palette.background.strong.color)),
        text_color: Some(palette.primary.base.text),
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: Radius::default(),
        },
        shadow: Shadow::default(),
    }
}

pub fn style_sidebar_rule(theme: &Theme) -> RuleAppearance {
    let palette = theme.extended_palette();
    RuleAppearance {
        color: palette.background.base.color,
        width: 3,
        radius: Radius::default(),
        fill_mode: FillMode::Full,
    }
}

pub fn style_player_area(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();
    Appearance {
        background: Some(iced::Background::Color(palette.background.base.color)),
        text_color: None,
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: Radius::default(),
        },
        shadow: Shadow::default(),
    }
}

pub fn style_list_item(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();
    Appearance {
        background: Some(iced::Background::Color(palette.background.strong.color)),
        text_color: None,
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: [5.0, 5.0, 5.0, 5.0].into(),
        },
        shadow: Shadow::default(),
    }
}
