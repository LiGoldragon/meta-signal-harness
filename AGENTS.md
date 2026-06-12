# Meta Signal Harness — Agent Instructions

Read `/home/li/primary/AGENTS.md` first, then `/home/li/primary/lore/AGENTS.md`.
This repository follows the primary workspace orchestration protocol.

## Purpose

`meta-signal-harness` is the meta policy contract for the `harness`
component. It carries privileged daemon configuration and future
authority-gated harness-instance lifecycle operations. Ordinary delivery,
prompt, transcript, and lifecycle observation traffic stays in
`signal-harness`; runtime behavior stays in `harness`.

## Local Rules

- Keep this crate contract-only: no actors, sockets, redb, daemon loops, or
  adapter code.
- Import the daemon startup configuration from `signal-harness`; do not define
  a second local mirror of `HarnessDaemonConfiguration`.
- Keep meta operations authority-shaped and closed. Add new operations only
  when the `harness` component has a concrete policy boundary for them.
- Keep NOTA text behind the crate's `nota-text` feature for CLI/tooling edges.
