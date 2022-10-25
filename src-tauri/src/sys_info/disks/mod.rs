use std::{ffi::OsString, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    name: OsString,
    mount_point: PathBuf,
    total_space: u64,
    available_space: u64,
    is_removable: bool,
}

impl Default for Disk {
    fn default() -> Self {
        Self {
            name: OsString::new(),
            mount_point: PathBuf::new(),
            total_space: 0,
            available_space: 0,
            is_removable: false,
        }
    }
}

impl Disk {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_name(&mut self, name: OsString) {
        self.name = name;
    }

    pub fn set_mount_point(&mut self, mount_point: PathBuf) {
        self.mount_point = mount_point;
    }

    pub fn set_total_space(&mut self, total_space: u64) {
        self.total_space = total_space;
    }

    pub fn set_available_space(&mut self, available_space: u64) {
        self.available_space = available_space;
    }

    pub fn set_is_removable(&mut self, is_removable: bool) {
        self.is_removable = is_removable;
    }
}
