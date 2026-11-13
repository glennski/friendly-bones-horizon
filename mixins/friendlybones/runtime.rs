mod logic {
// Engine-independent companion decisions. The native adapter uses this same file.

pub type Position = [f64; 3];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Roam,
    Follow,
    Guard,
}

impl Mode {
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Self::Roam),
            1 => Some(Self::Follow),
            2 => Some(Self::Guard),
            _ => None,
        }
    }
    pub fn id(self) -> u8 {
        match self {
            Self::Roam => 0,
            Self::Follow => 1,
            Self::Guard => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub roam: u8,
    pub follow_stop: u8,
    pub follow_start: u8,
    pub guard: u8,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            roam: 12,
            follow_stop: 6,
            follow_start: 16,
            guard: 10,
        }
    }
}
impl Settings {
    pub fn checked(roam: i64, stop: i64, start: i64, guard: i64) -> Self {
        let stop = stop.clamp(2, 12) as u8;
        Self {
            roam: roam.clamp(2, 32) as u8,
            follow_stop: stop,
            follow_start: start.clamp(4, 32).max(i64::from(stop) + 2) as u8,
            guard: guard.clamp(3, 24) as u8,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityConfig {
    pub health: f32,
    pub speed: f64,
    pub follow_range: f64,
    pub attack_range: f64,
    pub damage: f32,
    pub shot_interval: i32,
    pub unlimited_ammo: bool,
}
impl Default for EntityConfig {
    fn default() -> Self {
        Self {
            health: 20.0,
            speed: 0.25,
            follow_range: 32.0,
            attack_range: 36.0,
            damage: 1.0,
            shot_interval: 20,
            unlimited_ammo: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Companion {
    pub owner: Option<u128>,
    pub mode: Mode,
    pub settings: Settings,
    pub anchor: Position,
    pub ammo: u32,
    pub initialized: bool,
    pub reserve: Vec<u8>,
    pub arrows: Vec<Vec<u8>>,
    pub enabled: bool,
    // Transient, deliberately not part of the save format.
    pub owner_position: Option<Position>,
    pub alert_until: i64,
    pub alert_target: Option<u32>,
    pub melee: bool,
    pub following: bool,
}
impl Companion {
    pub fn new(position: Position) -> Self {
        Self {
            owner: None,
            mode: Mode::Roam,
            settings: Settings::default(),
            anchor: position,
            ammo: 0,
            initialized: false,
            reserve: Vec::new(),
            arrows: Vec::new(),
            enabled: true,
            owner_position: None,
            alert_until: 0,
            alert_target: None,
            melee: false,
            following: false,
        }
    }

    pub fn claim(&mut self, player: u128) -> bool {
        if self.owner.is_none() {
            self.owner = Some(player);
        }
        self.owner == Some(player)
    }

    pub fn apply_menu(
        &mut self,
        player: u128,
        player_pos: Position,
        mob_pos: Position,
        mode: u8,
        settings: Settings,
        recenter: bool,
    ) -> bool {
        if self.owner != Some(player)
            || !finite(player_pos)
            || !finite(mob_pos)
            || distance_sq(player_pos, mob_pos) > 144.0
        {
            return false;
        }
        let Some(mode) = Mode::from_id(mode) else {
            return false;
        };
        if mode != Mode::Follow && (mode != self.mode || recenter) {
            self.anchor = mob_pos;
        }
        self.mode = mode;
        self.settings = Settings::checked(
            settings.roam.into(),
            settings.follow_stop.into(),
            settings.follow_start.into(),
            settings.guard.into(),
        );
        self.following = false;
        true
    }

    pub fn alert(&mut self, attacker: u32, now: i64) {
        self.alert_until = now.saturating_add(200);
        self.alert_target = Some(attacker);
    }

    pub fn alert_active(&self, now: i64) -> bool {
        self.owner_position.is_some() && now < self.alert_until
    }

    pub fn allows_target(&self, pos: Position, now: i64) -> bool {
        finite(pos)
            && (self.alert_active(now)
                || self.mode != Mode::Roam
                || distance_sq(pos, self.anchor) <= (f64::from(self.settings.roam) + 4.0).powi(2))
    }

    pub fn weapon(&mut self, distance: f64, can_shoot: bool) -> bool {
        if !distance.is_finite() {
            self.melee = false;
            return false;
        }
        if !can_shoot || distance < 3.0 {
            self.melee = true;
        } else if distance >= 5.0 {
            self.melee = false;
        }
        self.melee
    }

    pub fn return_to(&mut self, position: Position, fighting: bool, now: i64) -> Option<Position> {
        if self.alert_active(now) {
            let owner = self.owner_position?;
            return (distance_sq(position, owner) > 36.0).then_some(owner);
        }
        match self.mode {
            Mode::Follow if !fighting => {
                let Some(owner) = self.owner_position else {
                    self.following = false;
                    return None;
                };
                let d = distance_sq(position, owner);
                if d > f64::from(self.settings.follow_start).powi(2) {
                    self.following = true;
                }
                if d < f64::from(self.settings.follow_stop).powi(2) {
                    self.following = false;
                }
                self.following.then_some(owner)
            }
            Mode::Guard => {
                let radius = if fighting {
                    f64::from(self.settings.guard)
                } else {
                    0.5
                };
                (distance_sq(position, self.anchor) > radius * radius).then_some(self.anchor)
            }
            Mode::Roam => (distance_sq(position, self.anchor)
                > (f64::from(self.settings.roam) + 6.0).powi(2))
            .then_some(self.anchor),
            _ => None,
        }
    }
}

pub fn finite(pos: Position) -> bool {
    pos.into_iter().all(f64::is_finite)
}
pub fn distance_sq(a: Position, b: Position) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| (a - b) * (a - b)).sum()
}
pub fn friendly_damage(
    bones_attacker: bool,
    bones_victim: bool,
    player_victim: bool,
    owner_attacking_own: bool,
    amount: f32,
    multiplier: f32,
) -> f32 {
    if (bones_attacker && (bones_victim || player_victim)) || owner_attacking_own {
        amount
            * if multiplier.is_finite() {
                multiplier.clamp(0.0, 1.0)
            } else {
                0.25
            }
    } else {
        amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn untrusted_settings_are_bounded_and_have_a_gap() {
        let s = Settings::checked(i64::MAX, i64::MAX, i64::MIN, i64::MIN);
        assert_eq!(
            s,
            Settings {
                roam: 32,
                follow_stop: 12,
                follow_start: 14,
                guard: 3
            }
        );
    }
    #[test]
    fn only_owner_in_reach_can_change_mode() {
        let mut c = Companion::new([0.0; 3]);
        c.claim(7);
        assert!(!c.claim(8));
        assert!(!c.apply_menu(8, [0.0; 3], [0.0; 3], 2, Settings::default(), true));
        assert!(!c.apply_menu(7, [13.0, 0.0, 0.0], [0.0; 3], 2, Settings::default(), true));
        assert!(!c.apply_menu(
            7,
            [f64::NAN, 0.0, 0.0],
            [0.0; 3],
            2,
            Settings::default(),
            true
        ));
        assert!(c.apply_menu(7, [12.0, 0.0, 0.0], [0.0; 3], 2, Settings::default(), true));
        assert_eq!(c.mode, Mode::Guard);
    }
    #[test]
    fn follow_keeps_moving_between_thresholds() {
        let mut c = Companion::new([0.0; 3]);
        c.mode = Mode::Follow;
        c.owner_position = Some([20.0, 0.0, 0.0]);
        assert_eq!(c.return_to([0.0; 3], false, 0), c.owner_position);
        assert_eq!(c.return_to([10.0, 0.0, 0.0], false, 0), c.owner_position);
        assert_eq!(c.return_to([16.0, 0.0, 0.0], false, 0), None);
    }
    #[test]
    fn guard_returns_after_fight_and_leash_is_bounded() {
        let mut c = Companion::new([0.0; 3]);
        c.mode = Mode::Guard;
        assert_eq!(c.return_to([5.0, 0.0, 0.0], true, 0), None);
        assert_eq!(c.return_to([11.0, 0.0, 0.0], true, 0), Some([0.0; 3]));
        assert_eq!(c.return_to([5.0, 0.0, 0.0], false, 0), Some([0.0; 3]));
    }
    #[test]
    fn alert_is_temporary_and_keeps_guard_post() {
        let mut c = Companion::new([1.0, 2.0, 3.0]);
        c.mode = Mode::Guard;
        c.owner_position = Some([30.0, 2.0, 3.0]);
        c.alert(42, 10);
        assert_eq!(c.return_to([0.0; 3], false, 20), c.owner_position);
        assert!(!c.alert_active(210));
        assert_eq!(c.mode, Mode::Guard);
        assert_eq!(c.anchor, [1.0, 2.0, 3.0]);
    }
    #[test]
    fn bow_sword_switch_has_hysteresis() {
        let mut c = Companion::new([0.0; 3]);
        assert!(c.weapon(2.0, true));
        assert!(c.weapon(4.0, true));
        assert!(!c.weapon(5.0, true));
        assert!(!c.weapon(4.0, true));
        assert!(c.weapon(20.0, false));
    }
    #[test]
    fn friendly_fire_does_not_reduce_other_combat() {
        assert_eq!(friendly_damage(true, true, false, false, 10.0, 0.25), 2.5);
        assert_eq!(friendly_damage(true, false, true, false, 10.0, 0.25), 2.5);
        assert_eq!(friendly_damage(false, true, false, true, 10.0, 0.25), 2.5);
        assert_eq!(friendly_damage(false, true, false, false, 10.0, 0.25), 10.0);
    }
    #[test]
    fn roam_target_and_return_limits_are_distinct() {
        let mut c = Companion::new([0.0; 3]);
        c.settings.roam = 2;
        assert!(c.allows_target([6.0, 0.0, 0.0], 0));
        assert!(!c.allows_target([7.0, 0.0, 0.0], 0));
        assert_eq!(c.return_to([9.0, 0.0, 0.0], true, 0), Some([0.0; 3]));
    }
}

}
// Native Horizon adapter. Built inside game::friendlybones on the pinned revision.
use super::Game;
use logic::{Companion, EntityConfig, Mode, Settings, distance_sq};
use horizon_client::{
    hud::HudBatch,
    items::ItemStack,
    mobs::{
        Mob, Species, Who,
        goals::{self, Cx, Goal},
        undead::BowAttack,
    },
    mod_api::{AttachmentCodec, AttachmentError, AttachmentType, Holder, Host, NativeScreenView},
    text::Text,
};
use horizon_world::nbt::Tag;
use serde_json::{Value, json};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, OnceLock, RwLock},
};

const ID: &str = "friendlybones";
const CHANNEL: &str = "friendlybones:menu";
const SKELETON: &str = "friendlybones:friendly_skeleton";
const WITHER: &str = "friendlybones:friendly_wither_skeleton";
const EGG: &str = "friendlybones:friendly_skeleton_spawn_egg";
type Shared = Arc<Mutex<Companion>>;
fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}
fn is_bones(kind: &str) -> bool {
    matches!(kind, SKELETON | WITHER)
}
fn position(m: &Mob) -> [f64; 3] {
    [m.position.x, m.position.y, m.position.z]
}

#[derive(Clone, Debug)]
struct Config {
    skeleton: EntityConfig,
    wither: EntityConfig,
    alert: bool,
    alert_radius: f64,
    friendly_fire: f32,
    announce: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            skeleton: EntityConfig::default(),
            wither: EntityConfig::default(),
            alert: true,
            alert_radius: 64.0,
            friendly_fire: 0.25,
            announce: true,
        }
    }
}
fn number(v: &Value, key: &str, default: f64, min: f64, max: f64) -> f64 {
    v[key]
        .as_f64()
        .filter(|n| n.is_finite())
        .unwrap_or(default)
        .clamp(min, max)
}
fn entity_config(v: &Value) -> EntityConfig {
    EntityConfig {
        health: number(v, "maxHealth", 20.0, 1.0, 10000.0) as f32,
        speed: number(v, "speed", 0.25, 0.01, 2.0),
        follow_range: number(v, "followRange", 32.0, 4.0, 128.0),
        attack_range: number(v, "attackRange", 36.0, 3.0, 128.0),
        damage: number(v, "damageMult", 1.0, 0.0, 100.0) as f32,
        shot_interval: number(v, "rateOfFire", 20.0, 1.0, 1200.0) as i32,
        unlimited_ammo: v["unlimitedAmmo"].as_bool().unwrap_or(true),
    }
}
fn load_config() -> Config {
    let Ok(dir) = Game::native_config_dir() else {
        return Config::default();
    };
    let path = dir.join("friendlybones.json");
    let default = json!({"skeleton":{"maxHealth":20.0,"speed":0.25,"followRange":32.0,"attackRange":36.0,
        "damageMult":1.0,"rateOfFire":20,"unlimitedAmmo":true},
        "wither_skeleton":{"maxHealth":20.0,"speed":0.25,"followRange":32.0,"attackRange":36.0,
        "damageMult":1.0,"rateOfFire":20,"unlimitedAmmo":true},
        "alertIfPlayerAttacked":true,"alertRadius":64.0,"friendlyFireDamageMultiplier":0.25,"announceStatusChat":true});
    let v = match std::fs::read(&path) {
        Ok(bytes) => match serde_json::from_slice::<Value>(&bytes) {
            Ok(v) if v.is_object() => v,
            _ => {
                eprintln!(
                    "[friendlybones] Invalid config {}; using defaults, original preserved",
                    path.display()
                );
                default
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if let Err(e) = std::fs::write(&path, serde_json::to_vec_pretty(&default).unwrap()) {
                eprintln!("[friendlybones] Config: {e}");
            }
            default
        }
        Err(e) => {
            eprintln!("[friendlybones] Config: {e}");
            default
        }
    };
    Config {
        skeleton: entity_config(if v["skeleton"].is_object() {
            &v["skeleton"]
        } else {
            &v
        }),
        wither: entity_config(&v["wither_skeleton"]),
        alert: v["alertIfPlayerAttacked"].as_bool().unwrap_or(true),
        alert_radius: number(&v, "alertRadius", 64.0, 1.0, 256.0),
        friendly_fire: number(&v, "friendlyFireDamageMultiplier", 0.25, 0.0, 1.0) as f32,
        announce: v["announceStatusChat"].as_bool().unwrap_or(true),
    }
}

static SAVED: AttachmentType<String> = AttachmentType {
    namespace: ID,
    name: "companion",
    default: String::new,
    codec: Some(AttachmentCodec {
        encode: |s| Ok([("json".into(), Tag::String(s.as_str().into()))].into()),
        decode: |c| match c.get(&"json".into()) {
            Some(Tag::String(s)) => s.to_string().map_err(|_| AttachmentError::Decode),
            _ => Err(AttachmentError::Decode),
        },
    }),
    copy_on_death: false,
    sync: false,
};
fn saved(c: &Companion) -> String {
    json!({"schema":1,"owner":c.owner.map(|u|format!("{u:032x}")),"mode":c.mode.id(),"anchor":c.anchor,
        "roam":c.settings.roam,"stop":c.settings.follow_stop,"start":c.settings.follow_start,
        "guard":c.settings.guard,"ammo":c.ammo,"reserve":c.reserve,"arrows":c.arrows,"initialized":c.initialized}).to_string()
}
fn restored(data: &str, pos: [f64; 3]) -> Result<Companion, String> {
    let mut c = Companion::new(pos);
    if data.is_empty() {
        return Ok(c);
    }
    if data.len() > 1048576 {
        return Err("Oversized saved state; original preserved".into());
    }
    let v = serde_json::from_str::<Value>(data).map_err(|e| e.to_string())?;
    if v["schema"].as_u64() != Some(1) {
        return Err("Unsupported saved companion schema; original preserved".into());
    }
    c.owner = match &v["owner"] {
        Value::Null => None,
        Value::String(s) if s.len() == 32 => Some(
            u128::from_str_radix(s, 16).map_err(|_| "Invalid saved owner; original preserved")?,
        ),
        _ => return Err("Invalid saved owner; original preserved".into()),
    };
    c.mode = v["mode"]
        .as_u64()
        .and_then(|n| u8::try_from(n).ok())
        .and_then(Mode::from_id)
        .ok_or("Invalid saved mode; original preserved")?;
    c.settings = Settings::checked(
        v["roam"].as_i64().unwrap_or(12),
        v["stop"].as_i64().unwrap_or(6),
        v["start"].as_i64().unwrap_or(16),
        v["guard"].as_i64().unwrap_or(10),
    );
    c.anchor = serde_json::from_value::<[f64; 3]>(v["anchor"].clone())
        .map_err(|_| "Invalid saved anchor; original preserved")?;
    if !logic::finite(c.anchor) {
        return Err("Non-finite saved anchor; original preserved".into());
    }
    c.ammo = v["ammo"].as_u64().unwrap_or(0).min(u32::MAX.into()) as u32;
    c.initialized = v["initialized"].as_bool().unwrap_or(false);
    c.reserve = serde_json::from_value::<Vec<u8>>(v["reserve"].clone()).unwrap_or_default();
    if c.reserve.len() > 65536 {
        return Err("Oversized reserved weapon; original preserved".into());
    }
    if !c.reserve.is_empty() && decode_stack(&c.reserve).is_none() {
        return Err("Invalid reserved weapon; original preserved".into());
    }
    c.arrows = serde_json::from_value::<Vec<Vec<u8>>>(v["arrows"].clone()).unwrap_or_default();
    if c.arrows.len() > 128 || c.arrows.iter().map(Vec::len).sum::<usize>() > 65536 {
        return Err("Oversized ammunition; original preserved".into());
    }
    c.ammo = 0;
    for bytes in &c.arrows {
        let s = decode_stack(bytes).ok_or("Invalid saved ammunition; original preserved")?;
        if !is_arrow(&s) {
            return Err("Invalid ammunition item; original preserved".into());
        }
        c.ammo = c.ammo.saturating_add(s.count);
    }
    if c.ammo > 4096 {
        return Err("Oversized ammunition count; original preserved".into());
    }
    Ok(c)
}
fn is_arrow(s: &ItemStack) -> bool {
    matches!(
        s.item.as_ref(),
        "minecraft:arrow" | "minecraft:tipped_arrow" | "minecraft:spectral_arrow"
    )
}
fn encode_stack(s: &ItemStack) -> Option<Vec<u8>> {
    horizon_world::nbt::write_network(
        &horizon_items::item_nbt::item_to_nbt(s),
        horizon_world::nbt::Limits::default(),
    )
    .ok()
}
fn decode_stack(bytes: &[u8]) -> Option<ItemStack> {
    if bytes.len() > 65536 {
        return None;
    }
    let (tag, n) = horizon_world::nbt::read_network(
        bytes,
        horizon_world::nbt::Limits {
            max_bytes: 65536,
            max_nodes: 4096,
            max_depth: 32,
        },
    )
    .ok()?;
    if n != bytes.len() {
        return None;
    }
    horizon_items::item_nbt::item_from_nbt(&tag)
}

struct Record {
    state: Shared,
    dimension: String,
    last_saved: String,
}
struct Ritual {
    dimension: String,
    pos: [i32; 3],
    owner: Option<u128>,
}
#[derive(Default)]
struct Runtime {
    bones: HashMap<u32, Record>,
    friends: Arc<RwLock<HashSet<u32>>>,
    config: Option<Config>,
    rituals: Vec<Ritual>,
    alarms: Vec<(i32, i32)>,
    players: HashMap<i32, u128>,
    hostile: HashSet<u32>,
}
// ponytail: one short lock per engine callback; split per-world state if measured contention warrants it.
static RUNTIME: OnceLock<Mutex<Runtime>> = OnceLock::new();
fn runtime() -> std::sync::MutexGuard<'static, Runtime> {
    lock(RUNTIME.get_or_init(|| Mutex::new(Runtime::default())))
}

