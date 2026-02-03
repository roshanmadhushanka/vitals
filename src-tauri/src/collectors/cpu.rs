use sysinfo::System;
use crate::types::{CpuInfo, CpuStats};

pub struct CpuCollector {
    sys: System,
}

impl CpuCollector {
    
    pub fn new() -> Self {
        Self {
            sys: System::new(),
        }
    }

    pub fn collect(&mut self) -> CpuStats {
        self.sys.refresh_cpu_all();

        let cores: Vec<CpuInfo> = self.sys
            .cpus()
            .iter()
            .map(|cpu| CpuInfo {
                name: cpu.name().to_string(),
                usage_percent: cpu.cpu_usage(),
                frequency_mhz: cpu.frequency(),
            })
            .collect();

        let total_usage = if cores.is_empty() {
            0.0
        } else {
            cores.iter().map(|c| c.usage_percent).sum::<f32>() / cores.len() as f32
        };

        CpuStats {
            core_count: cores.len(),
            cores,
            total_usage,
        }
    }
}

impl Default for CpuCollector {
    fn default() -> Self {
        Self::new()
    }
}