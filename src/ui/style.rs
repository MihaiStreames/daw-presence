use iced::Color;

// spacing
pub(super) const SPACING: f32 = 12.0;
pub(super) const SPACING_TIGHT: f32 = 6.0;

// padding
pub(super) const PADDING_PAGE: f32 = 20.0;
pub(super) const PADDING_CARD: f32 = 12.0;
pub(super) const PADDING_BUTTON_V: f32 = 7.0;
pub(super) const PADDING_BUTTON_H: f32 = 12.0;

// sidebar
pub(super) const SIDEBAR_WIDTH: f32 = 80.0;
pub(super) const SIDEBAR_ITEM_PADDING: f32 = 12.0;
pub(super) const SIDEBAR_TEXT_NUDGE: f32 = 1.0;
pub(super) const SIDEBAR_ITEM_SPACING: f32 = 2.0;
pub(super) const SIDEBAR_ACCENT_WIDTH: f32 = 3.0;

// settings
pub(super) const SETTING_ROW_PADDING: f32 = 10.0;
pub(super) const SECTION_BOTTOM_PADDING: f32 = 4.0;
pub(super) const INPUT_WIDTH: f32 = 100.0;
pub(super) const LABEL_SPACING: f32 = 2.0;

// text sizes
pub(super) const TEXT_HEADING: f32 = 24.0;
pub(super) const TEXT_LABEL: f32 = 14.0;
pub(super) const TEXT_DESCRIPTION: f32 = 12.0;

// border
pub(super) const BORDER_WIDTH: f32 = 1.0;
pub(super) const BORDER_RADIUS: f32 = 4.0;
pub(super) const BORDER_RADIUS_CARD: f32 = 6.0;

// title bar
pub(super) const TITLE_BAR_HEIGHT: f32 = 32.0;
pub(super) const TITLE_BUTTON_WIDTH: f32 = 46.0;
pub(super) const TITLE_GLYPH_SIZE: f32 = 10.0;

// icon
pub(super) const STATUS_ICON_SIZE: f32 = 12.0;
pub(super) const STATUS_ICON_TOP_OFFSET: f32 = 4.0;

/// Color roles for one theme mode
pub(super) struct Colors {
    pub(super) background_sunken: Color,
    pub(super) background_raised: Color,
    pub(super) background_selected: Color,
    pub(super) control: Color,
    pub(super) control_hover: Color,
    pub(super) text_muted: Color,
    pub(super) accent: Color,
    pub(super) accent_hover: Color,
    pub(super) accent_muted: Color,
    pub(super) danger: Color,
}

const DARK: Colors = Colors {
    background_sunken: Color::from_rgb(37.0 / 255.0, 38.0 / 255.0, 42.0 / 255.0),
    background_raised: Color::from_rgb(49.0 / 255.0, 52.0 / 255.0, 56.0 / 255.0),
    background_selected: Color::from_rgb(56.0 / 255.0, 58.0 / 255.0, 64.0 / 255.0),
    control: Color::from_rgb(50.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0),
    control_hover: Color::from_rgb(65.0 / 255.0, 65.0 / 255.0, 65.0 / 255.0),
    text_muted: Color::from_rgb(140.0 / 255.0, 140.0 / 255.0, 140.0 / 255.0),
    accent: Color::from_rgb(88.0 / 255.0, 166.0 / 255.0, 1.0),
    accent_hover: Color::from_rgb(108.0 / 255.0, 180.0 / 255.0, 1.0),
    accent_muted: Color::from_rgb(60.0 / 255.0, 120.0 / 255.0, 200.0 / 255.0),
    danger: Color::from_rgb(200.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0),
};

const LIGHT: Colors = Colors {
    background_sunken: Color::from_rgb(240.0 / 255.0, 241.0 / 255.0, 243.0 / 255.0),
    background_raised: Color::from_rgb(246.0 / 255.0, 247.0 / 255.0, 248.0 / 255.0),
    background_selected: Color::from_rgb(227.0 / 255.0, 229.0 / 255.0, 232.0 / 255.0),
    control: Color::from_rgb(233.0 / 255.0, 233.0 / 255.0, 233.0 / 255.0),
    control_hover: Color::from_rgb(221.0 / 255.0, 221.0 / 255.0, 221.0 / 255.0),
    text_muted: Color::from_rgb(97.0 / 255.0, 97.0 / 255.0, 97.0 / 255.0),
    accent: Color::from_rgb(9.0 / 255.0, 105.0 / 255.0, 218.0 / 255.0),
    accent_hover: Color::from_rgb(5.0 / 255.0, 80.0 / 255.0, 174.0 / 255.0),
    accent_muted: Color::from_rgb(128.0 / 255.0, 184.0 / 255.0, 240.0 / 255.0),
    danger: Color::from_rgb(196.0 / 255.0, 43.0 / 255.0, 28.0 / 255.0),
};

/// Picks one color out of a [`Colors`] set, for callers that don't have the theme yet
pub(super) type ColorRole = fn(&Colors) -> Color;

/// Build a text style that resolves `role` against the active theme
pub(super) fn text_role(role: ColorRole) -> impl Fn(&iced::Theme) -> iced::widget::text::Style {
    move |theme| iced::widget::text::Style {
        color: Some(role(colors(theme))),
    }
}

/// Pick the color set matching `theme`, which iced keeps in sync with the windows light/dark setting
pub(super) fn colors(theme: &iced::Theme) -> &'static Colors {
    if theme.extended_palette().is_dark {
        &DARK
    } else {
        &LIGHT
    }
}
