use crate::metrics::traits::Clear;
use compact_str::CompactString;

/// Network information
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct NetworkInfo {
    /// The name of your card's network interface
    pub interface_name: CompactString,

    /// Total bytes received since the network card was turned on
    pub total_rx_bytes: u64,
    /// Total data packets received
    pub total_rx_packets: u64,
    /// Total errors when accepting data
    pub total_rx_errors: u64,

    /// Total bytes transferred since the network card was turned on
    pub total_tx_bytes: u64,
    /// Total data packets transferred
    pub total_tx_packets: u64,
    /// Total errors when sending data
    pub total_tx_errors: u64,
}

impl Clear for NetworkInfo {
    fn clear_dynamic(&mut self) {
        self.interface_name.clear();
        // self.total_rx_bytes = 0;
        // self.total_rx_packets = 0;
        // self.total_rx_errors = 0;
        // self.total_tx_bytes = 0;
        // self.total_tx_packets = 0;
        // self.total_rx_errors = 0;
    }
}
