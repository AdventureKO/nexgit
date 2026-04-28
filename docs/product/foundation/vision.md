# Vision and Non-Goals

Nexgit is an open-source repo workflow manager for humans and agents.

The product should help contributors and maintainers understand, claim, implement, stack, submit, review, and maintain GitHub work from a local-first workflow.

## Product Identity

Nexgit is one Rust workflow engine with multiple frontends:

```text
Rust workflow engine
  -> headless CLI commands
  -> keyboard-first TUI
  -> app-server protocol
       -> desktop app
       -> future agent and automation integrations
```

The CLI, TUI, and desktop app should not become separate products with separate behavior. Rust owns product logic, and each frontend exposes that behavior for a different workflow.

## V1 Pillars

Better `gh`-style headless commands:

- JSON-first commands for scripts, maintainers, and agents.
- Clear errors and stable output shapes.
- Repo, issue, PR, review, CI, and stack workflows that are easier to automate than raw shell glue.

Serious TUI:

- Fast keyboard-first workflow for contributors and maintainers.
- Repo health, branch state, issue context, PR status, CI status, and next actions.
- Useful without the desktop app.

Visual workflow desktop app:

- Visual repo and stack state.
- Issue, PR, review, and CI context beside local branch work.
- A polished GUI over the same Rust engine, not a TypeScript reimplementation of Git behavior.

Open-source repo manager for the AI age:

- GitHub issues as atomic tickets.
- Milestones as live planning.
- Discussions for broad product and design direction.
- Structured commands and protocol surfaces that humans and agents can use safely.

## First Public Slice

The first public slice is not v1. It proves the local GitHub workflow data path:

```text
GitHub auth and repository state
  -> Rust app-server methods
  -> shared protocol data shapes
  -> issue, PR, check, and review-thread data
  -> TUI and desktop dashboards
```

That slice should be small enough for contributors to understand and broad enough to prove the architecture. Maintainers should land foundation work first when a missing shared contract would otherwise make contributor issues vague.

## Non-Goals

Nexgit is not a Graphite clone. Graphite can inspire stacked workflow ideas, but Nexgit should grow into an open-source repo workflow manager with CLI, TUI, desktop, maintainer, and agent-oriented surfaces.

Nexgit is not a replacement for Git. It should orchestrate Git workflows and expose useful repo state, while relying on Git as the source of truth.

Nexgit is not unconstrained AI automation. Agent-facing features should use explicit commands, narrow permissions, reviewable state, and clear maintainer controls.

Nexgit should not start as a generic Git GUI. The sharper product identity is local-first GitHub workflow management with strong support for stacked work and contributor coordination.
