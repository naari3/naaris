use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
pub struct Settings {
    pub mode: GameMode,
    pub game: GameSetting,
    pub key: KeyConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum GameMode {
    Free,
    TGM3Master,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Free
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
pub struct GameSetting {
    pub gravity: f64,
    pub are: usize,
    pub line_are: usize,
    pub das: usize,
    pub lock_delay: usize,
    pub line_clear_delay: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
pub struct KeyConfig {
    pub left: usize,
    pub right: usize,
    pub soft_drop: usize,
    pub hard_drop: usize,
    pub cw: usize,
    pub ccw: usize,
    pub hold: usize,
    pub restart: usize,
    pub pause: usize,
}
