use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::collectors::{
    CpuCollector, DiskCollector, MemoryCollector, NetworkCollector, ProcessCollector,
};
use crate::types::{MonitorConfig, SystemStats};

pub enum MonitorCommand {
    SetInterval(u64), // Update the collection interval
    Shutdown,         // Shutdown the monitor
}

pub struct SystemMonitor {
    thread_handle: Option<JoinHandle<()>>, // Handle to the spawned thread
    command_tx: Sender<MonitorCommand>,    // Channel to send commands to the spawned thread
    is_running: Arc<AtomicBool>, // AtomicBool allows thread safe read/write while ARC allows shared ownership across threads.
}

impl SystemMonitor {
    pub fn start(stats_tx: Sender<SystemStats>, config: MonitorConfig) -> Self {
        let (command_tx, command_rx) = mpsc::channel();

        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_clone = Arc::clone(&is_running);

        let thread_handle = thread::spawn(move || {
            Self::monitor_loop(
                command_rx,
                stats_tx,
                config,
                // ownership transfers to thread with move
                // that's why we need to pass the clone instead of is_running which we return as an output attribute
                is_running_clone,
            )
        });

        SystemMonitor {
            thread_handle: Some(thread_handle),
            command_tx,
            is_running,
        }
    }

    fn monitor_loop(
        command_rx: Receiver<MonitorCommand>,
        stats_tx: Sender<SystemStats>,
        mut config: MonitorConfig,
        is_running: Arc<AtomicBool>,
    ) {
        let mut cpu_collector = CpuCollector::new();
        let mut memory_collector = MemoryCollector::new();
        let mut disc_collector = DiskCollector::new();
        let mut network_collector = NetworkCollector::new();
        let mut process_collector = ProcessCollector::new(config.process_limit);

        // Memory ordering sequential consistency
        // Strongest ordering that prevents reordering across threads and gives a single global order
        while is_running.load(Ordering::SeqCst) {
            match command_rx.try_recv() {
                Ok(MonitorCommand::SetInterval(interval)) => {
                    config.update_interval_ms = interval;
                }
                Ok(MonitorCommand::Shutdown) => {
                    is_running.store(false, Ordering::SeqCst);
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            }

            let stats = SystemStats {
                cpu: cpu_collector.collect(),
                memory: memory_collector.collect(),
                disk: disc_collector.collect(),
                network: network_collector.collect(),
                process: process_collector.collect(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
            };

            if let Err(_) = stats_tx.send(stats) {
                break;
            }

            thread::sleep(Duration::from_millis(config.update_interval_ms));
        }
    }

    pub fn send_command(
        &self,
        command: MonitorCommand,
    ) -> Result<(), mpsc::SendError<MonitorCommand>> {
        self.command_tx.send(command)
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn shutdown(&mut self) {
        let _ = self.send_command(MonitorCommand::Shutdown);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for SystemMonitor {
    fn drop(&mut self) {
        self.shutdown();
    }
}
