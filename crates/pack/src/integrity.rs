use sha2::{Sha256, Digest as Sha2Digest};
use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum DigestAlgorithm {
    Sha256,
    Blake3,
}

pub fn compute_digest(path: &Path, algorithm: DigestAlgorithm) -> crate::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    match algorithm {
        DigestAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            let mut buffer = [0; 8192];
            
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            
            let result = hasher.finalize();
            Ok(format!("sha256:{}", hex::encode(result)))
        }
        DigestAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            let mut buffer = [0; 8192];
            
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            
            let result = hasher.finalize();
            Ok(format!("blake3:{}", result.to_hex()))
        }
    }
}

pub fn compute_digest_bytes(data: &[u8], algorithm: DigestAlgorithm) -> String {
    match algorithm {
        DigestAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            let result = hasher.finalize();
            format!("sha256:{}", hex::encode(result))
        }
        DigestAlgorithm::Blake3 => {
            let hash = blake3::hash(data);
            format!("blake3:{}", hash.to_hex())
        }
    }
}

pub fn verify_digest(path: &Path, expected: &str) -> crate::Result<bool> {
    let parts: Vec<&str> = expected.split(':').collect();
    if parts.len() != 2 {
        return Err(crate::PackError::IntegrityError(
            "Invalid digest format".to_string()
        ));
    }

    let algorithm = match parts[0] {
        "sha256" => DigestAlgorithm::Sha256,
        "blake3" => DigestAlgorithm::Blake3,
        _ => return Err(crate::PackError::IntegrityError(
            format!("Unsupported digest algorithm: {}", parts[0])
        )),
    };

    let actual = compute_digest(path, algorithm)?;
    Ok(actual == expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_compute_digest() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"hello world").unwrap();
        file.flush().unwrap();

        let digest = compute_digest(file.path(), DigestAlgorithm::Sha256).unwrap();
        assert!(digest.starts_with("sha256:"));
    }

    #[test]
    fn test_verify_digest() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"test data").unwrap();
        file.flush().unwrap();

        let digest = compute_digest(file.path(), DigestAlgorithm::Sha256).unwrap();
        assert!(verify_digest(file.path(), &digest).unwrap());
    }
}

