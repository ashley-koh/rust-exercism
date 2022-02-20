use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut final_minutes = minutes % 60;
        let mut final_hours = (hours + ( minutes / 60 )) % 24;

        if final_minutes < 0 { 
            final_minutes = 60 + final_minutes;
            final_hours -= 1;
        }
        
        if final_hours < 0 { final_hours = 24 + final_hours; }

        Clock { hours: final_hours, minutes: final_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut final_minutes = (self.minutes + minutes) % 60;
        let mut final_hours = (self.hours + (self.minutes + minutes) / 60) % 24;

        if final_minutes < 0 { 
            final_minutes = 60 + final_minutes;
            final_hours -= 1;
        }
        
        if final_hours < 0 { final_hours = 24 + final_hours; }

        Clock { hours: final_hours, minutes: final_minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Eq for Clock {}