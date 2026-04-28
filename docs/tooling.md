# Tooling: Linting, Formatting, and LSP

Nexgit treats linting, formatting, and editor/LSP setup as required project infrastructure.

The goal is to make contributions easier to review and reduce style debates.

## Tool versions

Tool versions are provided through the two supported development environment entrypoints:

- `mise.toml`
- `flake.nix`

Choose one setup path from [Development environments](environments.md), then run the tooling commands below.

## Formatting

Rust is formatted with `rustfmt`:

```bash
cargo fmt --all
```

Markdown, JSON, CSS, TypeScript, and TSX are formatted with Prettier:

```bash
pnpm format:prettier
```

Format everything:

```bash
pnpm format
```

Check formatting without writing changes:

```bash
pnpm format:check
```

Formatting config:

```text
.editorconfig
.prettierrc.json
.prettierignore
rust-toolchain.toml
rustfmt.toml
flake.nix
```

Nix files are formatted through the flake formatter:

```bash
pnpm nix:fmt
pnpm nix:fmt:check
```

## Linting

Rust linting uses Clippy with warnings denied:

```bash
pnpm lint:rust
```

Desktop linting uses ESLint flat config with TypeScript-aware rules:

```bash
pnpm lint:desktop
```

Run all linting:

```bash
pnpm lint
```

ESLint config:

```text
apps/desktop/eslint.config.mjs
```

## Type checking

Desktop TypeScript type checking:

```bash
pnpm typecheck
```

Rust workspace checking:

```bash
cargo check --workspace
```

## Full local check

Before opening a pull request, run:

```bash
pnpm check
```

If you changed Nix files, also run:

```bash
pnpm nix:check
pnpm nix:fmt:check
```

This runs:

1. Rust format check.
2. Prettier check.
3. Rust workspace check.
4. Rust Clippy.
5. Desktop ESLint.
6. Desktop TypeScript check.

For docs-only changes, this is usually enough:

```bash
pnpm format:check
git diff --check
```

## VS Code / Cursor / Windsurf setup

The repo includes recommended editor files:

```text
.vscode/extensions.json
.vscode/settings.json
```

Recommended extensions:

- rust-analyzer
- Even Better TOML
- Nix IDE
- ESLint
- Prettier
- EditorConfig

The settings enable:

- format on save
- Prettier for web/docs files
- rust-analyzer formatting for Rust
- ESLint fixes on save when explicitly requested
- rust-analyzer Clippy checks

## Other editors

Use the equivalent language servers:

- Rust: `rust-analyzer`
- Nix: `nixd`
- TypeScript/React: TypeScript language server
- ESLint: ESLint language server
- Formatting: Prettier, rustfmt, and nixfmt

## CI enforcement

GitHub Actions are defined in:

```text
.github/workflows/check.yml
```

CI checks the Nix flake, Nix formatting, Rust formatting, Rust compile/lint, Prettier formatting, desktop ESLint, desktop TypeScript, and desktop build.
