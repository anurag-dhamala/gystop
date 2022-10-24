use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleCpu {
    name: String,
    vendor_id: String,
    brand_name: String,
    mhz: u64,
    cpu_usage: f32,
}

impl SingleCpu {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            vendor_id: "".to_string(),
            brand_name: "".to_string(),
            mhz: 0,
            cpu_usage: 0.0,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_vendor_id(&mut self, vendor_id: String) {
        self.vendor_id = vendor_id;
    }

    pub fn set_brand_name(&mut self, brand_name: String) {
        self.brand_name = brand_name;
    }

    pub fn set_mhz(&mut self, mhz: u64) {
        self.mhz = mhz;
    }

    pub fn set_cpu_usage(&mut self, cpu_usage: f32) {
        self.cpu_usage = cpu_usage;
    }
}
