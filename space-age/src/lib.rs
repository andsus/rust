// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    secs: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { secs: s} //s, measured in seconds
    }
}

pub trait Planet {
    const EARTH_SECONDS_ANNUAL_PERIOD: f64 = 31557600.0;
    const PLANET_PERIOD_FACTOR: f64;

    fn years_during(d: &Duration) -> f64 {
        //"convert a duration ({:?}) to the number of years on this planet for that duration", d
        d.secs as f64/ (Self::EARTH_SECONDS_ANNUAL_PERIOD  * 
            Self::PLANET_PERIOD_FACTOR)
    }
}
/*
pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury { const PLANET_PERIOD_FACTOR: f64 = 0.2408467; }
impl Planet for Venus { const PLANET_PERIOD_FACTOR: f64 = 0.61519726; }
impl Planet for Earth { const PLANET_PERIOD_FACTOR: f64 = 1.0; }
impl Planet for Mars { const PLANET_PERIOD_FACTOR: f64 = 1.8808158; }
impl Planet for Jupiter { const PLANET_PERIOD_FACTOR: f64 = 11.862615; }
impl Planet for Saturn {const PLANET_PERIOD_FACTOR: f64 = 29.447498; }
impl Planet for Uranus {const PLANET_PERIOD_FACTOR: f64 = 84.016846; }
impl Planet for Neptune { const PLANET_PERIOD_FACTOR: f64 = 164.79132; }
*/
macro_rules! planet {
    ($struct_planet_name:ident, $planet_period_factor:expr) => {
        pub struct $struct_planet_name;
        impl Planet for $struct_planet_name {
            const PLANET_PERIOD_FACTOR: f64 = $planet_period_factor;
        }
        
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);