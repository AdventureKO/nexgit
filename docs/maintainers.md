# Maintainer Guide

This guide is for club officers, project leads, and trusted contributors who help keep Nexgit organized.

The goal is lightweight professionalism: enough structure to make the project healthy, but not so much process that students are afraid to contribute.

## Maintainer responsibilities

Maintainers are expected to:

- Keep the project welcoming and technically focused.
- Review pull requests in a reasonable time.
- Help new contributors find good tasks.
- Keep docs accurate.
- Protect the `main` branch from broken or unsafe changes.
- Make milestone, design, and architecture decisions transparent.

Maintainers are not expected to be available 24/7.

Product direction and dependency decisions live in
[product planning](product/README.md). Keep those docs aligned with GitHub
milestones before opening broad implementation work.

## Suggested roles for a university club

A small club project can use informal roles:

- **Project lead**: coordinates direction and meetings.
- **Rust lead**: helps with CLI, TUI, app-server, and core crates.
- **Desktop lead**: helps with Electron, React, TypeScript, and UI.
- **Docs/onboarding lead**: keeps beginner docs and issue labels healthy.
- **Triage helpers**: label issues, reproduce bugs, and connect contributors with tasks.

One person can hold multiple roles. Roles can rotate each semester.

## Triage process

For new issues:

1. Thank the reporter.
2. Check if the issue is understandable.
3. Ask for missing information if needed.
4. Add area labels when labels exist.
5. Mark beginner-friendly tasks as `good first issue` and `good for beginner` when appropriate.
6. Close duplicates with a link to the original issue.

New issues should normally stay unassigned. Treat them as a public task board:
contributors can comment that they want to work on an issue and then claim it
with `/assign`. Maintainers should only assign someone else after that person
has agreed to take the work.

Use milestones for release planning. Keep the first release milestone focused on
work that blocks a usable public release, and avoid putting every nice-to-have
task into the milestone.

Suggested labels:

```text
area: automation
area: cli
area: core
area: desktop
area: design
area: docs
area: protocol
area: tests
area: tui
good first issue
good for beginner
good first design issue
help wanted
kind: bug
kind: docs
kind: feature
kind: maintenance
needs: compliance
needs: authenticity-review
needs: duplicate-review
needs: design
needs: linked-context
needs: title
maintainer only
status: needs-triage
status: needs-author-feedback
status: in-progress
status: duplicate
status: superseded
status: stale
```

Issues labeled `maintainer only` are reserved for repository administration,
governance, or release coordination. Non-maintainers cannot claim those issues
with `/assign`.

## Issue claim deadlines

Treat assignees as short-term claims. The default claim window is 14 days from
assignment. The `Issue Claim Deadlines` workflow posts the due date on assigned
issues, checks open issues daily, and unassigns overdue claims so the work goes
back into the contributor pool.

For a faster sprint, set the repository variable `ISSUE_CLAIM_DAYS` to `7`.
For a one-off check, run the workflow manually and pass a different
`claim_days` value. Keep the value between 1 and 90 days.

If a contributor needs more time, ask for a progress update before extending the
claim. If there is no update by the deadline, let automation return the issue to
`status: needs-triage` so another contributor can claim it.

Triage commands:

- `/assign` assigns the issue or pull request to the commenter.
- `/assign @user` assigns it to another user; this is maintainer-only and should only be used for people who agreed to take the work.
- `/unassign` removes the commenter from the assignees.
- `/unassign @user` removes another assignee; this is maintainer-only.
- `/duplicate #123` closes the current issue or pull request as a duplicate.
- `/not-duplicate` clears a duplicate-review label when automation guessed wrong.
- `/authenticity-reviewed` clears an authenticity-review label after a maintainer has reviewed the PR.
- `/needs-info` marks the thread as waiting on the reporter or author.
- `/triaged` clears triage and author-feedback labels.
- `/superseded-by #123` closes a pull request in favor of a newer one.

## Pull request review process

Before merging, check:

- The PR has a clear description.
- The change is focused.
- Relevant checks were run.
- User-facing behavior is documented.
- New architecture decisions are reflected in docs.
- Beginner contributors received helpful explanations, not just terse requests.
- The PR includes the human-authorship attestation and does not show evidence
  of being AI-authored or agent-opened.

If a PR is confirmed to be AI-authored or opened by an AI agent, close it with a
short explanation and ask the contributor to pause repo contributions for 7 days.
Automation can flag explicit authorship markers, but maintainers make the final
call.

For a small change, one maintainer approval is enough.

For larger changes, especially architecture or dependency changes, ask for at least one extra review.

## Branch protection recommendation

Once GitHub Actions are added, protect `main` with:

- Required status checks.
- At least one approving review.
- No force pushes.
- No direct pushes except emergency maintainer fixes.

Recommended checks:

```text
pnpm format:check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
pnpm lint:desktop
pnpm desktop:typecheck
pnpm desktop:build
git diff --check
```

The checked-in GitHub Actions workflow in `.github/workflows/check.yml` enforces these core checks in CI.

## Meeting rhythm

For a university club, a practical rhythm is:

- Weekly or biweekly project meeting.
- Review open PRs first.
- Triage new issues second.
- Discuss milestone/design topics third.
- End by assigning small next steps.

Keep notes in GitHub issues, discussions, or docs so people who missed the meeting can catch up.

## Onboarding new contributors

A good onboarding flow:

1. Send them [docs/onboarding.md](onboarding.md).
2. Help them choose either the mise or Nix flake entrypoint in [docs/environments.md](environments.md).
3. Help them run `pnpm install` and `pnpm check`.
4. Give them a small issue.
5. Encourage a draft PR early.
6. Review with teaching-focused comments.

Avoid assigning first-time contributors large, vague features.

## Adding dependencies

Before adding a dependency, consider:

- Is it actively maintained?
- Does it have an acceptable license?
- Is it necessary now?
- Does it significantly increase build size or complexity?
- Is there a simpler option?

For large frontend UI libraries or Rust architecture dependencies, open an issue or discussion first.

## Security basics

- Do not commit secrets.
- Keep GitHub secret scanning enabled if available.
- Prefer loopback-only local app-server listeners.
- Do not expose local app-server methods to untrusted web content.
- Follow [../SECURITY.md](../SECURITY.md) for vulnerability reports.

## Release process placeholder

Nexgit does not have stable releases yet.

Before the first real release, define:

- Versioning policy.
- Changelog format.
- Binary build targets.
- Desktop packaging strategy.
- Who can publish releases.
- How to handle security fixes.

## When to update governance

The current governance model is intentionally light. Consider updating [../GOVERNANCE.md](../GOVERNANCE.md) if:

- More people need merge rights.
- Maintainer roles become unclear.
- The club wants elected/rotating project leads.
- Major architecture decisions need a formal proposal process.
