# meta-signal-harness

Meta signal contract for privileged harness policy operations.

This is the meta-only half of the Harness contract family. Ordinary delivery,
observation, and transcript traffic remains in `signal-harness`.

The channel carries three authenticated operations:

- `Configure(HarnessDaemonConfiguration)` applies the same typed record used
  by daemon startup.
- `ResolveModel(ModelResolutionRequest)` resolves exact models or capability
  profiles together with effort and continuation policy.
- `LaunchSession(SessionLaunchRequest)` starts a Claude, Codex, Pi, or fixture
  session with a typed agent identity, initial prompt, and continuation.

Model resolution and session launch return typed success, refusal, unavailable,
or unimplemented values. Their vocabulary is imported from `signal-harness` by
identity; this crate does not mirror it.

The binary/rkyv frame is always present. Dotos is an opt-in textual projection
through `dotos-text`; default builds have no textual codec. Every dependency is
pinned to an exact revision. See `ARCHITECTURE.md` and
`examples/canonical.dotos`.
