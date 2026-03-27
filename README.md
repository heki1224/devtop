# devtop

A developer-focused TUI system monitor written in Rust.

## Features

- Real-time CPU per-core sparklines with average usage
- Memory / Swap usage gauges
- Process list with developer context icons ([R]ust, [N]ode, [P]ython, [D]ocker)
- Sort by CPU / Memory / PID (`s` key)
- Filter processes by name (`/` key)

## Install

> crates.io publish coming soon (v0.1.0)

```bash
cargo install devtop
```

## Build from source

```bash
git clone https://github.com/heki1224/devtop
cd devtop
cargo build --release
./target/release/devtop
```

## Usage

```bash
devtop
```

### Keybindings

| Key | Action |
|---|---|
| `q` | Quit |
| `↑` / `↓` | Navigate process list |
| `s` | Cycle sort (CPU → Memory → PID) |
| `/` | Reset filter |
| Any char | Filter processes by name |
| `Backspace` | Delete filter character |

## Roadmap

- [ ] v0.2: Docker container integration
- [ ] v0.3: Network stats panel
- [ ] v0.4: Disk I/O panel
- [ ] v0.5: Theme system (multiple color themes via TOML)
- [ ] v0.6: Config file support (`~/.config/devtop/config.toml`)

## License

MIT
