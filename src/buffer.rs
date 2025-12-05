use std::io;

pub struct Buffer{
    data: String,
}

impl Buffer {
    pub fn new() -> io::Result<Self> {
        Ok( Self{ data: String::new() } )
    }

    pub fn add(&mut self, message: &str) -> io::Result<()> {
        self.data.push_str(message);
        Ok(())
    }

    /// return full Buffer data
    pub fn get(&mut self) -> io::Result<String> {
        return Ok(self.data.to_string());
    }

    /// clear buffer data
    pub fn delete_data(&mut self) -> io::Result<()> {
        self.data.clear();
        Ok(())
    }
    /// clear buffer data
    pub fn clear_data(&mut self) -> io::Result<()> {
        self.data.clear();
        Ok(())
    }
    /// clear buffer data
    pub fn clean_data(&mut self) -> io::Result<()> {
        self.data.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use crate::buffer::Buffer;

    #[test]
    fn test_buffer() -> io::Result<()> {
        let mut buf = Buffer::new()?;
        buf.add("Hello, World!")?;
        if buf.get()? == "Hello, World!" {
            buf.clear_data()?;
            Ok(())
        }
        else{
            buf.clear_data()?;
            Err(io::Error::new(io::ErrorKind::Other, "Buffer mismatch"))
        }
    }
}