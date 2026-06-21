<div align="center">

# WinForge

**Forge your perfect Windows setup — in minutes, not hours.**

WinForge is a standalone Windows desktop application that silently installs apps, applies system optimizations, and displays your hardware specs — all from a single, clean interface.

<br/>

[![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Backend-Rust-CE422B?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Windows](https://img.shields.io/badge/Platform-Windows%2010%2F11-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://www.microsoft.com/windows)
[![License: MIT](https://img.shields.io/badge/License-MIT-dfb743?style=for-the-badge)](LICENSE)
[![Stars](https://img.shields.io/github/stars/AwaisX007/WinForge?style=for-the-badge&color=dfb743)](https://github.com/AwaisX007/WinForge/stargazers)

</div>

---

## Why WinForge?

Setting up a fresh Windows PC is tedious. You open dozens of browser tabs, wait through installers, click through prompts, and tweak settings one by one.

WinForge changes that. **Select what you want. Hit install. Done.**

Everything runs silently in the background — no visible command prompt windows flashing, no installer pop-ups, and no system bloat. Just a clean setup experience.

---

## Features

### Silent App Installer
Install your entire software stack in one click using Windows Package Manager (`winget`) under the hood.

- 50+ curated apps across Developer Tools, Productivity, Creative, Gaming, and Utilities.
- **Installed App Detector** — scans on startup and marks already-installed apps with a gold badge so you never duplicate a download.
- **Post-Install Rescan** — automatically updates the UI state after installation finishes.

### Reversible System Tweaks
Apply and undo powerful Windows optimizations safely:

| Tweak | Description |
|-------|-------------|
| Clean Temp Files | Wipes temporary directories, prefetch, and flushes DNS |
| Debloat Windows | Removes pre-installed Microsoft bloatware |
| Ultimate Performance | Activates the hidden Ultimate Performance power plan |
| Visual Effects | Disables animations for a snappier responsiveness |
| Network Optimizer | Tunes TCP stack with Auto-Tuning and Chimney offload |
| Dark Mode | Enables system-wide dark mode |
| Disable Telemetry | Stops Windows tracking and diagnostic services |
| God Mode | Creates the all-settings God Mode folder on your Desktop |
| Daily Cleanup Task | Schedules automatic nightly cleanup at 3:00 AM |

Every tweak is **reversible** — WinForge detects applied tweaks on startup and displays a Revert button where applicable.

### PC Specs Dashboard
A live system info panel that pulls real hardware data:

- **CPU** — Model name, core count, and thread count
- **GPU** — Graphics card model
- **RAM** — Total capacity and clock speed (MHz)
- **Storage** — Free and total space across all fixed drives
- **OS** — Windows edition and build number
- **Security** — UEFI Secure Boot status and TPM version

> Works without admin rights — uses a dual-path query: WMI/CIM when elevated, falling back to the built-in `tpmtool` parser for non-admin sessions.

### Premium UI/UX
- Sleek dark glassmorphic design with a gold accent system.
- Uniform icons rendered with precise CSS color-filter transforms (eliminates invisible dark icons on dark backgrounds).
- Smooth hover micro-animations and transitions.
- Subtle checkbox outlines that shift to gold on hover.
- Live progress overlay with per-app status updates.
- **Copy Report** button to export your full setup log to the clipboard.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| UI | HTML5 + Vanilla CSS3 + JavaScript |
| Bundler | Vite |
| Desktop Shell | Tauri v1 |
| Backend | Rust (async Tokio, Tauri commands) |
| System Scripting | PowerShell (non-interactive, hidden execution) |
| Package Manager | Winget (locale-independent) |

---

## Getting Started

### Prerequisites

Make sure you have the following installed:

- [Node.js](https://nodejs.org/) — LTS version recommended
- [Rust & MSVC Toolchain](https://www.rust-lang.org/tools/install) — Required for Tauri compilation
- [Winget](https://learn.microsoft.com/en-us/windows/package-manager/winget/) — Built into Windows 10 (1709+) and Windows 11

### Clone & Install

```bash
git clone https://github.com/AwaisX007/WinForge.git
cd WinForge
npm install
```

### Development Mode

Launches the hot-reload Tauri development window:

```bash
npm run dev
```

### Production Build

Compiles the optimized Rust backend and bundles all assets into a single standalone executable:

```bash
npm run build
```

Output: `src-tauri/target/release/WinForge.exe`

---

## Project Structure

```
WinForge/
├── src/                    # Frontend (HTML, CSS, JavaScript)
│   ├── index.html          # Main application UI
│   └── assets/
│       └── icons/          # SVG icons for all apps and tweaks
├── src-tauri/              # Tauri + Rust backend
│   ├── src/
│   │   └── main.rs         # Tauri commands, PowerShell orchestration
│   ├── icons/              # App icons for the bundler
│   └── tauri.conf.json     # Tauri window and bundle configuration
├── .gitignore
├── package.json
└── README.md
```

---

## Contributing

Contributions are welcome. To get started:

1. Fork this repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -m "Add your feature"`
4. Push to your branch: `git push origin feature/your-feature`
5. Open a Pull Request

---

## License

This project is open-source and available under the [MIT License](LICENSE).
WinForge is code signed for free thanks to the [SignPath Foundation](https://signpath.org), which provides certificates to trusted open source projects.
---

<div align="center">

Made for the Windows community &nbsp;|&nbsp; Built with Rust & Tauri

</div>
