fn net_server(game: &mut crate::game::Game<'_>, channel: &str, data: &[u8]) {
    crate::game::friendlybones::from_server(game, channel, data);
}
