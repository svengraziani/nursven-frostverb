use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

use frostverb::{FACTORY_PRESETS, FrostVerbEngine, StereoFrame, host::factory_preset_by_name};

const SAMPLE_RATE: u32 = 48_000;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let out_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("target/frostverb-demo.wav");
    let preset_name = args.get(2).map(String::as_str).unwrap_or("Frozen Cave");
    let preset = factory_preset_by_name(preset_name).unwrap_or(&FACTORY_PRESETS[0]);

    let mut engine = FrostVerbEngine::new();
    engine.prepare(SAMPLE_RATE as f32);

    let seconds = 8.0;
    let samples = (SAMPLE_RATE as f32 * seconds) as usize;
    let mut audio = Vec::with_capacity(samples);

    for i in 0..samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let excitation = excitation(t);
        audio.push(engine.process_frame(excitation, &preset.params));
    }

    write_wav(Path::new(out_path), SAMPLE_RATE, &audio)?;
    let meters = engine.meters();
    println!(
        "rendered {} with preset '{}' peak={:.3} wet={:.3} limiter_gr={:.2} dB",
        out_path,
        preset.name,
        meters.output_peak,
        meters.wet_peak,
        meters.limiter_gain_reduction_db
    );
    Ok(())
}

fn excitation(t: f32) -> StereoFrame {
    let note_a = burst(t, 0.20, 0.34) * sine(t, 146.83);
    let note_b = burst(t, 1.15, 0.28) * sine(t, 220.00);
    let note_c = burst(t, 2.05, 0.42) * (0.7 * sine(t, 293.66) + 0.3 * sine(t, 440.00));
    let hit = burst(t, 3.20, 0.18) * filtered_click(t);
    let value = (note_a + note_b + note_c + hit) * 0.45;
    StereoFrame {
        left: value,
        right: value * 0.92,
    }
}

fn burst(t: f32, start: f32, decay: f32) -> f32 {
    if t < start {
        0.0
    } else {
        let x = t - start;
        (-x / decay).exp()
    }
}

fn sine(t: f32, hz: f32) -> f32 {
    (core::f32::consts::TAU * t * hz).sin()
}

fn filtered_click(t: f32) -> f32 {
    let carrier = sine(t, 880.0) + 0.4 * sine(t, 1760.0);
    carrier.signum() * carrier.abs().sqrt()
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