pub(super) fn world_ready() {
    *runtime() = Runtime::default();
}

pub(super) fn tick(g: &mut Game, dimension: &str) {
    if g.remote.is_some() {
        return;
    }
    let mut rt = runtime();
    let config = rt.config.get_or_insert_with(load_config).clone();
    let players = g.native_server_players();
    rt.players = players.iter().map(|p| (p.entity_id, p.uuid)).collect();
    let enabled = g.native_on(ID);
    let found: Vec<_> = g
        .mobs
        .iter()
        .filter(|m| is_bones(&m.kind.id) && !m.removed)
        .map(|m| (m.id, position(m), m.kind.id.clone()))
        .collect();
    let present: HashSet<u32> = found.iter().map(|(id, _, _)| *id).collect();
    rt.bones
        .retain(|id, r| r.dimension != dimension || present.contains(id));
    rt.hostile = g
        .mobs
        .iter()
        .filter(|m| {
            !m.removed
                && !is_bones(&m.kind.id)
                && m.kind.category == "monster"
                && m.species() != Species::Enderman
        })
        .map(|m| m.id)
        .collect();
    for (id, pos, kind) in found {
        if !rt.bones.contains_key(&id) {
            let data = match SAVED.get(g as &mut dyn Host, Holder::Entity(id)) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("[friendlybones] Cannot read entity {id}: {e:?}");
                    continue;
                }
            };
            let initial = match restored(&data, pos) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[friendlybones] Entity {id}: {e}");
                    continue;
                }
            };
            let state = Arc::new(Mutex::new(initial));
            let cfg = if kind == WITHER {
                config.wither.clone()
            } else {
                config.skeleton.clone()
            };
            if let Some(m) = g.mobs.iter_mut().find(|m| m.id == id) {
                let mut ty = (*m.kind).clone();
                ty.category = "creature".into();
                ty.flags.burns_in_day = false;
                m.kind = Arc::new(ty);
                m.persistent = true;
                m.extra.pick_up_loot = true;
                m.no_ai = false;
                m.attrs.max_health = cfg.health;
                m.attrs.follow_range = cfg.follow_range;
                m.attrs.speed = cfg.speed;
                let mut state_guard = lock(&state);
                if !state_guard.initialized {
                    m.health = cfg.health;
                    m.extra.set_main_hand(Some("minecraft:bow".into()));
                    m.extra.off_hand = Some("minecraft:iron_sword".into());
                    state_guard.initialized = true;
                }
                drop(state_guard);
                m.ai.goals.clear();
                m.ai.targets.clear();
                m.ai.goals.add(0, goals::Float);
                m.ai.goals.add(
                    1,
                    CompanionGoal {
                        state: state.clone(),
                        friends: rt.friends.clone(),
                        config: cfg.clone(),
                        bow: BowAttack::new(1.0, cfg.shot_interval, cfg.attack_range as f32),
                        melee: goals::MeleeAttack::new(1.1, true),
                        active: 0,
                        scan_at: 0,
                        move_at: 0,
                    },
                );
                m.ai.goals.add(5, goals::LookAtPlayer::new(12.0));
                m.ai.goals.add(6, goals::RandomLookAround::default());
            }
            rt.bones.insert(
                id,
                Record {
                    state,
                    dimension: dimension.into(),
                    last_saved: data,
                },
            );
        }
        if let Some(r) = rt.bones.get(&id) {
            let mut c = lock(&r.state);
            c.enabled = enabled;
            c.owner_position = players
                .iter()
                .find(|p| {
                    Some(p.uuid) == c.owner
                        && p.dimension == dimension
                        && p.health > 0.0
                        && p.game_mode != 3
                })
                .map(|p| p.position);
            if c.owner_position.is_none() {
                c.alert_until = 0;
                c.alert_target = None;
            }
            if c.ammo < 4096 && c.arrows.len() < 128 {
                let min = [pos[0] - 1.0, pos[1] - 0.5, pos[2] - 1.0];
                let max = [pos[0] + 1.0, pos[1] + 2.5, pos[2] + 1.0];
                for item in g.native_item_entities(dimension, min, max) {
                    if item.pickup_delay != 0 || !is_arrow(&item.stack) {
                        continue;
                    }
                    let take = item.stack.count.min(4096 - c.ammo);
                    if take == 0 || c.arrows.len() >= 128 {
                        break;
                    }
                    let acquired = ItemStack {
                        count: take,
                        ..item.stack.clone()
                    };
                    let Some(bytes) = encode_stack(&acquired).filter(|b| {
                        b.len() + c.arrows.iter().map(Vec::len).sum::<usize>() <= 65536
                    }) else {
                        continue;
                    };
                    let remainder = ItemStack {
                        count: item.stack.count - take,
                        ..item.stack
                    };
                    if g.native_set_item_entity(dimension, item.id, remainder) {
                        c.arrows.push(bytes);
                        c.ammo += take;
                    }
                }
            }
        }
    }
    *rt.friends.write().unwrap_or_else(|e| e.into_inner()) = rt.bones.keys().copied().collect();
    let alarms = std::mem::take(&mut rt.alarms);
    if config.alert {
        for (victim, attacker) in alarms {
            let Some(player) = players.iter().find(|p| p.entity_id == victim) else {
                continue;
            };
            if player.dimension != dimension {
                rt.alarms.push((victim, attacker));
                continue;
            }
            if !rt.hostile.contains(&(attacker as u32)) {
                continue;
            }
            for record in rt.bones.values().filter(|r| r.dimension == dimension) {
                let mut state = lock(&record.state);
                if state.owner == Some(player.uuid) {
                    if let Some(m) = g.mobs.iter().find(|m| {
                        rt.bones
                            .get(&m.id)
                            .is_some_and(|r| Arc::ptr_eq(&r.state, &record.state))
                    }) {
                        if position(m)
                            .into_iter()
                            .zip(player.position)
                            .all(|(a, b)| (a - b).abs() <= config.alert_radius)
                        {
                            state.alert(attacker as u32, g.world.game_time);
                        }
                    }
                }
            }
        }
    }
    // Save only changes, before AI; end-of-level hook flushes changes from this tick as well.
    persist(g, &mut rt, dimension);
    let pending = std::mem::take(&mut rt.rituals);
    for ritual in pending {
        if ritual.dimension != dimension {
            rt.rituals.push(ritual);
            continue;
        }
        let p = ritual.pos;
        let b1 = [p[0], p[1] - 1, p[2]];
        let b2 = [p[0], p[1] - 2, p[2]];
        let head = g.native_block(p);
        let kind = if head == "minecraft:jack_o_lantern" {
            SKELETON
        } else if head == "minecraft:wither_skeleton_skull" {
            WITHER
        } else {
            continue;
        };
        if g.native_block(b1) != "minecraft:bone_block"
            || g.native_block(b2) != "minecraft:bone_block"
        {
            continue;
        }
        if let Some(id) = spawn(
            g,
            kind,
            [p[0] as f64 + 0.5, p[1] as f64 - 1.0, p[2] as f64 + 0.5],
            ritual.owner,
            &config,
        ) {
            if let Some(air) = g.pack.assets.registry.default_state("minecraft:air") {
                for p in [p, b1, b2] {
                    g.native_set_block(dimension, p, air, 3);
                }
            }
            eprintln!("[friendlybones] Ritual spawned {id}");
        }
    }
}
fn persist(g: &mut Game, rt: &mut Runtime, dimension: &str) {
    for (id, record) in rt
        .bones
        .iter_mut()
        .filter(|(_, r)| r.dimension == dimension)
    {
        let data = saved(&lock(&record.state));
        if data != record.last_saved {
            match SAVED.set(g as &mut dyn Host, Holder::Entity(*id), data.clone()) {
                Ok(()) => record.last_saved = data,
                Err(e) => eprintln!("[friendlybones] Save {id}: {e:?}"),
            }
        }
    }
}
pub(super) fn end_tick(g: &mut Game, dimension: &str) {
    persist(g, &mut runtime(), dimension);
}
fn spawn(
    g: &mut Game,
    kind: &str,
    pos: [f64; 3],
    owner: Option<u128>,
    cfg: &Config,
) -> Option<u32> {
    let id = g.spawn_mob(kind, pos, true)?;
    let mut state = Companion::new(pos);
    state.owner = owner;
    if let Err(e) = SAVED.set(g as &mut dyn Host, Holder::Entity(id), saved(&state)) {
        eprintln!("[friendlybones] Spawn state: {e:?}");
        g.native_remove_mob(g.dim.id(), id);
        return None;
    }
    if let Some(m) = g.mobs.iter_mut().find(|m| m.id == id) {
        m.persistent = true;
        if let Some(owner) = owner.and_then(|u| {
            g.lan
                .as_ref()
                .and_then(|h| h.hub.guests.iter().find(|p| p.uuid == u))
        }) {
            m.custom_name = Some(format!("{}'s Bones", owner.name));
            m.name_visible = true;
        }
        let mut ty = (*m.kind).clone();
        ty.category = "creature".into();
        ty.flags.burns_in_day = false;
        m.kind = Arc::new(ty);
        m.health = if kind == WITHER {
            cfg.wither.health
        } else {
            cfg.skeleton.health
        };
    }
    Some(id)
}

