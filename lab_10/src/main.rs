use std::io;
use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Сделайте трейт Read для RotDecoder.
impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Читаем данные из входного потока в буфер
        let n = self.input.read(buf)?;
        // Проходим по прочитанным данным
        for byte in &mut buf[..n] {
            // Проверяем, является ли символ буквой
            if byte.is_ascii_alphabetic() {
                // Определяем базовый символ (A или a) для сдвига
                let base = if byte.is_ascii_lowercase() { b'a' } else { b'A' };
                // Выполняем сдвиг ROT с учетом цикличности алфавита
                *byte = (base + (*byte - base + self.rot) % 26) as u8;
            }
        }
        Ok(n)
    }
}

fn main() {
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

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
