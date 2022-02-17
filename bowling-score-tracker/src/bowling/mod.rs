// #[allow(dead_code)]
mod frame;
use frame::Frame;
use frame::FrameType;
use frame::ShotCounter;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    pub current_score: u16,
    is_completed: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        let frames: Vec<Frame> = vec![];

        BowlingGame {
            frames,
            current_score: 0,
            is_completed: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_completed { 
            return Err(Error::GameComplete) 
        } else if pins <= 10 {
            let length = self.frames.len();
            if length <= 12 {
                let mut latest_frame = self.frames.last_mut();

                match latest_frame {
                    Some(frame) => match frame.shot_counter {
                        ShotCounter::First => { // If second shot
                            let pinfall = frame.first_shot + pins;
                            if pinfall <= 10 {
                                frame.second_shot = pins;
                                frame.shot_counter = ShotCounter::Second;

                                if pinfall < 10 { // If open
                                    frame.frame_type = FrameType::Open;
                                    if length == 10 || length == 11 {
                                        self.is_completed = true;
                                    }
                                }
                                if pinfall == 10 { // if spare
                                    frame.frame_type = FrameType::Spare;
                                    if length == 11 {
                                        self.is_completed = true;
                                    }
                                }
                                
                                if length > 1 {
                                    // If previous frame was a Strike
                                    if let FrameType::Strike = self.frames[length-2].frame_type {
                                        self.current_score += pins;
                                    }
                                }

                                if length <= 10 { self.current_score += pins; }
                                return Ok(());
                            } else { return Err(Error::NotEnoughPinsLeft); }
                        },
                        ShotCounter::Second => { // If first shot

                            // If previous frame was a Strike/Spare
                            match frame.frame_type { 
                                FrameType::Open => {},
                                FrameType::Spare => {
                                    self.current_score += pins;
                                    if length == 10 {
                                        self.is_completed = true;
                                    }
                                },
                                FrameType::Strike => {
                                    if length == 11 { 
                                        self.is_completed = true;
                                    } else { 
                                        self.current_score += pins;
                                    }
                                }

                            };

                            if length > 1 {
                                // If there was a strike 2 frames ago
                                if let FrameType::Strike = self.frames[length-2].frame_type { 
                                    self.current_score += pins;
                                }
                            }

                            if length < 10 { self.current_score += pins; }
                            self.frames.push(Frame::new(pins));
                            return Ok(());
                        },
                    },
                    None => {
                        self.current_score += pins;
                        self.frames.push(Frame::new(pins));
                        return Ok(());
                    },
                }
            } else { Err(Error::GameComplete) }
        } else { Err(Error::NotEnoughPinsLeft) }
    }

    pub fn score(&self) -> Option<u16> {
        match self.is_completed {
            true => Some(self.current_score),
            false => None
        }
    }
}