#[derive(Clone, Debug)]
struct CompanionGoal {
    state: Shared,
    friends: Arc<RwLock<HashSet<u32>>>,
    config: EntityConfig,
    bow: BowAttack,
    melee: goals::MeleeAttack,
    active: u8,
    scan_at: i64,
    move_at: i64,
}
impl Goal for CompanionGoal {
    fn flags(&self) -> u8 {
        goals::MOVE | goals::LOOK | goals::TARGET
    }
    fn every_tick(&self) -> bool {
        true
    }
    fn can_use(&mut self, _: &mut Mob, _: &mut Cx) -> bool {
        lock(&self.state).enabled
    }
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.stop_attack(m, cx);
        m.ai.nav.stop();
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let mut state = lock(&self.state);
        let friends = self.friends.read().unwrap_or_else(|e| e.into_inner());
        let valid = |s: &horizon_client::mobs::Seen| {
            s.alive && s.hostile && s.species != Species::Enderman && !friends.contains(&s.id)
        };
        let now = cx.w.game_time;
        let target = m.ai.target.and_then(|who| match who {
            Who::Mob(id) => cx.w.seen(id),
            _ => None,
        });
        if target.is_some_and(|s| {
            !valid(s) || !state.allows_target([s.position.x, s.position.y, s.position.z], now)
        }) {
            m.ai.target = None;
        }
        if now >= self.scan_at {
            self.scan_at = now + 10;
            let alarm = state
                .alert_active(now)
                .then_some(state.alert_target)
                .flatten()
                .and_then(|id| cx.w.seen(id))
                .filter(|s| valid(s));
            let alert = state.alert_active(now);
            let center = if alert {
                state.owner_position.unwrap_or(position(m))
            } else {
                position(m)
            };
            let center = horizon_core::geometry::Vec3::new(center[0], center[1], center[2]);
            let range = if alert {
                16.0
            } else {
                self.config.attack_range
            };
            let found = alarm.or_else(|| {
                cx.w.nearby(center, range, |s| valid(s)).find(|s| {
                    state.allows_target([s.position.x, s.position.y, s.position.z], now)
                        && (alert || m.sees(cx.w, Who::Mob(s.id)))
                })
            });
            m.ai.target = found.map(|s| Who::Mob(s.id));
            if state.alert_active(now) && found.is_none() {
                state.alert_until = 0;
                state.alert_target = None;
            }
        }
        let target = m.target_info(cx.w);
        let fighting = target.as_ref().is_some_and(|t| t.alive);
        let return_to = state.return_to(position(m), fighting, now);
        if let Some(to) = return_to {
            let roaming = state.mode == Mode::Roam && !state.alert_active(now);
            if roaming {
                m.ai.target = None;
            }
            drop(friends);
            drop(state);
            self.stop_attack(m, cx);
            if now >= self.move_at {
                m.nav_move_to(cx.w, to, 1.2);
                self.move_at = now + 10;
            }
            return;
        }
        if let Some(target) = target.filter(|t| t.alive) {
            let can_shoot = self.config.unlimited_ammo || state.ammo > 0;
            let melee = state.weapon(m.distance_sqr(target.position).sqrt(), can_shoot);
            drop(friends);
            drop(state);
            let desired = if melee { 2 } else { 1 };
            if desired != self.active {
                self.stop_attack(m, cx);
                let correct = m.extra.main_hand.as_deref().is_some_and(|s| {
                    if melee {
                        s.ends_with("_sword")
                    } else {
                        s == "minecraft:bow"
                    }
                });
                if !correct {
                    let old = m
                        .extra
                        .main_hand_stack()
                        .cloned()
                        .or_else(|| m.extra.main_hand.as_ref().map(|s| ItemStack::new(s, 1)));
                    let mut c = lock(&self.state);
                    let reserved = decode_stack(&c.reserve);
                    let off = reserved
                        .or_else(|| m.extra.off_hand.as_ref().map(|s| ItemStack::new(s, 1)));
                    let encoded = old
                        .as_ref()
                        .map(|s| {
                            horizon_world::nbt::write_network(
                                &horizon_items::item_nbt::item_to_nbt(s),
                                horizon_world::nbt::Limits::default(),
                            )
                        })
                        .transpose();
                    if let Ok(encoded) = encoded {
                        c.reserve = encoded.unwrap_or_default();
                        m.extra.set_main_hand_stack(off);
                        m.extra.off_hand = old.as_ref().map(|s| s.item.clone());
                    }
                }
                self.active = desired;
                if melee {
                    self.melee.start(m, cx);
                } else {
                    self.bow.start(m, cx);
                }
            }
            if melee {
                self.melee.tick(m, cx);
            } else {
                let before = cx.events.len();
                self.bow.tick(m, cx);
                for event in &mut cx.events[before..] {
                    if let horizon_client::mobs::MobEvent::BowShot { arrow, .. } = event {
                        if let horizon_client::projectiles::ProjectileKind::Arrow {
                            base_damage,
                            effect,
                            ..
                        } = &mut arrow.kind
                        {
                            *base_damage *= f64::from(self.config.damage);
                            if m.species() == Species::WitherSkeleton {
                                *effect = Some(("minecraft:wither", 200));
                            }
                        }
                        if !self.config.unlimited_ammo {
                            let mut c = lock(&self.state);
                            if let Some(mut stack) =
                                c.arrows.first().and_then(|bytes| decode_stack(bytes))
                            {
                                if let Some(a) = arrow.arrow.as_mut() {
                                    a.pickup_item = ItemStack {
                                        count: 1,
                                        ..stack.clone()
                                    };
                                    a.spectral = stack.item.as_ref() == "minecraft:spectral_arrow";
                                    a.pickup = horizon_client::arrows::Pickup::Allowed;
                                }
                                stack.count = stack.count.saturating_sub(1);
                                c.ammo = c.ammo.saturating_sub(1);
                                if stack.count == 0 {
                                    c.arrows.remove(0);
                                } else if let Some(bytes) = encode_stack(&stack) {
                                    c.arrows[0] = bytes;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let mode = state.mode;
            let anchor = state.anchor;
            let radius = state.settings.roam;
            drop(friends);
            drop(state);
            self.stop_attack(m, cx);
            if mode == Mode::Roam && now >= self.move_at && m.ai.nav.is_done() {
                self.move_at = now + 40;
                for _ in 0..10 {
                    if let Some(to) = m.default_random_pos(cx.w, 10, 7) {
                        if distance_sq(to, anchor) <= f64::from(radius).powi(2) {
                            m.nav_move_to(cx.w, to, 1.0);
                            break;
                        }
                    }
                }
            } else if mode != Mode::Roam {
                m.ai.nav.stop();
            }
        }
    }
}
impl CompanionGoal {
    fn stop_attack(&mut self, m: &mut Mob, cx: &mut Cx) {
        match self.active {
            1 => self.bow.stop(m, cx),
            2 => self.melee.stop(m, cx),
            _ => {}
        }
        self.active = 0;
    }
}

pub(super) fn use_item(
    g: &mut Game,
    stack: &ItemStack,
    pos: [i32; 3],
    face: [i32; 3],
    claimed: &mut bool,
) {
    if !g.native_on(ID) || g.hand != horizon_client::first_person::Hand::Main {
        return;
    }
    let owner = g.native_acting_player();
    let to = [pos[0] + face[0], pos[1] + face[1], pos[2] + face[2]];
    if stack.item.as_ref() == EGG {
        let cfg = runtime().config.get_or_insert_with(load_config).clone();
        if spawn(
            g,
            SKELETON,
            [to[0] as f64 + 0.5, to[1] as f64, to[2] as f64 + 0.5],
            owner.as_ref().map(|p| p.uuid),
            &cfg,
        )
        .is_some()
        {
            *claimed = true;
            if owner.is_some_and(|p| p.game_mode != 1) {
                g.native_take_items(EGG, 1);
            }
        }
    } else if matches!(
        stack.item.as_ref(),
        "minecraft:jack_o_lantern" | "minecraft:wither_skeleton_skull"
    ) {
        let mut rt = runtime();
        if rt.rituals.len() < 256 {
            rt.rituals.push(Ritual {
                dimension: g.dim.id().into(),
                pos: to,
                owner: owner.map(|p| p.uuid),
            });
        }
    }
}
pub(super) fn interact(g: &mut Game, id: i32, claimed: &mut bool) {
    let Some(player) = g.native_acting_player() else {
        return;
    };
    let mut rt = runtime();
    let Some(record) = rt.bones.get_mut(&(id as u32)) else {
        return;
    };
    let Some(mob) = g.mobs.iter().find(|m| m.id == id as u32 && !m.removed) else {
        return;
    };
    if distance_sq(player.position, position(mob)) > 144.0 {
        return;
    }
    let mut state = lock(&record.state);
    let first = state.owner.is_none();
    if !state.claim(player.uuid) {
        return;
    }
    *claimed = true;
    let packet = menu_packet(id, &record.dimension, &state, "open");
    drop(state);
    drop(rt);
    if first {
        if let Some(m) = g.mobs.iter_mut().find(|m| m.id == id as u32) {
            m.custom_name = Some(format!("{}'s Bones", player.name));
            m.name_visible = true;
            m.needs_sync = true;
        }
    }
    if let Err(e) = g.native_net_send_to_player(player.uuid, CHANNEL, packet.to_string().as_bytes())
    {
        eprintln!("[friendlybones] Menu open: {e}");
    }
}
fn menu_packet(id: i32, dimension: &str, c: &Companion, op: &str) -> Value {
    json!({"schema":1,"op":op,"entity":id,"dimension":dimension,"mode":c.mode.id(),
        "roam":c.settings.roam,"stop":c.settings.follow_stop,"start":c.settings.follow_start,"guard":c.settings.guard})
}
pub(super) fn from_client(g: &mut Game, player: u128, channel: &str, data: &[u8]) {
    if channel != CHANNEL || data.len() > 1024 {
        return;
    }
    let Ok(v) = serde_json::from_slice::<Value>(data) else {
        return;
    };
    if v["schema"] != 1 || v["op"] != "set" {
        return;
    }
    let Some(id) = v["entity"].as_u64().and_then(|n| u32::try_from(n).ok()) else {
        return;
    };
    let Some(mode) = v["mode"].as_u64().and_then(|n| u8::try_from(n).ok()) else {
        return;
    };
    let Some(p) = g
        .native_server_players()
        .into_iter()
        .find(|p| p.uuid == player && p.health > 0.0 && p.game_mode != 3)
    else {
        return;
    };
    if v["dimension"].as_str() != Some(p.dimension.as_str()) {
        return;
    }
    let mut rt = runtime();
    let Some(r) = rt.bones.get_mut(&id) else {
        return;
    };
    if r.dimension != p.dimension {
        return;
    }
    let Some(m) = g.mobs.iter().find(|m| m.id == id && !m.removed) else {
        return;
    };
    let (Some(roam), Some(stop), Some(start), Some(guard), Some(recenter)) = (
        v["roam"].as_i64(),
        v["stop"].as_i64(),
        v["start"].as_i64(),
        v["guard"].as_i64(),
        v["recenter"].as_bool(),
    ) else {
        return;
    };
    let settings = Settings::checked(roam, stop, start, guard);
    let mut c = lock(&r.state);
    let old_mode = c.mode;
    if !c.apply_menu(player, p.position, position(m), mode, settings, recenter) {
        return;
    }
    let data = saved(&c);
    let response = menu_packet(id as i32, &r.dimension, &c, "state");
    let changed_mode = c.mode;
    let changed = old_mode != c.mode;
    let at = position(m);
    drop(c);
    if data != r.last_saved {
        match SAVED.set(g as &mut dyn Host, Holder::Entity(id), data.clone()) {
            Ok(()) => r.last_saved = data,
            Err(e) => eprintln!("[friendlybones] Menu save: {e:?}"),
        }
    }
    let announce = rt.config.as_ref().is_some_and(|c| c.announce);
    drop(rt);
    let _ = g.native_net_send_to_player(player, CHANNEL, response.to_string().as_bytes());
    let key = match changed_mode {
        Mode::Roam => "message.friendlybones.mode.wander",
        Mode::Follow => "message.friendlybones.mode.follow",
        Mode::Guard => "message.friendlybones.mode.guard",
    };
    if announce && changed {
        g.native_tell(player, Text::tr(key, &[]));
    }
    if changed || recenter {
        g.native_send_particles_in_place(
            &horizon_client::particles::ParticleOptions::named("minecraft:happy_villager"),
            None,
            [at[0], at[1] + 1.8, at[2]],
            [0.4, 0.3, 0.4],
            0.02,
            12,
        );
    }
}
pub(super) fn incoming_damage(victim: u32, player: bool, attacker: Option<u32>, amount: &mut f32) {
    let rt = runtime();
    let Some(cfg) = &rt.config else {
        return;
    };
    let bones_attacker = attacker.and_then(|id| rt.bones.get(&id));
    let bones_victim = rt.bones.get(&victim);
    let own = bones_victim.is_some_and(|v| {
        attacker
            .and_then(|id| rt.players.get(&(id as i32)))
            .is_some_and(|u| lock(&v.state).owner == Some(*u))
    });
    *amount = logic::friendly_damage(
        bones_attacker.is_some(),
        bones_victim.is_some(),
        player,
        own,
        *amount,
        cfg.friendly_fire,
    );
}
pub(super) fn damage_done(victim: i32, kind: &str, attacker: i32, amount: f32) {
    if kind == "minecraft:player" && amount > 0.0 {
        let mut rt = runtime();
        if rt.alarms.len() < 256 {
            rt.alarms.push((victim, attacker));
        }
    }
}
pub(super) fn loot(context: &horizon_client::mod_api::NativeLootDrops<'_>, drops: &mut Vec<ItemStack>) {
    let Some((id, kind)) = context.entity else {
        return;
    };
    if !is_bones(kind) || context.used {
        return;
    }
    let rt = runtime();
    let Some(r) = rt.bones.get(&id) else {
        return;
    };
    let c = lock(&r.state);
    // Preserve components when the engine rolls a drop for the stowed weapon.
    if let Some(reserve) = decode_stack(&c.reserve) {
        if let Some(drop) = drops.iter_mut().find(|s| s.item == reserve.item) {
            *drop = reserve;
        }
    }
}

// The UI implementation is appended by the staging tool.

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
