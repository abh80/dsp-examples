use std::io::{Seek, Write};
use std::{
    env,
    fs::File,
    io::{Read, SeekFrom},
};

const MAX_CEILING_UNSIGNED_INTEGER_16_BIT: u16 = 32767;
const DEFAULT_CEILING: f64 = MAX_CEILING_UNSIGNED_INTEGER_16_BIT as f64;

fn soft_clipping(sample: f64, ceiling: f64) -> f64 {
    ceiling * (sample / ceiling).tanh()
}

fn apply_gain(audio_data: &mut AudioData, linear_gain: f64) {
    for sample in &mut audio_data.samples {
        let mut scaled = *sample as f64 * linear_gain;
        scaled = soft_clipping(scaled, DEFAULT_CEILING);
        *sample = scaled.round() as i16;
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct RiffHeader {
    riff: [u8; 4],
    file_size: u32,
    wave: [u8; 4],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct FmtChunk {
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}

struct AudioData {
    num_channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
    samples: Vec<i16>,
}

fn read_u16(file: &mut File) -> std::io::Result<u16> {
    let mut buf = [0u8; 2];
    file.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32(file: &mut File) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_wav(path: &str) -> Result<AudioData, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;

    let mut riff = [0u8; 4];
    file.read_exact(&mut riff).map_err(|e| e.to_string())?;
    let file_size = read_u32(&mut file).map_err(|e| e.to_string())?;

    let mut wave = [0u8; 4];
    file.read_exact(&mut wave).map_err(|e| e.to_string())?;

    let _header = RiffHeader {
        riff,
        file_size,
        wave,
    };

    if &riff != b"RIFF" || &wave != b"WAVE" {
        return Err("Not a valid wav file".into());
    }

    let mut audio_data = AudioData {
        num_channels: 0,
        sample_rate: 0,
        bits_per_sample: 0,
        samples: Vec::new(),
    };

    let mut have_fmt = false;
    let mut have_data = false;

    while !(have_fmt && have_data) {
        let mut tag = [0u8; 4];
        file.read_exact(&mut tag)
            .map_err(|e| e.to_string())
            .expect("panic");

        let chunk_size = read_u32(&mut file).map_err(|e| e.to_string())?;
        if &tag == b"fmt " {
            let fmt = FmtChunk {
                audio_format: read_u16(&mut file).map_err(|e| e.to_string())?,
                num_channels: read_u16(&mut file).map_err(|e| e.to_string())?,
                sample_rate: read_u32(&mut file).map_err(|e| e.to_string())?,
                byte_rate: read_u32(&mut file).map_err(|e| e.to_string())?,
                block_align: read_u16(&mut file).map_err(|e| e.to_string())?,
                bits_per_sample: read_u16(&mut file).map_err(|e| e.to_string())?,
            };
            if fmt.audio_format != 1 {
                return Err("Only PCM wav file are supported".into());
            }
            let bits = fmt.bits_per_sample;

            if bits != 16 {
                return Err(format!(
                    "Only PCM 16 bit depth is supported (found {})",
                    bits
                ));
            }

            audio_data.num_channels = fmt.num_channels;
            audio_data.sample_rate = fmt.sample_rate;
            audio_data.bits_per_sample = fmt.bits_per_sample;
            have_fmt = true;

            if chunk_size > 16 {
                file.seek(SeekFrom::Current((chunk_size - 16) as i64))
                    .map_err(|e| e.to_string())
                    .expect("panic while seeking!");
            }
        } else if &tag == b"data" {
            let mut bytes = vec![0u8; chunk_size as usize];
            file.read_exact(&mut bytes).map_err(|e| e.to_string())?;

            audio_data.samples = bytes
                .chunks_exact(size_of::<i16>())
                .map(|sample| i16::from_le_bytes([sample[0], sample[1]]))
                .collect();

            have_data = true;
        } else {
            file.seek(SeekFrom::Current(chunk_size as i64))
                .map_err(|e| e.to_string())
                .expect("panic while seeking!");
        }
    }
    Ok(audio_data)
}

fn write_wav(path: &str, audio_data: &AudioData) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| e.to_string())?;

    let data_bytes = (audio_data.samples.len() * 2) as u32;
    let riff_size = 4 + 8 + 16 + 8 + data_bytes;

    file.write_all(b"RIFF").map_err(|e| e.to_string())?;
    file.write_all(&riff_size.to_le_bytes())
        .map_err(|e| e.to_string())?;
    file.write_all(b"WAVE").map_err(|e| e.to_string())?;

    file.write_all(b"fmt ").map_err(|e| e.to_string())?;
    file.write_all(&16u32.to_le_bytes())
        .map_err(|e| e.to_string())?;

    let byte_rate = (audio_data.num_channels as u32)
        * audio_data.sample_rate
        * (audio_data.bits_per_sample / 8) as u32;

    let block_align = audio_data.num_channels * (audio_data.bits_per_sample / 8);

    file.write_all(&1u16.to_le_bytes())
        .map_err(|e| e.to_string())?;
    file.write_all(&audio_data.num_channels.to_le_bytes())
        .map_err(|e| e.to_string())?;
    file.write_all(&audio_data.sample_rate.to_le_bytes())
        .map_err(|e| e.to_string())?;
    file.write_all(&byte_rate.to_le_bytes())
        .map_err(|e| e.to_string())?;
    file.write_all(&block_align.to_le_bytes())
        .map_err(|e| e.to_string())?;
    file.write_all(&audio_data.bits_per_sample.to_le_bytes())
        .map_err(|e| e.to_string())?;

    file.write_all(b"data").map_err(|e| e.to_string())?;
    file.write_all(&data_bytes.to_le_bytes())
        .map_err(|e| e.to_string())?;
    let mut sample_bytes = Vec::with_capacity(data_bytes as usize);
    for sample in &audio_data.samples {
        sample_bytes.extend_from_slice(&sample.to_le_bytes());
    }
    file.write_all(&sample_bytes).map_err(|e| e.to_string())?;

    Ok(())
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
