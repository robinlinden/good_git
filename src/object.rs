use flate2::read::ZlibDecoder;
use sha1::{Digest, Sha1};
use std::io::prelude::*;

#[derive(Debug)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Blob {
        Blob { content }
    }

    pub fn hash(self) -> String {
        let size = self.content.len();
        let data = format!("blob {size}\0");
        let mut data = data.as_bytes().to_vec();
        data.extend(self.content);

        hash(&data)
    }
}

pub enum Object {
    Blob(Blob),
}

impl Object {
    pub fn from_bytes(s: &[u8]) -> Option<Object> {
        if s.len() > 4 && &s[0..4] == b"blob" {
            // TODO: validate length in header
            if let Some(null_index) = s.iter().position(|&x| x == b'\0') {
                let d = s[null_index + 1..].to_vec();
                let blob = Blob::new(d);
                return Some(Object::Blob(blob));
            }
        }
        None
    }

    pub fn from_file(path: &std::path::Path) -> Option<Object> {
        let data = std::fs::read(path).ok()?;
        let mut z = ZlibDecoder::new(&data[..]);
        let mut s: Vec<u8> = vec![];
        z.read_to_end(&mut s).ok()?;

        Object::from_bytes(&s)
    }
}

pub fn hash(s: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(s);

    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::hash;
    use super::Blob;
    use super::Object;

    #[test]
    fn test_blob_from_bytes() {
        let s = b"blob 16\0what is up, doc?";
        let object = Object::from_bytes(s.as_ref()).unwrap();
        let Object::Blob(blob) = object;
        assert_eq!(blob.content, b"what is up, doc?");
    }

    #[test]
    fn test_blob_hash_is_correct() {
        // From https://git-scm.com/book/sv/v2/Git-Internals-Git-Objects
        let blob = Blob::new(b"what is up, doc?".to_vec());
        assert_eq!(blob.hash(), "bd9dbf5aae1a3862dd1526723246b20206e5fc37");
    }

    #[test]
    fn test_hash_is_correct() {
        // From https://git-scm.com/book/sv/v2/Git-Internals-Git-Objects
        let s = b"blob 16\0what is up, doc?";
        assert_eq!(hash(s), "bd9dbf5aae1a3862dd1526723246b20206e5fc37");
    }
}
