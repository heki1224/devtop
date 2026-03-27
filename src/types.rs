// src/types.rs
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_kb: u64,
    pub process_type: ProcessType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessType {
    Cargo,
    Node,
    Python,
    Docker,
    Other,
}

impl ProcessType {
    pub fn from_name(name: &str) -> Self {
        match name {
            n if n.contains("cargo") || n.contains("rustc") => Self::Cargo,
            n if n.contains("node") || n.contains("npm") => Self::Node,
            n if n.contains("python") => Self::Python,
            n if n.contains("docker") || n.contains("containerd") => Self::Docker,
            _ => Self::Other,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Cargo => "[R]",
            Self::Node => "[N]",
            Self::Python => "[P]",
            Self::Docker => "[D]",
            Self::Other => "   ",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortMode {
    Cpu,
    Memory,
    Pid,
}

impl SortMode {
    pub fn next(&self) -> Self {
        match self {
            Self::Cpu => Self::Memory,
            Self::Memory => Self::Pid,
            Self::Pid => Self::Cpu,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Memory => "MEM",
            Self::Pid => "PID",
        }
    }
}

#[derive(Debug)]
pub enum CollectorMessage {
    Cpu(Vec<f64>),      // per-core usage 0.0-100.0
    Memory(MemoryInfo),
    Process(Vec<ProcessInfo>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_type_from_name() {
        assert_eq!(ProcessType::from_name("cargo"), ProcessType::Cargo);
        assert_eq!(ProcessType::from_name("node"), ProcessType::Node);
        assert_eq!(ProcessType::from_name("python3"), ProcessType::Python);
        assert_eq!(ProcessType::from_name("dockerd"), ProcessType::Docker);
        assert_eq!(ProcessType::from_name("bash"), ProcessType::Other);
    }

    #[test]
    fn test_sort_mode_cycle() {
        assert_eq!(SortMode::Cpu.next(), SortMode::Memory);
        assert_eq!(SortMode::Memory.next(), SortMode::Pid);
        assert_eq!(SortMode::Pid.next(), SortMode::Cpu);
    }
}
