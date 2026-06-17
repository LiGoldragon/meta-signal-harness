use meta_signal_harness::{
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason, Configured,
    HarnessDaemonConfiguration, MetaHarnessFrame, MetaHarnessFrameBody, MetaHarnessReply,
    Operation, OperationKind, RequestUnimplemented, UnimplementedReason,
};
#[cfg(feature = "nota-text")]
use nota_next::{NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};
use signal_harness::{HarnessInstanceConfiguration, HarnessKind, HarnessName};
use signal_persona::origin::{OwnerIdentity, UnixUserIdentifier};
use signal_persona::{SocketMode, WirePath};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MetaHarnessFixture {
    exchange: ExchangeIdentifier,
}

impl MetaHarnessFixture {
    fn new() -> Self {
        Self {
            exchange: ExchangeIdentifier::new(
                SessionEpoch::new(1),
                ExchangeLane::Connector,
                LaneSequence::first(),
            ),
        }
    }

    fn configuration(&self) -> HarnessDaemonConfiguration {
        HarnessDaemonConfiguration {
            harness_socket_path: WirePath::new("/run/persona/harness.sock"),
            harness_socket_mode: SocketMode::new(0o600),
            supervision_socket_path: WirePath::new("/run/persona/harness-supervision.sock"),
            supervision_socket_mode: SocketMode::new(0o600),
            owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
            harnesses: vec![HarnessInstanceConfiguration {
                harness_name: HarnessName::new("designer"),
                harness_kind: HarnessKind::Codex,
                terminal_socket_path: Some(WirePath::new("/run/persona/terminal.sock")),
                pi_rpc_adapter: None,
            }],
        }
    }

    fn round_trip_request(&self, request: Operation) -> Operation {
        let frame = MetaHarnessFrame::new(MetaHarnessFrameBody::Request {
            exchange: self.exchange,
            request: request.clone().into_request(),
        });
        let bytes = frame.encode_length_prefixed().expect("encode request");
        let decoded = MetaHarnessFrame::decode_length_prefixed(&bytes).expect("decode request");
        match decoded.into_body() {
            MetaHarnessFrameBody::Request { request, .. } => request.payloads().head().clone(),
            other => panic!("expected request frame, got {other:?}"),
        }
    }

    fn round_trip_reply(&self, reply: MetaHarnessReply) -> MetaHarnessReply {
        let frame = MetaHarnessFrame::new(MetaHarnessFrameBody::Reply {
            exchange: self.exchange,
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
        });
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
}

#[test]
fn configure_request_carries_harness_daemon_configuration() {
    let fixture = MetaHarnessFixture::new();
    let request = Operation::Configure(fixture.configuration());

    assert_eq!(request.kind(), OperationKind::Configure);
    assert_eq!(fixture.round_trip_request(request.clone()), request);
}

#[test]
fn meta_harness_request_heads_are_contract_local_operations() {
    assert_eq!(<Operation as SignalOperationHeads>::HEADS, &["Configure"]);
}

#[test]
fn reply_variants_round_trip() {
    let fixture = MetaHarnessFixture::new();
    let replies = [
        MetaHarnessReply::Configured(Configured {
            generation: ConfigurationGeneration::new(7),
        }),
        MetaHarnessReply::ConfigurationRejected(ConfigurationRejected {
            reason: ConfigurationRejectionReason::ManagerAuthorityRequired,
        }),
        MetaHarnessReply::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::Configure,
            reason: UnimplementedReason::DependencyNotReady,
        }),
    ];

    for reply in replies {
        assert_eq!(fixture.round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn configuration_generation_projects_to_integer() {
    let generation = ConfigurationGeneration::new(11);
    assert_eq!(generation.value(), 11);
}

#[cfg(feature = "nota-text")]
#[test]
fn meta_harness_operations_encode_as_contract_local_nota_heads() {
    let fixture = MetaHarnessFixture::new();
    let request = Operation::Configure(fixture.configuration());
    let text = request.to_nota();

    assert!(text.starts_with("(Configure"));

    let decoded = NotaSource::new(&text)
        .parse::<Operation>()
        .expect("decode request nota");
    assert_eq!(decoded, request);
}
