//
// Created by abh80 on 08-07-2026.
//

// This is a shared header file with implementation in corresponding .cpp file for sharing the wav file utility functions
// like read and write. This is modified from gain example to support multiple pcm formats.

#ifndef DSP_EXAMPLES_WAV_FILE_UTIL_H
#define DSP_EXAMPLES_WAV_FILE_UTIL_H
#include <string>
#include "audio_data_util.h"

bool read_wav(const std::string &path, AudioData &data);

bool write_wav(const std::string &path, const AudioData &data);

#endif //DSP_EXAMPLES_WAV_FILE_UTIL_H
