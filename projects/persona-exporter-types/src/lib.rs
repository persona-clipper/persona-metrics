//! [![GitHub]](https://github.com/0DoubleDare/persona-exporter-types)&ensp;[![crates-io]](https://crates.io/crates/persona-exporter-types)&ensp;[![docs-rs]](crate)
//!
//! [GitHub]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! **Important**: *The crate is constantly updated and can undergo
//! significant changes.*
//!
//! A separate, lightweight crate containing all the structures
//! to be populated with the corresponding information.
//! This crate is used in a custom [exporter](https://github.com/0DoubleDare/persona-exporter).
//!
//! **Also important**:
//! All types and fields are primarily based on the [sysinfo](https://docs.rs/sysinfo/0.39.5/sysinfo/)
//! metrics collection library, but you are free to use them however
//! and in whatever context you wish. The documentation simply describes
//! the purpose of the fields from the perspective of this crate's developer
//! and is relevant only if you are using [sysinfo](https://docs.rs/sysinfo/0.39.5/sysinfo/)
//! (That does, however, absolve me of the responsibility of finishing the documentation :D )
//!
//! Now, here is a brief summary of the information, making full use of
//! all the structures like [`SystemInfo`], [`MemoryInfo`], [`DiskInfo`], [`NetworkInfo`],
//! [`CpuInfo`], [`ComponentsInfo`] with [`ComponentInfo`]:
//!
//! ```text
//! system: Some(
//!     SystemInfo {
//!         name: "NixOS",
//!         kernel_version: "7.0.10-zen1",
//!         kernel_long_version: "Linux 7.0.10-zen1",
//!         distribution_id: "nixos",
//!         distribution_id_like: [],
//!         boot_time: 1782542462,
//!         uptime: 11778,
//!         cpu_arch: "x86_64",
//!         os_version: "26.05",
//!         host_name: "nikita",
//!     },
//! ),
//! memory: Some(
//!     MemoryInfo {
//!         total_memory: 16543600640,
//!         used_memory: 8603865088,
//!         available_memory: 7939735552,
//!         total_swap: 25451552768,
//!         used_swap: 0,
//!         free_swap: 25451552768,
//!         load_avg: LoadAverage {
//!             one: 4.25,
//!             five: 2.61,
//!             fifteen: 2.27,
//!         },
//!     },
//! ),
//! disk: Some(
//!     DiskInfo {
//!         name: "/dev/nvme0n1p2",
//!         file_system: "ext4",
//!         kind: "SSD",
//!         total_space: 501889327104,
//!         available_space: 146903207936,
//!     },
//! ),
//! network: Some(
//!     NetworkInfo {
//!         interface_name: "wlp0s20f3",
//!         total_rx_bytes: 181719771,
//!         total_rx_packets: 151380,
//!         total_rx_errors: 0,
//!         total_tx_bytes: 23412959,
//!         total_tx_packets: 59081,
//!         total_tx_errors: 0,
//!     },
//! ),
//! cpu: Some(
//!     CpuInfo {
//!         cpu_usage: 39.92095,
//!         threads: 8,
//!         physical_core_count: 4,
//!     },
//! ),
//! components: Some(
//!     ComponentsInfo {
//!         count: 8,
//!         is_empty: false,
//!         components: [
//!             ComponentInfo {
//!                 id: "hwmon4_3",
//!                 name: "coretemp Core 1",
//!                 temp: 66.0,
//!                 critical_temp: 100.0,
//!                 max_temp: 66.0,
//!             },
//!             ComponentInfo {
//!                 id: "hwmon4_2",
//!                 name: "coretemp Core 0",
//!                 temp: 92.0,
//!                 critical_temp: 100.0,
//!                 max_temp: 92.0,
//!             },
//!             ComponentInfo {
//!                 id: "hwmon4_5",
//!                 name: "coretemp Core 3",
//!                 temp: 65.0,
//!                 critical_temp: 100.0,
//!                 max_temp: 65.0,
//!             },
//!             ComponentInfo {
//!                 id: "hwmon4_4",
//!                 name: "coretemp Core 2",
//!                 temp: 85.0,
//!                 critical_temp: 100.0,
//!                 max_temp: 85.0,
//!             },
//!             ComponentInfo {
//!                 id: "hwmon4_1",
//!                 name: "coretemp Package id 0",
//!                 temp: 92.0,
//!                 critical_temp: 100.0,
//!                 max_temp: 92.0,
//!             },
//!             ComponentInfo {
//!                 id: "hwmon2_1",
//!                 name: "acpitz temp1",
//!                 temp: 95.0,
//!                 critical_temp: 0.0,
//!                 max_temp: 95.0,
//!             },
//!             ComponentInfo {
//!                 id: "hwmon0_1",
//!                 name: "nvme Composite 511BS0512HB",
//!                 temp: 53.85,
//!                 critical_temp: 79.85,
//!                 max_temp: 53.85,
//!             },
//!             ComponentInfo {
//!                 id: "hwmon5_1",
//!                 name: "iwlwifi_1 temp1",
//!                 temp: 49.0,
//!                 critical_temp: 0.0,
//!                 max_temp: 49.0,
//!             },
//!         ],
//!     },
//! ),
//! timestamp: 1782554241316,
//! }
//! ```
//!
//! In addition to the structures themselves, the crate provides
//! functions for converting them: see [`ConvertTo`]
//!

mod metrics;
mod traits;

pub use metrics::*;
pub use traits::*;
