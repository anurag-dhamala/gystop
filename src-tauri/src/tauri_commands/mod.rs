//private functions to this module

use crate::sys_info::cpu::SingleCpu;
use sysinfo::{CpuExt, System, SystemExt};

fn get_sys_instance() -> System {
    let sys = System::new();
    sys
}

#[tauri::command]
pub fn get_cpu() -> Vec<SingleCpu> {
    let mut sys_instance = get_sys_instance();
    sys_instance.refresh_cpu();
    let mut vec: Vec<SingleCpu> = Vec::new();
    for cpu in sys_instance.cpus() {
        let mut single_cpu = SingleCpu::new();
        single_cpu.set_brand_name(cpu.brand().to_string());
        single_cpu.set_cpu_usage(cpu.cpu_usage());
        single_cpu.set_vendor_id(cpu.vendor_id().to_string());
        single_cpu.set_name(cpu.name().to_string());
        single_cpu.set_mhz(cpu.frequency());
        vec.push(single_cpu);
    }
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("{:#?}", vec);
    vec
}

#[tauri::command]
pub fn get_memory() {}

#[tauri::command]
pub fn get_disks() {}

#[tauri::command]
pub fn get_devices() {}

#[tauri::command]
pub fn get_temperature() {}
