use std::time::{Duration, Instant};
use sysinfo::Networks;
use tokio::sync::mpsc;
use crate::types::{CollectorMessage, NetworkInfo};

pub async fn start(tx: mpsc::Sender<CollectorMessage>) {
    let mut networks = Networks::new_with_refreshed_list();
    let mut last = Instant::now();

    // 初回は差分が 0 になるため 1 秒待ってから計測開始
    tokio::time::sleep(Duration::from_secs(1)).await;

    loop {
        let now = Instant::now();
        let elapsed_secs = now.duration_since(last).as_secs_f64().max(0.001);
        last = now;

        networks.refresh(false);

        let stats: Vec<NetworkInfo> = networks
            .iter()
            .filter(|(name, _)| *name != "lo" && *name != "lo0")
            .map(|(name, data)| NetworkInfo {
                name: name.clone(),
                rx_bps: (data.received() as f64 / elapsed_secs) as u64,
                tx_bps: (data.transmitted() as f64 / elapsed_secs) as u64,
            })
            .collect();

        if tx.send(CollectorMessage::Network(stats)).await.is_err() {
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
