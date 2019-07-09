use rust_aoj::io::value_reader::ValueReader;
use rust_aoj::io::value_writer::ValueWriter;

fn main() {
    let reader = ValueReader::new(Box::new(std::io::stdin()));
    let writer = ValueWriter::new(Box::new(std::io::stdout()));

    sieve(reader, writer);
}

pub fn sieve(mut reader: ValueReader, mut writer: ValueWriter) {
    loop {
        let end: i32 = match reader.read() {
            Some(end) => end,
            None => return
        };
        if end < 2 {
            writer.write(0).unwrap();
            continue;
        }
        // Create vector for prime number flags which manages only the odd number.
        // Num n is calculated from index. n = i * 2 + 1
        let mut flags = vec![true; ((end + 1) / 2) as usize];
        let flags_len = flags.len();
        flags[0] = false;
        for i in 1..flags_len {
            if !flags[i] {
                continue
            }
            let n = i * 2 + 1;
            let mut j = 3;
            let mut comp_index = (n * j - 1) / 2;
            while comp_index < flags_len {
                flags[comp_index] = false;
                j += 2;
                comp_index = (n * j - 1) / 2;
            }
        }
        // Count flags and add one for skip of two;
        writer.write(flags.iter().filter(|x| **x).count() + 1).unwrap();
    }
}
