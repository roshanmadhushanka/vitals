use sysinfo::Disks;
use crate::types::{DiskInfo, DiskStats};

pub struct DiskCollector {
    disks: Disks,
}

impl DiskCollector {

    pub fn new() -> DiskCollector {
        Self {
            disks: Disks::new_with_refreshed_list(),
        }
    }

    pub fn collect(&mut self) -> DiskStats {
        self.disks.refresh(true);

        let disk_infos: Vec<DiskInfo> = self.disks
            .iter()
            .map(|disk| -> DiskInfo {
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total.saturating_sub(available);

                DiskInfo {
                    name: disk.name().to_string_lossy().to_string(),
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    total_bytes: total,
                    used_bytes: used,
                    available_bytes: available,
                    usage_percent: if total > 0 {
                        (used as f32 / total as f32) * 100.0
                    } else {
                        0.0
                    },
                    file_system: disk.file_system().to_string_lossy().to_string(),
                }
            }).collect();

        DiskStats {
            disks: disk_infos,
        }
    }
}

impl Default for DiskCollector {

    fn default() -> Self {
        Self::new()
    }
}