pub mod cpu;
pub mod devices;
pub mod disks;
pub mod memory;
pub mod processes;
pub mod temperature;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalInfo {
    boot_time: u64,
    host_name: Option<String>,
    kernel_version: Option<String>,
    long_os_version: Option<String>,
    os_name: Option<String>,
    uptime: u64,
}

impl Default for GlobalInfo {
    fn default() -> Self {
        Self {
            boot_time: 0,
            host_name: Some("".to_string()),
            kernel_version: Some("".to_string()),
            long_os_version: Some("".to_string()),
            os_name: Some("".to_string()),
            uptime: 0,
        }
    }
}

impl GlobalInfo {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_boot_time(&mut self, boot_time: u64) {
        self.boot_time = boot_time;
    }
    pub fn set_host_name(&mut self, host_name: Option<String>) {
        self.host_name = host_name;
    }
    pub fn set_kernel_version(&mut self, kernel_version: Option<String>) {
        self.kernel_version = kernel_version;
    }
    pub fn set_long_os_version(&mut self, long_os_version: Option<String>) {
        self.long_os_version = long_os_version;
    }
    pub fn set_os_name(&mut self, os_name: Option<String>) {
        self.os_name = os_name;
    }
    pub fn set_uptime(&mut self, uptime: u64) {
        self.uptime = uptime;
    }
}
