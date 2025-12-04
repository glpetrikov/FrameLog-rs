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
    pub fn trace(message: &str){
        println!("{}", message.bright_black());
    }
    pub fn info(message: &str){
        println!("{}", message.green());
    }
    pub fn warn(message: &str){
        println!("{}", message.yellow());
    }
    pub fn error(message: &str){
        println!("{}", message.red());
    }
    pub fn fatal(message: &str){
        println!("{}", message.bright_red());
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::logger::Logger::trace(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::Logger::info(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::Logger::warn(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::Logger::error(&format!($($arg)*));
    };
}

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
