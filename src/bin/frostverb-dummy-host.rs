use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

use frostverb::{
    FACTORY_PRESETS, FrostVerbEngine, HostParameterSnapshot, StereoFrame,
    host::{PLUGIN_DESCRIPTOR, factory_preset_by_name},
};

const SAMPLE_RATE: u32 = 48_000;
const BLOCK_SIZE: usize = 64;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let out_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("target/frostverb-dummy-host.wav");
    let preset_name = args.get(2).map(String::as_str).unwrap_or("Whiteout");
    let preset = factory_preset_by_name(preset_name).unwrap_or(&FACTORY_PRESETS[4]);

    let mut state = HostParameterSnapshot::new(preset.params);
    let mut engine = FrostVerbEngine::new();
    engine.prepare(SAMPLE_RATE as f32);

    let seconds = 10.0;
    let total_frames = (SAMPLE_RATE as f32 * seconds) as usize;
    let mut rendered = Vec::with_capacity(total_frames);
    let mut input_block = vec![StereoFrame::ZERO; BLOCK_SIZE];
    let mut output_block = vec![StereoFrame::ZERO; BLOCK_SIZE];
    let mut frames_done = 0;

    while frames_done < total_frames {
        let frames_this_block = BLOCK_SIZE.min(total_frames - frames_done);
        for (offset, frame) in input_block.iter_mut().take(frames_this_block).enumerate() {
            let frame_index = frames_done + offset;
            let t = frame_index as f32 / SAMPLE_RATE as f32;
            *frame = dummy_host_input(t);
        }

        automate_dummy_host(&mut state, frames_done as f32 / total_frames as f32);
        engine.process_block(
            &input_block[..frames_this_block],
            &mut output_block[..frames_this_block],
            &state.params,
        );
        rendered.extend_from_slice(&output_block[..frames_this_block]);
        frames_done += frames_this_block;
    }

    write_wav(Path::new(out_path), SAMPLE_RATE, &rendered)?;
    let meters = engine.meters();
    println!(
        "{} dummy backend rendered {} using preset '{}' blocks={} peak={:.3} wet={:.3} limiter_gr={:.2} dB",
        PLUGIN_DESCRIPTOR.name,
        out_path,
        preset.name,
        total_frames.div_ceil(BLOCK_SIZE),
        meters.output_peak,
        meters.wet_peak,
        meters.limiter_gain_reduction_db
    );
    println!("final state:\n{}", state.encode_state());
    Ok(())
}

fn automate_dummy_host(state: &mut HostParameterSnapshot, progress: f32) {
    let slow = (progress * core::f32::consts::TAU).sin() * 0.5 + 0.5;
    let freeze = if progress > 0.48 {
        ((progress - 0.48) / 0.20).clamp(0.0, 1.0) * 0.65
    } else {
        0.0
    };

    state.set_normalized("storm", 0.18 + slow * 0.42).unwrap();
    state
        .set_normalized("wind", 0.20 + progress * 0.55)
        .unwrap();
    state.set_normalized("freeze", freeze).unwrap();
    state
        .set_normalized("mix", 0.32 + (progress * 0.16).min(0.16))
        .unwrap();
}

fn dummy_host_input(t: f32) -> StereoFrame {
    let phrase = pluck(t, 0.10, 146.83, 0.55)
        + pluck(t, 0.85, 220.00, 0.38)
        + pluck(t, 1.55, 293.66, 0.42)
        + pluck(t, 2.30, 196.00, 0.36)
        + pluck(t, 3.25, 329.63, 0.30)
        + pluck(t, 4.10, 246.94, 0.34);
    let repeats = phrase + pluck(t % 2.75, 0.35, 174.61, 0.22);
    StereoFrame {
        left: repeats * 0.42,
        right: repeats * 0.36 + sine(t, 0.19) * repeats * 0.08,
    }
}

fn pluck(t: f32, start: f32, hz: f32, decay: f32) -> f32 {
    if t < start {
        return 0.0;
    }
    let x = t - start;
    let envelope = (-x / decay).exp();
    let tone = sine(x, hz) * 0.75 + sine(x, hz * 2.01) * 0.18 + sine(x, hz * 3.02) * 0.07;
    tone * envelope
}

fn sine(t: f32, hz: f32) -> f32 {
    (core::f32::consts::TAU * t * hz).sin()
}

fn write_wav(path: &Path, sample_rate: u32, frames: &[StereoFrame]) -> io::Result<()> {
    let mut file = File::create(path)?;
    let data_bytes = frames.len() as u32 * 2 * 2;
    let riff_size = 36 + data_bytes;

    file.write_all(b"RIFF")?;
    file.write_all(&riff_size.to_le_bytes())?;
    file.write_all(b"WAVE")?;
    file.write_all(b"fmt ")?;
    file.write_all(&16_u32.to_le_bytes())?;
    file.write_all(&1_u16.to_le_bytes())?;
    file.write_all(&2_u16.to_le_bytes())?;
    file.write_all(&sample_rate.to_le_bytes())?;
    file.write_all(&(sample_rate * 2 * 2).to_le_bytes())?;
    file.write_all(&4_u16.to_le_bytes())?;
    file.write_all(&16_u16.to_le_bytes())?;
    file.write_all(b"data")?;
    file.write_all(&data_bytes.to_le_bytes())?;

    for frame in frames {
        write_i16(&mut file, frame.left)?;
        write_i16(&mut file, frame.right)?;
    }

    Ok(())
}

fn write_i16(file: &mut File, sample: f32) -> io::Result<()> {
    let scaled = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
    file.write_all(&scaled.to_le_bytes())
}
