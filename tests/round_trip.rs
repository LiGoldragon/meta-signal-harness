#[cfg(feature = "dotos-text")]
use dotos::{DotosDecode, DotosEncode, DotosSource};
use meta_signal_harness::{
    AgentIdentityToken, CapabilityProfile, ClaudeSessionIdentifier, CodexContinuationIdentifier,
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason, Configured,
    ContinuationHandle, ContinuationRequest, EffortRequest, HarnessDaemonConfiguration,
    HarnessKind, HarnessName, InitialPrompt, MetaHarnessFrame, MetaHarnessFrameBody,
    MetaHarnessReply, MetaHarnessRequest, MetaHarnessWire, ModelRequest, ModelResolutionRequest,
    ModelResolved, ModelSelector, ModelUnavailable, ModelUnavailableReason, NamedModel,
    OperationKind, RequestUnimplemented, SessionDirectory, SessionLaunchRefusalReason,
    SessionLaunchRefused, SessionLaunchRequest, SessionLaunched, UnimplementedReason,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RootCode, SessionEpoch,
    SignalOperationHeads, SubReply, VariantCode, WireContract, WireRoute,
};
use signal_harness::{HarnessInstanceConfiguration, TerminalSocketPath};
use signal_persona::schema::lib::{z2VNyf, z2VRBs, z2VSSX, z2VaTc, z2VckR, z2Veez};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn configuration() -> HarnessDaemonConfiguration {
    HarnessDaemonConfiguration {
        domain_socket_path: z2Veez::new("/run/persona/harness.sock".to_owned()),
        domain_socket_mode: z2VNyf::new(0o600),
        engine_management_socket_path: z2VckR::new(
            "/run/persona/harness-supervision.sock".to_owned(),
        ),
        engine_management_socket_mode: z2VSSX::new(0o600),
        owner_identity: z2VRBs::z2VWNV(z2VaTc::new(1000)),
        harnesses: vec![HarnessInstanceConfiguration {
            harness_name: HarnessName::new("designer"),
            harness_kind: HarnessKind::Codex,
            terminal_socket_path: Some(TerminalSocketPath::new("/run/persona/terminal.sock")),
            pi_rpc_adapter: None,
        }],
    }
}

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

