# Life in Weeks

A dynamic wallpaper generator that visualizes your life in weeks. Track your time, make it count.

![Life in Weeks Preview](https://via.placeholder.com/800x450/1c1c20/ff7864?text=Life+in+Weeks)

## Features

- **Three Visualization Modes:**
  - **Life in Weeks** - Your entire life from birth to expected lifespan
  - **Year End** - Weeks remaining until the end of the current year
  - **Next N Months** - Upcoming weeks for the next few months

- **Beautiful Themes:**
  - Soft Dark (default) - Easy on the eyes
  - Terminal Green - Matrix vibes for terminal enthusiasts
  - Minimal Ink - Classic poster aesthetic
  - Sunset Gradient - Warm to cool past-to-future fade

- **Weekly Auto-Update** - Wallpaper regenerates automatically each week

- **Cross-Platform** - Works on Windows and macOS

## Installation

### CLI (for power users)

Download the latest release from [GitHub Releases](https://github.com/atsedeweyn/life-in-weeks/releases).

**macOS/Linux:**
```bash
# Download and make executable
curl -L https://github.com/atsedeweyn/life-in-weeks/releases/latest/download/liw-linux-amd64 -o liw
chmod +x liw
sudo mv liw /usr/local/bin/

# Or using cargo
cargo install liw-cli
```

**Windows:**
```powershell
# Download liw.exe from releases and add to PATH
```

### GUI App (for everyone)

Download the installer for your platform from [GitHub Releases](https://github.com/atsedeweyn/life-in-weeks/releases):
- Windows: `Life-in-Weeks_x.x.x_x64_en-US.msi`
- macOS: `Life-in-Weeks_x.x.x_x64.dmg`

## CLI Usage

```bash
# Generate and set wallpaper (life mode)
liw generate --mode life --dob 1995-03-20

# Generate year-end mode with terminal theme
liw generate --mode year-end --theme terminal

# Preview only (don't set as wallpaper)
liw generate --mode life --dob 1990-01-15 --preview

# Configure defaults
liw config set dob 1995-03-20
liw config set lifespan 80
liw config set theme dark

# Show current config
liw config show

# Install weekly auto-update
liw schedule install

# Check schedule status
liw schedule status
```

### Available Modes

| Mode | Description | Required Args |
|------|-------------|---------------|
| `life` | Entire life in weeks | `--dob` (date of birth) |
| `year-end` | Until December 31st | None |
| `next-months` | Next N months | `--months` (optional, default: 6) |

### Available Themes

| Theme | Description |
|-------|-------------|
| `dark` | Soft dark with coral accents |
| `terminal` | Green on black, Matrix style |
| `minimal` | Black on cream, classic poster |
| `sunset` | Warm gradient from past to future |

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- Node.js 18+ (for Tauri GUI)

### Build CLI

```bash
git clone https://github.com/atsedeweyn/life-in-weeks.git
cd life-in-weeks
cargo build --release -p liw-cli

# Binary will be at target/release/liw
```

### Build GUI (Tauri)

**Additional prerequisites for Tauri:**

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Visual Studio Build Tools with C++ workload

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
    libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev \
    pkg-config libglib2.0-dev libgtk-3-dev
```

**Build:**
```bash
npm install
npm run tauri build
```

## Project Structure

```
life-in-weeks/
├── crates/
│   ├── liw-core/      # Shared Rust library
│   │   ├── config.rs  # Configuration management
│   │   ├── modes.rs   # Date calculation logic
│   │   ├── renderer.rs # Image generation
│   │   ├── wallpaper.rs # Cross-platform wallpaper API
│   │   └── scheduler.rs # OS task scheduling
│   └── liw-cli/       # CLI binary
├── src-tauri/         # Tauri backend
└── tauri-app/src/     # Web frontend
```

## Configuration

Config file location:
- **Linux:** `~/.config/life-in-weeks/config.toml`
- **macOS:** `~/Library/Application Support/life-in-weeks/config.toml`
- **Windows:** `%APPDATA%\life-in-weeks\config.toml`

Example config:
```toml
dob = "1995-03-20"
lifespan_years = 80
theme = "dark"
screen_width = 1920
screen_height = 1080
default_mode = "life"
```

## How It Works

1. **Calculate weeks** - Based on your selected mode, we compute the total weeks and how many have passed
2. **Generate image** - A grid is rendered with past weeks filled, current week highlighted, and future weeks outlined
3. **Set wallpaper** - The image is saved and set as your desktop wallpaper using platform-specific APIs
4. **Schedule updates** - An OS-level scheduled task regenerates the wallpaper every Monday at 6 AM

## Philosophy

The "Life in Weeks" concept was inspired by Tim Urban's [Wait But Why post](https://waitbutwhy.com/2014/05/life-weeks.html). Seeing your life as a finite grid of weeks creates perspective and motivation to make each week count.

This tool brings that visualization to your desktop, updating weekly to remind you of the passage of time.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) for details.
