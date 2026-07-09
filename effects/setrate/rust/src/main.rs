use std::env;
use wav_util::{read_wav, write_wav, AudioData};
fn apply_new_sample_rate(audio: &mut AudioData, sample_rate: u32) {
    audio.sample_rate = sample_rate;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage:\n  {} input.wav output.wav --setrate <value>\n",
            args[0]
        );
        return;
    }

    let input = &args[1];
    let output = &args[2];
    let mode = &args[3];
    let value: u32 = args[4].parse().unwrap();

    let sample_rate = match mode.as_str() {
        "--setrate" => value,
        _ => {
            eprintln!("Unknown mode");
            return;
        }
    };

    if sample_rate > 384000 {
        panic!("Sample rate cannot be higher than 384kHz")
    }

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
        audio.samples.len(),
    );

    println!(
        "Applying new sample rate: {} Hz", sample_rate
    );

    apply_new_sample_rate(&mut audio, sample_rate);

    if let Err(e) = write_wav(output, &audio) {
        eprintln!("{}", e);
        return;
    }

    println!("Wrote: {}", output);
}

