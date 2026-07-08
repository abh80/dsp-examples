add_executable(gain
        gain.cpp
        ../../shared/wav_file_util.cpp
        ../../shared/audio_data_util.cpp)

target_include_directories(gain PRIVATE ../../shared)