use time::{Duration, PrimitiveDateTime as DateTime};

const ONE_PICOSECOND: u32 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    match start.checked_add(Duration::seconds(ONE_PICOSECOND as i64)) {
        Some(time) => time,
        None => start,
    }
}
