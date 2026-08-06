//! Meta signal contract — privileged `harness` policy operations.
//!
//! Ordinary delivery and observation traffic belongs to `signal-harness`.
//! This contract carries authenticated daemon configuration, model
//! resolution, and harness-session launch requests. Runtime policy and
//! behavior remain in `harness`.

#[cfg(feature = "dotos-text")]
use dotos::{DotosDecode, DotosEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_harness::{
    AgentIdentityToken, CapabilityProfile, ClaudeSessionIdentifier, CodexContinuationIdentifier,
    ContinuationHandle, ContinuationRequest, EffortRequest, HarnessDaemonConfiguration,
    HarnessKind, HarnessName, InitialPrompt, ModelRequest, ModelResolutionRequest, ModelResolved,
    ModelSelector, ModelUnavailable, ModelUnavailableReason, NamedModel, PiContinuationIdentifier,
    SessionDirectory, SessionLaunchRefusalReason, SessionLaunchRefused, SessionLaunchRequest,
    SessionLaunched,
};

/// The meta Harness contract occupies the second wire seat in its family.
pub enum MetaHarnessWire {}

impl signal_frame::WireContract for MetaHarnessWire {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        signal_frame::ContractId::new(
            core::num::NonZeroU32::new(2).expect("the meta Harness contract id is non-zero"),
        ),
        signal_frame::WireRevision::new(core::num::NonZeroU16::MIN),
    );
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct ConfigurationGeneration(u64);

impl ConfigurationGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Configured {
    pub generation: ConfigurationGeneration,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigurationRejectionReason {
    ManagerAuthorityRequired,
    MalformedConfiguration,
    UnsupportedConfiguration,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationRejected {
    pub reason: ConfigurationRejectionReason,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub operation: OperationKind,
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel MetaHarness contract MetaHarnessWire {
        operation Configure(HarnessDaemonConfiguration),
        operation ResolveModel(ModelResolutionRequest),
        operation LaunchSession(SessionLaunchRequest),
    }
    reply MetaHarnessReply {
        Configured(Configured),
        ConfigurationRejected(ConfigurationRejected),
        ModelResolved(ModelResolved),
        ModelUnavailable(ModelUnavailable),
        SessionLaunched(SessionLaunched),
        SessionLaunchRefused(SessionLaunchRefused),
        RequestUnimplemented(RequestUnimplemented),
    }
}

pub type MetaHarnessRequest = Operation;
pub type MetaHarnessFrame = Frame;
pub type MetaHarnessFrameBody = FrameBody;
pub type MetaHarnessReplyEnvelope = ReplyEnvelope;
pub type MetaHarnessRequestBuilder = RequestBuilder;

impl From<HarnessDaemonConfiguration> for MetaHarnessRequest {
    fn from(payload: HarnessDaemonConfiguration) -> Self {
        Self::Configure(payload)
    }
}

impl From<ModelResolutionRequest> for MetaHarnessRequest {
    fn from(payload: ModelResolutionRequest) -> Self {
        Self::ResolveModel(payload)
    }
}

impl From<SessionLaunchRequest> for MetaHarnessRequest {
    fn from(payload: SessionLaunchRequest) -> Self {
        Self::LaunchSession(payload)
    }
}
