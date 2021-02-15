use std::fmt;

#[derive(PartialEq,Debug)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
    _rem: (), // prevent struct literal creation
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock::compute_clock(hours, minutes);
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock::compute_clock(self.hours, self.minutes + minutes);
       
    }


     pub fn compute_clock( hours: i32, minutes: i32) -> Clock {
        let total_min = (hours * 60 + minutes).rem_euclid(24 * 60);
        let hrs = (total_min / 60).rem_euclid(24) ;
        let mins = total_min.rem_euclid(60);
        Clock {
            hours: hrs,
            minutes: mins,
            _rem: ()
        }

     }
}
// bound fmt::Display to Clock
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

