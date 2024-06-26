/*！

本模块利用 log crate 为你提供了日志功能，使用方式见 main.rs.

*/

use log::{ Level, LevelFilter, Log, Metadata, Record};

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };

        println!("\u{1B}[{}m", color);
        println!("[{}] {}", record.level(), record.args());
        println!("\u{1B}[0m")

        // println!(
        //     "\u{1B}[{}m[{}] {}\u{1B}[0m",
        //     color,
        //     record.level(),
        //     record.args(),
        // );
    }
    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Trace,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,//不会记录任何日志
    });
}
