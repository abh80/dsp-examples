//
// Created by abh80 on 08-07-2026.
//

#include <wav_file_util.h>
#include <iostream>
#include <cstdio>
#include <cmath>
#include <exception>
#include <stdexcept>

double softClip(double x, double ceiling) {
    return ceiling * std::tanh(x / ceiling);
}

void apply_gain_pcm24(AudioData &data, double gain) {
    // I WILL make it later its a bit complicated
    throw std::runtime_error("24-bit PCM not implemented");
}

void apply_gain_pcm8(AudioData &data, double gain) {
    auto *samples = data.samples.data();
    const size_t count = data.samples.size();

    for (size_t i = 0; i < count; ++i) {
        double sample = static_cast<int>(samples[i]) - 128.0;
        // PCM 8 bit is unsigned by default (its the only exception; frm 0 to 255 so we subtract 128)
        sample *= gain;
        sample = softClip(sample, 127.0);
        samples[i] = static_cast<uint8_t>(std::lround(sample + 128.0));
    }
}

void apply_gain_pcm16(AudioData &data, double gain) {
    auto *samples = reinterpret_cast<int16_t *>(data.samples.data());
    const size_t count = data.samples.size() / sizeof(int16_t);
    for (size_t i = 0; i < count; ++i) {
        double sample = static_cast<double>(samples[i]) * gain;
        sample *= gain;
        sample = softClip(sample, 32767.0);
        samples[i] = static_cast<int16_t>(std::lround(sample));
    }
}

void apply_gain_pcm32(AudioData & data, double gain) {
    auto *samples = reinterpret_cast<int32_t *>(data.samples.data());
    const size_t count = data.samples.size() / sizeof(int32_t);
    for (size_t i = 0; i < count; ++i) {
        double sample = static_cast<double>(samples[i]) * gain;
        sample *= gain;
        sample = softClip(sample, 2147483647.0);
        samples[i] = static_cast<int32_t>(std::llround(sample));
    }
}

void apply_gain(AudioData &data, double gain) {
    switch (data.bits_per_sample) {
        case 8:
            apply_gain_pcm8(data, gain);
            break;
        case 16:
            apply_gain_pcm16(data, gain);
            break;
        case 24:
            apply_gain_pcm24(data, gain);
            break;
        case 32:
            apply_gain_pcm32(data, gain);
            break;
        default:
            throw std::runtime_error("This should not happen; check if you are using correct header files");
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

    try {
        AudioData audio;
        if (!read_wav(inPath, audio)) return 1;

        std::cout << "Loaded: " << audio.sample_rate << " Hz, "
                << audio.num_channels << " channel(s), "
                << audio.samples.size() << " samples\n";
        std::cout << "Applying gain: " << linearGain << "x ("
                << 20.0 * std::log10(linearGain) << " dB)\n";

        apply_gain(audio, linearGain);

        if (!write_wav(outPath, audio)) return 1;
        std::cout << "Wrote: " << outPath << "\n";
        return 0;
    } catch (const std::exception &error) {
        std::cerr << "Error: " << error.what() << "\n";
        return 1;
    }
}