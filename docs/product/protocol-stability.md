# Protocol Stability

The app-server protocol is the shared boundary between the Rust engine, desktop app, and future agent or automation integrations.

## Source of Truth

Rust protocol types are authoritative.

TypeScript protocol types must be generated from Rust or explicitly synchronized in the same pull request as protocol changes. A protocol change is incomplete if the desktop-facing TypeScript types are stale.

## Method Names

Use dotted method names:

```text
system.version
repo.status
stack.list
```

Prefer singular resource namespaces unless the existing method already uses another form. Keep backward-compatible aliases only when needed.

## Errors

Error codes should be stable, lowercase `snake_case`, and meaningful to callers.

Examples:

```text
method_not_found
repo_status_failed
serialization_error
```

Error messages can be human-readable and may change. Error codes are the stable automation surface.

## Versioning

Increment `PROTOCOL_VERSION` when a change breaks existing clients.

Breaking changes include:

- Removing a method.
- Renaming a method without keeping an alias.
- Removing or renaming response fields.
- Changing a field type.
- Changing error-code semantics.

Non-breaking changes include:

- Adding optional response fields.
- Adding new methods.
- Adding new event types.
- Improving human-readable error messages while preserving error codes.

## Events

Events should be introduced when operations become long-running or stateful.

Expected future event families:

```text
repo.changed
operation.started
operation.progress
operation.completed
operation.failed
```

Do not add event streams before a workflow needs them.
