use pyo3::prelude::*;
use sha1::Sha1;
use std::time::Duration;

#[pyclass]
#[derive(Clone)]
struct DUCOHasher {
    hasher: Sha1
}

#[pymethods]
impl DUCOHasher {
    #[new]
    pub fn new(data: &[u8]) -> Self {
        Self {hasher: Sha1::from(data)}
    }

    #[warn(non_snake_case)] // recommend changing (needs to be changed in miners, too)
    pub fn DUCOS1(&mut self, expected_hash: &[u8], diff: u128, eff: u64) -> u128 {
        for nonce in 0..(100*diff+1) {
            let mut temp_hasher = self.hasher.clone();

            temp_hasher.update(nonce.to_string().as_bytes());
            let result = temp_hasher.digest().bytes();

            if eff != 0 {
                if nonce % 5000 == 0 {
                    std::thread::sleep(Duration::new(eff/100, 0))
                }
            }

            if result == expected_hash {
                self.hasher.reset();
                return nonce;
            }
            
        }
        return 0;
    }

}

#[pymodule]
fn libducohasher(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<DUCOHasher>()?;
    Ok(())
}