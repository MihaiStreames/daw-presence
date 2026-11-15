use iced::Border;
use iced::Length;
use iced::alignment::Vertical;
use iced::widget::button;
use iced::widget::row;
use iced::widget::text;
use iced::widget::text_input;

use super::accent_button;
use super::setting_row;
use crate::state::Message;
use crate::ui::style;

/// Props for a labeled input field with action button
pub(in crate::ui) struct InputProps<'a> {
    pub(in crate::ui) label: &'static str,
    pub(in crate::ui) description: &'a str,
    pub(in crate::ui) description_color: style::ColorRole,
    pub(in crate::ui) placeholder: &'static str,
    pub(in crate::ui) value: &'a str,
    pub(in crate::ui) button_label: &'static str,
    pub(in crate::ui) on_submit: Option<Message>,
}

pub(in crate::ui) fn labeled_input<'a>(
    props: InputProps<'a>,
    on_input: impl Fn(String) -> Message + 'a,
) -> iced::Element<'a, Message> {
    let input_row = row(vec![
        text_input(props.placeholder, props.value)
            .on_input(on_input)
            .width(Length::Fixed(style::INPUT_WIDTH))
            .style(input_style)
            .into(),
        button(text(props.button_label).size(style::TEXT_LABEL))
            .padding([style::PADDING_BUTTON_V, style::PADDING_BUTTON_H])
            .style(accent_button)
            .on_press_maybe(props.on_submit)
            .into(),
    ])
    .spacing(style::SPACING_TIGHT)
    .align_y(Vertical::Center);

    setting_row(props.label, props.description, props.description_color, input_row)
}

fn input_style(theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    let colors = style::colors(theme);
    let palette = theme.palette();

    let border_color = match status {
        text_input::Status::Active => colors.text_muted,
        text_input::Status::Hovered | text_input::Status::Focused { .. } => colors.accent,
        text_input::Status::Disabled => colors.control,
    };

    text_input::Style {
        background: iced::Background::Color(palette.background),
        border: Border {
            color: border_color,
            width: style::BORDER_WIDTH,
            radius: style::BORDER_RADIUS.into(),
        },
        icon: palette.text,
        placeholder: colors.text_muted,
        value: palette.text,
        selection: colors.accent_muted,
    }
}
