use tetris::{Sound, TGM3Sound};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum StandaloneSound {
    Bottom,
    Hold,
    Lock,
    Erase,
    Fall,
    PieceI,
    PieceO,
    PieceT,
    PieceL,
    PieceJ,
    PieceS,
    PieceZ,
    RankUp,
    Cool,
}

impl From<Sound> for StandaloneSound {
    fn from(s: Sound) -> Self {
        use StandaloneSound::*;
        match s {
            Sound::Bottom => Bottom,
            Sound::Hold => Hold,
            Sound::Lock => Lock,
            Sound::Erase => Erase,
            Sound::Fall => Fall,
            Sound::PieceI => PieceI,
            Sound::PieceO => PieceO,
            Sound::PieceT => PieceT,
            Sound::PieceL => PieceL,
            Sound::PieceJ => PieceJ,
            Sound::PieceS => PieceS,
            Sound::PieceZ => PieceZ,
            Sound::RankUp => RankUp,
        }
    }
}

impl From<TGM3Sound> for StandaloneSound {
    fn from(s: TGM3Sound) -> Self {
        use StandaloneSound::*;
        match s {
            TGM3Sound::Cool => Cool,
        }
    }
}

pub fn init() {
    use StandaloneSound::*;
    music::bind_sound_file(Bottom, "./assets/bottom.wav");
    music::bind_sound_file(Hold, "./assets/hold.wav");
    music::bind_sound_file(Lock, "./assets/lock.wav");
    music::bind_sound_file(Erase, "./assets/erase.wav");
    music::bind_sound_file(Fall, "./assets/fall.wav");
    music::bind_sound_file(PieceI, "./assets/piece_i.wav");
    music::bind_sound_file(PieceO, "./assets/piece_o.wav");
    music::bind_sound_file(PieceT, "./assets/piece_t.wav");
    music::bind_sound_file(PieceL, "./assets/piece_l.wav");
    music::bind_sound_file(PieceJ, "./assets/piece_j.wav");
    music::bind_sound_file(PieceS, "./assets/piece_s.wav");
    music::bind_sound_file(PieceZ, "./assets/piece_z.wav");
    music::bind_sound_file(RankUp, "./assets/rank_up.wav");
    music::bind_sound_file(Cool, "./assets/cool.wav");
}
