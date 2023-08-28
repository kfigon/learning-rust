#[cfg(test)]
mod test {
    // 4 basic traits:
    // Read - byte oriented
    // BufRead - byte/string oriented, buffered IO
    // Write - bute/string oriented
    // Seek

    static PATH: &str = "src/main.rs";

    use std::fs::{self, File};
    use std::io::{Read, BufReader, Write};

    #[test]
    fn read_helper_test() {
        let d = fs::read_to_string(PATH).unwrap();
        assert!(!d.is_empty());
    }

    #[test]
    fn read_test() {
        let mut f = File::open(PATH).unwrap();
        let mut d = String::new();
        f.read_to_string(&mut d).unwrap();

        assert!(!d.is_empty());
    }

    #[test]
    fn buffered_read_test() {
        let mut f = BufReader::new(File::open(PATH).unwrap());
        let mut d = String::new();
        f.read_to_string(&mut d).unwrap();

        assert!(!d.is_empty());
    }

    #[test]
    fn writer_test() {
        // Vec implements Write
        let mut w = Vec::<u8>::new();
        // + BufWriter::new()
        write!(&mut w, "foobar").unwrap();
        // or w.write("foobar".as_bytes());
        assert_eq!(w, "foobar".bytes().collect::<Vec<_>>())
    }
}