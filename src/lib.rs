//! Meta Signal contract — privileged `harness` policy operations.
//!
//! The Persona manager configures the harness daemon, resolves a model and
//! launches a harness session through this contract. Ordinary delivery,
//! observation, transcript and usage traffic belongs to `signal-harness`,
//! whose daemon configuration and model-resolution and session-launch nouns
//! this contract imports by identity.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against a fresh
//! generation.
//!
//! # The wire
//!
//! One request is one [`Signal`] frame carrying the rkyv archive of [`Query`];
//! one answer is one frame of [`Response`]. The contract carries no envelope,
//! exchange identifier or route code: the `Query` and `Response` heads are the
//! discrimination and the connection is the correlation. The frame carries no
//! contract discriminator either, so this contract has its own socket
//! (`HarnessDaemonConfiguration`'s meta socket) and never shares one with the
//! `signal-persona` engine-management lifecycle.

pub mod generated;
pub use generated::signal::*;

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};
pub use signal_harness::{
    HarnessDaemonConfiguration, ModelResolutionRequest, ModelResolved, ModelUnavailable,
    SessionLaunchRefused, SessionLaunchRequest, SessionLaunched,
};

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");
