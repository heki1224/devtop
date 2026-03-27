use std::time::Duration;
use sysinfo::System;
use tokio::sync::mpsc;
use crate::types::{CollectorMessage, MemoryInfo};

pub async fn start(tx: mpsc::Sender<CollectorMessage>) {
    let mut sys = System::new();

    loop {
        sys.refresh_memory();
        let info = MemoryInfo {
            total: sys.total_memory(),
            used: sys.used_memory(),
            swap_total: sys.total_swap(),
            swap_used: sys.used_swap(),
        };
        if tx.send(CollectorMessage::MemoryUpdate(info)).await.is_err() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}

#[cfg(test)]
mod tests {
    use crate::types::MemoryInfo;

    #[test]
    fn test_memory_info_fields() {
        let info = MemoryInfo {
            total: 16 * 1024 * 1024 * 1024,
            used: 8 * 1024 * 1024 * 1024,
            swap_total: 4 * 1024 * 1024 * 1024,
            swap_used: 1 * 1024 * 1024 * 1024,
        };
        assert_eq!(info.total, 16 * 1024 * 1024 * 1024);
        assert!(info.used < info.total);
    }
}
