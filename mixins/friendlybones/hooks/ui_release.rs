fn ui_release(game: &crate::game::Game<'_>, screen: &NativeScreenView<'_>, button: i32, canceled: &mut bool) {
    crate::game::friendlybones::ui_mouse(game, screen, button, false, canceled);
}
