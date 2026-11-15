use iced::padding;
use iced::widget::column;
use iced::widget::container;
use iced::widget::scrollable;
use iced::widget::text;

use crate::state::AppState;
use crate::state::IntervalDraft;
use crate::state::Message;
use crate::ui::components;
use crate::ui::strings;
use crate::ui::style;

/// Render the settings page
pub(in crate::ui) fn settings_view(state: &AppState) -> iced::Element<'_, Message> {
    let content = column(vec![
        text(strings::SETTINGS_TITLE).size(style::TEXT_HEADING).into(),
        components::section_heading(strings::SECTION_PRIVACY),
        components::setting_toggle(
            strings::HIDE_PROJECT_NAME,
            strings::HIDE_PROJECT_NAME_DESC,
            state.settings.hide_project_name,
            Message::ToggleHideProjectName(!state.settings.hide_project_name),
        ),
        components::setting_toggle(
            strings::HIDE_SYSTEM_USAGE,
            strings::HIDE_SYSTEM_USAGE_DESC,
            state.settings.hide_system_usage,
            Message::ToggleHideSystemUsage(!state.settings.hide_system_usage),
        ),
        components::section_heading(strings::SECTION_BEHAVIOR),
        components::setting_toggle(
            strings::MINIMIZE_TO_TRAY,
            strings::MINIMIZE_TO_TRAY_DESC,
            state.settings.close_to_tray,
            Message::ToggleCloseToTray(!state.settings.close_to_tray),
        ),
        components::setting_toggle(
            strings::AUTO_START,
            strings::AUTO_START_DESC,
            state.auto_start_enabled,
            Message::ToggleAutoStart(!state.auto_start_enabled),
        ),
        components::section_heading(strings::SECTION_TIMING),
        interval_editor(state),
    ])
    .spacing(style::SPACING_TIGHT);

    scrollable(container(content).padding(padding::right(style::SPACING)))
        .style(components::scrollable_style)
        .into()
}

fn interval_editor(state: &AppState) -> iced::Element<'_, Message> {
    let (description, color): (&str, style::ColorRole) = match &state.interval_draft {
        IntervalDraft::Invalid(error) => (error.as_str(), |colors| colors.danger),
        IntervalDraft::Applied => (strings::APPLIED, |colors| colors.accent),
        IntervalDraft::Unchanged | IntervalDraft::Valid(_) => {
            (strings::UPDATE_INTERVAL_DESC, |colors| colors.text_muted)
        }
    };

    let can_apply = matches!(state.interval_draft, IntervalDraft::Valid(_));

    components::labeled_input(
        components::InputProps {
            label: strings::UPDATE_INTERVAL,
            description,
            description_color: color,
            placeholder: strings::UPDATE_INTERVAL_PLACEHOLDER,
            value: &state.interval_input,
            button_label: strings::APPLY,
            on_submit: can_apply.then_some(Message::ApplyInterval),
        },
        Message::UpdateIntervalInput,
    )
}
