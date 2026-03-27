use std::time::Duration;
use sysinfo::System;
use tokio::sync::mpsc;
use crate::types::CollectorMessage;

pub async fn start(tx: mpsc::Sender<CollectorMessage>) {
    let mut sys = System::new();
    // 初回は2回refreshが必要（差分計算のため）
    sys.refresh_cpu_all();
    tokio::time::sleep(Duration::from_millis(200)).await;

    loop {
        sys.refresh_cpu_all();
        let usage: Vec<f64> = sys.cpus().iter()
            .map(|c| c.cpu_usage() as f64)
            .collect();
        if tx.send(CollectorMessage::Cpu(usage)).await.is_err() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}
