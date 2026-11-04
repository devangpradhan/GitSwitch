# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-11-04

### 🚀 Initial Production Release

GitSwitch v1.0.0 is an offline-first desktop utility application engineered for developers, systems engineers, and open-source maintainers to manage multiple GitHub accounts, cryptographic SSH identities, and automatic directory-based Git configuration switching (`includeIf` hooks).

---

### ✨ Core Features & Highlights

#### 1. Global Configuration Engine
- **Direct Host I/O**: Safely reads, parses, and atomically rewrites `~/.gitconfig` and `~/.ssh/config` files.
- **Zero-Latency State Switches**: Benchmarked at sub-millisecond execution (`< 0.5ms`, calibrated ~`0.18ms`).
- **Atomic Persistence**: Thread-safe storage with automatic `.bak` backups and local configuration stored in `os.UserConfigDir()/GitSwitch/config.json`.
- **Preserved User Directives**: Intelligently updates managed sections while preserving custom user aliases, credential helpers, merge tools, and third-party host configs.

#### 2. Screen 1: Precision Rotary Dial Dashboard
- **Interactive VST-Style Rotary Knob**: Audio console-style metallic rotary dial with knurled grip, needle notch, and central glowing GitHub emblem.
- **Circumferential Profile Nodes**: Dynamic ticks positioned along a 240° arc (`01. WORK`, `02. PERSONAL`, `03. CLIENT`, `04. ENTERPRISE`).
- **Dual Control Ergonomics**: Switch profiles via direct tick clicking or continuous mouse-drag rotation with angle snapping.
- **Active Profile Preview Card**: Monospace terminal telemetry card displaying handles, emails, active SSH key, scope paths, switch latency stopwatch, and quick-action buttons:
  - `⚡ Apply to Terminal`: Generates and copies environment export commands to clipboard.
  - `⚡ Test Probe`: Dispatches an instant SSH diagnostic test to GitHub.

#### 3. Screen 2: Profiles Management Matrix
- **Card-Based Identity Overview**: Grid view of configured accounts with active status badges (`CURRENT ACTIVE` vs `Standby`) and left glow accent pillars.
- **Quick-Switch Triggers & Shortcuts**: Keyboard bindings (`⌘1`, `⌘2`, `⌘3`, `⌘4`) for instantaneous identity swapping.
- **Credential & Scope Telemetry**: Displays GPG signing key verification (`0x9F4C2A1E`), associated SSH key types (Ed25519 / RSA), and bound directory scopes (`auto-switch on cd`).
- **Profile Deletion Protection**: Enforces safety safeguards against removing the sole remaining identity.

#### 4. Screen 3: Add New GitHub Identity Modal
- **Bits UI Dialog Primitive**: Accessible modal container with backdrop blur and dark terminal aesthetics.
- **Structured Fields**:
  - `01. Profile Identifier` (with interactive color and tag picker).
  - `02. GitHub Username` (`user.name`).
  - `03. Committing Email` (`user.email`).
  - `04. Associated SSH Key` selector (supporting existing keys or generating new Ed25519 keypairs) with Enclave Safe fingerprint previews.
  - `05. Auto-Switch Scope Paths` (`gitdir:includeIf hook`).
- **Real-Time Stream Animation**: Smooth terminal animation logging step-by-step progress (`> Initializing profile directory...`, `> Compiling gitdir:includeIf hook stanza...`, `> Writing ~/.gitconfig... [0.12ms] OK`).

#### 5. Screen 4: Folder Mappings & Automatic Profile Detection
- **`includeIf.gitdir` Management Matrix**: Overview of path-to-profile bindings evaluated sequentially by Git.
- **Interactive Live PWD Path Resolver**: Real-time test input (`pwd: <path>`) that instantly evaluates directory transition matches with live latency reporting.
- **`STRICT_DISPATCH` Toggle**: Switch between strict sub-shell dispatch and passive detection.
- **Mapping Controls**: Inline tools to add scope patterns (`~/Projects/*`) or unlink existing bindings.

#### 6. Screen 5: SSH Keys & Cryptographic Identities Manager
- **Keyring Overview**: Tracks keys in memory, passphrase cache TTL (`Session Persistent`), socket paths, and 24-hour switch metrics (`142 OK / 0 ERR`).
- **Cryptographic Key Cards**: Displays Ed25519 256-bit elliptic curve and RSA 4096-bit specs, SHA-256 fingerprints with one-click copy, and "Copy Public Key" triggers.
- **Integrated Diagnostic Terminal Console**:
  - Live command echo: `$ ssh -T -i ~/.ssh/<key> git@github.com`
  - OpenSSH debug output stream (`debug1: Connecting...`, `debug1: Connection established...`).
  - GitHub authentication handshake greeting (`Hi <username>! You have successfully authenticated, but GitHub does not provide shell access.`).
  - Roundtrip latency stopwatch (`28ms · TLS v1.3 · Handshake OK`).
  - Buffer clearing tool.

#### 7. Screen 6: Host Configuration Inspector (`⌘,`)
- **Direct Configuration Audit**: Live dual-pane inspector for reviewing generated `~/.gitconfig` and `~/.ssh/config` files with syntax formatting and one-click file copying.

---

### 🛠️ Architecture & System Design
- **Desktop Wrapper**: Wails v2 (Go backend) with native Windows Mica/Dark options and `embed.FS` asset server.
- **Frontend**: Svelte (TypeScript) styled with Tailwind CSS enforcing a dark hacker/terminal-inspired palette:
  - Charcoal Obsidian (`#090e15`, `#0b141c`, `#0d1117`)
  - Neon Mint Accents (`#3fb950`, `#2ea043`)
  - Branch Blue (`#38bdf8`) & Cryptographic Violet (`#6366f1`)
- **Component Primitive System**: Custom Bits UI wrapper wrappers (`Button`, `Dialog`, `Tabs`, `Select`, `Switch`, `Badge`, `Card`, `Separator`).
- **Zero-Install Desktop Runner**: Bundled `desktop-server.js` and `run-desktop.bat` enabling instant windowed execution via Microsoft Edge App Mode (`--app`).
- **Security Guarantee**: Offline-first architecture; private keys never leave local disk (`~/.ssh/`); zero external network telemetry.
