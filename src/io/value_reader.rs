use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub struct ValueReader {
    br: BufReader<Box<dyn Read>>
}

impl ValueReader {
    pub fn new(read: Box<dyn Read>) -> ValueReader {
        let br = BufReader::new(read);
        ValueReader { br }
    }

    pub fn read<T: std::str::FromStr>(&mut self) -> Option<T> {
        let mut s = String::new();
        self.br.read_line(&mut s).unwrap();
        s.trim().parse().ok()
    }

    pub fn read_vec<T: std::str::FromStr>(&mut self) -> Vec<T> {
        let mut s = String::new();
        self.br.read_line(&mut s).unwrap();
        s.trim().split_whitespace()
            .map(|e| e.parse().ok().unwrap()).collect()
    }
}
