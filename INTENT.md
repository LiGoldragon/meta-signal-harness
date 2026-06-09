# INTENT — meta-signal-harness

*The meta-only wire contract for privileged `harness` daemon configuration.
Companion to `Cargo.toml` and the ordinary `signal-harness` contract.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is for the `meta-signal-harness`
contract. Workspace-shape intent stays in `primary/INTENT.md`; the component
daemon intent stays in `harness/INTENT.md`; ordinary harness delivery, prompt,
transcript, and worker-lifecycle traffic stays in `signal-harness/INTENT.md`.

## Why this repo exists

Every Persona component has exactly two contracts: `signal-<component>`
(ordinary working signal) and `meta-signal-<component>` (meta policy signal).
`meta-signal-harness` is the second leg for `harness` — the authority surface
the Persona manager uses to configure the `harness-daemon`. Before this repo,
`harness` had only its ordinary contract; this completes the pair.

## The channel shape

The meta plane's baseline content is daemon configuration. The channel carries
a single `Configure` operation whose payload is the typed
`HarnessDaemonConfiguration` imported from `signal-harness` — the same record
that is the daemon's binary startup message. Reconfiguration arrives over this
meta plane as the same typed record, never as flags.

- **Request:** `Configure(HarnessDaemonConfiguration)`.
- **Replies:** `Configured` (carries the applied `ConfigurationGeneration`),
  `ConfigurationRejected` (typed reason), `RequestUnimplemented` (reached the
  meta surface but the runtime path is not built yet).

Component-specific privileged runtime actions (harness-instance lifecycle) are
additional operations that extend this channel as they are designed; daemon
configuration is the foundation they build on.
