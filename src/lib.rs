use pyo3::prelude::*;
use std::time::Duration;
use sha1_smol::Sha1;

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

    #[allow(non_snake_case)] // recommend changing (needs to be changed in miners, too)
    pub fn DUCOS1(&mut self, expected_hash: &[u8], diff: u128, eff: u64) -> u128 {
        let mut buffer = itoa::Buffer::new();
        if eff != 0 // this really is a micro-optimization
        {
            for nonce in 0..(100*diff+1) {
                let mut temp_hasher = self.hasher.clone();

                let str = buffer.format(nonce);
                temp_hasher.update(str.as_bytes());


                if nonce % 5000 == 0 {
                    std::thread::sleep(Duration::new(eff/100, 0))
                }

                if temp_hasher.digest().bytes() == expected_hash {
                    self.hasher.reset();
                    return nonce;
                }

            }
        }
        else
        {
            for nonce in 0..(100*diff+1) {
                let mut temp_hasher = self.hasher.clone();

                let str = buffer.format(nonce);
                temp_hasher.update(str.as_bytes());

                if temp_hasher.digest().bytes() == expected_hash {
                    self.hasher.reset();
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
