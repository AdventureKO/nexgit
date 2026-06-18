# Development Guide

This guide explains how to run Nexgit locally after entering one of the supported development environments.

## Development environment entrypoints

Choose one supported setup path:

### Option A: mise

```bash
mise install
pnpm install
```

### Option B: Nix flake

```bash
nix develop
pnpm install
```

See [Development environments](environments.md) for details. The project docs intentionally do not maintain separate manual installation instructions for Rust, Node.js, pnpm, or other toolchain packages.

## Common commands

### Full check

```bash
pnpm check
```

### Formatting

```bash
pnpm format
pnpm format:check
```

### Rust checks

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

### Desktop checks

```bash
pnpm lint:desktop
pnpm --filter @nexgit/desktop typecheck
pnpm --filter @nexgit/desktop build
```

### Nix flake checks

```bash
pnpm nix:check
pnpm nix:fmt:check
```

### Docs/text check

```bash
pnpm format:check
git diff --check
```

This catches formatting issues, trailing whitespace, and some accidental text problems.

## Running the CLI

Show help:

```bash
cargo run -p nexgit-cli -- --help
```

Run the TUI:

```bash
cargo run -p nexgit-cli -- tui
```

Run headless commands:

```bash
cargo run -p nexgit-cli -- repo status --json
cargo run -p nexgit-cli -- stack list --json
```

### Scaffold command output examples

These commands currently return **placeholder scaffold behavior**. Verify your build is working by running them and confirming you see output matching the examples below. As development progresses, these outputs will contain real Git and GitHub data.

Version:

```bash
cargo run -p nexgit-cli -- version --json
```

Scaffold output:

```json
{
  "name": "nexgit",
  "version": "0.1.0",
  "protocolVersion": 1
}
```

Repository status:

```bash
cargo run -p nexgit-cli -- repo status --json
```

Scaffold output:

```json
{
  "root": null,
  "currentBranch": null,
  "isClean": true,
  "pendingChanges": 0
}
```

Stack list:

```bash
cargo run -p nexgit-cli -- stack list --json
```

Scaffold output:

```json
[]
```

## Running the app-server

Default stdio transport:

```bash
cargo run -p nexgit-cli -- app-server --listen stdio://
```

Smoke test with one request:

```bash
printf '{"type":"request","id":1,"method":"system.version","params":{}}\n' \
  | cargo run -q -p nexgit-cli -- app-server --listen stdio://
```

Expected shape:

```json
{"type":"ready","serverName":"nexgit","version":"0.1.0","protocolVersion":1,"transport":"stdio"}
{"type":"response","id":1,"ok":true,"result":{"name":"nexgit","protocolVersion":1,"version":"0.1.0"}}
```

WebSocket transport for development:

```bash
cargo run -p nexgit-cli -- app-server --listen ws://127.0.0.1:0
```

Unix socket transport on Unix-like systems:

```bash
cargo run -p nexgit-cli -- app-server --listen unix://
```

## Running the desktop app

After entering either supported environment and running `pnpm install`, start Electron development mode:

```bash
pnpm desktop:dev
```

The desktop app starts the CLI app-server automatically. Lookup order:

1. `NEXGIT_CLI_PATH`, if set.
2. `../../target/debug/nexgit`, relative to `apps/desktop`.
3. `cargo run -p nexgit-cli -- app-server --listen stdio://`.
4. `nexgit` from `PATH`.

To force a specific binary:

```bash
cargo build -p nexgit-cli
NEXGIT_CLI_PATH="$PWD/target/debug/nexgit" pnpm desktop:dev
```

## Generating TypeScript protocol definitions

```bash
cargo run -p nexgit-cli -- app-server generate-ts
```

The current generated output is a scaffold. As the protocol grows, we should either generate the desktop types from Rust or keep a clear process for updating both sides.

## Troubleshooting

### `pnpm` is not available

Make sure you entered one of the supported environments:

```bash
mise install
```

or:

```bash
nix develop
```

See [Development environments](environments.md).

### Desktop cannot find the CLI

Build the CLI first and set `NEXGIT_CLI_PATH`:

```bash
cargo build -p nexgit-cli
NEXGIT_CLI_PATH="$PWD/target/debug/nexgit" pnpm desktop:dev
```

### Cargo is slow the first time

The first `cargo check` downloads and compiles dependencies. Later runs should be faster.

### The TUI looks broken

Try a modern terminal emulator and make sure your terminal window is large enough. The current TUI is only a scaffold.

### TypeScript cannot find `window.nexgit`

Make sure `apps/desktop/src/renderer/src/env.d.ts` is included by `tsconfig.json`. If you moved files, update the include paths.

## Clean builds

Remove Rust build output:

```bash
cargo clean
```

Remove JavaScript dependencies and Electron build output:

```bash
rm -rf node_modules apps/desktop/node_modules apps/desktop/out
pnpm install
```
