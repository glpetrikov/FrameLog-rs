/* =================================================
 * FrameLog-rs, MIT - License
 * ─────────────────────────────────────────────────
 * FrameLog-rs
 * FileName
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.5
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * file handler
 * =================================================
 */

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, Read, Seek, SeekFrom, BufRead, BufReader};

pub struct FileHandler {
    file: File,
    file_name: String,
}

impl FileHandler {
    /// accepts the name of the file where to log
    pub fn new(file_name: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)?;
        Ok(Self { file, file_name: file_name.to_string(),})
    }

    /// writes data to the end of the file
    pub fn write(&mut self, message: &str) -> io::Result<()> {
        write!(self.file, "{}", message)?;
        Ok(())
    }
    /// writes data to the end of the file with line termination
    pub fn writeln(&mut self, message: &str) -> io::Result<()> {
        writeln!(self.file, "{}", message)?;
        Ok(())
    }

    /// Guaranteed to write to disk, useful for fatal errors, attention to flush everything in the buffer
    pub fn writeln_sync(&mut self, message: &str) -> io::Result<()> {
        writeln!(self.file, "{}", message)?;
        self.file.flush()?;
        Ok(())
    }

    /// reads the entire file
    pub fn read(&mut self) -> io::Result<String> {
        use std::io::Seek;
        use std::io::SeekFrom;

        self.file.seek(SeekFrom::Start(0))?;

        let mut contents = String::new();
        let mut buf_reader = BufReader::new(&mut self.file);
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    /// reads only one line
    pub fn read_line(&mut self, line_number: usize) -> io::Result<Option<String>> {
        let target_line = line_number -1;
        self.file.seek(SeekFrom::Start(0))?;
        let buf_reader = BufReader::new(&mut self.file);

        for (i, line) in buf_reader.lines().enumerate() {
            let line = line?;
            if i == target_line {
                return Ok(Some(line));
            }
        }

        Ok(None)
    }

    /// opens a new file
    pub fn set_file(&mut self, file_name: &str) -> io::Result<()> {
        self.file.flush()?;
        self.file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)?;
        self.file_name = file_name.to_string();
        Ok(())
    }
    /// close the current file
    pub fn close_file(&mut self) -> io::Result<()> {
        self.file.flush()?;

        let file = std::mem::replace(
            &mut self.file,
            OpenOptions::new().read(true).open(&self.file_name)?,
        );

        drop(file);
        Ok(())
    }


    /// delete current file
    pub fn delete_current(&self) -> io::Result<()> {
        fs::remove_file(&self.file_name)?;
        Ok(())
    }

    /// deletes the file with the given name
    pub fn delete_file(file_name: &str) -> io::Result<()> {
        fs::remove_file(file_name)?;
        Ok(())
    }
}
