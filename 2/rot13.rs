use std::io::Read;
use std::convert::TryFrom;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

impl <R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.input.read(buf)?; // ? is called try operator
        for b in &mut buf[..size] { // from beginning to end of size, excluding
            if b.is_ascii_alphabetic() {
            let base = match u8::try_from(*b) {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Conversion error: {:?}", e);
                    0
                }
            };
            *b = (*b - base + self.rot) % 26 + base;
        }
        }
        Ok(size)
    }

}
// Implement the `Read` trait for `RotDecoder`.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
