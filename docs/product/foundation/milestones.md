# Milestone Model

GitHub milestones are the live planning surface. These milestone definitions explain the intended arc without creating a second planning board.

## v0.1.0 - GitHub App-Server Data

Goal: prove the local app-server path for read-only GitHub workflow data before TUI and desktop screens build on it.

Done when:

- GitHub API access has one Rust-owned adapter boundary.
- The current GitHub repository and auth state can be resolved clearly.
- Shared issue, PR, check, and review-thread data shapes exist.
- App-server methods expose read-only issue, PR, checks, and review data.
- Fixture or adapter tests give later contributors a pattern to copy.
- Beginner-friendly docs/design issues remain available while maintainer-owned foundation issues land.

## v0.2.0 - TUI Workflow Dashboard

Goal: make the Ratatui app useful as a keyboard-first GitHub workflow dashboard over the shared app-server data.

Expected work:

- Show current GitHub auth and repository state.
- Render issue and PR queues.
- Add selected PR details, checks, and review-thread summaries.
- Keep empty, loading, no-auth, no-repo, permission, and error states readable.

## v0.3.0 - Desktop Workflow Dashboard

Goal: make the Electron app a visual dashboard over the same stdio app-server path.

Expected work:

- Show current GitHub auth and repository state.
- Render issue and PR panels.
- Add checks and review-thread summaries.
- Make app-server startup, loading, empty, and error states clear.
- Keep renderer code behind the typed preload/app-server boundary.

## v0.4.0 - Workflow Actions

Goal: add safe write workflows only after the read-only data model is proven.

Expected work:

- Explicit target confirmation before mutations.
- Dry-run previews for write actions.
- Issue claim/unclaim, review reply/resolve, and check rerun candidates.
- Clear permission and failure-state handling.

## v1.0.0 - Open Source Repo Workflow Manager

Goal: a stable local-first workflow system for Git, GitHub, maintainers, contributors, and agent-friendly automation.

Expected work:

- Stable CLI command shapes.
- Stable app-server protocol versioning.
- Documented install and release process.
- Reliable packaging for CLI and desktop.
- Clear governance and maintainer operations.

## Planning Rule

Do not add broad implementation issues when atomic tickets are possible. Broad issues may exist only as maintainer-owned planning or design context.

Student-facing issues should be concrete enough to claim without a planning
meeting. Every task needs a definition of done and a verification section that
names the command, manual check, screenshot, or acceptance criteria maintainers
will use during review.

Contributor-facing issues should have one behavior, one layer, and one expected
output. They should list the relevant files, dependencies, out-of-scope work,
and the fastest useful verification command. If a task requires a new shared
architecture decision, keep it maintainer-owned until that decision lands.
