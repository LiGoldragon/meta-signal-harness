# meta-signal-harness — architecture

*Meta policy contract for the `harness` component.*

## Surface

`meta-signal-harness` is the privileged companion contract to
`signal-harness`. It carries the meta plane for `harness`; ordinary message
delivery, interaction, status, transcript, and lifecycle observation traffic
stays in `signal-harness`.

## Direction

This repo is the second leg of the harness contract pair. Every Persona
component has exactly two contracts: the ordinary `signal-<component>` working
signal and the meta `meta-signal-<component>` policy signal. `meta-signal-harness`
is the authority surface the Persona manager uses to configure the
`harness-daemon`; before it, `harness` had only its ordinary contract. Daemon
configuration is the foundation the meta plane builds on, and component-specific
privileged runtime actions (harness-instance lifecycle) extend this channel as
they are designed.

The current channel has two operations:

```text
MetaHarnessRequest                         MetaHarnessReply
├─ Configure(HarnessDaemonConfiguration)   ├─ Configured
└─ ResolveModel(ModelResolutionRequest)    ├─ ConfigurationRejected
                                           ├─ ModelResolved
                                           ├─ ModelUnavailable
                                           └─ RequestUnimplemented
```

`HarnessDaemonConfiguration` and the model-resolution nouns are imported from
`signal-harness`. The startup binary file and the meta reconfiguration
operation use the same typed record; configuration never arrives as flags.
`ResolveModel` is schema-only here: the `harness` component owns exact-model or
capability/profile resolution, effort support checks, provider availability,
and continuation validation. If a request cannot be served, the reply is the
shared typed `ModelUnavailable` value and orchestrate decides retry,
escalation, or fallback.

## Boundaries

This crate owns:

- the meta request and reply vocabulary for `harness`;
- typed configuration-generation and rejection records;
- the privileged schema operation that asks `harness` to resolve a model and
  validate fresh/prefer/require continuation policy using `signal-harness`
  nouns;
- NOTA and rkyv derives for the meta contract.

This crate does not own:

- the `harness` daemon runtime;
- ordinary delivery or transcript traffic;
- model-to-provider resolution logic;
- adapter launch or delivery behavior;
- session-reuse policy, retry, escalation, or fallback decisions;
- engine-management supervision protocol details.

## Invariants

- Every component has exactly two public contracts:
  `signal-<component>` and `meta-signal-<component>`.
- `Configure` carries `signal-harness::HarnessDaemonConfiguration`; no local
  mirror type is allowed.
- Runtime reconfiguration may be rejected by the daemon until `harness` owns a
  hot-configuration reducer, but the rejection is typed.
- Model resolution uses shared `signal-harness` nouns; no mirrored local model,
  effort, continuation, or unavailable-reason types are allowed.
- Future privileged harness-instance lifecycle operations extend this meta
  contract only after their authority boundary is concrete in `harness`.

## Code Map

```text
src/lib.rs    payloads, signal_channel! declaration, and component aliases
```
