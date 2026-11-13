fn use_item(game: &mut crate::game::Game<'_>, stack: &ItemStack, pos: [i32; 3], face: [i32; 3], claimed: &mut bool) {
    crate::game::friendlybones::use_item(game, stack, pos, face, claimed);
}
