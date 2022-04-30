use std::time::{Duration, Instant};

use arrayvec::ArrayVec;

use crate::{Board, FallingPiece, Game, GameState, Input, Piece, Sound, TetrisEvent};

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Game,
    Clear,
    Roll(Roll),
    End,
}

#[derive(Debug, Clone, Copy)]
pub enum Roll {
    Normal,
    Invisible,
}

#[derive(Debug, Clone)]
pub enum TGM3Event {
    StatusChange(Status),
    GotCool,
    GotRegret,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum TGM3Sound {
    Cool,
    GameClear,
}

#[derive(Debug, Clone)]
pub struct TGM3Master {
    pub inner: Game,
    level: usize,
    speed_level: usize,
    grade_points: usize,
    grade: usize,
    roll_points: usize,
    start_time: Instant,
    section_times: [Option<Duration>; 9],
    cool_line_section_times: [Option<Duration>; 9],
    cools: [Option<bool>; 9],
    regrets: [Option<bool>; 9],
    status: Status,
    start_roll_timer: Option<usize>,
    roll_timer: Option<usize>,
    envets: Vec<TGM3Event>,
    sounds: Vec<TGM3Sound>,
    opacity_timers: ArrayVec<ArrayVec<Option<usize>, 10>, 40>,
}

impl TGM3Master {
    pub fn new() -> Self {
        let mut opacity_timers = ArrayVec::new();
        for _ in 0..40 {
            opacity_timers.push(ArrayVec::from([None; 10]));
        }
        let mut me = Self {
            inner: Game::new(),
            level: 0,
            speed_level: 0,
            grade_points: 0,
            grade: 0,
            roll_points: 0,
            start_time: Instant::now(),
            section_times: [None; 9],
            cool_line_section_times: [None; 9],
            cools: [None; 9],
            regrets: [None; 9],
            status: Status::Game,
            start_roll_timer: None,
            roll_timer: None,
            envets: vec![],
            sounds: vec![],
            opacity_timers,
        };
        me.sync_settings();
        me
    }

    pub fn get_level(&self) -> usize {
        self.level
    }

    fn get_are(&self) -> usize {
        match self.speed_level {
            0..=699 => 27,
            700..=799 => 18,
            800..=999 => 14,
            1000..=1099 => 8,
            1100..=1199 => 7,
            _ => 6,
        }
    }

    fn get_line_are(&self) -> usize {
        match self.speed_level {
            0..=599 => 27,
            600..=699 => 18,
            700..=799 => 14,
            800..=1099 => 8,
            1100..=1199 => 7,
            _ => 6,
        }
    }

    fn get_das(&self) -> usize {
        match self.speed_level {
            0..=499 => 15,
            0..=899 => 9,
            _ => 6,
        }
    }

    fn get_line_clear_delay(&self) -> usize {
        match self.speed_level {
            0..=499 => 40,
            500..=599 => 25,
            600..=699 => 16,
            700..=799 => 12,
            800..=1099 => 6,
            1100..=1199 => 5,
            _ => 4,
        }
    }

    fn get_lock_delay(&self) -> usize {
        match self.speed_level {
            0..=899 => 30,
            900..=1099 => 17,
            _ => 15,
        }
    }

    fn get_gravity(&self) -> f64 {
        match self.speed_level {
            0..=29 => 4.0 / 256.0,
            30..=34 => 6.0 / 256.0,
            35..=39 => 8.0 / 256.0,
            40..=49 => 10.0 / 256.0,
            50..=59 => 12.0 / 256.0,
            60..=69 => 16.0 / 256.0,
            70..=79 => 32.0 / 256.0,
            80..=89 => 48.0 / 256.0,
            90..=99 => 64.0 / 256.0,
            100..=119 => 80.0 / 256.0,
            120..=139 => 96.0 / 256.0,
            140..=159 => 112.0 / 256.0,
            160..=169 => 128.0 / 256.0,
            170..=199 => 144.0 / 256.0,
            200..=219 => 4.0 / 256.0,
            220..=229 => 32.0 / 256.0,
            230..=232 => 64.0 / 256.0,
            233..=235 => 96.0 / 256.0,
            236..=238 => 128.0 / 256.0,
            239..=242 => 160.0 / 256.0,
            243..=246 => 192.0 / 256.0,
            247..=250 => 224.0 / 256.0,
            251..=299 => 1.0,
            300..=329 => 2.0,
            330..=359 => 3.0,
            360..=399 => 4.0,
            400..=419 => 5.0,
            420..=449 => 4.0,
            450..=499 => 3.0,
            _ => 20.0,
        }
    }

