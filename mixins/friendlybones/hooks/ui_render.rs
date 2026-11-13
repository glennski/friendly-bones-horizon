fn ui_render(game: &crate::game::Game<'_>, batch: &mut HudBatch, screen: &NativeScreenView<'_>) {
    crate::game::friendlybones::ui_render(game, batch, screen);
}
