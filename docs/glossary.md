# Glossary

This page explains terms used in Nexgit. Add to it whenever a term confuses new contributors.

## App-server

A long-running mode of the CLI that accepts structured requests from another program.

In Nexgit:

```bash
nexgit app-server --listen stdio://
```

The desktop app talks to this server instead of reimplementing Git logic in TypeScript.

## CLI

Command-line interface. A program used by typing commands in a terminal.

Example:

```bash
nexgit repo status --json
```

## TUI

Terminal user interface. A full-screen interface that runs inside a terminal.

Nexgit uses Ratatui for the TUI.

## Desktop app

The graphical app in `apps/desktop`. It uses Electron, React, and TypeScript.

## Electron

A framework for building desktop apps with web technologies. Electron apps have a main process and renderer processes.

## React

The UI library used by the desktop renderer.

## Preload script

An Electron script that safely exposes a small API from the main process to the renderer.

In Nexgit, the renderer calls:

```ts
window.nexgit.getVersion();
```

## Renderer

The browser-like part of an Electron app that displays the user interface.

## Main process

The Node.js side of an Electron app. In Nexgit, it launches the Rust app-server.

## Protocol

The message format used between the desktop app and app-server.

Example request:

```json
{ "type": "request", "id": 1, "method": "system.version", "params": {} }
```

## Stacked changes

A workflow where a feature is split into multiple dependent branches or pull requests instead of one large pull request.

Example:

```text
main
  └─ branch-a
       └─ branch-b
            └─ branch-c
```

## Stack

A group of related branches or changes that build on each other.

## Headless command

A command that can run without a TUI or GUI, usually useful for scripts and automation.

Example:

```bash
nexgit stack list --json
```

## Workspace

A Cargo workspace is a group of Rust crates managed together. This repository has one Rust workspace at the root.

A pnpm workspace is a group of JavaScript/TypeScript packages managed together. This repository currently includes `apps/desktop`.

## Crate

A Rust package. Examples:

- `crates/core`
- `crates/protocol`
- `apps/cli`

## Package

A JavaScript/TypeScript package. The desktop package is `@nexgit/desktop`.

## Pull request or PR

A proposed code change on GitHub. Contributors open PRs, maintainers review them, and approved PRs are merged.
