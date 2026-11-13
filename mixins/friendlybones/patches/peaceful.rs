fn peaceful(value: bool, m: &Mob) -> bool {
    // Companions are exempt from the Peaceful despawn of disallowed types.
    value
        || matches!(
            m.kind.id.as_str(),
            "friendlybones:friendly_skeleton" | "friendlybones:friendly_wither_skeleton"
        )
}
