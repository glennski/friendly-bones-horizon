fn tick_post(game: &mut crate::game::Game<'_>, dimension: &str) {
    crate::game::friendlybones::end_tick(game, dimension);
}
