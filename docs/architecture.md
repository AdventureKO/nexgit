# Architecture Overview

Nexgit is designed around one rule:

> Rust owns the product logic. CLI, TUI, and desktop are frontends over the same core behavior.

This helps prevent the desktop app and CLI from drifting apart.

## High-level shape

```text
apps/cli
  ├─ headless commands
  ├─ Ratatui TUI
  └─ app-server
        ↓
crates/core
        ↑
apps/desktop
  └─ Electron + React frontend talks to app-server
```

## Applications

### `apps/cli`

The Rust CLI produces the `nexgit` binary.

Responsibilities:

- Parse command-line arguments.
- Run headless commands.
- Run the Ratatui TUI.
- Run the local app-server.
- Call shared logic from `crates/core`.

Important files:

```text
apps/cli/src/main.rs
apps/cli/src/args.rs
apps/cli/src/tui/
apps/cli/src/server/
apps/cli/src/commands/
```

### `apps/desktop`

The desktop app is Electron + React + TypeScript.

Responsibilities:

- Provide a polished GUI.
- Launch or connect to the local `nexgit app-server`.
- Expose a narrow typed API to the renderer through Electron preload.
- Avoid duplicating Git/product logic in TypeScript.

Important files:

```text
apps/desktop/src/main/cli-server.ts
apps/desktop/src/main/ipc.ts
apps/desktop/src/preload/index.ts
apps/desktop/src/renderer/src/App.tsx
apps/desktop/src/shared/protocol.ts
```

## Crates

### `crates/core`

Shared Rust product/domain logic.

This is where real Git and stack behavior should move as it is implemented.

Current examples:

- repository status placeholder
- stack list placeholder

Future examples:

- repository discovery
- branch metadata
- stack graph operations
- submit/sync workflow
- conflict state

### `crates/protocol`

App-server wire protocol types.

Responsibilities:

- Define request/response/event message shapes.
- Define the protocol version.
- Provide generated or source-of-truth TypeScript definitions.

## App-server protocol

The app-server uses JSON messages.

For `stdio://` and `unix://`, messages are newline-delimited JSON.

For `ws://`, messages are WebSocket text frames containing JSON.

Request:

```json
{ "type": "request", "id": 1, "method": "system.version", "params": {} }
```

Success response:

```json
{
  "type": "response",
  "id": 1,
  "ok": true,
  "result": { "name": "nexgit", "version": "0.1.0", "protocolVersion": 1 }
}
```

Error response:

```json
{
  "type": "response",
  "id": 1,
  "ok": false,
  "error": { "code": "method_not_found", "message": "unknown app-server method" }
}
```

Event:

```json
{ "type": "event", "event": "repo.changed", "payload": {} }
```

## Current methods

```text
system.version
repo.status
stack.list
stacks.list
```

`stack.list` and `stacks.list` currently map to the same placeholder behavior.

## Transport choices

### `stdio://`

Default for desktop integration.

Pros:

- Easy to spawn as a child process.
- No port management.
- No local network exposure.

### `unix://`

Useful for local tools on Unix-like systems.

Pros:

- Local IPC semantics.
- Easier multi-client support than stdio.

### `ws://`

Useful for debugging, remote TUI experiments, or future integrations.

Pros:

- Common protocol.
- Easy browser/devtool experimentation.
- Supports long-running connections and events.

Security note: websocket listeners should bind to loopback by default and use authentication before non-local use.

## Electron process boundaries

```text
Renderer React UI
  ↓ window.nexgit
Preload script
  ↓ ipcRenderer
Main process
  ↓ child_process.spawn + stdio protocol
Rust app-server
```

Renderer code should not spawn processes or access the filesystem directly. That work belongs in Electron main or the Rust app-server.

## Design principles

- Keep core behavior reusable.
- Keep frontend layers thin.
- Prefer typed boundaries.
- Avoid hidden global state.
- Make commands scriptable.
- Keep beginner-friendly code comments around non-obvious architecture.
- Add tests around behavior before large refactors.

## Future architecture splits

As the project grows, we may split more crates:

```text
crates/git       Git command and repository abstraction
crates/github    GitHub API and auth integration
crates/stack     Stack model and operations
crates/protocol  App-server wire types
```

Do not split too early. Start simple and split when boundaries become clear.
