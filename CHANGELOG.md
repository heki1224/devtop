# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2026-03-29

### Added
- Disk I/O panel: per-device read/write bytes/sec using sysinfo `Disks` delta API
- macOS system volume filtering (`/System/Volumes/*`)
- Device basename display with mount point column

## [0.3.0] - 2026-03-28

### Added
- Network stats panel: per-interface RX/TX bytes/sec using sysinfo `Networks` delta API
- Loopback interface filtering (`lo` / `lo0`)

## [0.2.0] - 2026-03-28

### Added
- Docker container panel: container name, status, CPU%, memory usage
- Auto-hides when Docker daemon is unavailable
- Async Docker stats collection via bollard

## [0.1.0] - 2026-03-27

### Added
- Real-time CPU per-core sparklines with 60-step history
- Memory / Swap usage gauges
- Process list with developer context icons (`[R]`ust, `[N]`ode, `[P]`ython, `[D]`ocker)
- Sort by CPU / Memory / PID (`s` key)
- Process filter by name (any char / `Backspace` / `/` to reset)
- CI workflow with clippy `-D warnings`
