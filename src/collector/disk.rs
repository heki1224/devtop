use std::path::Path;
use std::time::{Duration, Instant};
use sysinfo::Disks;
use tokio::sync::mpsc;
use crate::types::{CollectorMessage, DiskInfo};

pub async fn start(tx: mpsc::Sender<CollectorMessage>) {
    let mut disks = Disks::new_with_refreshed_list();
    let mut last = Instant::now();

    // 初回は差分が 0 になるため 1 秒待ってから計測開始
    tokio::time::sleep(Duration::from_secs(1)).await;

    loop {
        let now = Instant::now();
        let elapsed_secs = now.duration_since(last).as_secs_f64().max(0.001);
        last = now;

        disks.refresh(false);

        let mut stats: Vec<DiskInfo> = disks
            .iter()
            .filter(|d| !d.mount_point().to_string_lossy().starts_with("/System/Volumes"))
            .map(|d| {
                let usage = d.usage();
                let name = Path::new(d.name())
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| d.name().to_string_lossy().into_owned());
                DiskInfo {
                    name,
                    mount_point: d.mount_point().to_string_lossy().into_owned(),
                    read_bps: (usage.read_bytes as f64 / elapsed_secs) as u64,
                    write_bps: (usage.written_bytes as f64 / elapsed_secs) as u64,
                }
            })
            .collect();

        stats.sort_by(|a, b| a.name.cmp(&b.name));

        if tx.send(CollectorMessage::Disk(stats)).await.is_err() {
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
