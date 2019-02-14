cmake_minimum_required(VERSION 3.13)
project(NovelRT_SDK)

add_executable(NovelRT_SDK main.cpp)

set_target_properties(NovelRT_SDK PROPERTIES CXX_STANDARD 17 CXX_STANDARD_REQUIRED ON)
