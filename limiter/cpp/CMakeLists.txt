add_executable(limiter main.cpp
        ../../shared/wav_file_util.cpp
        ../../shared/audio_data_util.cpp)

target_include_directories(limiter PRIVATE ../../shared)