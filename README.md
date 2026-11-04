# GitSwitch (v1.8.0) ⚡

> **Precision Offline-First Desktop Utility for Managing Multiple GitHub Accounts, Cryptographic SSH Identities, and Automatic Folder-Based Git Switching (`includeIf` Hooks)**

---

## 🎨 Visual Aesthetic & Philosophy

GitSwitch synthesizes utilitarian terminal conventions with precision-engineered developer ergonomics:
- **Base Surfaces**: Charcoal Obsidian (`#090e15`, `#0b141c`, `#0d1117`)
- **Neon Accents**: Active Merged Mint (`#3fb950`, `#2ea043`)
- **Secondary Telemetry**: Branch Blue (`#38bdf8`, `#58a6ff`) & Cryptographic Violet (`#6366f1`, `#bc8cff`)
- **Typography**: Dual-pairing of `Geist` for crisp structural hierarchy and `JetBrains Mono` / `Fira Code` for hashes, keys, and commands.

---

## 🏗️ Architecture & Tech Stack

```
GitSwitch/
├── wails.json                   # Wails v2 Configuration
├── go.mod                       # Go 1.21 Module with Wails v2
├── main.go                      # Wails Desktop Application Entrypoint (Windows Mica/Dark Options)
├── app.go                       # Core Go Backend Engine (Direct ~/.gitconfig & ~/.ssh/config I/O)
├── desktop-server.js            # Standalone Zero-Install Desktop Runner
├── run-desktop.bat              # One-Click Launch Script for Windows
└── frontend/
    ├── package.json             # Svelte, Vite, Tailwind CSS, Bits UI
    ├── svelte.config.js
    ├── vite.config.ts
    ├── tailwind.config.js
    ├── postcss.config.js
    ├── tsconfig.json
    ├── dist/                    # Embedded Production Bundle (zero-dependency Wails asset server)
    │   └── index.html
    └── src/
        ├── app.css              # Terminal scanlines, knurled dial styling, neon glow
        ├── main.ts              # Svelte app entry
        ├── App.svelte           # Unified application shell with keyboard shortcuts (⌘1, ⌘2, ⌘N, ⌘,)
        ├── types/               # TypeScript interfaces (Profile, FolderMapping, SSHKeyInfo, etc.)
        ├── stores/              # Reactive Svelte Stores (profiles, mappings, ssh, status)
        ├── components/
        │   ├── ui/              # Modular Bits UI Primitive Wrappers:
        │   │   ├── Button.svelte
        │   │   ├── Dialog.svelte
        │   │   ├── Tabs.svelte
        │   │   ├── Select.svelte
        │   │   ├── Switch.svelte
        │   │   ├── Badge.svelte
        │   │   ├── Card.svelte
        │   │   └── Separator.svelte
        │   ├── DashboardRotary.svelte     # Screen 1: Central VST Audio Plugin Rotary Knob
        │   ├── ProfileCard.svelte         # Screen 1: Terminal-Style Active Preview & Action Deck
        │   ├── ProfilesView.svelte        # Screen 2: Identities Matrix & Quick Switchers
        │   ├── Modals/
        │   │   ├── AddProfileModal.svelte # Screen 3: Add New GitHub Identity Modal + Stream Log
        │   │   └── ConfigViewerModal.svelte # Live Host Configuration Inspector
        │   ├── FolderMappingsView.svelte  # Screen 4: Automatic Directory Detection & Live PWD Probe
        │   ├── SSHKeysView.svelte         # Screen 5: Keyring Manager & Interactive Terminal Probe
        │   └── TitleBar.svelte            # macOS-style window bullets, daemon telemetry & tabs
        └── wailsjs/                       # Go Bindings & Runtime Bridges
```

---

## ⚡ Core Screens & Features

### 1. Dashboard (Rotary Dial UI)
- **Central Rotary Knob**: VST console-style interactive metallic dial with GitHub emblem.
- **Dynamic Circumferential Ticks**: `01. WORK`, `02. PERSONAL`, `03. CLIENT`, `04. ENTERPRISE` placed around a 240-degree arc.
- **Interactive Switching**: Supports clicking circumferential ticks or dragging the knob with mouse to switch profiles.
- **Active Preview Card**: Shows user handles, committing email, active SSH key, scope paths, switch latency stopwatch (`< 0.5ms`), with **"Apply to Terminal"** and **"Test Probe"** buttons.

### 2. Profiles Management Screen
- Card-based layout displaying all saved profiles with green glow accent pillar on active profile.
- Keyboard shortcuts (`⌘1`, `⌘2`, `⌘3`), active badges, bound folder scopes, and quick-switch toggles.

### 3. Add New GitHub Account Modal
- Bits UI Dialog form with fields:
  - `01. Profile Identifier` (with color/tag picker)
  - `02. GitHub Username` (`user.name`)
  - `03. Committing Email` (`user.email`)
  - `04. Associated SSH Key` selector (Existing keys or Generate New Ed25519 keypair) with Enclave Safe fingerprint preview.
  - `05. Auto-Switch Scope Paths` (`gitdir:includeIf hook`)
- **"Save & Link Identity"**: Plays smooth real-time terminal stream animation logging each step.

### 4. Folder Mappings (Automatic Profile Detection)
- Management panel for `includeIf.gitdir` hooks with `STRICT_DISPATCH` toggle.
- **Real-Time PWD Path Resolver**: Interactive path input bar (`pwd: ~/Developer/Work/infra-core`) with live evaluation against configured workspaces.
- Displays workspace status (`includeIf ACTIVE`, `includeIf READY`) and unlink tools.

### 5. SSH Keys & Cryptographic Identities Manager
- Agent status bar: `gittu://crypt/ssh-agent`, PID, keys in memory, TTL caching.
- Registered key cards displaying algorithms (Ed25519 256-bit, RSA 4096-bit), SHA-256 fingerprints with one-click copy, and "Copy Public Key".
- **Interactive Terminal Connection Test Console**: Live command echo (`$ ssh -T -i ~/.ssh/id_ed25519_work git@github.com`), OpenSSH debug output stream, and GitHub handshake greeting.

### 6. Host Configuration Inspector (`⌘,`)
- Live inspector for inspecting generated `~/.gitconfig` and `~/.ssh/config` files with syntax preservation and one-click copy.

---

## 🚀 Running the Application

### Option A: One-Click Desktop Runner (No Toolchain Required)
Double-click `run-desktop.bat` or run:
```bash
node desktop-server.js
```
This starts the local daemon on `http://127.0.0.1:49201` and opens Microsoft Edge in standalone App Mode (`--app`) with custom window sizing and dark theme.

### Option B: Wails v2 Build
If Go and Wails CLI are installed on your machine:
```bash
# Start live development mode with hot reload
wails dev

# Build production binary (gitswitch.exe)
wails build
```

---

## 🔒 Security & Privacy Notice
- Private keys never leave your machine (`~/.ssh/`).
- File updates to `~/.gitconfig` and `~/.ssh/config` are executed atomically with automatic `.bak` backups.
- Offline-first: zero telemetry sent to external third parties.
