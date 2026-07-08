//
// Created by abh80 on 08-07-2026.
// Set rate changes the sample rate without changing audio data; So only the fmt header will be modified and decoders will pick them up.

#include <wav_file_util.h>
#include <iostream>
#include <cstdio>
#include <cmath>
#include <exception>
#include <stdexcept>

void set_rate(AudioData &data, int32_t sample_rate) {
    if (sample_rate > 384000) throw std::runtime_error("Max sample rate can be 384000");
    data.sample_rate = sample_rate;
}

int main(int argc, char **argv) {
    if (argc < 5) {
        std::cerr << "Usage:\n"
                << "  " << argv[0] << " input.wav output.wav --setrate <value>\n";
        return 1;
    }

    std::string inPath = argv[1];
    std::string outPath = argv[2];
    std::string mode = argv[3];
    double value = std::atof(argv[4]);

    int32_t sample_rate;
    if (mode == "--setrate") {
        sample_rate = static_cast<int32_t>(value);
    } else {
        std::cerr << "Unknown mode: " << mode << " (use --setrate)\n";
        return 1;
    }

    try {
        AudioData audio;
        if (!read_wav(inPath, audio)) return 1;

        std::cout << "Loaded: " << audio.sample_rate << " Hz, "
                << audio.num_channels << " channel(s), "
                << audio.samples.size() << " samples, @" << audio.sample_rate << " Hz\n";
        std::cout << "Applying new sample rate: " << sample_rate << " Hz\n";

        set_rate(audio, sample_rate);

        if (!write_wav(outPath, audio)) return 1;
        std::cout << "Wrote: " << outPath << "\n";
        return 0;
    } catch (const std::exception &error) {
        std::cerr << "Error: " << error.what() << "\n";
        return 1;
    }
}