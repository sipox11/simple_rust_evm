extern crate cast;

use cast::isize;
use std::collections::HashMap;

// Memory is modeled as a key-value store, where keys are the 256-bit offsets, holding 256-bit values
#[derive(Debug)]
pub struct Memory {
    data: HashMap<usize, isize>,
    fmp: usize, // Free memory pointer -> Pointing to the next empty memory location
}

impl Memory {
    pub fn new() -> Self {
        let mut mem = Self {
            data: HashMap::new(),
            fmp: 0x60, // Leave the first 2 32-byte offsets for the scratchpad
        };
        // Persist the fmp
        mem.store_at(0x40, isize(mem.fmp).unwrap());
        // Return instance
        mem
    }

    // Store a 256-bit value on a specific offset, without messing with the fmp
    pub fn store_at(&mut self, offset: usize, val: isize) {
        self.data.insert(offset, val);
    }
    /// Store a 256-bit value on the next available empty offset, and increase the free memory pointer
    pub fn store(&mut self, val: isize) {
        self.data.insert(self.fmp, val);
        self.fmp += 0x20;
    }
    /// Reads a 256-bit value from a specific offset in memory
    pub fn load(&self, offset: usize) -> isize {
        *(self.data.get(&offset).unwrap())
    }
}