# Dependency Decisions

Dependencies should make the first GitHub app-server data milestone easier to build without locking Nexgit into a heavy architecture too early.

## Rust Policy

Keep now:

- `clap`
- `serde` and `serde_json`
- `thiserror`
- `anyhow`
- `tokio`
- `tracing`
- `ratatui`
- `crossterm`

Early GitHub app-server direction:

- Use the installed `gh` CLI behind a Rust adapter for first GitHub API reads.
- Use the installed `git` CLI behind a Rust adapter for local repository identity and status when needed.
- Keep `crates/core` small and testable.
- Parse stable command output formats where practical.
- Add test-focused dev dependencies only when implementation needs them.

Allowed test dependencies when needed:

- `tempfile` for isolated repository fixtures.
- `assert_cmd` for CLI integration tests.
- `predicates` for readable command-output assertions.

Deferred:

- `git2`
- `gix`
- GitHub API clients such as `octocrab`

Reasoning: a Git CLI adapter avoids native dependency and packaging complexity while the product shape is still forming. Revisit pure Rust or libgit-based backends after the repo-status and stack-model boundaries are clearer.

## Electron Policy

Keep now:

- Electron
- React
- TypeScript
- Vite and `electron-vite`
- ESLint and Prettier

Allowed when needed:

- `lucide-react` for consistent icon UI.

Deferred:

- Zustand
- TanStack Query
- Graph visualization libraries
- Component systems

Reasoning: the desktop app should stay thin until the workflow is clearer. Add state, cache, graph, and component libraries only when current product screens need them.

## Dependency Review Rule

Before adding a dependency, document:

- What problem it solves.
- Why the standard library or current stack is not enough.
- License and maintenance status.
- Packaging or build impact.
- Whether the dependency belongs in core, CLI, protocol, or desktop.
