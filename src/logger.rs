/* =================================================
 * FrameLog-rs, MIT - License
 * ─────────────────────────────────────────────────
 * FrameLog-rs
 * logger.rs
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.4
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
        println!("{}", message.bright_black());
    }
    /// info print a green message to the console, does not support formatting
    pub fn info(message: &str){
        println!("{}", message.green());
    }
    /// warn print a yellow message to the console, does not support formatting
    pub fn warn(message: &str){
        println!("{}", message.yellow());
    }
    /// error print a red message to the console, does not support formatting
    pub fn error(message: &str){
        println!("{}", message.red());
    }
    /// fatal error a bright red message to the console, does not support formatting
    pub fn fatal(message: &str){
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        trace!("Trace Test");
        info!("Info Test");
        warn!("Warn Test");
        error!("Error Test");
        fatal!("Fatal Test");
    }
}
