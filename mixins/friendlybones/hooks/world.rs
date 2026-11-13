fn world(phase: WorldPhase) {
    if phase == WorldPhase::WorldOpenReady {
        crate::game::friendlybones::world_ready();
    }
}
