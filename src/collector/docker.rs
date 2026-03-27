use std::collections::HashMap;
use std::time::Duration;
use bollard::Docker;
use bollard::container::{ListContainersOptions, Stats, StatsOptions};
use futures_util::StreamExt;
use tokio::sync::mpsc;
use crate::types::{CollectorMessage, ContainerInfo};

/// CPU%を計算する。prev_statsがない場合（初回）は0.0を返す。
/// prev: (prev_cpu_total, prev_system_cpu)
pub fn calculate_cpu_percent(
    cpu_total: u64,
    system_cpu: u64,
    online_cpus: u64,
    prev: Option<(u64, u64)>,
) -> f64 {
    let Some((prev_cpu, prev_system)) = prev else { return 0.0 };
    let cpu_delta = cpu_total.saturating_sub(prev_cpu) as f64;
    let system_delta = system_cpu.saturating_sub(prev_system) as f64;
    if system_delta <= 0.0 {
        return 0.0;
    }
    (cpu_delta / system_delta) * (online_cpus as f64) * 100.0
}

pub async fn start(tx: mpsc::Sender<CollectorMessage>) {
    let docker = match Docker::connect_with_local_defaults() {
        Ok(d) => d,
        Err(_) => return,
    };
    if docker.ping().await.is_err() {
        return;
    }

    let mut prev_stats: HashMap<String, (u64, u64)> = HashMap::new();

    loop {
        let containers = collect_containers(&docker, &mut prev_stats).await;
        if tx.send(CollectorMessage::Docker(containers)).await.is_err() {
            break;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn collect_containers(
    docker: &Docker,
    prev_stats: &mut HashMap<String, (u64, u64)>,
) -> Vec<ContainerInfo> {
    let options = ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    };
    let list = match docker.list_containers(Some(options)).await {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut result = Vec::new();

    for summary in list {
        let id = match &summary.id {
            Some(id) => id.clone(),
            None => continue,
        };
        let short_id = id.chars().take(12).collect::<String>();
        let name = summary
            .names
            .as_ref()
            .and_then(|n| n.first())
            .map(|n| n.trim_start_matches('/').to_string())
            .unwrap_or_else(|| short_id.clone());
        let status = summary.status.clone().unwrap_or_else(|| "unknown".to_string());

        let stats_opt = get_stats(docker, &id).await;

        let (cpu_percent, memory_bytes, memory_limit) = if let Some(stats) = stats_opt {
            let cpu_total = stats.cpu_stats.cpu_usage.total_usage;
            let system_cpu = stats.cpu_stats.system_cpu_usage.unwrap_or(0);
            let online_cpus = stats.cpu_stats.online_cpus.unwrap_or(1);

            let prev = prev_stats.get(&id).copied();
            let cpu_pct = calculate_cpu_percent(cpu_total, system_cpu, online_cpus, prev);
            prev_stats.insert(id.clone(), (cpu_total, system_cpu));

            let mem_usage = stats.memory_stats.usage.unwrap_or(0);
            let mem_limit = stats.memory_stats.limit.unwrap_or(0);
            (cpu_pct, mem_usage, mem_limit)
        } else {
            prev_stats.remove(&id);
            (0.0, 0, 0)
        };

        result.push(ContainerInfo {
            id: short_id,
            name,
            status,
            cpu_percent,
            memory_bytes,
            memory_limit,
        });
    }

    result
}

async fn get_stats(docker: &Docker, container_id: &str) -> Option<Stats> {
    let options = StatsOptions { stream: false, one_shot: true };
    let mut stream = docker.stats(container_id, Some(options));
    stream.next().await?.ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cpu_percent_no_prev() {
        let result = calculate_cpu_percent(1000, 10000, 4, None);
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_cpu_percent_normal() {
        // cpu_delta=100, system_delta=1000, cpus=4 → 100/1000*4*100 = 40.0%
        let result = calculate_cpu_percent(1100, 11000, 4, Some((1000, 10000)));
        assert!((result - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_cpu_percent_zero_system_delta() {
        let result = calculate_cpu_percent(1100, 10000, 4, Some((1000, 10000)));
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_cpu_percent_clamp_on_overflow() {
        // saturating_sub でアンダーフロー防止（prev > current の場合）
        let result = calculate_cpu_percent(500, 9000, 4, Some((1000, 10000)));
        assert!((result - 0.0).abs() < f64::EPSILON); // cpu_delta=0 (saturated)
    }
}
