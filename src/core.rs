//! Engine-independent companion decisions. The native adapter uses this same file.

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
