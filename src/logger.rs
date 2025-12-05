/* =================================================
 * FrameLog-rs, MIT - License
 * ─────────────────────────────────────────────────
 * FrameLog-rs
 * logger.rs
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.5
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Logger
 * =================================================
 */

use colored::*;

pub struct Logger;
impl Logger {
    /// trace print a gray message to the console, does not support formatting
    pub fn trace(message: &str){
        print!("{}", message.bright_black());
    }
    /// info print a green message to the console, does not support formatting
    pub fn info(message: &str){
        print!("{}", message.green());
    }
    /// warn print a yellow message to the console, does not support formatting
    pub fn warn(message: &str){
        print!("{}", message.yellow());
    }
    /// error print a red message to the console, does not support formatting
    pub fn error(message: &str){
        print!("{}", message.red());
    }
    /// fatal error a bright red message to the console, does not support formatting
    pub fn fatal(message: &str){
        print!("{}", message.bright_red());
    }


    /// trace line print a gray message to the console, does not support formatting
    pub fn traceln(message: &str){
        println!("{}", message.bright_black());
    }
    /// info line print a green message to the console, does not support formatting
    pub fn infoln(message: &str){
        println!("{}", message.green());
    }
    /// warn line print a yellow message to the console, does not support formatting
    pub fn warnln(message: &str){
        println!("{}", message.yellow());
    }
    /// error line print a red message to the console, does not support formatting
    pub fn errorln(message: &str){
        println!("{}", message.red());
    }
    /// fatal line error a bright red message to the console, does not support formatting
    pub fn fatalln(message: &str){
        println!("{}", message.bright_red());
    }
}

/// trace a gray message to the console, support formatting
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::logger::Logger::trace(&format!($($arg)*));
    };
}

/// info a green message to the console, support formatting
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::Logger::info(&format!($($arg)*));
    };
}

/// warn a yellow message to the console, support formatting
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::Logger::warn(&format!($($arg)*));
    };
}

/// error a red message to the console, support formatting
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::Logger::error(&format!($($arg)*));
    };
}

/// fatal error a bright red message to the console, support formatting
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::logger::Logger::fatal(&format!($($arg)*));
    };
}

/// trace a gray line message to the console, support formatting
#[macro_export]
macro_rules! traceln {
    ($($arg:tt)*) => {
        $crate::logger::Logger::traceln(&format!($($arg)*));
    };
}

/// info a green line message to the console, support formatting
#[macro_export]
macro_rules! infoln {
    ($($arg:tt)*) => {
        $crate::logger::Logger::infoln(&format!($($arg)*));
    };
}

/// warn a yellow line message to the console, support formatting
#[macro_export]
macro_rules! warnln {
    ($($arg:tt)*) => {
        $crate::logger::Logger::warnln(&format!($($arg)*));
    };
}

/// error a red line message to the console, support formatting
#[macro_export]
macro_rules! errorln {
    ($($arg:tt)*) => {
        $crate::logger::Logger::errorln(&format!($($arg)*));
    };
}

/// fatal error a bright red line message to the console, support formatting
#[macro_export]
macro_rules! fatalln {
    ($($arg:tt)*) => {
        $crate::logger::Logger::fatalln(&format!($($arg)*));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_loggget_prints() {
        trace!("Trace Test");
        info!("Info Test");
        warn!("Warn Test");
        error!("Error Test");
        fatal!("Fatal Test");
    }
    #[test]
    fn test_loggget_line_prints() {
        traceln!("Line Trace Test");
        infoln!("Line Info Test");
        warnln!("Line Warn Test");
        errorln!("Line Error Test");
        fatalln!("Line Fatal Test");
    }
}
