use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SingleCpu {
    vendor_name: String,
    model_name: String,
    core_id: i32,
    mhz: f64,
    cache_size: String,
    clock: String,
    cpu_usage: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuData {
    architecture: String,
    byte_order: String,
    operation_modes: String,
    cpu_arr: Option<Vec<SingleCpu>>,
}

impl CpuData {
    pub fn new() -> Self {
        Self {
            architecture: "".to_string(),
            byte_order: "".to_string(),
            operation_modes: "".to_string(),
            cpu_arr: None,
        }
    }

    pub fn set_architecture(&mut self, architecture: String) {
        self.architecture = architecture;
    }

    pub fn set_byte_order(&mut self, byte_order: String) {
        self.byte_order = byte_order;
    }

    pub fn set_operation_modes(&mut self, operation_modes: String) {
        self.operation_modes = operation_modes;
    }

    pub fn set_cpu_arr(&mut self, cpu_arr: Option<Vec<SingleCpu>>) {
        self.cpu_arr = cpu_arr;
    }
}
