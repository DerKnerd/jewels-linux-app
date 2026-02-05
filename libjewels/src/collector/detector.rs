use crate::collector::{Bios, Cpu, Device, Drive, Kernel, Mainboard, OperatingSystem};
use smbioslib::*;
use sysinfo::System;

const UNKNOWN_VALUE: &str = "Unbekannt";

fn read_dmi(path: &str) -> io::Result<String> {
    let data = fs::read_to_string(format!("/sys/class/dmi/id/{path}"))?;

    Ok(data.trim().to_string())
}

fn get_block_devices() -> Result<Vec<Drive>, Box<dyn std::error::Error>> {
    let mut devices = Vec::new();

    // Read disk stats from /proc/diskstats
    let diskstats = procfs::diskstats()?;

    for stat in diskstats {
        // Skip loop devices, ram disks, and other virtual devices
        if stat.name.starts_with("loop")
            || stat.name.starts_with("ram")
            || stat.name.starts_with("dm-")
        {
            continue;
        }

        let base_path = format!("/sys/class/block/{}", stat.name);

        // Determine if it's a partition (simple heuristic)
        let is_partition = !fs::exists(format!("{base_path}/device")).unwrap_or(false);
        if is_partition {
            continue;
        }

        let model =
            fs::read_to_string(format!("{base_path}/device/model")).map(|s| s.trim().to_string());
        let manufacturer = if stat.name.starts_with("nvme") {
            fs::read_to_string(format!("/sys/class/nvme/{}/device/vendor", &stat.name[..5]))
        } else {
            fs::read_to_string(format!("{base_path}/device/vendor"))
        }
        .map(|s| s.trim().to_string());

        // Get size from /sys/class/block/{device}/size
        let size_path = format!("/sys/class/block/{}/size", stat.name);
        let size_sectors = fs::read_to_string(&size_path)
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);
        let size_bytes = size_sectors * 512; // 512 bytes per sector
        let device = Drive {
            name: stat.name,
            size: size_bytes as f64 / 1024f64 / 1024f64 / 1024f64,
            model: model
                .map(|m| m.trim().to_string())
                .unwrap_or(UNKNOWN_VALUE.into()),
            manufacturer: manufacturer
                .map(|m| m.trim().to_string())
                .unwrap_or(UNKNOWN_VALUE.into()),
        };

        devices.push(device);
    }

    Ok(devices)
}

pub fn collect_device_info() -> Device {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpus = sys.cpus();
    let cpu_manufacturer = cpus[0].vendor_id();
    let cpu_manufacturer = if cpu_manufacturer == "AuthenticAMD" {
        "AMD"
    } else if cpu_manufacturer == "GenuineIntel" {
        "Intel"
    } else {
        UNKNOWN_VALUE
    }
    .to_string();
    let cpu = Cpu {
        model: cpus[0].brand().into(),
        cores: num_cpus::get_physical() as i32,
        manufacturer: cpu_manufacturer,
        speed: cpus[0].frequency() as f64 / 1000.0 / 1000.0,
        threads: num_cpus::get() as i32,
    };
    let hostname = System::host_name();
    let os = OperatingSystem {
        name: System::name().unwrap_or(UNKNOWN_VALUE.into()),
        version: System::os_version(),
    };
    let kernel = Kernel {
        version: System::kernel_version().unwrap_or(UNKNOWN_VALUE.into()),
        architecture: System::cpu_arch(),
    };
    let bios = Bios {
        version: read_dmi("bios_version").unwrap_or(UNKNOWN_VALUE.into()),
        manufacturer: read_dmi("bios_vendor").unwrap_or(UNKNOWN_VALUE.into()),
    };
    let model = read_dmi("product_name").unwrap_or(UNKNOWN_VALUE.into());
    let manufacturer = read_dmi("sys_vendor").unwrap_or(UNKNOWN_VALUE.into());
    let mainboard = Mainboard {
        manufacturer: read_dmi("board_vendor").unwrap_or(UNKNOWN_VALUE.into()),
        version: read_dmi("board_version").unwrap_or(UNKNOWN_VALUE.into()),
        model: read_dmi("board_name").unwrap_or(UNKNOWN_VALUE.into()),
    };

    let drives = get_block_devices().ok();

    Device {
        id: machine_uid::get().unwrap_or(uuid::Uuid::new_v4().to_string()),
        hostname,
        model,
        manufacturer,
        os: Some(os),
        storage: drives.as_ref().map(|d| d.iter().map(|d| d.size).sum()),
        ram: Some(sys.total_memory() as f64 / 1024f64 / 1024f64 / 1024f64),
        cpu: Some(cpu),
        bios: Some(bios),
        mainboard: Some(mainboard),
        kernel: Some(kernel),
        drives,
    }
}
