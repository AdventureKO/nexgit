# New Contributor Onboarding

Welcome to Nexgit. This guide is for people who want to contribute but are not sure where to start.

## You are welcome here if

- You are new to open source.
- You are learning Rust, TypeScript, React, Electron, Git, or GitHub.
- You mostly want to help with docs, testing, design, or user experience.
- You are experienced and want to help mentor others.

## The project in one minute

Nexgit is a Git workflow tool with three user-facing surfaces:

1. **CLI**: scriptable headless commands.
2. **TUI**: terminal UI built with Ratatui.
3. **Desktop**: Electron + React app.

All of these should share the same Rust core logic.

```text
CLI commands ┐
TUI          ├─> Rust core logic
Desktop UI   ┘      ↑
              app-server protocol
```

## First setup

Choose one supported development environment entrypoint.

### Option A: mise

```bash
mise install
pnpm install
cargo check --workspace
```

### Option B: Nix flake

```bash
nix develop
pnpm install
cargo check --workspace
```

See [environments.md](environments.md) for more detail.

## First commands to try

Run the CLI help:

```bash
cargo run -p nexgit-cli -- --help
```

Run the placeholder TUI:

```bash
cargo run -p nexgit-cli -- tui
```

Run a headless command:

```bash
cargo run -p nexgit-cli -- repo status --json
```

### Scaffold command outputs

The following examples show what you should see when you run these commands. These outputs are **placeholder scaffold behavior** and confirm your build is working. As the project develops, these commands will return real Git and GitHub data instead of empty/null values.

Version information:

```bash
cargo run -p nexgit-cli -- version --json
```

Expected output (scaffold):

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

Expected output (scaffold):

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

Expected output (scaffold):

```json
[]
```

Run the desktop typecheck:

```bash
pnpm --filter @nexgit/desktop typecheck
```

Run the full project check:

```bash
pnpm check
```

## Suggested first contributions

### Documentation

Good files to improve:

- `README.md`
- `CONTRIBUTING.md`
- `docs/development.md`
- `docs/glossary.md`

Examples:

- Add a missing troubleshooting step.
- Explain an acronym.
- Add a screenshot once the UI has one.
- Fix stale command output.

### Rust CLI/TUI

Good starting points:

- Improve `apps/cli/src/tui/ui.rs` layout text.
- Improve command descriptions in `apps/cli/src/args.rs`.
- Add a small app-server method in `apps/cli/src/server/handler.rs`.

### Desktop UI

Good starting points:

- Improve `apps/desktop/src/renderer/src/App.tsx` layout.
- Improve styling in `apps/desktop/src/renderer/src/styles.css`.
- Add typed helper functions around `window.nexgit`.

## How to choose an issue

Look for issues labeled:

- `good first issue`
- `good for beginner`
- `good first design issue`
- `kind: docs`
- `help wanted`
- `area: cli`
- `area: desktop`
- `area: tui`

If labels are not set up yet, ask a maintainer or propose a small task.

## How to ask for a review

Open a pull request and include:

- What changed.
- Why you changed it.
- What commands you ran.
- Anything you want reviewers to focus on.

It is okay to say:

```text
This is my first Rust PR. I would appreciate extra feedback on style and structure.
```

## Learning goals for club members

This project is a good way to learn:

- Rust workspace organization.
- CLI design with `clap`.
- Terminal UIs with Ratatui.
- Electron process architecture.
- React + TypeScript app structure.
- JSON protocol design.
- GitHub collaboration workflows.
- Code review etiquette.

## If you get stuck

Try these steps:

1. Re-read the relevant doc.
2. Search existing issues and PRs.
3. Run the command again and copy the exact error.
4. Ask for help with context.

Getting stuck is part of contributing. Please ask questions.
