fn loot(context: &NativeLootDrops<'_>, drops: &mut Vec<ItemStack>) {
    crate::game::friendlybones::loot(context, drops);
}
