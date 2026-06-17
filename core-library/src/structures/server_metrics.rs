pub struct SysInfoResponse {
    pub total_memory: u64,
    pub used_memory: u64,
    pub used_and_total_memory_percentage: u64,
    pub total_swap: u64,
    pub used_total_swap: u64,
    pub used_and_total_swap_percentage: u64,
    pub cpu_usage: f64,
    
}