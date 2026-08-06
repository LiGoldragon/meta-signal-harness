# skills — meta-signal-harness

Work here when the change concerns the privileged `harness` policy contract.

Before editing, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- `../signal-harness/ARCHITECTURE.md`
- `../harness/ARCHITECTURE.md`

Rules:

- Keep the crate contract-only. Do not add runtime code.
- Use `HarnessDaemonConfiguration` from `signal-harness`.
- Import model-resolution and session-launch vocabulary from `signal-harness`
  by identity; do not mirror producer types.
- Keep operation names contract-local and authority-shaped.
- Keep Dotos projection opt-in and keep the default binary contract free of a
  textual codec.
- Pin every repository dependency to an exact revision.
- Add short-header/body, rkyv, and Dotos round-trip witnesses when adding
  operations or payload variants.
