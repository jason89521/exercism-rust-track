// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const SECONDS_PER_YEAR: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let s = s as f64;
        Self {
            earth_years: s / SECONDS_PER_YEAR,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! create_planet {
    ($name: ident, $a: expr) => {
        pub struct $name;
        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                d.earth_years / $a
            }
        }
    };
}

create_planet!(Mercury, 0.2408467);
create_planet!(Venus, 0.61519726);
create_planet!(Earth, 1.0);
create_planet!(Mars, 1.8808158);
create_planet!(Jupiter, 11.862615);
create_planet!(Saturn, 29.447498);
create_planet!(Uranus, 84.016846);
create_planet!(Neptune, 164.79132);
