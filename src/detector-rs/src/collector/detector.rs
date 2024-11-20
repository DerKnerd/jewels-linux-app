use crate::collector::{Bios, Cpu, Device, Drive, Kernel, Mainboard, OperatingSystem};
use smbioslib::*;
use sysinfo::{Disks, System};

const UNKNOWN_VALUE: &str = "Unbekannt";

pub fn collect_device_info() -> Device {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpus = sys.cpus();
    let cpu = Cpu {
        model: cpus[0].brand().into(),
        cores: sys.physical_core_count().unwrap_or(cpus.len()) as i32,
        manufacturer: cpus[0].vendor_id().into(),
        speed: cpus.iter().next().unwrap().frequency() as f64 / 1000f64 / 1000f64,
        threads: cpus.len() as i32,
    };
    let hostname = System::host_name();
    let os = OperatingSystem {
        name: System::name().unwrap_or(UNKNOWN_VALUE.into()),
        version: System::os_version(),
    };
    let kernel = Kernel {
        version: System::kernel_version().unwrap_or(UNKNOWN_VALUE.into()),
        architecture: System::cpu_arch().unwrap(),
    };
    let (bios, mainboard, manufacturer, cpu, model) = if let Ok(smbios) = table_load_from_device() {
        let mainboard = smbios
            .collect::<SMBiosBaseboardInformation>()
            .first()
            .map(|baseboard| Mainboard {
                manufacturer: baseboard.manufacturer().to_string(),
                version: baseboard.version().to_string(),
                model: baseboard.product().to_string(),
            });
        let bios = smbios
            .collect::<SMBiosInformation>()
            .first()
            .map(|info| Bios {
                manufacturer: info.vendor().to_string(),
                version: info.version().to_string(),
            });
        let (manufacturer, model) = smbios
            .collect::<SMBiosSystemInformation>()
            .first()
            .map(|info| {
                (
                    info.manufacturer().to_string(),
                    info.product_name().to_string(),
                )
            })
            .unwrap_or((UNKNOWN_VALUE.into(), UNKNOWN_VALUE.into()));
        let cpu = if let Some(smbios_cpu) = smbios
            .collect::<SMBiosProcessorInformation>()
            .iter()
            .filter(|cpu: &&SMBiosProcessorInformation| {
                cpu.processor_type()
                    .map(|t| t.value == ProcessorType::CentralProcessor)
                    .unwrap_or(false)
            })
            .collect::<Vec<&SMBiosProcessorInformation>>()
            .first()
        {
            let threads = if let Some(ThreadCount::Count(threads)) = smbios_cpu.thread_count() {
                threads as i32
            } else if let Some(ThreadCount::SeeThreadCount2) = smbios_cpu.thread_count() {
                if let Some(ThreadCount2::Count(threads)) = smbios_cpu.thread_count_2() {
                    threads as i32
                } else {
                    -1
                }
            } else {
                -1
            };
            let cores = if let Some(CoreCount::Count(cores)) = smbios_cpu.core_count() {
                cores as i32
            } else if let Some(CoreCount::SeeCoreCount2) = smbios_cpu.core_count() {
                if let Some(CoreCount2::Count(cores)) = smbios_cpu.core_count_2() {
                    cores as i32
                } else {
                    -1
                }
            } else {
                -1
            };
            let speed = if let Some(ProcessorSpeed::MHz(speed)) = smbios_cpu.current_speed() {
                speed as f64 / 1000f64
            } else {
                -1f64
            };
            let manufacturer = smbios_cpu.processor_manufacturer().to_string();

            Cpu {
                manufacturer,
                model: cpu.model,
                speed,
                cores,
                threads,
            }
        } else {
            cpu
        };

        (bios, mainboard, manufacturer, cpu, model)
    } else {
        (None, None, UNKNOWN_VALUE.into(), cpu, UNKNOWN_VALUE.into())
    };

    let disks = Disks::new_with_refreshed_list();
    let drives = disks
        .iter()
        .map(|disk| Drive {
            name: disk
                .name()
                .to_str()
                .map(|val| val.to_string())
                .unwrap_or_default(),
            size: disk.total_space() as f64 / 1024f64 / 1024f64 / 1024f64,
        })
        .collect::<Vec<_>>();
    let storage = disks
        .iter()
        .map(|drive| drive.total_space())
        .reduce(|a, b| a + b)
        .map(|storage| storage as f64 / 1024f64 / 1024f64 / 1024f64);

    Device {
        id: machine_uid::get().unwrap_or(uuid::Uuid::new_v4().to_string()),
        hostname,
        model,
        manufacturer,
        os: Some(os),
        storage,
        ram: Some(sys.total_memory() as f64 / 1024f64 / 1024f64 / 1024f64),
        cpu: Some(cpu),
        bios,
        mainboard,
        kernel: Some(kernel),
        drives: Some(drives),
    }
}
