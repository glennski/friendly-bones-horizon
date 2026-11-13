fn interact(game: &mut crate::game::Game<'_>, entity: i32, claimed: &mut bool) {
    crate::game::friendlybones::interact(game, entity, claimed);
}
