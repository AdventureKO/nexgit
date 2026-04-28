# Nexgit

Nexgit is an early-stage open-source project for building a better Git workflow tool: a serious headless CLI, a keyboard-first TUI, and a polished desktop app for stacked changes and GitHub-style collaboration.

This repository is being developed as a university club project. The goal is to be welcoming to new contributors while still using practices that look like a real open-source project.

> Status: **pre-alpha scaffold**. The CLI, TUI, app-server, and desktop app are wired together, but most Git/product behavior is still placeholder logic.

## What Nexgit is trying to become

Nexgit aims to provide:

- Headless commands for scripts and automation.
- A Ratatui terminal UI for fast keyboard workflows.
- A desktop GUI inspired by stacked-diff tools like Graphite.
- A local app-server exposed by the Rust CLI, similar in shape to Codex's app-server model.
- Shared Rust core logic so the CLI, TUI, and desktop app behave consistently.

## Current architecture

```text
Desktop React UI
  ↓ window.nexgit preload API
Electron main process
  ↓ newline-delimited JSON over stdio://
nexgit app-server
  ↓
Rust core logic
```

The desktop app starts the Rust CLI in app-server mode:

```bash
nexgit app-server --listen stdio://
```

The app-server also has scaffolding for:

```bash
nexgit app-server --listen ws://127.0.0.1:0
nexgit app-server --listen unix://
```

## Repository layout

```text
apps/
  cli/        Rust CLI binary, headless commands, Ratatui TUI, app-server
  desktop/    Electron + React + TypeScript desktop frontend

crates/
  core/       Shared Rust product/domain logic
  protocol/   App-server protocol types and generated TypeScript definitions

docs/         Contributor, architecture, development, and maintainer docs
```

## Quick start

Nexgit supports two development environment entrypoints. Choose one.

### Option A: mise

```bash
mise install
pnpm install
pnpm check
```

### Option B: Nix flake

```bash
nix develop
pnpm install
pnpm check
```

See [Development environments](docs/environments.md) for details. The project docs intentionally avoid separate manual toolchain installation instructions; `mise.toml` and `flake.nix` are the supported environment definitions.

### Run the CLI/TUI

```bash
cargo run -p nexgit-cli -- tui
```

### Run headless command stubs

```bash
cargo run -p nexgit-cli -- repo status --json
cargo run -p nexgit-cli -- stack list --json
```

### Run the app-server directly

```bash
cargo run -p nexgit-cli -- app-server --listen stdio://
```

Smoke test the protocol:

```bash
printf '{"type":"request","id":1,"method":"system.version","params":{}}\n' \
  | cargo run -q -p nexgit-cli -- app-server --listen stdio://
```

### Run the desktop app

```bash
pnpm desktop:dev
```

The desktop app will try to use `NEXGIT_CLI_PATH` first. If it is not set, it looks for `target/debug/nexgit`. If that binary is missing, it falls back to running Cargo directly.

## Useful commands

```bash
pnpm format
pnpm format:check
pnpm lint
pnpm typecheck
pnpm check
pnpm nix:check
cargo check --workspace
```

Root pnpm shortcuts:

```bash
pnpm tui
pnpm app-server
pnpm desktop:dev
pnpm desktop:build
pnpm desktop:typecheck
```

## Documentation

Start here if you are new:

- [Contributing guide](CONTRIBUTING.md)
- [New contributor onboarding](docs/onboarding.md)
- [Development setup](docs/development.md)
- [Development environments](docs/environments.md)
- [Tooling: linting, formatting, and LSP](docs/tooling.md)
- [Architecture overview](docs/architecture.md)
- [Product planning](docs/product/README.md)
- [v0.1.0 - GitHub App-Server Data milestone](https://github.com/Codelab-Davis/nexgit/milestone/1)
- [Glossary](docs/glossary.md)
- [Club guide](docs/club-guide.md)
- [Code of conduct](CODE_OF_CONDUCT.md)

Maintainers should also read:

- [Governance](GOVERNANCE.md)
- [Maintainer guide](docs/maintainers.md)
- [Security policy](SECURITY.md)

## How to contribute

You do not need to be an expert in Rust, TypeScript, Electron, or Git internals to help.

Good first contributions include:

- Improving documentation.
- Adding examples and screenshots.
- Cleaning up error messages.
- Improving the placeholder TUI layout.
- Adding small protocol methods.
- Writing tests for existing behavior.
- Filing clear bugs or design notes.

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## License

Nexgit is dual-licensed under either the [MIT License](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option. See
[LICENSE](LICENSE) for the contribution licensing notice.
