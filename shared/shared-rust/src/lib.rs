use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

pub struct AudioData {
    pub num_channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub samples: Vec<u8>,
}
#[repr(C, packed)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct RiffHeader {
    riff: [u8; 4],
    file_size: u32,
    wave: [u8; 4],
}

pub fn read_wav(path: &str) -> std::io::Result<AudioData> {
    let mut file = File::open(path)?;
    let riff = read_four_bytes(&mut file)?;
    let file_size = read_u32(&mut file)?;
    let wave = read_four_bytes(&mut file)?;

    let _header = RiffHeader {
        riff,
        file_size,
        wave,
    };
    let mut has_fmt = false;
    let mut has_data = false;

    let mut audio = AudioData {
        num_channels: 0,
        sample_rate: 0,
        bits_per_sample: 0,
        samples: Vec::new(),
    };
    while !(has_fmt && has_data) {
        let tag = read_four_bytes(&mut file)?;
        let chunk_size = read_u32(&mut file)?;

        if &tag == b"fmt " {
            let audio_format = read_u16(&mut file)?;
            let num_channels = read_u16(&mut file)?;
            let sample_rate = read_u32(&mut file)?;
            file.seek(SeekFrom::Current(6))?;
            let bits_per_sample = read_u16(&mut file)?;

            if audio_format != 1 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid WAV header",
                ));
            }
            match bits_per_sample {
                8 | 16 | 24 | 32 => {}
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Unsupported PCM bit depth",
                    ));
                }
            }
            if chunk_size > 16 {
                file.seek(SeekFrom::Current((chunk_size - 16) as i64))?;
            }
            has_fmt = true;

            audio.num_channels = num_channels;
            audio.bits_per_sample = bits_per_sample;
            audio.sample_rate = sample_rate;
        } else if &tag == b"data" {
            let mut bytes = vec![0u8; chunk_size as usize];
            file.read_exact(&mut bytes)?;
            audio.samples = bytes;
            has_data = true;
        } else {
            file.seek(SeekFrom::Current(chunk_size as i64))?;
        }
    }

    Ok(audio)
}
#[repr(C, packed)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct FmtChunk {
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}

pub fn write_wav(path: &str, audio: &AudioData) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let file_size: u32 = 4 + 8 + 16 + 8 + audio.samples.len() as u32;
    let _header = RiffHeader {
        riff: *b"RIFF",
        file_size,
        wave: *b"WAVE",
    };

    file.write_all(bytemuck::bytes_of(&_header))?;

    let _fmt = FmtChunk {
        audio_format: 1,
        num_channels: audio.num_channels,
        sample_rate: audio.sample_rate,
        byte_rate: audio.sample_rate * (audio.num_channels * (audio.bits_per_sample / 8)) as u32,
        block_align: audio.num_channels * (audio.bits_per_sample / 8),
        bits_per_sample: audio.bits_per_sample,
    };

    file.write_all(b"fmt ")?;
    file.write_all(&16u32.to_le_bytes())?;
    file.write_all(bytemuck::bytes_of(&_fmt))?;

    file.write_all(b"data")?;
    file.write_all(&audio.samples.len().to_le_bytes())?;
    file.write_all(&audio.samples)?;

    Ok(())
}

fn read_u32(file: &mut File) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    return Ok(u32::from_le_bytes(buf));
}

fn read_u16(file: &mut File) -> std::io::Result<u16> {
    let mut buf = [0u8; 2];
    file.read_exact(&mut buf)?;
    return Ok(u16::from_le_bytes(buf));
}

fn read_four_bytes(file: &mut File) -> std::io::Result<[u8; 4]> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    Ok(buf)
}
