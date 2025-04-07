#[cfg(windows)]
extern crate atty;
extern crate chrono;
#[cfg(feature = "colored")]
extern crate colored;
extern crate log;
#[cfg(windows)]
extern crate winapi;

use chrono::Local;
#[cfg(feature = "colored")]
use colored::*;
use log::{Level, Log, Metadata, Record, SetLoggerError};
struct SimpleLogger {
    level: Level,
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let target = if !record.target().is_empty() {
            record.target()
        } else {
            record.module_path().unwrap_or_default()
        };
        if target.contains("unimap") && self.enabled(record.metadata()) {
            let level_string = {
                #[cfg(feature = "colored")]
                {
                    match record.level() {
                        Level::Error => record.level().to_string().red(),
                        Level::Warn => record.level().to_string().yellow(),
                        Level::Info => record.level().to_string().cyan(),
                        Level::Debug => record.level().to_string().purple(),
                        Level::Trace => record.level().to_string().normal(),
                    }
                }
                #[cfg(not(feature = "colored"))]
                {
                    record.level().to_string()
                }
            };
            {
                print!(
                    "\n{} [{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                    level_string,
                    record.args()
                );
            }
        }
    }

    fn flush(&self) {}
}

#[cfg(windows)]
fn set_up_color_terminal() {
    use atty::Stream;

    if atty::is(Stream::Stdout) {
        unsafe {
            use winapi::um::consoleapi::*;
            use winapi::um::handleapi::*;
            use winapi::um::processenv::*;
            use winapi::um::winbase::*;
            use winapi::um::wincon::*;

            let stdout = GetStdHandle(STD_OUTPUT_HANDLE);

            if stdout == INVALID_HANDLE_VALUE {
                return;
            }

            let mut mode: winapi::shared::minwindef::DWORD = 0;

            if GetConsoleMode(stdout, &mut mode) == 0 {
                return;
            }

            SetConsoleMode(stdout, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    #[cfg(all(windows, feature = "colored"))]
    set_up_color_terminal();

    let logger = SimpleLogger { level };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}

pub fn init_by_env() {
    match std::env::var("UNIMAP_LOG_LEVEL") {
        Ok(x) => match x.to_lowercase().as_str() {
            "trace" => init_with_level(log::Level::Trace).unwrap(),
            "debug" => init_with_level(log::Level::Debug).unwrap(),
            "info" => init_with_level(log::Level::Info).unwrap(),
            "warn" => init_with_level(log::Level::Warn).unwrap(),
            _ => init_with_level(log::Level::Error).unwrap(),
        },
        _ => init_with_level(log::Level::Error).unwrap(),
    }
}
