use std::time::Duration;
use sysinfo::{Disks, System, Networks};
use tokio::time::sleep;
use paas_core::structures::server_metrics::ServerMetrics;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut sys = System::new();
    let mut disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();
    tracing::info!("Starting persona-agent-rs");

    loop {
        let sys_info = ServerMetrics::collect_system_metrics(&mut sys);
        let disk_info = ServerMetrics::collect_disk_metrics(&mut disks,  "/");
        let network_info = ServerMetrics::collect_network_metrics(&mut networks);

        let machine_metrics = ServerMetrics {
            sys_info,
            disk_info,
            network_data: network_info,
        };

        sleep(Duration::from_secs(5)).await;
    }
}
