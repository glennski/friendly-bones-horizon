fn damage_post(entity: i32, kind: &str, causing: i32, amount: f32) {
    crate::game::friendlybones::damage_done(entity, kind, causing, amount);
}
