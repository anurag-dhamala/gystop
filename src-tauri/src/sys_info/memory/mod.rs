use serde::{Serialze, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    total_count: u64,
    total_swap: u64,
    total_mem: u64,
    total_avail: u64,
    free_swap: u64,
    used_swap: u64
}


impl Default for Memory {
    fn default() -> Self {
        Self {
            total_count: 0,
            total_swap: 0,
            total_mem: 0,
            total_avail: 0,
            free_swap: 0,
            used_swap: 0
        }
    }
}

impl Memory {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_total_count(&mut self, total_count: u64){
        self.total_count  = total_count;
    }

    pub fn set_total_swap(&mut self, total_swap: u64) {
        self.total_swap = total_swap;
    }

    pub fn set_total_mem(&mut self, total_mem: u64) {
        self.total_mem = total_mem;
    }

    pub fn set_total_avail(&mut self, total_avail: u64) {
        self.total_avail = total_avail;
    }

    pub fn set_free_swap(&mut self, free_swap: u64) {
        self.free_swap = free_swap;
    }

    pub fn set_used_swap(&mut self, used_swap: u64){
        self.used_swap = used_swap;
    }
}
