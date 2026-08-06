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
configuration is the foundation the meta plane builds on; model resolution and
harness-instance launch are its current privileged runtime requests.

The current channel has three operations:

```text
Configure(HarnessDaemonConfiguration)
└─ Configured | ConfigurationRejected | RequestUnimplemented
ResolveModel(ModelResolutionRequest)
└─ ModelResolved | ModelUnavailable | RequestUnimplemented
LaunchSession(SessionLaunchRequest)
└─ SessionLaunched | SessionLaunchRefused | RequestUnimplemented
```

`HarnessDaemonConfiguration` and the model-resolution nouns are imported from
`signal-harness`. The startup binary file and the meta reconfiguration
operation use the same typed record; configuration never arrives as flags.
`ResolveModel` is schema-only here: the `harness` component owns exact-model or
capability/profile resolution, effort support checks, provider availability,
and continuation validation. If a request cannot be served, the reply is the
shared typed `ModelUnavailable` value and orchestrate decides retry,
escalation, or fallback.

`LaunchSession` is likewise schema-only here. Its request carries a concrete
`HarnessKind`, orchestrator-minted `AgentIdentityToken`, `InitialPrompt`, and
continuation policy. Its reply is either `SessionLaunched`, the typed
`SessionLaunchRefused`, or `RequestUnimplemented`; process creation remains
runtime behavior owned by `harness`.

The family wire allocation is explicit: this meta contract is contract id `2`,
wire revision `1`. Request route roots are the declared operation ordinals
(`Configure = 0`, `ResolveModel = 1`, `LaunchSession = 2`) with variant `0`.
Replies retain the root of the operation they answer. The short header and the
typed frame body are tested together.

## Boundaries

This crate owns:

- the meta request and reply vocabulary for `harness`;
- typed configuration-generation and rejection records;
- the privileged schema operation that asks `harness` to resolve a model and
  validate fresh/prefer/require continuation policy using `signal-harness`
  nouns;
- opt-in Dotos and always-on rkyv derives for the meta contract.

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
- Session launch uses shared `signal-harness` nouns; no mirrored local harness
  kind, identity, prompt, continuation, result, or refusal types are allowed.
- Default builds contain no textual codec. Dotos projection is enabled only by
  `dotos-text`.
- Every repository dependency is an exact immutable revision.

## Code Map

```text
src/lib.rs                 payloads, bound channel declaration, public aliases
examples/canonical.dotos   canonical textual policy values
tests/round_trip.rs        request/reply short-header and body witnesses
```
