fn damage_in(victim: u32, kind: &str, attacker: Option<u32>, amount: &mut f32) {
    crate::game::friendlybones::incoming_damage(victim, kind == "minecraft:player", attacker, amount);
}
