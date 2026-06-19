use std::path::Path;
use sysinfo::{System, Disks, Networks, Disk, Components, LoadAvg};

#[derive(Debug)]
pub struct ServerMetrics {
    pub sys_info: SystemInfo,
    pub disk_info: DiskInfo,
    pub network_data: NetworkInfo,
    pub timestamp: u64,
}

impl ServerMetrics {
    pub fn collect_system_metrics(sys: &mut System) -> SystemInfo {
        sys.refresh_all();

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let total_swap = sys.total_swap();
        let used_swap = sys.used_swap();

        let total_cores = sys.cpus().len();
        let threads = System::physical_core_count().unwrap_or(total_cores);

        let load_avg = System::load_average();

        SystemInfo {
            total_memory,
            used_memory,
            total_swap,
            used_swap,
            cpu_usage: sys.global_cpu_usage(),
            physical_cpus_count: total_cores,
            threads,
            load_average: [load_avg.one, load_avg.five, load_avg.fifteen],
        }
    }

    pub fn collect_disk_metrics(disks: &mut Disks, mount_point: &str) -> DiskInfo {
        let disk = disks.iter_mut().find(|disk| disk.mount_point() == Path::new(&mount_point));

        if let Some(disk) = disk {
            disk.refresh();

            let total_space = disk.total_space();
            let available_space = disk.available_space();

            return DiskInfo {
                total_space: total_space,
                available_space: available_space,
            }
        }

        DiskInfo { ..Default::default() }
    }

    pub fn collect_network_metrics(networks: &mut Networks) -> NetworkInfo {
        networks.refresh(false);

        let main_interface = networks.iter()
            .filter(|(name, _)| {
                let n = name.as_str();
                n != "lo" && !n.starts_with("br-") && !n.starts_with("docker") && !n.starts_with("veth")
            })
            .max_by_key(|(_, data)| {
                data.total_received() + data.total_transmitted()
            })
            .map(|(name, _)| name.clone());

        if let Some(interface) = main_interface {
            if let Some(data) = networks.get(&interface) {
                // println!("{}", data.)
                return NetworkInfo {
                    interface_name: interface,
                    total_rx_bytes: data.total_received(),
                    total_tx_bytes: data.total_transmitted(),
                    total_rx_package: data.total_packets_received(),
                    total_tx_package: data.total_packets_transmitted(),
                    total_rx_errors: data.total_errors_on_received(),
                    total_tx_errors: data.total_errors_on_transmitted(),
                }
            }
        }

        NetworkInfo {
            interface_name: "unknown".to_string(),
            ..Default::default()
        }
    }

    pub fn collect_components_metrics(components: &mut Components) {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct SystemInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub cpu_usage: f32,
    pub physical_cpus_count: usize,
    pub threads: usize,
    pub load_average: [f64; 3],
}

#[derive(Debug, Default)]
pub struct DiskInfo {
    pub total_space: u64,
    pub available_space: u64,
}

#[derive(Debug, Default)]
pub struct NetworkInfo {
    pub interface_name: String,

    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
    pub total_rx_package: u64,
    pub total_tx_package: u64,
    pub total_rx_errors: u64,
    pub total_tx_errors: u64
}

#[derive(Debug, Default)]
pub struct ComponentInfo {

}