#![allow(clippy::ptr_arg)]
mod file;
mod time;

use std::{fmt, io, panic::Location, process};

pub use chrono::FixedOffset;
use crossterm::{cursor::MoveToColumn, execute};

fn mount_log(tag: &str, message: impl fmt::Display, time_zone: Option<&FixedOffset>) -> String {
    format!(
        "🔹 [{}] {} - {}",
        time::now(time_zone).get_date_and_hour(),
        tag,
        message
    )
}

pub struct Logger {
    write_in_files: bool,
    time_zone: Option<FixedOffset>,
    column_two: u16,
}
impl Logger {
    pub fn new(write_in_files: bool, time_zone: Option<FixedOffset>, column_two: u16) -> Logger {
        Logger {
            write_in_files,
            time_zone,
            column_two,
        }
    }

    fn get_time_zone(&self) -> Option<&FixedOffset> {
        match &self.time_zone {
            Some(time_zone) => Some(time_zone),
            None => None,
        }
    }

    pub fn clear_terminal(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn log(&self, tag: &str, message: impl fmt::Display) {
        let time_zone = self.get_time_zone();
        let log = mount_log(tag, message, time_zone);
        if self.write_in_files {
            file::writeln(&log, time_zone)
        }
        println!("{log}");
    }

    pub fn log_without_ln(&self, tag: &str, message: impl fmt::Display) {
        let time_zone = self.get_time_zone();
        let log = mount_log(tag, message, time_zone);
        if self.write_in_files {
            file::write(&log, time_zone)
        }
        print!("{log}");
    }

    pub fn log_error(&self, tag: &str, description: impl fmt::Display, error: impl fmt::Display) {
        let time_zone = self.get_time_zone();
        let log = format!(
            "\n❌ [{}] {} ERROR:\n❌ {}\n❌ {}\n",
            time::now(time_zone).get_date_and_hour(),
            tag,
            description,
            error,
        );
        eprintln!("\x1b[0;31m{}\x1b[0m", &log);
        if self.write_in_files {
            file::writeln(&log, time_zone)
        }
    }

    pub fn println(&self, content: impl fmt::Display) {
        let content = content.to_string();
        let time_zone = self.get_time_zone();
        println!("{}", content);
        if self.write_in_files {
            file::writeln(&content, time_zone)
        }
    }

    pub fn print(&self, content: impl fmt::Display) {
        let content = content.to_string();
        let time_zone = self.get_time_zone();
        print!("{}", content);
        if self.write_in_files {
            file::write(&content, time_zone)
        }
    }

    pub fn print_fail(&self) {
        let _ = execute!(io::stdout(), MoveToColumn(self.column_two));
        self.println("FAIL ❌");
    }
    pub fn print_ok(&self) {
        let _ = execute!(io::stdout(), MoveToColumn(self.column_two));
        self.println("OK ✅");
    }

    #[track_caller]
    pub fn throw_error(&self, description: impl fmt::Display, error: impl fmt::Display) -> ! {
        let location = Location::caller();
        let time_zone = self.get_time_zone();
        let log = format!(
            "\n❌ [{}] CRITICAL ERROR - {}:\n❌ {}\n❌ {}\n",
            time::now(time_zone).get_date_and_hour(),
            location,
            description,
            error,
        );
        eprintln!("\x1b[0;31m{}\x1b[0m", &log);
        if self.write_in_files {
            file::writeln(&log, time_zone)
        }
        process::exit(1)
    }
}