    fn cool_border(&self, rank: usize) -> Duration {
        let set = match self.level / 100 {
            0 => Duration::from_secs_f32(52.0),
            1 => Duration::from_secs_f32(52.0),
            2 => Duration::from_secs_f32(49.0),
            3 => Duration::from_secs_f32(45.0),
            4 => Duration::from_secs_f32(45.0),
            5 => Duration::from_secs_f32(42.0),
            6 => Duration::from_secs_f32(42.0),
            7 => Duration::from_secs_f32(38.0),
            8 => Duration::from_secs_f32(38.0),
            _ => unreachable!(),
        };
        if rank > 0 {
            let prev_rank = rank - 1;
            if let Some(prev) = self.cool_line_section_times[prev_rank] {
                let player_border = prev + Duration::from_secs(2);
                if player_border < set {
                    return player_border;
                }
            }
        }
        set
    }

    fn current_rank(&self) -> usize {
        self.level / 100
    }

    fn prev_rank(&self) -> Option<usize> {
        if self.level < 100 {
            return None;
        }
        Some(self.level / 100 - 1)
    }

    fn regret_border(&self, rank: usize) -> Duration {
        match rank {
            0 => Duration::from_secs_f32(90.0),
            1 => Duration::from_secs_f32(75.0),
            2 => Duration::from_secs_f32(75.0),
            3 => Duration::from_secs_f32(68.0),
            4 => Duration::from_secs_f32(60.0),
            5 => Duration::from_secs_f32(60.0),
            6 => Duration::from_secs_f32(50.0),
            7 => Duration::from_secs_f32(50.0),
            8 => Duration::from_secs_f32(50.0),
            9 => Duration::from_secs_f32(50.0),
            _ => unreachable!(),
        }
    }

