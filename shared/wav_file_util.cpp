//
// Created by abh80 on 08-07-2026.
//

#include <fstream>
#include <ios>
#include <iostream>
#include <cstring>
#include "wav_file_util.h"

#pragma pack(push, 1)
struct RiffHeader {
    char riff[4];
    uint32_t file_size;
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

bool read_wav(const std::string &path, AudioData &data) {
    std::ifstream file(path, std::ios::binary);
    if (!file) {
        std::cerr << "Could not open input file: " << path << "\n";
        return false;
    }
    RiffHeader header{};

    file.read(reinterpret_cast<char *>(&header), sizeof(header));
    if (std::strncmp(header.riff, "RIFF", 4) != 0 || std::strncmp(header.wave, "WAVE", 4) != 0) {
        std::cerr << "Not a valid WAV file (missing RIFF/WAVE tags)\n";
        return false;
    }

    bool have_fmt = false;
    bool have_data = false;

    while (file && !(have_fmt && have_data)) {
        char tag[4];
        uint32_t chunk_size;

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

            switch (fmt.bitsPerSample) {
                case 8:
                case 16:
                case 24:
                case 32:
                    break;

                default:
                    throw std::runtime_error("Unsupported PCM bit depth");
            }

            data.num_channels = fmt.numChannels;
            data.bits_per_sample = fmt.bitsPerSample;
            data.sample_rate = fmt.sampleRate;

            have_fmt = true;

            if (chunk_size > 16) file.seekg(chunk_size - sizeof(fmt), std::ios::cur);
        } else if (std::strncmp(tag, "data", 4) == 0) {
            data.samples.resize(chunk_size);
            file.read(reinterpret_cast<char *>(data.samples.data()), chunk_size);
            have_data = true;
        } else {
            file.seekg(chunk_size, std::ios::cur);
        }
    }

    if (!have_fmt || !have_data) {
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

    uint32_t data_bytes = data.samples.size(); // same bytes as length of samples vector
    uint32_t riff_size = 4 + 8 + 16 + 8 + data_bytes;

    RiffHeader header{
        {'R', 'I', 'F', 'F'}, riff_size, {'W', 'A', 'V', 'E'}
    };

    file.write(reinterpret_cast<const char *>(&header), sizeof(header));
    uint32_t byte_rate = data.sample_rate * data.num_channels *  (data.bits_per_sample / 8);
    uint16_t block_align = data.num_channels * (data.bits_per_sample / 8);

    FmtChunk fmt {1, data.num_channels, data.sample_rate, byte_rate, block_align, data.bits_per_sample};
    file.write("fmt ", 4);
    uint32_t fmt_size = 16;
    file.write(reinterpret_cast<const char *>(&fmt_size), 4);
    file.write(reinterpret_cast<char *>(&fmt), sizeof(fmt));

    file.write("data", 4);
    file.write(reinterpret_cast<const char *>(&data_bytes), sizeof(data_bytes));
    file.write(reinterpret_cast<const char *>(data.samples.data()), data_bytes);

    return true;
}
