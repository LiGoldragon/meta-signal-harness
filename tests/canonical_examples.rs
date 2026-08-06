#![cfg(feature = "dotos-text")]

use dotos::{DotosEncode, DotosSource};
use meta_signal_harness::{
    AgentIdentityToken, CapabilityProfile, CodexContinuationIdentifier, ContinuationHandle,
    ContinuationRequest, EffortRequest, HarnessKind, InitialPrompt, MetaHarnessReply,
    MetaHarnessRequest, ModelRequest, ModelResolutionRequest, ModelSelector, ModelUnavailable,
    ModelUnavailableReason, SessionLaunchRefusalReason, SessionLaunchRefused, SessionLaunchRequest,
};

const CANONICAL: &str = include_str!("../examples/canonical.dotos");

fn resolution_request() -> ModelResolutionRequest {
    ModelResolutionRequest {
        model: ModelRequest {
            selector: ModelSelector::CapabilityProfile(CapabilityProfile::new("deep-design")),
            effort: EffortRequest::ExtraHigh,
        },
        continuation: ContinuationRequest::Prefer(ContinuationHandle::Codex(
            CodexContinuationIdentifier::new("codex-session-7"),
        )),
    }
}

fn launch_request() -> SessionLaunchRequest {
    SessionLaunchRequest {
        harness_kind: HarnessKind::Pi,
        agent_identity: AgentIdentityToken::new("xk3f"),
        initial_prompt: InitialPrompt::new("You are agent xk3f. Map the repo."),
        continuation: ContinuationRequest::Fresh,
    }
}

#[test]
fn canonical_meta_requests_round_trip() {
    let expected = [
        MetaHarnessRequest::ResolveModel(resolution_request()),
        MetaHarnessRequest::LaunchSession(launch_request()),
    ];

    for value in expected {
        let text = value.to_dotos();
        assert!(CANONICAL.lines().any(|line| line == text));
        assert_eq!(
            DotosSource::new(&text)
                .parse::<MetaHarnessRequest>()
                .expect("decode request"),
            value
        );
    }
}

#[test]
fn canonical_meta_replies_round_trip() {
    let expected = [
        MetaHarnessReply::ModelUnavailable(ModelUnavailable {
            request: resolution_request(),
            reason: ModelUnavailableReason::ProviderUnavailable,
        }),
        MetaHarnessReply::SessionLaunchRefused(SessionLaunchRefused {
            request: launch_request(),
            reason: SessionLaunchRefusalReason::LauncherUnavailable,
            detail: "Pi launcher is offline".to_owned(),
        }),
    ];

    for value in expected {
        let text = value.to_dotos();
        assert!(CANONICAL.lines().any(|line| line == text));
        assert_eq!(
            DotosSource::new(&text)
                .parse::<MetaHarnessReply>()
                .expect("decode reply"),
            value
        );
    }
}
