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
        KeyCode::Char('K') => Action::SelectPrevFile,
        KeyCode::Char('J') => Action::SelectNextFile,
        KeyCode::Char('H') => Action::TreeScrollLeft,
        KeyCode::Char('L') => Action::TreeScrollRight,
        KeyCode::Char('b') => Action::ToggleTree,
        KeyCode::Char('R') => Action::Refresh,
        KeyCode::Char('j') => Action::ScrollDown,
        KeyCode::Char('k') => Action::ScrollUp,
        KeyCode::Char('h') => Action::ScrollLeft,
        KeyCode::Char('l') => Action::ScrollRight,
        KeyCode::Char('g') => Action::PrefixG,
        KeyCode::Char('G') => Action::GoBottom,
        KeyCode::Char('n') => Action::NextChange,
        KeyCode::Char('N') => Action::PrevChange,
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
    fn maps_shift_jk_to_tree_file_selection() {
        let prev = map_key(KeyEvent::new(KeyCode::Char('K'), KeyModifiers::SHIFT));
        let next = map_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::SHIFT));

        assert!(matches!(prev, Action::SelectPrevFile));
        assert!(matches!(next, Action::SelectNextFile));
    }

    #[test]
    fn maps_shift_hl_to_tree_horizontal_scroll() {
        let left = map_key(KeyEvent::new(KeyCode::Char('H'), KeyModifiers::SHIFT));
        let right = map_key(KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT));

        assert!(matches!(left, Action::TreeScrollLeft));
        assert!(matches!(right, Action::TreeScrollRight));
    }

    #[test]
    fn does_not_map_lowercase_r_to_refresh() {
        let action = map_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
        assert!(matches!(action, Action::None));
    }

    #[test]
    fn does_not_map_arrow_keys_to_tree_navigation() {
        let left = map_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        let right = map_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        let up = map_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        let down = map_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

        assert!(matches!(left, Action::None));
        assert!(matches!(right, Action::None));
        assert!(matches!(up, Action::None));
        assert!(matches!(down, Action::None));
    }
}
