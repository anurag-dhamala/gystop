use crate::sys_info::cpu::SingleCpu;
use crate::sys_info::disks::Disk;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

//private functions to this module
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
pub fn get_disks() -> Vec<Disk> {
    let mut sys_instance = get_sys_instance();
    sys_instance.refresh_disks();
    sys_instance.refresh_disks_list();

    let mut vec: Vec<Disk> = Vec::new();
    for disk in sys_instance.disks() {
        let mut single_disk = Disk::new();
        single_disk.set_mount_point(disk.mount_point().to_path_buf());
        single_disk.set_total_space(disk.total_space());
        single_disk.set_available_space(disk.available_space());
        single_disk.set_name(disk.name().to_os_string());
        single_disk.set_is_removable(disk.is_removable());
        vec.push(single_disk);
    }
    println!("{:#?}", vec);
    vec
}

#[tauri::command]
pub fn get_devices() {}

#[tauri::command]
pub fn get_temperature() {}
