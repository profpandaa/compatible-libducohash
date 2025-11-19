use pyo3::prelude::*;
use sha1::digest::DynDigest;
use sha1::{Digest, Sha1};
use std::time::Duration;

#[pyclass]
#[derive(Clone)]
struct DUCOHasher {
    hasher: Sha1,
}

#[pymethods]
impl DUCOHasher {
    #[new]
    pub fn new(data: &[u8]) -> Self {
        Self {
            hasher: Sha1::new_with_prefix(data),
        }
    }

    #[allow(non_snake_case)] // recommend changing (needs to be changed in miners, too)
    pub fn DUCOS1(&mut self, expected_hash: &[u8], diff: u128, eff: u64) -> u128 {
        let mut buffer = itoa::Buffer::new();

        // this really is a micro-optimization
        if eff != 0 {
            for nonce in 0..(100 * diff + 1) {
                let mut temp_hasher = self.hasher.clone();

                let str = buffer.format(nonce);
                DynDigest::update(&mut temp_hasher, str.as_bytes());

                if nonce % 5000 == 0 {
                    std::thread::sleep(Duration::new(eff / 100, 0))
                }
                if temp_hasher.finalize().as_slice() == expected_hash {
                    return nonce;
                }
            }
        } else {
            for nonce in 0..(100 * diff + 1) {
                let mut temp_hasher = self.hasher.clone();

                let str = buffer.format(nonce);
                DynDigest::update(&mut temp_hasher, str.as_bytes());

                if temp_hasher.finalize().as_slice() == expected_hash {
                    return nonce;
                }
            }
        }
        0
    }
}

#[pymodule]
fn libducohasher(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<DUCOHasher>()?;
    Ok(())
}
