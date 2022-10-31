use crate::sys_info::cpu::SingleCpu;
use crate::sys_info::disks::Disk;
use crate::sys_info::GlobalInfo;
use crate::sys_info::networks::Networks;
use sysinfo::{ComponentExt, CpuExt, DiskExt, NetworkExt, NetworksExt, System, SystemExt};
use crate::sys_info::memory::Memory;

//private functions to this module
fn get_sys_instance() -> System {
    let mut sys = System::new();
    sys.refresh_all();
    sys
}

#[tauri::command]
pub fn get_cpu() -> Vec<SingleCpu> {
    let sys_instance = get_sys_instance();
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
    // std::thread::sleep(std::time::Duration::from_millis(500));
    println!("{:#?}", vec);
    vec
}

#[tauri::command]
pub fn get_memory() -> Memory{
    let system_instance = get_sys_instance();
    let mut memory_instance = Memory::new();
    memory_instance.set_total_mem(system_instance.total_memory());
    memory_instance.set_total_swap(system_instance.total_swap());
    memory_instance.set_total_avail(system_instance.free_memory());
    memory_instance.set_free_swap(system_instance.free_swap());
    memory_instance.set_used_swap(system_instance.total_swap());
    memory_instance
}

#[tauri::command]
pub fn get_disks() -> Vec<Disk> {
    let sys_instance = get_sys_instance();
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

#[tauri::command]
pub fn get_network_info() -> Vec<Networks> {
    let sys_instance = get_sys_instance();
    let mut  network_vec : Vec<Networks> = Vec::new();
    for (&key, network) in sys_instance.networks() {
        let mut network_info = Networks::new();
        network_info.set_total_received(network.total_received());
        network_info.set_errors_on_received(network.errors_on_received());
        network_info.set_total_transmitted(network.total_transmitted());
        network_info.set_errors_on_transmitted(network.errors_on_transmitted());
        network_info.set_packets_received(network.packets_received());
        network_info.set_packets_transmitted(network.packets_transmitted());
        network_vec.push(network_info);
    }
    network_vec
}

#[tauri::command]
pub fn get_global_info() -> GlobalInfo {
    let sys_instance = get_sys_instance();
    let mut global_info = GlobalInfo::new();
    global_info.set_boot_time(sys_instance.boot_time());
    global_info.set_host_name(sys_instance.host_name());
    global_info.set_kernel_version(sys_instance.kernel_version());
    global_info.set_uptime(sys_instance.uptime());
    global_info.set_boot_time(sys_instance.boot_time());
    global_info.set_os_name(sys_instance.name());
    global_info.set_long_os_version(sys_instance.long_os_version());
    println!("{:#?}", global_info);
    global_info
}
