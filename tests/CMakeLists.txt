PROJECT(robotstxt_test)
CMAKE_MINIMUM_REQUIRED(VERSION 3.0)

SET(CMAKE_CXX_STANDARD 11)
SET(CMAKE_POSITION_INDEPENDENT_CODE ON)

SET(VERSION "0.0.0")

############ helper libs ############

INCLUDE(CPack)
INCLUDE(CheckCCompilerFlag)
INCLUDE(ExternalProject)

############ dependencies ##############

CONFIGURE_FILE(CMakeLists.txt.in libs/CMakeLists.txt)
EXECUTE_PROCESS(COMMAND ${CMAKE_COMMAND} -G "${CMAKE_GENERATOR}" . RESULT_VARIABLE result WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/libs)

IF (result)
    MESSAGE(FATAL_ERROR "Failed to download dependencies: ${result}")
ENDIF ()

EXECUTE_PROCESS(COMMAND ${CMAKE_COMMAND} --build . RESULT_VARIABLE result WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/libs)

IF (result)
    MESSAGE(FATAL_ERROR "Failed to download dependencies: ${result}")
ENDIF ()

ADD_SUBDIRECTORY(
        ${CMAKE_CURRENT_BINARY_DIR}/libs/abseil-cpp-src
        ${CMAKE_CURRENT_BINARY_DIR}/libs/abseil-cpp-build
        EXCLUDE_FROM_ALL
)


INCLUDE(CTest)

# googletest
ADD_SUBDIRECTORY(${CMAKE_CURRENT_BINARY_DIR}/libs/gtest-src
        ${CMAKE_CURRENT_BINARY_DIR}/libs/gtest-build
        EXCLUDE_FROM_ALL)

SET(INSTALL_GTEST 0)
SET(gtest_force_shared_crt ON CACHE BOOL "" FORCE)

IF (CMAKE_VERSION VERSION_LESS 2.8.11)
    INCLUDE_DIRECTORIES(${gtest_SOURCE_DIR}/include)
ENDIF ()


########### compiler flags ##############

SET(COMPILER_FLAGS_TO_CHECK
        "-Wall" "-Werror=implicit-function-declaration"
        )

########### project files ###############

INCLUDE_DIRECTORIES(./src)

######### targets ###########

SET(LIBROBOTS_LIBS)

#SET(robots_SRCS ./robots.cc)
SET(robots_LIBS absl::base absl::container absl::strings)

ADD_LIBRARY(robots SHARED ./target/debug/librust_robotstxt.dylib)
TARGET_LINK_LIBRARIES(robots ${robots_LIBS})
LIST(APPEND LIBROBOTS_LIBS "robots")

############ tests ##############

ENABLE_TESTING()

ADD_EXECUTABLE(robots-test ./src/robots_test.cc)
TARGET_LINK_LIBRARIES(robots-test ${LIBROBOTS_LIBS} gtest_main)
ADD_TEST(NAME robots-test COMMAND robots-test)
