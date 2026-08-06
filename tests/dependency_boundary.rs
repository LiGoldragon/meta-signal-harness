use std::process::Command;

#[test]
fn runtime_tree_is_one_current_signal_family_without_historical_codec() {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--all-features"])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success());
    let tree = String::from_utf8(output.stdout).expect("dependency tree");
    let lock = include_str!("../Cargo.lock");
    assert!(!tree.contains("nota"));
    assert_eq!(lock.matches("name = \"signal-frame\"").count(), 1, "{tree}");
    assert!(
        tree.contains("signal-frame.git?rev=8aa0bcaeb29fe9e461a11706a469638d2fd109ac#8aa0bcae")
    );
    for retired_frame in ["01676293", "0786fbe8", "f46872e7"] {
        assert!(!tree.contains(retired_frame), "{tree}");
    }
    assert!(
        tree.contains("signal-harness.git?rev=c05eacf167b9bee5245bdf46906428c8dee47c1b#c05eacf1")
    );
    assert!(
        tree.contains("signal-persona.git?rev=2802259fb1344495b1ad3b701fe81e0b7f9df9c3#2802259f")
    );
}

#[test]
fn historical_features_and_moving_pins_are_absent() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("schema").exists());
    assert!(root.join("examples/canonical.dotos").is_file());

    for source in [include_str!("../Cargo.toml"), include_str!("../src/lib.rs")] {
        for forbidden in ["nota", ".schema", "branch ="] {
            assert!(!source.to_ascii_lowercase().contains(forbidden));
        }
    }
}

#[test]
fn meta_contract_uses_the_direct_current_frame_binding() {
    let source = include_str!("../src/lib.rs");
    assert!(source.contains("channel MetaHarness contract MetaHarnessWire"));
    assert!(source.contains("ContractId::new"));
    assert!(!source.contains("ChannelRequest"));
    assert!(!source.contains("ChannelReply"));
}
