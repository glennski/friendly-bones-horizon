fn goals(mob: &mut Mob) {
    // Companions start frozen as persistent creatures that do not burn in daylight; the
    // authoritative adapter (friendlybones runtime) installs their goals on its first tick.
    if matches!(
        mob.kind.id.as_str(),
        "friendlybones:friendly_skeleton" | "friendlybones:friendly_wither_skeleton"
    ) {
        let mut kind = (*mob.kind).clone();
        kind.category = "creature".into();
        kind.flags.burns_in_day = false;
        mob.kind = std::sync::Arc::new(kind);
        mob.persistent = true;
        mob.no_ai = true;
        mob.ai.goals.clear();
        mob.ai.targets.clear();
    }
}
