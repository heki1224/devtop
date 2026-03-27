use tokio::sync::mpsc;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;
use crate::types::{CollectorMessage, ContainerInfo, MemoryInfo, ProcessInfo, SortMode};

pub struct App {
    pub cpu_history: Vec<Vec<f64>>,  // [core_index][time_index] スパークライン用
    pub memory: MemoryInfo,
    pub processes: Vec<ProcessInfo>,
    pub filter: String,
    pub sort_mode: SortMode,
    pub selected: usize,
    pub should_quit: bool,
    rx: mpsc::Receiver<CollectorMessage>,
    pub containers: Vec<ContainerInfo>,
    pub docker_available: bool,
}

const CPU_HISTORY_LEN: usize = 60;

impl App {
    pub fn new(rx: mpsc::Receiver<CollectorMessage>) -> Self {
        Self {
            cpu_history: Vec::new(),
            memory: MemoryInfo { total: 1, used: 0, swap_total: 1, swap_used: 0 },
            processes: Vec::new(),
            filter: String::new(),
            sort_mode: SortMode::Cpu,
            selected: 0,
            should_quit: false,
            rx,
            containers: Vec::new(),
            docker_available: false,
        }
    }

    /// collectorからのメッセージを非ブロッキングで処理
    pub fn tick(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                CollectorMessage::Cpu(cores) => {
                    if self.cpu_history.len() != cores.len() {
                        self.cpu_history = vec![Vec::new(); cores.len()];
                    }
                    for (i, &usage) in cores.iter().enumerate() {
                        self.cpu_history[i].push(usage);
                        if self.cpu_history[i].len() > CPU_HISTORY_LEN {
                            self.cpu_history[i].remove(0);
                        }
                    }
                }
                CollectorMessage::Memory(info) => {
                    self.memory = info;
                }
                CollectorMessage::Process(procs) => {
                    self.processes = self.apply_sort(procs);
                }
                CollectorMessage::Docker(containers) => {
                    self.docker_available = true;
                    self.containers = containers;
                }
            }
        }
    }

    pub fn apply_sort(&self, mut procs: Vec<ProcessInfo>) -> Vec<ProcessInfo> {
        match self.sort_mode {
            SortMode::Cpu => procs.sort_by(|a, b| {
                b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
            }),
            SortMode::Memory => procs.sort_by(|a, b| b.memory_kb.cmp(&a.memory_kb)),
            SortMode::Pid => procs.sort_by(|a, b| a.pid.cmp(&b.pid)),
        }
        procs
    }

    pub fn filtered_processes(&self) -> Vec<&ProcessInfo> {
        if self.filter.is_empty() {
            self.processes.iter().collect()
        } else {
            self.processes.iter()
                .filter(|p| p.name.to_lowercase().contains(&self.filter.to_lowercase()))
                .collect()
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Down => {
                let len = self.filtered_processes().len();
                if len > 0 { self.selected = (self.selected + 1).min(len - 1); }
            }
            KeyCode::Up => {
                if self.selected > 0 { self.selected -= 1; }
            }
            KeyCode::Char('s') => {
                self.sort_mode = self.sort_mode.next();
                self.processes = self.apply_sort(self.processes.clone());
                self.selected = 0;
            }
            KeyCode::Char('/') => {
                self.filter.clear();
            }
            KeyCode::Char(c) if c != '/' => {
                self.filter.push(c);
                self.selected = 0;
            }
            KeyCode::Backspace => {
                self.filter.pop();
            }
            _ => {}
        }
    }

    pub fn poll_event(&self) -> anyhow::Result<Option<KeyEvent>> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                return Ok(Some(key));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ProcessType;

    fn make_process(name: &str, cpu: f32, mem: u64, pid: u32) -> ProcessInfo {
        ProcessInfo { pid, name: name.to_string(), cpu_usage: cpu, memory_kb: mem, process_type: ProcessType::Other }
    }

    #[test]
    fn test_filter_processes() {
        let (tx, rx) = mpsc::channel(10);
        drop(tx);
        let mut app = App::new(rx);
        app.processes = vec![
            make_process("cargo", 10.0, 100, 1),
            make_process("bash", 1.0, 50, 2),
        ];
        app.filter = "car".to_string();
        let filtered = app.filtered_processes();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "cargo");
    }

    #[test]
    fn test_sort_by_memory() {
        let (tx, rx) = mpsc::channel(10);
        drop(tx);
        let mut app = App::new(rx);
        app.sort_mode = SortMode::Memory;
        let procs = vec![
            make_process("a", 5.0, 200, 1),
            make_process("b", 10.0, 100, 2),
        ];
        let sorted = app.apply_sort(procs);
        assert_eq!(sorted[0].name, "a");
    }

    #[test]
    fn test_app_docker_initial_state() {
        let (tx, rx) = mpsc::channel(10);
        drop(tx);
        let app = App::new(rx);
        assert!(!app.docker_available);
        assert!(app.containers.is_empty());
    }

    #[test]
    fn test_tick_handles_docker_message() {
        use crate::types::ContainerInfo;
        let (tx, rx) = mpsc::channel(10);
        let mut app = App::new(rx);
        let containers = vec![ContainerInfo {
            name: "web".to_string(),
            status: "running".to_string(),
            cpu_percent: 3.0,
            memory_bytes: 100,
            memory_limit: 1000,
        }];
        tx.blocking_send(CollectorMessage::Docker(containers)).unwrap();
        app.tick();
        assert!(app.docker_available);
        assert_eq!(app.containers.len(), 1);
        assert_eq!(app.containers[0].name, "web");
    }
}
