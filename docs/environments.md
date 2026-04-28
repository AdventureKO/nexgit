# Development Environments

Nexgit has two supported development environment entrypoints:

1. `mise`
2. Nix flakes

Use one of them. The project docs intentionally do not maintain separate manual installation steps for Rust, Node.js, pnpm, or other toolchain packages. Keeping the environment definitions in code avoids stale setup instructions.

## Option A: mise

Use this if you want the simplest setup and already use mise.

Source of truth:

```text
mise.toml
rust-toolchain.toml
```

Enter the project with:

```bash
mise install
pnpm install
```

Then run:

```bash
pnpm check
```

## Option B: Nix flake

Use this if you prefer Nix-managed reproducible shells.

Source of truth:

```text
flake.nix
flake.lock
```

`rust-toolchain.toml` remains in the repository for Rust tooling outside the Nix shell.

Enter the project with:

```bash
nix develop
pnpm install
```

Then run:

```bash
pnpm check
```

## Which should contributors choose?

For a university club project:

- Use `mise` if you want a lightweight language-tool version manager.
- Use `nix develop` if you already use Nix or want the shell to provide more system dependencies.

Both paths should support the same project commands.

## Project commands are the same

After entering either environment:

```bash
pnpm install
pnpm check
pnpm desktop:dev
cargo run -p nexgit-cli -- tui
```

## Maintainer note

When tool versions change, update the relevant environment files and docs together:

- `mise.toml`
- `flake.nix`
- `flake.lock`
- `package.json`
- `pnpm-lock.yaml`
- `rust-toolchain.toml`
- `docs/environments.md`
- `docs/tooling.md`
