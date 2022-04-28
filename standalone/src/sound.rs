pub fn init() {
    use tetris::Sound::*;
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
}
