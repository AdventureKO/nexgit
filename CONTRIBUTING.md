# Contributing to Nexgit

Thanks for helping with Nexgit. This project is meant to be friendly for university club contributors while still following habits used by serious open-source projects.

You do **not** need to know everything before contributing. It is normal to ask questions, open draft pull requests, and learn as you go.

## Quick checklist

Before opening a pull request:

- [ ] Read the relevant docs or code around the area you are changing.
- [ ] Keep the change focused on one topic.
- [ ] Run the fastest relevant checks.
- [ ] Explain what changed and why.
- [ ] Mention anything you are unsure about.

## Ways to contribute

Good beginner-friendly contributions:

- Fix unclear docs or add missing setup notes.
- Improve command help text.
- Add examples to the README or docs.
- Share UX notes, screenshots, or Figma links on design issues.
- Make error messages more helpful.
- Improve the TUI placeholder layout.
- Add small app-server methods with tests.
- Add TypeScript types for existing protocol responses.
- Reproduce and document bugs.

More advanced contributions:

- Implement real Git repository detection.
- Build the stack/branch data model.
- Add GitHub auth and PR APIs.
- Improve app-server transports.
- Add desktop UI features.
- Add integration tests.

If you are unsure where to start, read [docs/onboarding.md](docs/onboarding.md).
For product direction, read [docs/product/README.md](docs/product/README.md).

## Claiming an issue

Issues usually start unassigned so contributors can pick up work like tickets.

To claim an issue:

1. Comment that you would like to work on it.
2. Wait for any maintainer guidance if the issue is unclear.
3. Comment `/assign` when you are ready to take it.

If you stop working on an issue, comment `/unassign` so another contributor can
pick it up.

Assigned issues are temporary claims, not permanent ownership. The repository
uses a 14-day claim window by default. When an issue is assigned, automation
posts the due date on the issue. If that due date passes without the work moving
forward, automation may unassign the issue so someone else can pick it up.

If you need more time, leave a progress update before the due date. A maintainer
can decide whether to keep the claim active or return the issue to the board.

