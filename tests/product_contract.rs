use std::{fs, process::Command};

use frostverb::{
    FACTORY_PRESETS, FrostVerbEngine, FrostVerbParams, HostParameterSnapshot, PARAMETER_DEFS,
    StateError, StereoFrame, factory_presets_json,
    host::{PLUGIN_DESCRIPTOR, factory_preset_by_name},
};

#[test]
fn factory_preset_json_file_matches_code_export() {
    let file_json = fs::read_to_string("presets/factory_presets.json").unwrap();
    assert_eq!(strip_ws(&file_json), strip_ws(&factory_presets_json()));
}

#[test]
fn public_parameter_contract_has_unique_ids_and_valid_defaults() {
    for (index, param) in PARAMETER_DEFS.iter().enumerate() {
        assert!(!param.id.is_empty());
        assert!(!param.name.is_empty());
        assert!(param.min < param.max, "{}", param.id);
        assert!(
            (param.min..=param.max).contains(&param.default),
            "{} default out of range",
            param.id
        );
        assert!(param.automatable, "{} should be automatable", param.id);

        for previous in PARAMETER_DEFS.iter().take(index) {
            assert_ne!(param.id, previous.id, "duplicate id {}", param.id);
        }
    }
}

#[test]
fn host_snapshot_clamps_nonfinite_and_out_of_range_values() {
    let mut snapshot = HostParameterSnapshot::default();

    snapshot.set_normalized("coldness", 2.0).unwrap();
    snapshot.set_normalized("wind", -1.0).unwrap();
    snapshot.set_normalized("freeze", f32::NAN).unwrap();

    assert_eq!(snapshot.normalized("coldness"), Some(1.0));
    assert_eq!(snapshot.normalized("wind"), Some(0.0));
    assert_eq!(snapshot.normalized("freeze"), Some(0.0));
    assert_eq!(
        snapshot.set_normalized("missing", 0.5),
        Err(StateError::UnknownParameter)
    );
}

#[test]
fn host_state_rejects_missing_header_and_bad_numbers() {
    assert_eq!(
        HostParameterSnapshot::decode_state("coldness=0.5\n"),
        Err(StateError::MalformedState)
    );
    assert_eq!(
        HostParameterSnapshot::decode_state("frostverb-state-v1\ncoldness=not-a-number\n"),
        Err(StateError::MalformedState)
    );
}

#[test]
fn all_factory_presets_are_unique_and_lookupable() {
    for (index, preset) in FACTORY_PRESETS.iter().enumerate() {
        assert_eq!(
            factory_preset_by_name(preset.name).map(|found| found.params),
            Some(preset.params)
        );
        for previous in FACTORY_PRESETS.iter().take(index) {
            assert_ne!(preset.name, previous.name);
        }
    }
}

#[test]
fn process_block_matches_per_frame_processing() {
    let params = FACTORY_PRESETS[4].params;
    let input = deterministic_input(2048);

    let mut frame_engine = FrostVerbEngine::new();
    frame_engine.prepare(48_000.0);
    let frame_output = input
        .iter()
        .copied()
        .map(|frame| frame_engine.process_frame(frame, &params))
        .collect::<Vec<_>>();

    let mut block_engine = FrostVerbEngine::new();
    block_engine.prepare(48_000.0);
    let mut block_output = vec![StereoFrame::ZERO; input.len()];
    block_engine.process_block(&input, &mut block_output, &params);

    assert_eq!(frame_output, block_output);
    assert_eq!(frame_engine.meters(), block_engine.meters());
}

