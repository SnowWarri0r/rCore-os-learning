use crate::sbi::console_putchar;
use core::{
    fmt::{self, Write},
    str,
};

struct Stdout;
const LEVEL: Option<&str> = option_env!("LOG");
#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    SILENT = 0,
    ERROR = 1,
    WARN = 2,
    INFO = 3,
    DEBUG = 4,
    TRACE = 5,
}
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

const LOG: LogLevel = match LEVEL {
    Some(s) => {
        let bytes = s.as_bytes();
        match bytes {
            b"SILENT" => LogLevel::SILENT,
            b"ERROR" => LogLevel::ERROR,
            b"WARN" => LogLevel::WARN,
            b"INFO" => LogLevel::INFO,
            b"DEBUG" => LogLevel::DEBUG,
            b"TRACE" => LogLevel::TRACE,
            _ => LogLevel::INFO,
        }
    }
    None => LogLevel::INFO,
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
pub fn print_by_level(args: fmt::Arguments, level: LogLevel) {
    if level > LOG {
        return;
    }
    print(args);
}

#[macro_export]
macro_rules! print {
  // + 代表该宏表达式可重复1次或多次
  // ? 代表该宏表达式是可选的，可出现零次或一次
  // literal 代表该宏参数匹配一个字面量表达式
  // tt 代表 tokentree，即可以是多个由括号包裹的或者单个关键字等 https://doc.rust-lang.org/reference/tokens.html
    ($fmt: literal $(, $($arg: tt)+)?) => {
      $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
  ($fmt: literal $(, $($arg: tt)+)?) => {
    $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
  };
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_by_level(format_args!(concat!("\x1b[31m", "[ERROR][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::ERROR);
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_by_level(format_args!(concat!("\x1b[93m", "[WARN][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::WARN);
    };
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_by_level(format_args!(concat!("\x1b[34m", "[INFO][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::INFO);
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_by_level(format_args!(concat!("\x1b[32m", "[DEBUG][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::DEBUG);
    };
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_by_level(format_args!(concat!("\x1b[90m", "[TRACE][0] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::TRACE);
    };
}
