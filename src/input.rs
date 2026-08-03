use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    SelectPrevFile,
    SelectNextFile,
    ToggleTree,
    Refresh,
    TreeScrollLeft,
    TreeScrollRight,
    ScrollDown,
    ScrollUp,
    PageDown,
    PageUp,
    ScrollLeft,
    ScrollRight,
    PrefixG,
    GoBottom,
    NextChange,
    PrevChange,
    ToggleComments,
    ToggleStage,
    ToggleStageAll,
    GitCommit,
    GitPull,
    GitPush,
    CheckoutFile,
    DeleteFile,
    ConfirmYes,
    ShowHelp,
    CloseOverlay,
    Quit,
    None,
}

pub fn map_key(key: KeyEvent) -> Action {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return match key.code {
            KeyCode::Char('d') => Action::PageDown,
            KeyCode::Char('u') => Action::PageUp,
            _ => Action::None,
        };
    }

    match key.code {
        KeyCode::Char('j') => Action::SelectNextFile,
        KeyCode::Char('k') => Action::SelectPrevFile,
        KeyCode::Char('h') => Action::TreeScrollLeft,
        KeyCode::Char('l') => Action::TreeScrollRight,
        KeyCode::Char('J') => Action::ScrollDown,
        KeyCode::Char('K') => Action::ScrollUp,
        KeyCode::Char('H') => Action::ScrollLeft,
        KeyCode::Char('L') => Action::ScrollRight,
        KeyCode::Char('b') => Action::ToggleTree,
        KeyCode::Char('c') => Action::ToggleComments,
        KeyCode::Char('C') => Action::GitCommit,
        KeyCode::Char('p') => Action::GitPull,
        KeyCode::Char('P') => Action::GitPush,
        KeyCode::Char('R') => Action::Refresh,
        KeyCode::Char('g') => Action::PrefixG,
        KeyCode::Char('G') => Action::GoBottom,
        KeyCode::Char('n') => Action::NextChange,
        KeyCode::Char('N') => Action::PrevChange,
        KeyCode::Char(' ') => Action::ToggleStage,
        KeyCode::Char('A') => Action::ToggleStageAll,
        KeyCode::Char('!') => Action::CheckoutFile,
        KeyCode::Char('d') => Action::DeleteFile,
        KeyCode::Char('y') => Action::ConfirmYes,
        KeyCode::Char('?') => Action::ShowHelp,
        KeyCode::Up => Action::ScrollUp,
        KeyCode::Down => Action::ScrollDown,
        KeyCode::Left => Action::ScrollLeft,
        KeyCode::Right => Action::ScrollRight,
        KeyCode::Esc => Action::CloseOverlay,
        KeyCode::Char('q') => Action::Quit,
        _ => Action::None,
    }
}

#[cfg(test)]
mod tests {
    use super::{map_key, Action};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn maps_b_to_toggle_tree() {
        let action = map_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
        assert!(matches!(action, Action::ToggleTree));
    }

    #[test]
    fn maps_shift_r_to_refresh() {
        let action = map_key(KeyEvent::new(KeyCode::Char('R'), KeyModifiers::SHIFT));
        assert!(matches!(action, Action::Refresh));
    }

    #[test]
    fn maps_jk_to_file_selection() {
        let prev = map_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
        let next = map_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));

        assert!(matches!(prev, Action::SelectPrevFile));
        assert!(matches!(next, Action::SelectNextFile));
    }

    #[test]
    fn maps_hl_to_tree_horizontal_scroll() {
        let left = map_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
        let right = map_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));

        assert!(matches!(left, Action::TreeScrollLeft));
        assert!(matches!(right, Action::TreeScrollRight));
    }

    #[test]
    fn maps_shift_jk_to_content_scroll() {
        let down = map_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::SHIFT));
        let up = map_key(KeyEvent::new(KeyCode::Char('K'), KeyModifiers::SHIFT));

        assert!(matches!(down, Action::ScrollDown));
        assert!(matches!(up, Action::ScrollUp));
    }

    #[test]
    fn maps_shift_hl_to_content_horizontal_scroll() {
        let left = map_key(KeyEvent::new(KeyCode::Char('H'), KeyModifiers::SHIFT));
        let right = map_key(KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT));

        assert!(matches!(left, Action::ScrollLeft));
        assert!(matches!(right, Action::ScrollRight));
    }

    #[test]
    fn does_not_map_lowercase_r_to_refresh() {
        let action = map_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
        assert!(matches!(action, Action::None));
    }

    #[test]
    fn maps_arrow_keys_to_content_scroll() {
        let left = map_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        let right = map_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        let up = map_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        let down = map_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

        assert!(matches!(left, Action::ScrollLeft));
        assert!(matches!(right, Action::ScrollRight));
        assert!(matches!(up, Action::ScrollUp));
        assert!(matches!(down, Action::ScrollDown));
    }
}
