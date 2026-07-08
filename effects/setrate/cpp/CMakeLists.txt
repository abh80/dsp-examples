add_executable(
        set_rate
        main.cpp
        ../../../shared/wav_file_util.cpp
        ../../../shared/audio_data_util.cpp)


target_include_directories(set_rate PRIVATE ../../../shared)