#[test]
fn interleaved_processing_matches_frame_processing() {
    let params = FACTORY_PRESETS[6].params;
    let input = deterministic_input(1024);

    let mut frame_engine = FrostVerbEngine::new();
    frame_engine.prepare(44_100.0);
    let frame_output = input
        .iter()
        .copied()
        .map(|frame| frame_engine.process_frame(frame, &params))
        .collect::<Vec<_>>();

    let mut interleaved = Vec::with_capacity(input.len() * 2);
    for frame in &input {
        interleaved.push(frame.left);
        interleaved.push(frame.right);
    }

    let mut interleaved_engine = FrostVerbEngine::new();
    interleaved_engine.prepare(44_100.0);
    interleaved_engine.process_interleaved_in_place(&mut interleaved, &params);

    for (index, expected) in frame_output.iter().enumerate() {
        assert_eq!(interleaved[index * 2], expected.left);
        assert_eq!(interleaved[index * 2 + 1], expected.right);
    }
}

#[test]
fn sanitized_params_clamp_every_public_field() {
    let params = FrostVerbParams::from_array([
        -1.0,
        2.0,
        f32::NAN,
        f32::INFINITY,
        0.5,
        0.6,
        0.7,
        0.8,
        0.9,
        1.0,
        -0.2,
        1.2,
    ]);

    assert_eq!(
        params.as_array(),
        [0.0, 1.0, 0.0, 0.0, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.0, 1.0]
    );
}

#[test]
fn plugin_descriptor_is_stable_for_wrappers() {
    assert_eq!(PLUGIN_DESCRIPTOR.name, "Frost Verb");
    assert_eq!(PLUGIN_DESCRIPTOR.vendor, "Nursven");
    assert_eq!(PLUGIN_DESCRIPTOR.bundle_id, "com.nursven.frostverb");
    assert_eq!(PLUGIN_DESCRIPTOR.audio_inputs, 2);
    assert_eq!(PLUGIN_DESCRIPTOR.audio_outputs, 2);
}

#[test]
fn renderer_binary_writes_valid_stereo_wav() {
    let path =
        std::env::temp_dir().join(format!("frostverb-render-test-{}.wav", std::process::id()));
    let status = Command::new(env!("CARGO_BIN_EXE_frostverb-render"))
        .arg(&path)
        .arg("Crystal Tunnel")
        .status()
        .unwrap();

    assert!(status.success());

    let bytes = fs::read(&path).unwrap();
    let _ = fs::remove_file(&path);

    assert!(bytes.len() > 44);
    assert_eq!(&bytes[0..4], b"RIFF");
    assert_eq!(&bytes[8..12], b"WAVE");
    assert_eq!(&bytes[12..16], b"fmt ");
    assert_eq!(&bytes[22..24], &2_u16.to_le_bytes());
    assert_eq!(&bytes[24..28], &48_000_u32.to_le_bytes());
    assert_eq!(&bytes[36..40], b"data");
    assert_eq!(
        u32::from_le_bytes(bytes[40..44].try_into().unwrap()) as usize,
        bytes.len() - 44
    );
}

#[test]
fn dummy_host_binary_writes_valid_stereo_wav() {
    let path = std::env::temp_dir().join(format!(
        "frostverb-dummy-host-test-{}.wav",
        std::process::id()
    ));
    let status = Command::new(env!("CARGO_BIN_EXE_frostverb-dummy-host"))
        .arg(&path)
        .arg("Whiteout")
        .status()
        .unwrap();

    assert!(status.success());

    let bytes = fs::read(&path).unwrap();
    let _ = fs::remove_file(&path);

    assert!(bytes.len() > 44);
    assert_eq!(&bytes[0..4], b"RIFF");
    assert_eq!(&bytes[8..12], b"WAVE");
    assert_eq!(&bytes[22..24], &2_u16.to_le_bytes());
    assert_eq!(&bytes[24..28], &48_000_u32.to_le_bytes());
    assert_eq!(&bytes[36..40], b"data");
}

fn deterministic_input(frames: usize) -> Vec<StereoFrame> {
    (0..frames)
        .map(|i| {
            let t = i as f32 / 48_000.0;
            StereoFrame {
                left: (t * 417.0 * core::f32::consts::TAU).sin() * 0.35,
                right: (t * 293.0 * core::f32::consts::TAU).cos() * 0.27,
            }
        })
        .collect()
}

fn strip_ws(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}
