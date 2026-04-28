# Security Policy

Nexgit is currently pre-alpha, but security issues should still be handled carefully.

## Supported versions

Nexgit does not have stable releases yet. Security fixes currently target the `main` branch.

## Reporting a vulnerability

Please do **not** open a public GitHub issue for a security vulnerability.

Until the project has a dedicated security contact, report vulnerabilities privately by emailing the current project maintainers at [codelabdavis@gmail.com](mailto:codelabdavis@gmail.com).

When reporting, include as much detail as you can:

- Affected component, such as CLI, desktop, app-server, or docs.
- Steps to reproduce.
- What you expected to happen.
- What actually happened.
- Potential impact.
- Suggested fix, if you have one.

## What counts as a security issue?

Examples:

- App-server exposed beyond localhost without authentication.
- Desktop renderer can execute arbitrary commands.
- Path traversal or unsafe file access.
- Secret/token leakage.
- Dependency vulnerabilities with practical impact.
- Git operations that can unexpectedly destroy user work.

## Local app-server security expectations

The app-server should be treated as sensitive because it may eventually access repositories, credentials, and GitHub APIs.

Default expectations:

- Prefer `stdio://` for desktop integration.
- Bind network transports to loopback by default.
- Add authentication before non-local websocket use.
- Keep Electron renderer access narrow through preload APIs.
- Do not expose arbitrary shell execution to the renderer.

## Maintainer response

Maintainers should:

1. Acknowledge the report privately.
2. Reproduce or assess the issue.
3. Decide severity and scope.
4. Prepare a fix privately if needed.
5. Publish a security note or release once fixed, if appropriate.

Because this is a student project, response times may vary, but reports should be treated seriously.