    fn get_grade_point_bonus(&self, n: usize) -> usize {
        [
            [10, 20, 40, 50],
            [10, 20, 30, 40],
            [10, 20, 30, 40],
            [10, 15, 30, 40],
            [10, 15, 20, 40],
            [5, 15, 20, 30],
            [5, 10, 20, 30],
            [5, 10, 15, 30],
            [5, 10, 15, 30],
            [5, 10, 15, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
            [2, 12, 13, 30],
        ][self.grade][n - 1]
    }

    fn get_roll_points(&self, n: usize) -> usize {
        [4, 8, 12, 26][n - 1]
    }
    fn get_mroll_points(&self, n: usize) -> usize {
        [10, 20, 30, 100][n - 1]
    }

    fn sync_settings(&mut self) {
        self.inner.set_gravity(self.get_gravity());
        self.inner.set_are(self.get_are());
        self.inner.set_line_are(self.get_line_are());
        self.inner.set_das(self.get_das());
        self.inner.set_lock_delay(self.get_lock_delay());
        self.inner.set_line_clear_delay(self.get_line_clear_delay());
    }

    fn level_up(&mut self, up: usize, line_clear: bool) {
        let rank = self.current_rank();
        let prev = self.level;

        if line_clear || (prev + up) % 100 > prev % 100 && prev + up < 998 {
            self.level += up;
            self.speed_level += up;
        }

        if self.level % 100 >= 70 && rank < 9 && self.cool_line_section_times[rank].is_none() {
            self.cool_line_section_times[rank] = Some(self.current_section_time());
        }

        if self.level % 100 >= 80 && rank < 9 && self.cools[rank].is_none() {
            if let Some(current_cool_section_time) = self.cool_line_section_times[rank] {
                let cool = self.cool_border(rank) > current_cool_section_time;
                self.cools[rank] = Some(cool);
                println!("cool: {cool}");
                if cool {
                    self.envets.push(TGM3Event::GotCool);
                    self.sounds.push(TGM3Sound::Cool);
                }
            }
        }

        if line_clear {
            if prev % 100 > self.level % 100 || self.level > 998 {
                if self.level > 999 {
                    self.level = 999;
                }
                self.rank_up();
            }
        }
    }

    fn section_time_total(&self) -> Duration {
        self.section_times
            .into_iter()
            .filter_map(|t| t)
            .fold(Duration::from_secs(0), |sum, x| sum + x)
    }

    fn current_section_time(&self) -> Duration {
        Instant::now() - self.start_time - self.section_time_total()
    }

    fn rank_up(&mut self) {
        let section_time = self.current_section_time();
        if self.level == 999 {
            self.status = Status::Clear;
            self.envets.push(TGM3Event::StatusChange(Status::Clear));
            self.sounds.push(TGM3Sound::GameClear);
            self.section_times[8] = Some(section_time);
            let regret = self.regret_border(8) < section_time;
            self.regrets[8] = Some(regret);
            if regret {
                self.envets.push(TGM3Event::GotRegret);
            }
        } else {
            if let Some(prev_rank) = self.prev_rank() {
                self.section_times[prev_rank] = Some(section_time);
                let regret = self.regret_border(prev_rank) < section_time;
                self.regrets[prev_rank] = Some(regret);
                if !regret {
                    if let Some(cool) = self.cools[prev_rank] {
                        if cool {
                            self.speed_level += 100;
                        }
                    }
                }
            }
            self.inner.get_sound_queue().push(Sound::RankUp);
        }
    }

    fn game_line_clear(&mut self, n: usize) {
        self.grade_points += self.get_grade_point_bonus(n) * 1 /* combo */ * (self.level / 250 + 1);
        if self.grade < 31 && self.grade_points >= 100 {
            self.grade_points = 0;
            self.grade += 1;
        }
        let up = match n {
            3 => 4,
            4 => 6,
            _ => n,
        };
        self.level_up(up, true);
    }

    fn roll_line_clear(&mut self, n: usize, roll: Roll) {
        self.roll_points += match roll {
            Roll::Normal => self.get_roll_points(n),
            Roll::Invisible => self.get_mroll_points(n),
        }
    }

    fn game_update(&mut self) {
        self.inner.update();
        let events = self.inner.get_event_queue().clone();
        for e in events.iter() {
            match e {
                TetrisEvent::LineCleared(n) => {
                    self.game_line_clear(*n);
                }
                TetrisEvent::PieceSpawned(_) => {
                    self.level_up(1, false);
                }
                _ => {}
            }
        }
        self.sync_settings();
    }

    fn is_all_cool(&mut self) -> bool {
        self.cools.into_iter().all(|c| c.unwrap_or(false))
    }

    fn set_opacity_timer(&mut self, piece: &FallingPiece, time: usize) {
        let (x, y) = piece.piece_position;
        for (rel_x, rel_y) in piece.piece_state.get_cells().into_iter() {
            if let Some(timers_x) = self.opacity_timers.get_mut((-rel_y + y as i16) as usize) {
                if let Some(timer) = timers_x.get_mut((rel_x + x as i16) as usize) {
                    *timer = Some(time);
                }
            }
        }
    }

    fn roll_update(&mut self, roll: Roll) {
        self.inner.update();
        for (y, timers_x) in self.opacity_timers.iter_mut().enumerate() {
            for (x, timer) in timers_x.into_iter().enumerate() {
                let original_cells = self.inner.get_board().cells;
                let valid = original_cells
                    .get(y)
                    .map_or(false, |timers_x| timers_x.get(x).is_some());
                if !valid {
                    *timer = None;
                    continue;
                }
                if let Some(timer) = timer.as_mut() {
                    if *timer > 0 {
                        *timer -= 1;
                    }
                }
            }
        }

        let events = self.inner.get_event_queue().clone();
        for e in events.iter() {
            match e {
                TetrisEvent::LineCleared(n) => {
                    self.roll_line_clear(*n, roll);
                }
                TetrisEvent::PieceLocked(p) => {
                    let time = match roll {
                        Roll::Normal => 60 * 5,
                        Roll::Invisible => 4,
                    };
                    self.set_opacity_timer(p, time);
                }
                TetrisEvent::LineShrinked(shrinked) => {
                    for y in shrinked.iter().rev() {
                        self.opacity_timers.remove(*y);
                    }
                    for _ in 0..shrinked.len() {
                        self.opacity_timers.insert(0, ArrayVec::from([None; 10]));
                    }
                }
                _ => {}
            }
        }
    }

    pub fn get_tgm3events(&mut self) -> &mut Vec<TGM3Event> {
        self.envets.as_mut()
    }

    pub fn get_tgm3sounds(&mut self) -> &mut Vec<TGM3Sound> {
        self.sounds.as_mut()
    }

    pub fn get_opacity_timers(&self) -> ArrayVec<ArrayVec<Option<usize>, 10>, 40> {
        self.opacity_timers.clone()
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn get_aggregate_grade(&self) -> usize {
        self.cools
            .into_iter()
            .filter_map(|t| t)
            .fold(0, |sum, c| sum + if c { 1 } else { 0 })
            + self.roll_points / 100
            + [
                0, 1, 2, 3, 4, 5, 5, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 11, 12, 12, 12, 13, 13,
                14, 14, 15, 15, 16, 16, 17,
            ][self.grade]
            - self
                .regrets
                .into_iter()
                .filter_map(|t| t)
                .fold(0, |sum, c| sum + if c { 1 } else { 0 })
    }
}

impl GameState for TGM3Master {
    fn update(&mut self) {
        match self.status {
            Status::Game => self.game_update(),
            Status::Clear => {
                if let None = self.start_roll_timer {
                    self.start_roll_timer = Some(150)
                }
                if let Some(timer) = self.start_roll_timer.as_mut() {
                    if *timer == 0 {
                        self.inner.clear_board();
                        // self.status = Status::Roll(if true {
                        self.status = Status::Roll(if self.is_all_cool() {
                            Roll::Invisible
                        } else {
                            Roll::Normal
                        });
                    } else {
                        *timer -= 1;
                    }
                }
            }
            Status::Roll(roll) => {
                self.roll_update(roll);
                if let None = self.roll_timer {
                    self.roll_timer = Some(3238);
                }
                if let Some(timer) = self.roll_timer.as_mut() {
                    if *timer == 0 {
                        self.status = Status::End;
                        return;
                    }
                    *timer -= 1;
                }
            }
            _ => {}
        }
    }

    fn get_board(&self) -> Board {
        self.inner.get_board()
    }

    fn get_current_piece(&self) -> Option<FallingPiece> {
        self.inner.get_current_piece()
    }

    fn get_locked_piece(&self) -> Option<FallingPiece> {
        self.inner.get_locked_piece()
    }

    fn get_hold(&self) -> Option<Piece> {
        self.inner.get_hold()
    }

    fn get_next(&self) -> Piece {
        self.inner.get_next()
    }

    fn get_next_next(&self) -> Piece {
        self.inner.get_next_next()
    }

    fn get_next_next_next(&self) -> Piece {
        self.inner.get_next_next_next()
    }

    fn get_sound_queue(&mut self) -> &mut Vec<Sound> {
        self.inner.get_sound_queue()
    }

    fn get_event_queue(&mut self) -> &mut Vec<TetrisEvent> {
        self.inner.get_event_queue()
    }

    fn set_input(&mut self, input: Input) {
        self.inner.set_input(input)
    }
}
