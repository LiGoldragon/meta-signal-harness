use meta_signal_harness::{
    ByteViewable, ConfigurationRejected, ConfigurationRejectionReason, Configured,
    HarnessDaemonConfiguration, MetaOperationKind, ModelResolutionRequest, ModelUnavailable, Query,
    RequestUnimplemented, Response, Restorable, SessionLaunchRefused, SessionLaunchRequest, Signal,
    Signalizable, UnimplementedReason,
};
use signal_harness::{
    CapabilityProfile, ContinuationHandle, ContinuationRequest, EffortRequest,
    HarnessInstanceConfiguration, HarnessKind, ModelRequest, ModelSelector, ModelUnavailableReason,
    SessionLaunchRefusalReason,
};
use signal_persona::OwnerIdentity;

fn configuration() -> HarnessDaemonConfiguration {
    HarnessDaemonConfiguration {
        domain_socket_path: "/run/user/1000/harness/harness.sock".into(),
        domain_socket_mode: 0o600,
        meta_socket_path: "/run/user/1000/harness/meta-harness.sock".into(),
        meta_socket_mode: 0o600,
        engine_management_socket_path: "/run/user/1000/harness/supervision.sock".into(),
        engine_management_socket_mode: 0o600,
        owner_identity: OwnerIdentity::UnixUser(1000),
        harness_instance_configurations: vec![HarnessInstanceConfiguration {
            harness_name: "designer".into(),
            harness_kind: HarnessKind::Codex,
            terminal_socket_path_option: Some("/run/user/1000/terminal.sock".into()),
            pi_rpc_jsonl_adapter_configuration_option: None,
        }],
    }
}

fn resolution_request() -> ModelResolutionRequest {
    ModelResolutionRequest {
        model_request: ModelRequest {
            model_selector: ModelSelector::CapabilityProfile(CapabilityProfile::from(
                "deep-design",
            )),
            effort_request: EffortRequest::ExtraHigh,
        },
        continuation_request: ContinuationRequest::Prefer(ContinuationHandle::Codex(
            "codex-session-7".into(),
        )),
    }
}

fn launch_request() -> SessionLaunchRequest {
    SessionLaunchRequest {
        harness_kind: HarnessKind::Pi,
        agent_identity_token: "xk3f".into(),
        initial_prompt: "You are agent xk3f. Map the repo.".into(),
        continuation_request: ContinuationRequest::Fresh,
    }
}

fn queries() -> Vec<Query> {
    vec![
        Query::Configure(configuration()),
        Query::ResolveModel(resolution_request()),
        Query::LaunchSession(launch_request()),
    ]
}

fn responses() -> Vec<Response> {
    vec![
        Response::Configured(Configured {
            configuration_generation: 3,
        }),
        Response::ConfigurationRejected(ConfigurationRejected {
            configuration_rejection_reason: ConfigurationRejectionReason::ManagerAuthorityRequired,
        }),
        Response::ModelUnavailable(ModelUnavailable {
            model_resolution_request: resolution_request(),
            model_unavailable_reason: ModelUnavailableReason::ProviderUnavailable,
        }),
        Response::SessionLaunchRefused(SessionLaunchRefused {
            session_launch_request: launch_request(),
            session_launch_refusal_reason: SessionLaunchRefusalReason::LauncherUnavailable,
            session_launch_refusal_detail: "Pi launcher is offline".into(),
        }),
        Response::RequestUnimplemented(RequestUnimplemented {
            meta_operation_kind: MetaOperationKind::LaunchHarnessSession,
            unimplemented_reason: UnimplementedReason::NotBuiltYet,
        }),
    ]
}

#[test]
fn queries_round_trip_through_received_bytes() {
    for query in queries() {
        let received =
            Signal::<Query>::from(query.signalize().expect("query archives").bytes().to_vec());
        assert_eq!(received.restore().expect("query restores"), query);
    }
}

#[test]
fn responses_round_trip_through_received_bytes() {
    for response in responses() {
        let received = Signal::<Response>::from(
            response
                .signalize()
                .expect("response archives")
                .bytes()
                .to_vec(),
        );
        assert_eq!(received.restore().expect("response restores"), response);
    }
}

#[test]
fn malformed_archive_is_rejected() {
    let received = Signal::<Query>::from(vec![0xff; 3]);
    assert!(received.restore().is_err());
}

#[test]
fn configuration_names_three_distinct_sockets() {
    let configuration = configuration();
    let sockets = [
        &configuration.domain_socket_path,
        &configuration.meta_socket_path,
        &configuration.engine_management_socket_path,
    ];
    assert_ne!(sockets[0], sockets[1]);
    assert_ne!(sockets[1], sockets[2]);
    assert_ne!(sockets[0], sockets[2]);
}

#[cfg(feature = "datom")]
fn budget() -> datom_codec::Budget {
    datom_codec::Budget {
        remaining: 16384,
        reader: protos::ReaderBudget { remaining: 16384 },
        depth: 0,
        maximum_depth: 1024,
    }
}

#[cfg(feature = "datom")]
#[test]
fn queries_and_responses_round_trip_as_datom_text() {
    use datom_codec::{Actualizing, Datomizable, Potential};
    use protos::{Protosizable, Textualizable};

    for query in queries() {
        let text = query.clone().datomize(vec![]).protosize().textualize();
        let restored = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .expect("Datom restores");
        assert_eq!(restored, query);
    }
    for response in responses() {
        let text = response.clone().datomize(vec![]).protosize().textualize();
        let restored = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .expect("Datom restores");
        assert_eq!(restored, response);
    }
}

#[cfg(feature = "datom")]
#[test]
fn every_canonical_datom_line_actualizes_into_a_contract_head() {
    use datom_codec::{Actualizing, Potential};

    let canonical = include_str!("../examples/canonical.datom");
    let mut lines = 0;
    for line in canonical.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        lines += 1;
        let as_query = Potential::<Query>::from(line.to_owned())
            .actualize(&mut budget())
            .is_ok();
        let as_response = Potential::<Response>::from(line.to_owned())
            .actualize(&mut budget())
            .is_ok();
        assert!(
            as_query || as_response,
            "canonical line is no contract head: {line}"
        );
    }
    assert_eq!(lines, 7, "canonical file should carry seven contract heads");
}
