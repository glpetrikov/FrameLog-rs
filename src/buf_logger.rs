use std::io;
use colored::Colorize;
use crate::buffer;

pub struct Logger{
    buf: buffer::Buffer,
}
impl crate::buf_logger::Logger {
    pub fn new() -> io::Result<Self> {
        let buf = buffer::Buffer::new()?;
        Ok(Self { buf })
    }

    /// print all data of Buffer to Console
    pub fn flush(&mut self) -> io::Result<()> {
        println!("{}", self.buf.get()?);
        self.buf.clear_data()?;
        Ok(())
    }

    /// trace print a gray message to the Logger Buffer, does not support formatting
    pub fn trace(&mut self, message: &str){
        self.buf.add(&(message.bright_black().to_string() +"\n"));
    }
    /// info print a green message to the Logger Buffer, does not support formatting
    pub fn info(&mut self, message: &str){
        self.buf.add(&(message.green().to_string() + "\n"));
    }
    /// warn print a yellow message to the Logger Buffer, does not support formatting
    pub fn warn(&mut self, message: &str){
        self.buf.add(&(message.yellow().to_string() + "\n"));
    }
    /// error print a red message to the Logger Buffer, does not support formatting
    pub fn error(&mut self, message: &str){
        self.buf.add(&(message.red().to_string() + "\n"));
    }
    /// fatal error a bright red message to the Logger Buffer, does not support formatting
    pub fn fatal(&mut self, message: &str){
        self.buf.add(&(message.bright_red().to_string() + "\n"));
    }


    /// trace line print a gray message to the Logger Buffer, does not support formatting
    pub fn traceln(&mut self, message: &str){
        self.buf.add(&(message.bright_black().to_string() + "\n"));
    }
    /// info line print a green message to the Logger Buffer, does not support formatting
    pub fn infoln(&mut self, message: &str){
        self.buf.add( &(message.green().to_string() + "\n"));
    }
    /// warn line print a yellow message to the Logger Buffer, does not support formatting
    pub fn warnln(&mut self, message: &str){
        self.buf.add( &(message.yellow().to_string() + "\n"));
    }
    /// error line print a red message to the Logger Buffer, does not support formatting
    pub fn errorln(&mut self, message: &str){
        self.buf.add(&(message.red().to_string() + "\n"));
    }
    /// fatal line error a bright red message to the Logger Buffer, does not support formatting
    pub fn fatalln(&mut self, message: &str){
        self.buf.add(&(message.bright_red().to_string() + "\n"));
    }
}

/// trace to existing Logger
#[macro_export]
macro_rules! buf_trace {
    ($logger:expr, $($arg:tt)*) => {
        $logger.trace(&format!($($arg)*));
    };
}

/// info to existing Logger
#[macro_export]
macro_rules! buf_info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.info(&format!($($arg)*));
    };
}

/// warn to existing Logger
#[macro_export]
macro_rules! buf_warn {
    ($logger:expr, $($arg:tt)*) => {
        $logger.warn(&format!($($arg)*));
    };
}

/// error to existing Logger
#[macro_export]
macro_rules! buf_error {
    ($logger:expr, $($arg:tt)*) => {
        $logger.error(&format!($($arg)*));
    };
}

/// fatal to existing Logger
#[macro_export]
macro_rules! buf_fatal {
    ($logger:expr, $($arg:tt)*) => {
        $logger.fatal(&format!($($arg)*));
    };
}

/// trace line to existing Logger
#[macro_export]
macro_rules! buf_traceln {
    ($logger:expr, $($arg:tt)*) => {
        $logger.traceln(&format!($($arg)*));
    };
}

/// info line to existing Logger
#[macro_export]
macro_rules! buf_infoln {
    ($logger:expr, $($arg:tt)*) => {
        $logger.infoln(&format!($($arg)*));
    };
}

/// warn line to existing Logger
#[macro_export]
macro_rules! buf_warnln {
    ($logger:expr, $($arg:tt)*) => {
        $logger.warnln(&format!($($arg)*));
    };
}

/// error line to existing Logger
#[macro_export]
macro_rules! buf_errorln {
    ($logger:expr, $($arg:tt)*) => {
        $logger.errorln(&format!($($arg)*));
    };
}

/// fatal line to existing Logger
#[macro_export]
macro_rules! buf_fatalln {
    ($logger:expr, $($arg:tt)*) => {
        $logger.fatalln(&format!($($arg)*));
    };
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_logger_prints() -> std::io::Result<()> {
        use crate::buf_logger::Logger;
        let mut log = Logger::new()?;

        log.trace("Trace Test");
        log.info("Info Test");
        log.warn("Warn Test");
        log.error("Error Test");
        log.fatal("Fatal Test");
        log.flush()?;

        Ok(())
    }

    #[test]
    fn test_logger_line_prints() -> std::io::Result<()> {
        use crate::buf_logger::Logger;
        let mut log = Logger::new()?;

        log.traceln("Line Trace Test");
        log.infoln("Line Info Test");
        log.warnln("Line Warn Test");
        log.errorln("Line Error Test");
        log.fatalln("Line Fatal Test");
        log.flush()?;

        Ok(())
    }

    #[test]
    fn test_logger_prints_macros() -> std::io::Result<()> {
        use crate::buf_logger::Logger;
        let mut log = Logger::new()?;

        buf_trace!(log, "Trace Test");
        buf_info!(log, "Info Test");
        buf_warn!(log, "Warn Test");
        buf_error!(log, "Error Test");
        buf_fatal!(log, "Fatal Test");
        log.flush()?;

        Ok(())
    }

    #[test]
    fn test_logger_line_prints_macros() -> std::io::Result<()> {
        use crate::buf_logger::Logger;
        let mut log = Logger::new()?;

        buf_traceln!(log, "Line Trace Test");
        buf_infoln!(log, "Line Info Test");
        buf_warnln!(log, "Line Warn Test");
        buf_errorln!(log, "Line Error Test");
        buf_fatalln!(log, "Line Fatal Test");
        log.flush()?;

        Ok(())
    }
}
