use iced::padding;
use iced::widget::container;
use iced::widget::text;

use crate::state::Message;
use crate::ui::style;

pub(in crate::ui) fn section_heading(label: &'static str) -> iced::Element<'static, Message> {
    container(
        text(label)
            .size(style::TEXT_LABEL)
            .style(style::text_role(|colors| colors.accent)),
    )
    .padding(padding::top(style::SPACING).bottom(style::SECTION_BOTTOM_PADDING))
    .into()
}
