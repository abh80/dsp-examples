//
// Created by abh80 on 08-07-2026.
//

#ifndef DSP_EXAMPLES_AUDIO_DATA_UTIL_H
#define DSP_EXAMPLES_AUDIO_DATA_UTIL_H
#include <cstdint>
#include <vector>

struct AudioData {
    uint16_t num_channels; // usually we have 2 - 10 channels (atmos ones) 16 bit can easily represent this value.
    uint32_t sample_rate; // sample rates go till 192k - 384khz so we can 32 bit to easily represent this value.
    uint16_t bits_per_sample; // this the bit depth; how hi res around 24 or 16 for pcm16le.
    std::vector<uint8_t> samples; // clever part is to store byte by byte so we dont have to write tons of methods just to parse different pcm formats.
};

#endif //DSP_EXAMPLES_AUDIO_DATA_UTIL_H
