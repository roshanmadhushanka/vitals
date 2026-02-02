use serde::{Deserialize, Serialize};

// Stat Models

// Represents single core CPU usage
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,
    pub usage_percent: f32,
    pub frequency_mhz: u64,
}

// Overall CPU statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuStats {
    pub cores: Vec<CpuInfo>,
    pub total_usage: f32,
    pub core_count: usize,
}

// Memory statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub swap_total: u64,
    pub swap_used: u64,
}

// Single disk information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: f32,
    pub file_system: String,
}

// Overall disk stats
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiskStats {
    pub disks: Vec<DiskInfo>,
}

// Network interface information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
    pub rx_rate_bytes_sec: f64,
    pub tx_rate_bytes_sec: f64,
}

// Overall network statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    pub interfaces: Vec<NetworkInterface>,
    pub total_rx_rate: f64,
    pub total_tx_rate: f64,
}

// Single process information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub status: String,
    pub user: Option<String>,
}

// Process list with summary
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessStats {
    pub processes: Vec<ProcessInfo>,
    pub total_count: usize,
    pub running_count: usize,
}

// Complete system snapshot
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub disk: DiskStats,
    pub network: NetworkStats,
    pub process: ProcessStats,
    pub timestamp: u64,
}

// Monitor configuration
#[derive(Clone, Debug)]
pub struct MonitorConfig {
    pub update_interval_ms: u64,
    pub process_limit: usize,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            update_interval_ms: 1000,
            process_limit: 50,
        }
    }
}

impl SystemStats {
    pub fn empty() -> Self {
        Self {
            cpu: CpuStats {
                cores: vec![],
                total_usage: 0.0,
                core_count: 0,
            },
            memory: MemoryStats {
                total_bytes: 0,
                used_bytes: 0,
                available_bytes: 0,
                usage_percent: 0.0,
                swap_total: 0,
                swap_used: 0,
            },
            disk: DiskStats {
                disks: vec![]
            },
            network: NetworkStats {
                interfaces: vec![],
                total_rx_rate: 0.0,
                total_tx_rate: 0.0,
            },
            process: ProcessStats {
                processes: vec![],
                total_count: 0,
                running_count: 0,
            },
            timestamp: 0,
        }
    }
}

impl MemoryStats {

    pub fn format_used(&self) -> String {
        format_bytes(self.used_bytes)
    }

    pub fn format_total(&self) -> String {
        format_bytes(self.total_bytes)
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    match bytes {
        b if b >= TB => format!("{:.2} TB", b as f64 / TB as f64),
        b if b >= GB => format!("{:.2} GB", b as f64 / GB as f64),
        b if b >= MB => format!("{:.2} MB", b as f64 / MB as f64),
        b if b >= KB => format!("{:.2} KB", b as f64 / KB as f64),
        b => format!("{} B", b),
    }
}