use std::io::BufWriter;
use std::io::Write;
use std::io::Error;

pub struct ValueWriter {
    bw: BufWriter<Box<dyn Write>>
}

impl ValueWriter {
    pub fn new(write: Box<dyn Write>) -> ValueWriter {
        let bw = BufWriter::new(write);
        ValueWriter { bw }
    }

    pub fn write<T: ToString>(&mut self, v: T) -> Result<(), Error> {
        self.bw.write((v.to_string() + &"\n".to_string()).as_bytes())?;
        return self.bw.flush();
    }

    pub fn write_vec<T: ToString>(&mut self, vec: Vec<T>) -> Result<(), Error> {
        if vec.len() < 1 {
            return Ok(())
        }
        self.bw.write(vec[0].to_string().as_bytes())?;
        for v in &vec[1..] {
            self.bw.write((" ".to_string() + &v.to_string()).as_bytes())?;
        }
        self.bw.write("\n".as_bytes())?;
        return self.bw.flush();
    }
}
