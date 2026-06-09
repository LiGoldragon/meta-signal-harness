# meta-signal-harness

Meta signal contract for privileged harness daemon configuration.

The meta-only wire contract for `harness` — the second leg of the two-contract
pair (`signal-harness` ordinary + `meta-signal-harness` meta). The meta plane's
baseline content is daemon configuration: a typed `Configure` operation
carrying `harness`'s `*DaemonConfiguration` (the same record that is the daemon's
binary startup message), with `Configured` / `ConfigurationRejected` /
`RequestUnimplemented` replies.

Default builds carry `nota-text` for CLI/debug projection; the wire is
binary/rkyv. See `INTENT.md`.
