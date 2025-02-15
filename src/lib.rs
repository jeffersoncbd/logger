use chrono::{FixedOffset, Utc};

pub fn log(file: &str, message: &str) -> () {
    let time_zone = FixedOffset::west_opt(3 * 60 * 60).expect("Falha ao tentar criar timezone");
    let now = Utc::now().with_timezone(&time_zone).format("%Y-%m-%d %H:%M:%S");
    println!("[{}] - {}: {}", now, file, message)
}
