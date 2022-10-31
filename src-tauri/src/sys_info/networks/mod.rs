use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Networks {
    packets_received: u64,
    packets_transmitted: u64,
    total_received: u64,
    total_transmitted: u64,
    errors_on_received: u64,
    errors_on_transmitted: u64,
}

impl Default for Networks {
     fn default() -> Self {
        Self {
            packets_received: 0,
            packets_transmitted: 0,
            total_received: 0,
            total_transmitted: 0,
            errors_on_received: 0,
            errors_on_transmitted: 0
        }
    }
}

impl Networks {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_packets_received(&mut self, packets_received: u64) {
        self.packets_received = packets_received;
    }

    pub fn set_packets_transmitted(&mut self, packets_transmitted: u64)  {
        self.packets_transmitted = packets_transmitted;
    }

    pub fn set_total_received(&mut self, total_received: u64 ) {
        self.total_received = total_received;
    }

    pub fn set_total_transmitted(&mut self, total_transmitted: u64) {
        self.total_transmitted = total_transmitted;
    }

    pub fn set_errors_on_received(&mut self, errors_on_received: u64) {
        self.errors_on_received = errors_on_received;
    }

    pub fn set_errors_on_transmitted(&mut self, errors_on_transmitted: u64)  {
        self.errors_on_transmitted = errors_on_transmitted;
    }
}