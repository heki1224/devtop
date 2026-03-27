use std::time::Duration;
use sysinfo::{ProcessesToUpdate, System};
use tokio::sync::mpsc;
use crate::types::{CollectorMessage, ProcessInfo, ProcessType};

pub async fn start(tx: mpsc::Sender<CollectorMessage>) {
    let mut sys = System::new();

    loop {
        sys.refresh_processes(ProcessesToUpdate::All, true);
        let mut processes: Vec<ProcessInfo> = sys.processes().values()
            .map(|p| {
                let name = p.name().to_string_lossy().to_string();
                ProcessInfo {
                    pid: p.pid().as_u32(),
                    name: name.clone(),
                    cpu_usage: p.cpu_usage(),
                    memory_kb: p.memory() / 1024,
                    process_type: ProcessType::from_name(&name),
                }
            })
            .collect();

        processes.sort_by(|a, b| {
            b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
        });

        if tx.send(CollectorMessage::ProcessUpdate(processes)).await.is_err() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(2000)).await;
    }
}
