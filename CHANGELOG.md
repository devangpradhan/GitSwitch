# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-11-12

### Production Release

GitSwitch `v1.0.0` is an offline-first desktop application (Tauri v2 + Rust + Svelte 5) and Stdio MCP server for managing multiple GitHub accounts, SSH identities, and automatic directory-based Git configuration switching (`includeIf.gitdir`).

### Added
- **Clean First-Run Experience**: Starts with zero pre-seeded/demo credentials and automatically scans `~/.ssh/*.pub` for real local keys with SHA-256 fingerprints.
- **Tauri v2 Desktop App (`gitswitch`)**: Native lightweight shell with atomic `~/.gitconfig`, `~/.gitconfig-<id>`, and `~/.ssh/config` management and automatic `.gitswitch.bak` backups.
- **Interactive Rotary Dial Dashboard & Profiles Matrix**: Quick profile switching (`⌘1`–`⌘9` / `Ctrl+1`–`Ctrl+9`), onboarding empty states, and terminal environment export generator.
- **Ed25519 Key Generator & Real OpenSSH Probe**: Creates real keys in `~/.ssh/` via `ssh-keygen` and executes real `ssh -Tv -i <key> git@github.com` diagnostics without simulated fallbacks.
- **Stdio MCP Server (`gitswitch-mcp`)**: JSON-RPC `2.0` MCP server exposing 14 tools and 3 resources, plus `mcp_config.json`.
- **GitHub Actions CI & Release Workflows**: Automated `.github/workflows/ci.yml` and `.github/workflows/release.yml` for cross-platform GitHub Release downloads, plus MIT `LICENSE`.
