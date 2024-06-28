use crate::sbi::console_putchar;
use alloc::{boxed::Box, format};
use core::{
    fmt::{self, Write},
    str,
};
use lazy_static::lazy_static;
use log::{Level, LevelFilter, Log};

struct Stdout;
const LEVEL: Option<&str> = option_env!("LOG");

// #[derive(PartialEq, PartialOrd)]
// pub enum LogLevel {
//     SILENT = 0,
//     ERROR = 1,
//     WARN = 2,
//     INFO = 3,
//     DEBUG = 4,
//     TRACE = 5,
// }
// unsafe读取环境变量实现, stupid
// pub struct Settings {
//     level: Option<LogLevel>,
// }
// impl Settings {
//     pub fn get_level(&mut self) -> &LogLevel {
//         if let None = self.level {
//             let log_level = match LEVEL {
//                 Some(s) => match s {
//                     "SILENT" => LogLevel::SILENT,
//                     "ERROR" => LogLevel::ERROR,
//                     "WARN" => LogLevel::WARN,
//                     "INFO" => LogLevel::INFO,
//                     "DEBUG" => LogLevel::DEBUG,
//                     "TRACE" => LogLevel::TRACE,
//                     _ => LogLevel::INFO,
//                 },
//                 None => LogLevel::INFO,
//             };
//             self.level = Some(log_level);
//         }
//         self.level.as_ref().unwrap()
//     }
// }
// static mut SETTINGS: Settings = Settings { level: None };
// pub fn print_by_level(args: fmt::Arguments, level: LogLevel) {
//     unsafe {
//         let cur_level = SETTINGS.get_level();
//         if level > *cur_level {
//             return;
//         }
//     }
//     print(args);
// }

const LOG: LevelFilter = match LEVEL {
    Some(s) => {
        let bytes = s.as_bytes();
        match bytes {
            b"SILENT" => LevelFilter::Off,
            b"ERROR" => LevelFilter::Error,
            b"WARN" => LevelFilter::Warn,
            b"INFO" => LevelFilter::Info,
            b"DEBUG" => LevelFilter::Debug,
            b"TRACE" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        }
    }
    None => LevelFilter::Info,
};

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

// pub fn print_by_level(args: fmt::Arguments, level: LogLevel) {
//     if level > LOG {
//         return;
//     }
//     print(args);
// }

#[macro_export]
macro_rules! print {
  // + 代表该宏表达式可重复1次或多次
  // ? 代表该宏表达式是可选的，可出现零次或一次
  // literal 代表该宏参数匹配一个字面量表达式
  // tt 代表 tokentree，即可以是多个由括号包裹的或者单个关键字等 https://doc.rust-lang.org/reference/tokens.html
    ($fmt: literal $(, $($arg: tt)+)?) => {
      $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    };
}

#[macro_export]
macro_rules! println {
  ($fmt: literal $(, $($arg: tt)+)?) => {
    $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
  };
}
pub struct Logger;

pub fn init() {
    Logger::new().init();
}

impl Logger {
    fn new() -> &'static Self {
        &Self
    }
    fn init(&'static self) {
        log::set_max_level(LevelFilter::Trace);
        log::set_logger(self).unwrap();
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        &metadata.level().to_level_filter() <= &LOG
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level_string = match record.level() {
                Level::Error => format!("\x1b[31m[{}]", record.level().as_str()),
                Level::Warn => format!("\x1b[93m[{}]", record.level().as_str()),
                Level::Info => format!("\x1b[34m[{}]", record.level().as_str()),
                Level::Debug => format!("\x1b[32m[{}]", record.level().as_str()),
                Level::Trace => format!("\x1b[90m[{}]", record.level().as_str()),
            };
            let message = format!("{}[0] {}\x1b[0m", level_string, record.args());
            println!("{}", message);
        }
    }
    fn flush(&self) {}
}

// #[macro_export]
// macro_rules! error {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::console::print_by_level(format_args!(concat!("\x1b[31m", "[ERROR][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::ERROR)
//     };
// }

// #[macro_export]
// macro_rules! warn {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::console::print_by_level(format_args!(concat!("\x1b[93m", "[WARN][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::WARN)
//     };
// }

// #[macro_export]
// macro_rules! info {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::console::print_by_level(format_args!(concat!("\x1b[34m", "[INFO][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::INFO)
//     };
// }

// #[macro_export]
// macro_rules! debug {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::console::print_by_level(format_args!(concat!("\x1b[32m", "[DEBUG][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::DEBUG)
//     };
// }

// #[macro_export]
// macro_rules! trace {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::console::print_by_level(format_args!(concat!("\x1b[90m", "[TRACE][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::TRACE)
//     };
// }
