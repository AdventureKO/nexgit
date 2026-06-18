# Governance

Nexgit is a university club open-source project. Governance should be clear enough to protect the project, but light enough that students can learn by participating.

## Project model

Nexgit uses a maintainer-led model.

Maintainers are responsible for:

- Setting technical direction.
- Reviewing and merging pull requests.
- Keeping the project welcoming.
- Helping contributors find useful work.
- Maintaining docs and project hygiene.

Contributors are encouraged to challenge decisions with respectful, technical reasoning.

## Contributor roles

### Contributor

Anyone who opens issues, submits pull requests, improves docs, tests the app, or helps others.

### Reviewer

A trusted contributor who reviews code and helps maintain quality, but may not have merge rights.

### Maintainer

A trusted contributor with merge rights and responsibility for project health.

### Project lead

A maintainer or club officer who coordinates roadmap conversations, meetings, and releases. This role may rotate by semester.

## Decision making

Most decisions happen in pull requests and issues.

Small decisions can be made by the maintainer reviewing the work.

Larger decisions should get broader input, especially if they affect:

- Public CLI behavior.
- App-server protocol shape.
- Desktop architecture.
- Major dependencies.
- Long-term roadmap direction.
- Security model.

For large decisions, prefer opening an issue with:

- Problem statement.
- Proposed solution.
- Alternatives considered.
- Tradeoffs.
- Impact on contributors and users.

## Merge policy

A pull request can be merged when:

- It has at least one maintainer approval.
- Required checks pass, once CI exists.
- The change is documented if it affects users or contributors.
- Any blocking review comments are resolved.

Maintainers may merge small docs fixes quickly.

Large or controversial changes should remain open long enough for reasonable feedback.

## Adding maintainers

New maintainers should demonstrate:

- Consistent, constructive contributions.
- Good technical judgment.
- Respectful review behavior.
- Willingness to help newer contributors.
- Care for project documentation and long-term maintainability.

For a university club, maintainer roles can be reviewed each semester.

## Removing maintainers

A maintainer may step down at any time.

A maintainer may be removed for:

- Long-term inactivity when access is no longer needed.
- Repeated violation of the code of conduct.
- Unsafe handling of repository permissions or secrets.
- Acting against project interests.

Removal should be handled respectfully and privately when possible.

## Code of conduct

All project spaces follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Security

Security reports follow [SECURITY.md](SECURITY.md).

## Evolution

This governance file should evolve as the project grows. If the contributor base becomes large, Nexgit may add:

- A formal design proposal process.
- More explicit maintainer areas.
- Release managers.
- Semester-based leadership rotation.
