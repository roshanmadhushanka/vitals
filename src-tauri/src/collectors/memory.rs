use sysinfo::System;
use crate::types::MemoryStats;

pub struct MemoryCollector {
    sys: System,
}

impl MemoryCollector {

    pub fn new() -> Self {
        Self {
            sys: System::new(),
        }
    }

    pub fn collect(&mut self) -> MemoryStats {
        self.sys.refresh_memory();

        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        let available = self.sys.available_memory();

        let usage_percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        MemoryStats {
            total_bytes: total,
            used_bytes: used,
            available_bytes: available,
            usage_percent,
            swap_total: self.sys.total_swap(),
            swap_used: self.sys.used_swap(),
        }
    }
}

impl Default for MemoryCollector {

    fn default() -> Self {
        Self::new()
    }
}