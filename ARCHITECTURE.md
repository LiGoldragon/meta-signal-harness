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

The current channel has one operation:

```text
MetaHarnessRequest                         MetaHarnessReply
└─ Configure(HarnessDaemonConfiguration)   ├─ Configured
                                           ├─ ConfigurationRejected
                                           └─ RequestUnimplemented
```

`HarnessDaemonConfiguration` is imported from `signal-harness`. The startup
binary file and the meta reconfiguration operation use the same typed record;
configuration never arrives as flags.

## Boundaries

This crate owns:

- the meta request and reply vocabulary for `harness`;
- typed configuration-generation and rejection records;
- NOTA and rkyv derives for the meta contract.

This crate does not own:

- the `harness` daemon runtime;
- ordinary delivery or transcript traffic;
- adapter launch or delivery behavior;
- engine-management supervision protocol details.

## Invariants

- Every component has exactly two public contracts:
  `signal-<component>` and `meta-signal-<component>`.
- `Configure` carries `signal-harness::HarnessDaemonConfiguration`; no local
  mirror type is allowed.
- Runtime reconfiguration may be rejected by the daemon until `harness` owns a
  hot-configuration reducer, but the rejection is typed.
- Future privileged harness-instance lifecycle operations extend this meta
  contract only after their authority boundary is concrete in `harness`.

## Code Map

```text
src/lib.rs    payloads, signal_channel! declaration, and component aliases
```
