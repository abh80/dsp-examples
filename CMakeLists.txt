cmake_minimum_required(VERSION 3.15)
project(dsp_examples)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(gain/cpp)
add_subdirectory(effects/setrate/cpp)
add_subdirectory(limiter/cpp)