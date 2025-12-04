use std::fs::{self, File};
use std::io::{self, Write, Read, Seek, SeekFrom, BufRead, BufReader};

pub struct FileHandler {
    file: File,
    file_name: String,
}

impl FileHandler {
    /// accepts the name of the file where to log
    pub fn new(file_name: &str) -> io::Result<Self> {
        let file = File::create(file_name)?;
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
        self.file = File::open(file_name)?;
        Ok(())
    }
    /// close the current file
    pub fn close_file(&mut self) -> io::Result<()> {
        self.file.flush()?;
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
