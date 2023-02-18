use std::slice::Iter;

const MAX_PINS: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Status {
    None,
    Strike,
    Spare(u16, u16),
    Open(u16, u16),
}

pub struct BowlingGame {
    left_pins: u16,
    is_first_throw: bool,
    frames: [Status; 10],
    nth: usize,
    bonus_throws: Vec<u16>,
    bonus_count: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            left_pins: MAX_PINS,
            is_first_throw: true,
            frames: [Status::None; 10],
            nth: 0,
            bonus_throws: vec![],
            bonus_count: 0,
        }
    }

    fn is_game_complete(&self) -> bool {
        let all_frame_complete = self.frames.iter().all(|frame| frame != &Status::None);

        all_frame_complete && self.bonus_count == 0
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_first_throw {
            self.left_pins = MAX_PINS;
        }

        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        if pins > self.left_pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.bonus_count > 0 {
            self.bonus_throws.push(pins);
            self.bonus_count -= 1;
            if pins != MAX_PINS {
                self.left_pins = MAX_PINS - pins;
                self.is_first_throw = false
            }
            return Ok(());
        }

        if self.is_first_throw {
            if pins == MAX_PINS {
                self.frames[self.nth] = Status::Strike;
                if self.nth == 9 {
                    self.bonus_count = 2;
                }
                self.nth += 1;
            } else {
                self.left_pins -= pins;
                self.is_first_throw = false;
            }
        } else {
            let first_throw_pins = MAX_PINS - self.left_pins;
            if first_throw_pins + pins == MAX_PINS {
                self.frames[self.nth] = Status::Spare(first_throw_pins, pins);
                if self.nth == 9 {
                    self.bonus_count = 1
                }
            } else {
                self.frames[self.nth] = Status::Open(first_throw_pins, pins);
            }
            self.nth += 1;
            self.is_first_throw = true;
        }

        return Ok(());
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            return None;
        }

        println!("{:#?}", self.frames);

        let mut score = 0;
        let mut iter = self.frames.iter();
        while let Some(status) = iter.next() {
            match status {
                Status::Strike => {
                    score += 10;
                    match (peak(&iter, 1), peak(&iter, 2)) {
                        (Some(Status::Strike), Some(Status::Strike)) => score += 20,
                        (Some(Status::Strike), Some(Status::Spare(first, _))) => {
                            score += 10 + first
                        }
                        (Some(Status::Strike), Some(Status::Open(first, _))) => score += 10 + first,
                        (Some(Status::Strike), None) => score += 10 + self.bonus_throws[0],
                        (Some(Status::Spare(_, _)), _) => score += 10,
                        (Some(Status::Open(first, second)), _) => score += first + second,
                        _ => {}
                    }
                }
                Status::Spare(_, _) => {
                    score += 10;
                    match peak(&iter, 1) {
                        Some(Status::Strike) => score += 10,
                        Some(Status::Spare(first, _)) => score += first,
                        Some(Status::Open(first, _)) => score += first,
                        _ => {}
                    }
                }
                Status::Open(first, second) => score += first + second,
                _ => {}
            }
            println!("{}", score);
        }

        self.bonus_throws.iter().for_each(|bonus| score += bonus);

        Some(score)
    }
}

fn peak<'a>(iter: &'a Iter<Status>, mut nth: u16) -> Option<&'a Status> {
    let mut iter = iter.clone();
    let mut result = None;
    while nth > 0 {
        nth -= 1;
        result = iter.next();
    }

    result
}
