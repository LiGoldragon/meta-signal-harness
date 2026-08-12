# Meta Signal Harness — Agent Instructions

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
- Keep Dotos text behind the crate's opt-in `dotos-text` feature.

## Protos estate status

Stack: correct-new destination
Status: active component contract, current checkout legacy-wired
This checkout is not proof of correct-new adoption.
