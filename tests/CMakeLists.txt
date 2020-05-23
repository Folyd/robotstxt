project(robots)
cmake_minimum_required(VERSION 3.0)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

set(VERSION "0.0.0")

############ helper libs ############

include(CPack)
include(CheckCCompilerFlag)
include(ExternalProject)

############ dependencies ##############

configure_file(CMakeLists.txt.in libs/CMakeLists.txt)
execute_process(COMMAND ${CMAKE_COMMAND} -G "${CMAKE_GENERATOR}" . RESULT_VARIABLE result WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/libs)

IF (result)
    message(FATAL_ERROR "Failed to download dependencies: ${result}")
ENDIF ()

execute_process(COMMAND ${CMAKE_COMMAND} --build . RESULT_VARIABLE result WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/libs)

IF (result)
    message(FATAL_ERROR "Failed to download dependencies: ${result}")
ENDIF ()

add_subdirectory(
        ${CMAKE_CURRENT_BINARY_DIR}/libs/abseil-cpp-src
        ${CMAKE_CURRENT_BINARY_DIR}/libs/abseil-cpp-build
        EXCLUDE_FROM_ALL
)


include(CTest)

# googletest
add_subdirectory(${CMAKE_CURRENT_BINARY_DIR}/libs/gtest-src
        ${CMAKE_CURRENT_BINARY_DIR}/libs/gtest-build
        EXCLUDE_FROM_ALL)

set(INSTALL_GTEST 0)
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)

IF (CMAKE_VERSION VERSION_LESS 2.8.11)
    include_directories(${gtest_SOURCE_DIR}/include)
ENDIF ()

########### compiler flags ##############

set(COMPILER_FLAGS_TO_CHECK
        "-Wall" "-Werror=implicit-function-declaration"
        )

########### project files ###############

include_directories(./src)

############ tests ##############

enable_testing()

add_executable(robots-test ./src/robots_test.cc)
target_link_libraries(robots-test absl::base absl::container absl::strings gtest_main)

############ Rust lib ############

# Set default ExternalProject root directory
set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/Rust)

# Add rust_example
ExternalProject_Add(
        rust_robots
        DOWNLOAD_COMMAND ""
        CONFIGURE_COMMAND ""
        BUILD_COMMAND cargo build COMMAND cargo build --release
        BINARY_DIR "${CMAKE_SOURCE_DIR}"
        INSTALL_COMMAND ""
        LOG_BUILD ON)

# Create dependency
add_dependencies(robots-test rust_robots)

# Specify link libraries
target_link_libraries(robots-test
        debug "${CMAKE_SOURCE_DIR}/target/debug/librust_robotstxt.a"
        optimized "${CMAKE_SOURCE_DIR}/target/release/librust_robotstxt.a")

set_target_properties(robots-test PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED ON)

add_test(NAME robots-test COMMAND robots-test)
