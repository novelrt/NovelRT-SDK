cmake_minimum_required(VERSION 3.13)
project(NovelRT_SDK)
include(ExternalProject)

set(CMAKE_CXX_STANDARD 17)

set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/NovelRusT)

ExternalProject_Add(
        NovelRusT
        DOWNLOAD_COMMAND ""
        CONFIGURE_COMMAND ""
        BUILD_COMMAND cargo build COMMAND cargo build --release
        BINARY_DIR "${CMAKE_SOURCE_DIR}/lib/NovelRusT"
        INSTALL_COMMAND ""
        LOG_BUILD ON
)

add_executable(NovelRT_SDK main.cpp)

set_target_properties(NovelRT_SDK PROPERTIES CXX_STANDARD 17 CXX_STANDARD_REQUIRED ON)
