Rust
use wasm_bindgen::prelude::*;
use entropy::shannon::ShannonEntropy;
use std::collections::HashMap;

// TSU = Tufure System Entropy
#[wasm_bindgen]
pub struct TSU {
    bytes_seen: u64,
    byte_freq: [u64; 256],
}

#[wasm_bindgen]
impl TSU {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        TSU {
            bytes_seen: 0,
            byte_freq: [0; 256],
        }
    }

    // Feed any data (WASM blob, ZK proof, IPFS chunk, etc.)
    pub fn update(&mut self, data: &[u8]) {
        for &b in data {
            self.byte_freq[b as usize] += 1;
            self.bytes_seen += 1;
        }
    }

    // Current Shannon entropy in bits/byte (0.0 → 8.0)
    #[wasm_bindgen(getter)]
    pub fn entropy(&self) -> f64 {
        if self.bytes_seen == 0 { return 0.0; }
        let mut h = 0.0f64;
        for &count in &self.byte_freq {
            if count == 0 { continue; }
            let p = count as f64 / self.bytes_seen as f64;
            h -= p * p.log2();
        }
        h
    }

    // Chalice reward estimate (1 Chalice ≈ 1 kB·bit of proven entropy)
    #[wasm_bindgen(getter)]
    pub fn chalice_estimate(&self) -> f64 {
        (self.bytes_seen as f64 / 1024.0) * self.entropy()  // kB·bit
    }

    // Reset for next epoch
    pub fn reset(&mut self) {
        self.bytes_seen = 0;
        self.byte_freq = [0; 256];
    }
}
