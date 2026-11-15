use super::*;

const FL_STUDIO_REGEX: &str = "^(.*?)(?= - FL Studio)";

fn pattern(regex: &str) -> TitlePattern {
    TitlePattern::new(regex.to_owned())
}

#[test]
fn matches_invalid_regex_is_false() {
    assert!(!pattern("[invalid").matches("title"));
}

#[test]
fn project_strips_unsaved_marker() {
    assert_eq!(
        pattern(FL_STUDIO_REGEX).project("My Song* - FL Studio"),
        Project::Named("My Song".to_owned())
    );
}

#[test]
fn project_unsaved_marker_only_is_untitled() {
    assert_eq!(pattern(FL_STUDIO_REGEX).project("* - FL Studio"), Project::Untitled);
}

#[test]
fn project_named_untitled_is_untitled() {
    assert_eq!(
        pattern(FL_STUDIO_REGEX).project("Untitled - FL Studio"),
        Project::Untitled
    );
}

#[test]
fn project_lookbehind_without_group_uses_full_match() {
    assert_eq!(
        pattern("(?<=Bitwig Studio - ).*").project("Bitwig Studio - My Project"),
        Project::Named("My Project".to_owned())
    );
}

#[test]
fn project_empty_title_is_unknown() {
    assert_eq!(pattern(".*").project(""), Project::Unknown);
}

#[test]
fn project_no_match_is_unknown() {
    assert_eq!(
        pattern(FL_STUDIO_REGEX).project("Random Window Title"),
        Project::Unknown
    );
}

#[test]
fn project_invalid_regex_is_unknown() {
    assert_eq!(pattern("[invalid").project("title"), Project::Unknown);
}
