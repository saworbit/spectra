use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

const SAMPLE_SIZE: usize = 8192; // Read first 8KB

/// Calculates Shannon Entropy.
/// Returns a value between 0.0 (uniform) and 8.0 (random).
pub fn calculate_shannon_entropy(path: &Path) -> io::Result<f32> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; SAMPLE_SIZE];

    // We only read the "Head" of the file for speed
    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == 0 {
        return Ok(0.0);
    }
    let data = &buffer[0..bytes_read];

    let mut frequencies = [0u32; 256];
    for &byte in data {
        frequencies[byte as usize] += 1;
    }

    let len = data.len() as f32;
    let mut entropy = 0.0;

    for &count in frequencies.iter() {
        if count > 0 {
            let p = count as f32 / len;
            entropy -= p * p.log2();
        }
    }

    Ok(entropy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_low_entropy() {
        // Repeated bytes should have 0 entropy
        let mut file = NamedTempFile::new().unwrap();
        let zeros = [0u8; 1000];
        file.write_all(&zeros).unwrap();

        let ent = calculate_shannon_entropy(file.path()).unwrap();
        assert_eq!(ent, 0.0);
    }

    #[test]
    fn test_medium_entropy() {
        // Text data should have moderate entropy
        let mut file = NamedTempFile::new().unwrap();
        let text = b"The quick brown fox jumps over the lazy dog. ".repeat(10);
        file.write_all(&text).unwrap();

        let ent = calculate_shannon_entropy(file.path()).unwrap();
        assert!(ent > 3.0 && ent < 6.0);
    }
}
