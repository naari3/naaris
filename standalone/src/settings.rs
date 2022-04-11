use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub game: GameSetting,
    pub key: KeyConfig,
}

#[derive(Deserialize)]
pub struct GameSetting {
    pub gravity: f64,
    pub are: usize,
    pub line_are: usize,
    pub das: usize,
    pub lock_delay: usize,
    pub line_clear_delay: usize,
}

#[derive(Deserialize)]
pub struct KeyConfig {
    pub left: usize,
    pub right: usize,
    pub soft_drop: usize,
    pub hard_drop: usize,
    pub cw: usize,
    pub ccw: usize,
    pub hold: usize,
    pub restart: usize,
}
