#[derive(Clone)]
struct ClientMenu {
    entity: u32,
    dimension: String,
    mode: u8,
    settings: Settings,
    recenter: bool,
    dirty: bool,
}
#[derive(Default)]
struct Ui {
    menu: Option<ClientMenu>,
    drag: Option<usize>,
    focus: usize,
    close: bool,
    pending: Option<Vec<u8>>,
}
static UI: OnceLock<Mutex<Ui>> = OnceLock::new();
fn ui() -> std::sync::MutexGuard<'static, Ui> {
    lock(UI.get_or_init(|| Mutex::new(Ui::default())))
}
fn tr(key: &str) -> String {
    Text::tr(key, &[]).plain()
}
fn settings_value(s: Settings, index: usize) -> u8 {
    match index {
        0 => s.roam,
        1 => s.follow_stop,
        2 => s.follow_start,
        _ => s.guard,
    }
}
fn slider_bounds(index: usize) -> (u8, u8) {
    match index {
        0 => (2, 32),
        1 => (2, 12),
        2 => (4, 32),
        _ => (3, 24),
    }
}
fn slider_set(menu: &mut ClientMenu, index: usize, value: i64) {
    let mut s = menu.settings;
    match index {
        0 => s.roam = value.clamp(2, 32) as u8,
        1 => {
            s.follow_stop = value.clamp(2, 12) as u8;
            s.follow_start = s.follow_start.max(s.follow_stop + 2);
        }
        2 => {
            s.follow_start = value.clamp(4, 32) as u8;
            s.follow_stop = s.follow_stop.min(s.follow_start - 2);
        }
        _ => s.guard = value.clamp(3, 24) as u8,
    }
    menu.settings = s;
    menu.dirty = true;
}
fn outbound(menu: &ClientMenu) -> Vec<u8> {
    json!({"schema":1,"op":"set","entity":menu.entity,"dimension":menu.dimension,
    "mode":menu.mode,"roam":menu.settings.roam,"stop":menu.settings.follow_stop,"start":menu.settings.follow_start,
    "guard":menu.settings.guard,"recenter":menu.recenter}).to_string().into_bytes()
}
pub(super) fn from_server(g: &mut Game, channel: &str, data: &[u8]) {
    if channel != CHANNEL || data.len() > 1024 {
        return;
    }
    let Ok(v) = serde_json::from_slice::<Value>(data) else {
        return;
    };
    if v["schema"] != 1 {
        return;
    }
    let Some(entity) = v["entity"].as_u64().and_then(|n| u32::try_from(n).ok()) else {
        return;
    };
    let Some(mode) = v["mode"]
        .as_u64()
        .and_then(|n| u8::try_from(n).ok())
        .filter(|n| *n < 3)
    else {
        return;
    };
    let Some(dimension) = v["dimension"].as_str().filter(|s| s.len() < 128) else {
        return;
    };
    let menu = ClientMenu {
        entity,
        dimension: dimension.into(),
        mode,
        settings: Settings::checked(
            v["roam"].as_i64().unwrap_or(12),
            v["stop"].as_i64().unwrap_or(6),
            v["start"].as_i64().unwrap_or(16),
            v["guard"].as_i64().unwrap_or(10),
        ),
        recenter: false,
        dirty: false,
    };
    if v["op"] == "open" {
        let mut state = ui();
        state.menu = Some(menu);
        state.focus = 0;
        state.close = false;
        state.drag = None;
        drop(state);
        if !g.native_mod_screen_open(ID) {
            ui().menu = None;
        }
    } else if v["op"] == "state" {
        let mut state = ui();
        if state
            .menu
            .as_ref()
            .is_some_and(|m| m.entity == entity && m.dimension == dimension && !m.dirty)
        {
            state.menu = Some(menu);
        }
    }
}
fn panel(screen: &NativeScreenView<'_>) -> [f32; 2] {
    [
        screen.rect[0] + screen.rect[2] * 0.5 - 142.0,
        screen.rect[1] + screen.rect[3] * 0.5 - 120.0,
    ]
}
fn ours(g: &Game, screen: &NativeScreenView<'_>) -> bool {
    screen.id == "mod" && g.native_mod_screen() == Some(ID)
}
pub(super) fn ui_render(g: &Game, batch: &mut HudBatch, screen: &NativeScreenView<'_>) {
    if !ours(g, screen) {
        return;
    }
    let mut state = ui();
    let focus = state.focus;
    let drag = state.drag;
    let Some(menu) = state.menu.as_mut() else {
        return;
    };
    let [x, y] = panel(screen);
    if let Some(i) = drag {
        let (min, max) = slider_bounds(i);
        let fraction = ((screen.mouse[0] - (x + 12.0)) / 260.0).clamp(0.0, 1.0);
        slider_set(
            menu,
            i,
            i64::from(min) + ((max - min) as f32 * fraction).round() as i64,
        );
    }
    g.native_fill(batch, [x, y, 284.0, 240.0], 0xef181b24);
    g.native_text(
        batch,
        &tr("screen.friendlybones.menu"),
        [x + 12.0, y + 10.0],
        0xffffff,
        true,
    );
    for (i, key) in ["wander", "follow", "guard"].into_iter().enumerate() {
        let bx = x + 12.0 + i as f32 * 88.0;
        let selected = menu.mode == i as u8;
        g.native_fill(
            batch,
            [bx, y + 30.0, 84.0, 22.0],
            if selected { 0xff38784d } else { 0xff343a49 },
        );
        if focus == i {
            g.native_fill(batch, [bx, y + 50.0, 84.0, 2.0], 0xfff2d36b);
        }
        g.native_text(
            batch,
            &tr(&format!("screen.friendlybones.mode.{key}")),
            [bx + 5.0, y + 37.0],
            0xffffff,
            false,
        );
    }
    let keys = ["roam_radius", "follow_stop", "follow_start", "guard_radius"];
    for (i, key) in keys.into_iter().enumerate() {
        let sy = y + 62.0 + i as f32 * 32.0;
        let value = settings_value(menu.settings, i);
        let (min, max) = slider_bounds(i);
        g.native_text(
            batch,
            &format!("{}: {}", tr(&format!("screen.friendlybones.{key}")), value),
            [x + 12.0, sy],
            0xffffff,
            false,
        );
        g.native_fill(
            batch,
            [x + 12.0, sy + 13.0, 260.0, 6.0],
            if focus == i + 3 {
                0xff747b8e
            } else {
                0xff414859
            },
        );
        let t = (value - min) as f32 / (max - min) as f32;
        g.native_fill(
            batch,
            [x + 12.0 + t * 252.0, sy + 10.0, 8.0, 12.0],
            0xffd8e5f0,
        );
    }
    for (i, key) in ["screen.friendlybones.recenter", "gui.done"]
        .into_iter()
        .enumerate()
    {
        let bx = x + 12.0 + i as f32 * 134.0;
        g.native_fill(
            batch,
            [bx, y + 204.0, 126.0, 22.0],
            if focus == i + 7 {
                0xff68714a
            } else {
                0xff343a49
            },
        );
        g.native_text(batch, &tr(key), [bx + 5.0, y + 211.0], 0xffffff, false);
    }
}
pub(super) fn ui_mouse(
    g: &Game,
    screen: &NativeScreenView<'_>,
    button: i32,
    pressed: bool,
    canceled: &mut bool,
) {
    if !ours(g, screen) || button != 0 {
        return;
    }
    *canceled = true;
    let [x, y] = panel(screen);
    let [mx, my] = screen.mouse;
    let mut state = ui();
    if !pressed {
        state.drag = None;
        return;
    }
    let Some(menu) = state.menu.as_mut() else {
        return;
    };
    if (y + 30.0..y + 52.0).contains(&my) && (x + 12.0..x + 272.0).contains(&mx) {
        let index = ((mx - x - 12.0) / 88.0).floor() as u8;
        menu.mode = index.min(2);
        menu.dirty = true;
        state.focus = index as usize;
        return;
    }
    for i in 0..4 {
        let sy = y + 62.0 + i as f32 * 32.0;
        if (sy..sy + 24.0).contains(&my) && (x + 12.0..x + 272.0).contains(&mx) {
            let (min, max) = slider_bounds(i);
            let fraction = ((mx - (x + 12.0)) / 260.0).clamp(0.0, 1.0);
            slider_set(
                menu,
                i,
                i64::from(min) + ((max - min) as f32 * fraction).round() as i64,
            );
            state.drag = Some(i);
            state.focus = i + 3;
            return;
        }
    }
    if (y + 204.0..y + 226.0).contains(&my) {
        if (x + 12.0..x + 138.0).contains(&mx) {
            menu.recenter = true;
            menu.dirty = true;
            state.focus = 7;
        } else if (x + 146.0..x + 272.0).contains(&mx) {
            state.close = true;
        }
    }
}
pub(super) fn ui_key(
    g: &Game,
    screen: &NativeScreenView<'_>,
    key: &str,
    modifiers: u32,
    canceled: &mut bool,
) {
    if !ours(g, screen) {
        return;
    }
    let mut state = ui();
    match key {
        "key.keyboard.tab" => {
            state.focus = (state.focus + if modifiers & 1 != 0 { 8 } else { 1 }) % 9;
            *canceled = true;
        }
        "key.keyboard.escape" => {
            state.close = true;
            *canceled = true;
        }
        "key.keyboard.left" | "key.keyboard.right" => {
            let focus = state.focus;
            let delta = if key.ends_with("right") { 1 } else { -1 };
            if let Some(menu) = state.menu.as_mut() {
                if (3..7).contains(&focus) {
                    slider_set(
                        menu,
                        focus - 3,
                        i64::from(settings_value(menu.settings, focus - 3)) + delta,
                    );
                } else if focus < 3 {
                    menu.mode = ((i64::from(menu.mode) + delta).rem_euclid(3)) as u8;
                    menu.dirty = true;
                }
            }
            *canceled = true;
        }
        "key.keyboard.enter" | "key.keyboard.space" => {
            let focus = state.focus;
            if let Some(menu) = state.menu.as_mut() {
                if focus < 3 {
                    menu.mode = focus as u8;
                    menu.dirty = true;
                } else if focus == 7 {
                    menu.recenter = true;
                    menu.dirty = true;
                } else if focus == 8 {
                    state.close = true;
                }
            }
            *canceled = true;
        }
        _ => {}
    }
}
pub(super) fn ui_closing(g: &Game, screen: &NativeScreenView<'_>) {
    if !ours(g, screen) {
        return;
    }
    let mut state = ui();
    if let Some(menu) = state.menu.take() {
        if menu.dirty {
            state.pending = Some(outbound(&menu));
        }
    }
    state.drag = None;
}
pub(super) fn ui_tick(g: &mut Game) {
    let mut state = ui();
    let close = std::mem::take(&mut state.close);
    if close {
        if let Some(menu) = state.menu.take() {
            if menu.dirty {
                state.pending = Some(outbound(&menu));
            }
        }
        state.drag = None;
    }
    let packet = state.pending.take();
    drop(state);
    if let Some(packet) = packet {
        if let Err(e) = g.native_net_send_to_server(CHANNEL, &packet) {
            eprintln!("[friendlybones] Menu update: {e}");
        }
    }
    if close {
        g.native_mod_screen_close(ID);
    }
}
