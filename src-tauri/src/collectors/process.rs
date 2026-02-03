use sysinfo::{System, ProcessStatus, Process};
use crate::types::{ProcessInfo, ProcessStats};

pub struct ProcessCollector {
    sys: System,
    limit: usize,
}

impl ProcessCollector {

    pub fn new(limit: usize) -> Self {
        let mut sys = System::new();
        Self { sys, limit }
    }

    pub fn collect(&mut self) -> ProcessStats {
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let mut processes: Vec<ProcessInfo> = self.sys.processes()
            .iter()
            .map(|(pid, process)| -> ProcessInfo {
                ProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string_lossy().to_string(),
                    cpu_usage: process.cpu_usage(),
                    memory_usage: process.memory(),
                    status: format!("{:?}", process.status()),
                    user: process.user_id().map(|u| format!("{:?}", u)),
                }
            }).collect();

        let total_count = processes.len();
        let running_count = self.sys
            .processes()
            .values()
            .filter(|p| matches!(p.status(), ProcessStatus::Run))
            .count();
        processes.sort_by(|a, b| {
            b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
        });

        processes.truncate(self.limit);

        ProcessStats {
            processes,
            total_count,
            running_count,
        }
    }
}