mod components;
mod pages;
mod strings;
mod style;
pub mod tray;

use iced::Length;
use iced::widget::container;
use iced::widget::mouse_area;
use iced::widget::row;
use iced::widget::stack;

use crate::state::AppState;
use crate::state::Message;
use crate::state::Page;

/// Render the main app layout
pub fn view(state: &AppState) -> iced::Element<'_, Message> {
    let content = match state.active_page {
        Page::Home => pages::home_view(state),
        Page::Settings => pages::settings_view(state),
    };

    let page = container(content)
        .padding(iced::Padding {
            top: style::TITLE_BAR_HEIGHT,
            ..iced::Padding::new(style::PADDING_PAGE)
        })
        .width(Length::Fill)
        .height(Length::Fill);

    let sidebar = mouse_area(components::sidebar(state)).on_press(Message::DragWindow);
    let page = stack(vec![page.into(), components::title_bar()]);

    row(vec![sidebar.into(), page.into()])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
