#[derive(Clone)]
pub enum FrameType {
    Strike,
    Spare,
    Open
}

#[derive(Clone)]
pub enum ShotCounter {
    First,
    Second,
}

#[derive(Clone)]
pub struct Frame {
    pub first_shot: u16,
    pub second_shot: u16,
    pub frame_type: FrameType,
    pub shot_counter: ShotCounter,
}

impl Frame {
    pub fn new(first_shot: u16) -> Self {
        match first_shot {
            10 => { // If strike
                Frame {
                    first_shot: 10,
                    second_shot: 0,
                    frame_type: FrameType::Strike,
                    shot_counter: ShotCounter::Second,
                }
            },
            _ => { // if either spare or open
                Frame {
                    first_shot,
                    second_shot: 0,
                    frame_type: FrameType::Open,
                    shot_counter: ShotCounter::First,
                }
            }
        }
    }
}