Milestones group issues that need to land for a release. The
[v0.1.0 - GitHub App-Server Data milestone](https://github.com/Codelab-Davis/nexgit/milestone/1)
is the current foundation milestone. For a first contribution, start with open
issues labeled `good first issue`, `good for beginner`, or `good first design
issue`.

## Design contributions

You do not need to write code to contribute design work.

Useful design contributions include:

- UX notes on confusing flows.
- Screenshots or sketches.
- Figma links.
- Copy improvements.
- Accessibility observations.
- Icons, assets, or design-token suggestions.

Use GitHub Discussions for broad product or design direction. Use Issues for
scoped design tasks that someone can claim and finish. If a design issue is a
good first contribution, maintainers will label it `good first design issue`.

## Project structure

```text
apps/cli        Rust CLI, TUI, headless commands, app-server
apps/desktop    Electron + React + TypeScript desktop app
crates/core     Shared Rust product logic
crates/protocol App-server protocol types
docs/           Project docs
```

See [docs/architecture.md](docs/architecture.md) for more detail.

## Local setup

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

See [docs/environments.md](docs/environments.md). The project docs intentionally do not maintain separate manual toolchain installation instructions.

Run the TUI:

```bash
cargo run -p nexgit-cli -- tui
```

Run desktop development:

```bash
pnpm desktop:dev
```

For a detailed setup guide and troubleshooting, see [docs/development.md](docs/development.md).

## Development workflow

1. Update your local `main` branch.
2. Create a feature branch.
3. Make a focused change.
4. Run relevant checks.
5. Open a pull request.

Example:

```bash
git checkout main
git pull
git checkout -b docs/improve-onboarding
# make changes
cargo fmt --all
cargo check --workspace
git diff --check
git add .
git commit -m "docs: improve onboarding guide"
git push --set-upstream origin docs/improve-onboarding
```

If you are new to GitHub pull requests, it is okay to open a draft PR and ask for help.

## Commit messages

Use short, clear commit messages. Conventional Commit-style prefixes are encouraged:

```text
feat: add repo status protocol method
fix: handle app-server startup errors
docs: explain desktop setup
chore: update dependencies
test: add protocol handler tests
refactor: split TUI event handling
```

Do not worry about perfection. A maintainer can help polish commit history before merge.

## Pull request expectations

A good PR description answers:

- What changed?
- Why is this change useful?
- How did you test it?
- Is there any follow-up work?

Keep PRs small when possible. A small, reviewed PR is better than a large PR that is hard to understand.

## Human-authored pull requests

Pull requests must be human-authored by the contributor opening them.

Do not use AI tools or autonomous coding agents to create commits, open pull
requests, or generate substantial code, docs, tests, or copy for a PR under your
name. You may read documentation, ask maintainers for help, and use normal editor
features, but the submitted change and explanation must be your own work.

Every PR must include the authorship checklist item from the template. PR
hygiene automation checks for that attestation and for explicit AI/agent
authorship markers in the PR title, body, or commit messages. Automated checks
are not treated as proof by themselves; maintainers review the evidence before
taking action.

If maintainers confirm that a PR was AI-authored or opened by an AI agent, the
PR may be closed and the contributor may receive a 7-day contribution pause.
During that pause, the contributor should not claim issues, open PRs, or request
reviews in this repository. Repeated violations may lead to a longer pause.

## Checks to run

The repo has required formatting, linting, typechecking, and LSP/editor setup. See [docs/tooling.md](docs/tooling.md) for details.

Before most pull requests, run:

```bash
pnpm check
```

For Rust-only changes, the most relevant checks are:

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

For desktop TypeScript changes:

```bash
pnpm lint:desktop
pnpm desktop:typecheck
pnpm desktop:build
```

For docs-only changes:

```bash
pnpm format:check
git diff --check
```

If a check fails and you do not know why, include the error in your PR or ask in the club chat/discussion.

## Rust guidelines

- Keep shared business logic in `crates/core` where possible.
- Keep app-server protocol types in `crates/protocol`.
- Keep CLI/TUI-specific code in `apps/cli`.
- Prefer clear code over clever code.
- Use helpful error messages.
- Add tests when behavior is non-trivial.

## Desktop guidelines

- The renderer should call `window.nexgit`, not spawn processes directly.
- Electron main process owns the CLI app-server child process.
- Keep the preload API narrow and typed.
- Prefer TypeScript types for app-server responses.
- Avoid adding large UI dependencies without discussing them first.

## App-server protocol guidelines

The protocol uses JSON messages.

Request:

```json
{ "type": "request", "id": 1, "method": "system.version", "params": {} }
```

Response:

```json
{ "type": "response", "id": 1, "ok": true, "result": { "name": "nexgit" } }
```

When adding a method:

1. Define or update Rust response types.
2. Add handling in the app-server.
3. Update TypeScript types if the desktop uses it.
4. Add docs or examples if useful.
5. Add tests when practical.

## Review culture

Reviews should be kind, specific, and technical.

Good review comments:

- Explain the reason behind the suggestion.
- Ask questions when something is unclear.
- Distinguish required changes from optional ideas.
- Help newer contributors learn.

Avoid comments that attack the person instead of the code.

## Asking for help

When asking for help, include:

- What you are trying to do.
- What command you ran.
- What happened.
- What you expected to happen.
- Relevant error output.

Example:

```text
I am trying to run the desktop app.
I chose the mise setup path.
I ran `pnpm desktop:dev`.
It failed with `command not found: pnpm`.
What should I try next?
```

## Security issues

Do not open public issues for security vulnerabilities. Follow [SECURITY.md](SECURITY.md).

## Code of conduct

All contributors are expected to follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