fn round_trip_request(request: MetaHarnessRequest) -> MetaHarnessRequest {
    let frame = request
        .clone()
        .into_frame(exchange())
        .expect("request route is declared");
    assert_eq!(frame.short_header().binding(), MetaHarnessWire::BINDING);
    assert_eq!(
        frame.short_header().route().root().value(),
        request.kind() as u8
    );
    assert_eq!(frame.short_header().route().variant().value(), 0);
    let bytes = frame.encode_length_prefixed().expect("encode request");
    let decoded = MetaHarnessFrame::decode_length_prefixed(&bytes).expect("decode request");
    match decoded.into_body() {
        MetaHarnessFrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(reply: MetaHarnessReply, operation: OperationKind) -> MetaHarnessReply {
    let route = WireRoute::new(RootCode::new(operation as u8), VariantCode::new(0));
    let frame = MetaHarnessFrame::new(
        route,
        MetaHarnessFrameBody::Reply {
            exchange: exchange(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
        },
    );
    assert_eq!(frame.short_header().binding(), MetaHarnessWire::BINDING);
    assert_eq!(frame.short_header().route(), route);
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = MetaHarnessFrame::decode_length_prefixed(&bytes).expect("decode reply");
    match decoded.into_body() {
        MetaHarnessFrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

#[cfg(feature = "dotos-text")]
fn round_trip_dotos<Value>(value: Value, expected: &str)
where
    Value: DotosEncode + DotosDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_dotos();
    assert_eq!(text, expected);
    let recovered = DotosSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(recovered, value);
}

#[test]
fn every_meta_request_uses_the_direct_bound_frame() {
    let requests = [
        MetaHarnessRequest::Configure(configuration()),
        MetaHarnessRequest::ResolveModel(resolution_request()),
        MetaHarnessRequest::LaunchSession(launch_request()),
    ];

    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
    assert_eq!(
        <MetaHarnessRequest as SignalOperationHeads>::HEADS,
        &["Configure", "ResolveModel", "LaunchSession"]
    );
}

#[test]
fn model_resolution_replies_are_direct_frame_payloads() {
    let replies = [
        MetaHarnessReply::ModelResolved(ModelResolved {
            harness: HarnessName::new("designer"),
            harness_kind: HarnessKind::Claude,
            model: NamedModel::new("claude-sonnet-4"),
            effort: EffortRequest::High,
            continuation: ContinuationHandle::Claude(ClaudeSessionIdentifier::new(
                "claude-session-1",
            )),
        }),
        MetaHarnessReply::ModelUnavailable(ModelUnavailable {
            request: resolution_request(),
            reason: ModelUnavailableReason::ProviderUnavailable,
        }),
        MetaHarnessReply::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::ResolveModel,
            reason: UnimplementedReason::DependencyNotReady,
        }),
    ];

    for reply in replies {
        assert_eq!(
            round_trip_reply(reply.clone(), OperationKind::ResolveModel),
            reply
        );
    }
}

#[test]
fn session_launch_replies_are_direct_frame_payloads() {
    let replies = [
        MetaHarnessReply::SessionLaunched(SessionLaunched {
            agent_identity: AgentIdentityToken::new("xk3f"),
            child_process_id: 41,
            session_directory: Some(SessionDirectory::new("/run/persona/sessions/xk3f")),
            continuation: Some(ContinuationHandle::Codex(CodexContinuationIdentifier::new(
                "codex-session-8",
            ))),
        }),
        MetaHarnessReply::SessionLaunchRefused(SessionLaunchRefused {
            request: launch_request(),
            reason: SessionLaunchRefusalReason::LauncherUnavailable,
            detail: "Pi launcher is offline".to_owned(),
        }),
        MetaHarnessReply::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::LaunchSession,
            reason: UnimplementedReason::NotBuiltYet,
        }),
    ];

    for reply in replies {
        assert_eq!(
            round_trip_reply(reply.clone(), OperationKind::LaunchSession),
            reply
        );
    }
}

#[test]
fn configuration_replies_remain_typed() {
    let replies = [
        MetaHarnessReply::Configured(Configured {
            generation: ConfigurationGeneration::new(7),
        }),
        MetaHarnessReply::ConfigurationRejected(ConfigurationRejected {
            reason: ConfigurationRejectionReason::ManagerAuthorityRequired,
        }),
    ];

    for reply in replies {
        assert_eq!(
            round_trip_reply(reply.clone(), OperationKind::Configure),
            reply
        );
    }
}

#[cfg(feature = "dotos-text")]
#[test]
fn meta_policy_values_round_trip_through_dotos() {
    round_trip_dotos(
        MetaHarnessRequest::ResolveModel(resolution_request()),
        "(ResolveModel {{CapabilityProfile.deep-design ExtraHigh} Prefer.Codex.codex-session-7})",
    );
    round_trip_dotos(
        MetaHarnessRequest::LaunchSession(launch_request()),
        "(LaunchSession {Pi xk3f (|You are agent xk3f. Map the repo.|) Fresh})",
    );
    round_trip_dotos(
        MetaHarnessReply::ModelUnavailable(ModelUnavailable {
            request: resolution_request(),
            reason: ModelUnavailableReason::ProviderUnavailable,
        }),
        "(ModelUnavailable {{{CapabilityProfile.deep-design ExtraHigh} Prefer.Codex.codex-session-7} ProviderUnavailable})",
    );
    round_trip_dotos(
        MetaHarnessReply::SessionLaunchRefused(SessionLaunchRefused {
            request: launch_request(),
            reason: SessionLaunchRefusalReason::LauncherUnavailable,
            detail: "Pi launcher is offline".to_owned(),
        }),
        "(SessionLaunchRefused {{Pi xk3f (|You are agent xk3f. Map the repo.|) Fresh} LauncherUnavailable (Pi launcher is offline)})",
    );
}
