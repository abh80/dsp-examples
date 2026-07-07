#include <cmath>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

double softClip(double x, double ceiling) {
    return ceiling * std::tanh(x / ceiling);
}

#pragma pack(push, 1)
struct RiffHeader {
  char riff[4];
  uint32_t fileSize;
  char wave[4];
};

struct FmtChunk {
  uint16_t audioFormat; // 1 = PCM
  uint16_t numChannels;
  uint32_t sampleRate;
  uint32_t byteRate;
  uint16_t blockAlign;
  uint16_t bitsPerSample;
};
#pragma pack(pop)

struct AudioData {
  uint16_t numChannels = 0;
  uint32_t sampleRate = 0;
  uint16_t bitsPerSample = 0;
  std::vector<int16_t> samples;
};

bool read_wav(const std::string &path, AudioData &out) {
  std::ifstream file(path, std::ios::binary);
  if (!file) {
    std::cerr << "Could not open input file: " << path << "\n";
    return false;
  }
  RiffHeader header{};

  file.read(reinterpret_cast<char *>(&header), sizeof(header));
  if (std::strncmp(header.riff, "RIFF", 4) != 0 ||
      std::strncmp(header.wave, "WAVE", 4) != 0) {
    std::cerr << "Not a valid WAV file (missing RIFF/WAVE tags)\n";
    return false;
  }

  bool haveFmt = false;
  bool haveData = false;

  while (file && !(haveFmt && haveData)) {
    char tag[4];
    uint32_t chunk_size = 0;

    file.read(tag, 4);
    file.read(reinterpret_cast<char *>(&chunk_size), 4);

    if (!file)
      break;

    if (std::strncmp(tag, "fmt ", 4) == 0) {
      FmtChunk fmt{};
      file.read(reinterpret_cast<char *>(&fmt), sizeof(fmt));

      if (fmt.audioFormat != 1) {
        std::cerr << "Only uncompressed PCM WAV is supported "
                     "(this file uses a compressed/extended format)\n";
        return false;
      }
      if (fmt.bitsPerSample != 16) {
        std::cerr << "Only 16-bit PCM is supported (this file is "
                  << fmt.bitsPerSample << "-bit)\n";
        return false;
      }

      out.numChannels = fmt.numChannels;
      out.sampleRate = fmt.sampleRate;
      out.bitsPerSample = fmt.bitsPerSample;

      haveFmt = true;
      if (chunk_size > sizeof(fmt))
        file.seekg(chunk_size - sizeof(fmt), std::ios::cur);
    } else if (std::strncmp(tag, "data", 4) == 0) {
      out.samples.resize(chunk_size / sizeof(int16_t));
      file.read(reinterpret_cast<char *>(out.samples.data()), chunk_size);
      haveData = true;
    } else
      file.seekg(chunk_size, std::ios::cur);
  }
  if (!haveFmt || !haveData) {
    std::cerr << "WAV file is missing fmt or data chunk\n";
    return false;
  }
  return true;
}

bool write_wav(const std::string &path, const AudioData &data) {
  std::ofstream file(path, std::ios::binary);

  if (!file) {
    std::cerr << "Could not open output file " << path << "\n";
    return false;
  }

  uint32_t dataBytes =
      static_cast<uint32_t>(data.samples.size() * sizeof(uint16_t));
  uint32_t byteRate = static_cast<uint32_t>(data.sampleRate * data.numChannels *
                                            (data.bitsPerSample / 8));
  uint16_t blockAlign =
      static_cast<uint32_t>(data.numChannels * (data.bitsPerSample / 8));
  uint32_t riffSize =
      static_cast<uint32_t>(4 + 8 + sizeof(FmtChunk) + 8 + dataBytes);

  RiffHeader header{{'R', 'I', 'F', 'F'}, riffSize, {'W', 'A', 'V', 'E'}};

  file.write(reinterpret_cast<char *>(&header), sizeof(header));

  file.write("fmt ", 4);
  uint32_t fmtSize = sizeof(FmtChunk);
  FmtChunk fmt{1,        data.numChannels, data.sampleRate,
               byteRate, blockAlign,       data.bitsPerSample};
  file.write(reinterpret_cast<char *>(&fmtSize), 4);
  file.write(reinterpret_cast<char *>(&fmt), sizeof(fmt));

  file.write("data", 4);
  file.write(reinterpret_cast<char *>(&dataBytes), 4);
  file.write(reinterpret_cast<const char *>(data.samples.data()), dataBytes);

  return true;
}

double clip(double scaled) {
  if (scaled > 32767.0)
    return 32767.0;
  else if (scaled < -32768.0)
    return -32768.0;

  return scaled;
}

void apply_gain(AudioData &data, double linearGain) {
  for (int16_t &sample : data.samples) {
    double scaled = static_cast<double>(sample) * linearGain;
    scaled = softClip(scaled, 32767.0);
    sample = static_cast<int16_t>(std::lround(scaled));
  }
}

int main(int argc, char **argv) {
    if (argc < 5) {
        std::cerr << "Usage:\n"
                  << "  " << argv[0] << " input.wav output.wav --db <value>\n"
                  << "  " << argv[0] << " input.wav output.wav --linear <value>\n";
        return 1;
    }

    std::string inPath = argv[1];
    std::string outPath = argv[2];
    std::string mode = argv[3];
    double value = std::atof(argv[4]);

    double linearGain;
    if (mode == "--db") {
        linearGain = pow(10, (value / 20));
    } else if (mode == "--linear") {
        linearGain = value;
    } else {
        std::cerr << "Unknown mode: " << mode << " (use --db or --linear)\n";
        return 1;
    }

    AudioData audio;
    if (!read_wav(inPath, audio)) return 1;

    std::cout << "Loaded: " << audio.sampleRate << " Hz, "
              << audio.numChannels << " channel(s), "
              << audio.samples.size() << " samples\n";
    std::cout << "Applying gain: " << linearGain << "x ("
              << 20.0 * std::log10(linearGain) << " dB)\n";

    apply_gain(audio, linearGain);

    if (!write_wav(outPath, audio)) return 1;
    std::cout << "Wrote: " << outPath << "\n";
    return 0;
}