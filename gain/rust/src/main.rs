use std::env;

use wav_util::{read_wav, write_wav, AudioData};
const MAX_CEILING_UNSIGNED_INTEGER_16_BIT: u16 = 32767;
const DEFAULT_CEILING: f64 = MAX_CEILING_UNSIGNED_INTEGER_16_BIT as f64;

fn soft_clipping(sample: f64, ceiling: f64) -> f64 {
    ceiling * (sample / ceiling).tanh()
}

fn apply_gain(audio_data: &mut AudioData, linear_gain: f64) {
    match audio_data.bits_per_sample {
        8 => {
            for s in &mut audio_data.samples {
                let sample = (*s as i16) - 128;
                let scaled = sample as f64 * linear_gain;
                *s = (soft_clipping(scaled, 127.0).round() + 128.0) as u8;
            }
        }

        16 => {
            for s in audio_data.samples.chunks_exact_mut(2) {
                let sample = i16::from_le_bytes([s[0], s[1]]);
                let scaled =
                    soft_clipping(sample as f64 * linear_gain, DEFAULT_CEILING).round() as i16;
                s.copy_from_slice(&scaled.to_le_bytes());
            }
        }

        24 => {
            for s in audio_data.samples.chunks_exact_mut(3) {
                let sample = ((i32::from_le_bytes([s[0], s[1], s[2], 0])) << 8) >> 8;
                let scaled = soft_clipping(sample as f64 * linear_gain, 8388608.0).round() as i32;
                let bytes = scaled.to_le_bytes();

                s[0] = bytes[0];
                s[1] = bytes[1];
                s[2] = bytes[2];
            }
        }

        32 => {
            for s in audio_data.samples.chunks_exact_mut(4) {
                let sample = i32::from_le_bytes([s[0], s[1], s[2], s[3]]);
                let scaled =
                    soft_clipping(sample as f64 * linear_gain, 2147483647.0).round() as i32;
                s.copy_from_slice(&scaled.to_le_bytes());
            }
        }

        _ => panic!("Unsupported bit depth"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage:\n  {} input.wav output.wav --db <value>\n  {} input.wav output.wav --linear <value>",
            args[0], args[0]
        );
        return;
    }

    let input = &args[1];
    let output = &args[2];
    let mode = &args[3];
    let value: f64 = args[4].parse().unwrap();

    let linear_gain = match mode.as_str() {
        "--db" => 10f64.powf(value / 20.0),
        "--linear" => value,
        _ => {
            eprintln!("Unknown mode");
            return;
        }
    };

    let mut audio = match read_wav(input) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    println!(
        "Loaded: {} Hz, {} channel(s), {} samples",
        audio.sample_rate,
        audio.num_channels,
        audio.samples.len()
    );

    println!(
        "Applying gain: {:.3}x ({:.2} dB)",
        linear_gain,
        20.0 * linear_gain.log10()
    );

    apply_gain(&mut audio, linear_gain);

    if let Err(e) = write_wav(output, &audio) {
        eprintln!("{}", e);
        return;
    }

    println!("Wrote: {}", output);
}
