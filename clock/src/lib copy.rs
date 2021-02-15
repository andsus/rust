use std::fmt;

#[derive(PartialEq,Debug)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock::calc_clock(hours, minutes);

           /*
        let tot_min = hours * 60 + minutes;
        let mut hrs = if tot_min < 0 { 23 + ((tot_min / 60) % 24)  } else { (tot_min / 60) % 24};
        let mut mins = if tot_min < 0 { 60 + (tot_min % 60)} else { tot_min % 60};
        if mins == 60 {  // special case for Clock(22,60)
            hrs += 1;
            mins = 0;
        }

        Self {
            hours: hrs,
            minutes: mins ,
        }
        */

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock::calc_clock(self.hours, self.minutes + minutes);
       
    }

    pub fn calc_clock( hours: i32, minutes: i32) -> Clock {
        let total_min = hours * 60 + minutes;
        let mut hrs = if total_min >= 0 {(total_min / 60) % 24} else { 23 + ((total_min / 60) % 24) } ;
        let mut mins = if total_min >= 0 { total_min  % 60 } else { 60 + (total_min % 60)};
        if mins == 60 { 
            hrs += 1 ;
            mins = 0;
        } 

        Clock {
            hours: hrs,
            minutes: mins,
        }

     }
}
// bound fmt::Display to Clock
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

trait ClockTrait {
    fn new(hours: i32, minutes: i32) -> Self;
    fn add_minutes(&self, minutes: i32) -> Self;
}
