fn ui_key(game: &crate::game::Game<'_>, screen: &NativeScreenView<'_>, key: &str, modifiers: u32, canceled: &mut bool) {
    crate::game::friendlybones::ui_key(game, screen, key, modifiers, canceled);
}
