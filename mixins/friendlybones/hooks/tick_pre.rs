fn tick_pre(game: &mut crate::game::Game<'_>, dimension: &str) {
    crate::game::friendlybones::tick(game, dimension);
}
