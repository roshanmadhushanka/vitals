use sysinfo::Networks;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::types::{NetworkInterface, NetworkStats};

struct PreviousReading {
    received: u64,
    transmitted: u64,
    timestamp: Instant,
}

pub struct NetworkCollector {
    networks: Networks,
    previous: HashMap<String, PreviousReading>,
}

impl NetworkCollector {

    pub fn new() -> NetworkCollector {
        Self {
            networks: Networks::new_with_refreshed_list(),
            previous: HashMap::new(),
        }
    }

    pub fn collect(&mut self) -> NetworkStats {
        self.networks.refresh(true);

        let now = Instant::now();
        let mut interfaces = Vec::new();
        let mut total_rx_rate = 0.0;
        let mut total_tx_rate = 0.0;

        for (name, network) in self.networks.iter() {
            let received = network.total_received();
            let transmitted = network.total_transmitted();

            let (rx_rate, tx_rate) = match  self.previous.get(name) {
                Some(prev) => {
                    let elapsed = now.duration_since(prev.timestamp);
                    let secs = elapsed.as_secs_f64();

                    if secs > 0.0 {
                        let rx = received.saturating_sub(prev.received) as f64 / secs;
                        let tx = transmitted.saturating_sub(prev.transmitted) as f64 / secs;
                        (rx, tx)
                    } else {
                        (0.0, 0.0)
                    }
                }
                None => (0.0, 0.0),
            };

            self.previous.insert(name.to_string(), PreviousReading {
                received,
                transmitted,
                timestamp: now,
            });

            total_rx_rate += rx_rate;
            total_tx_rate += tx_rate;

            interfaces.push(NetworkInterface {
                name: name.to_string(),
                received_bytes: received,
                transmitted_bytes: transmitted,
                rx_rate_bytes_sec: rx_rate,
                tx_rate_bytes_sec: tx_rate,
            })
        }

        NetworkStats {
            interfaces,
            total_rx_rate,
            total_tx_rate,
        }
    }
}

impl Default for NetworkCollector {

    fn default() -> Self {
        Self::new()
    }
}