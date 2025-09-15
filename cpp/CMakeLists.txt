
# Set the minimum required version of CMake
cmake_minimum_required(VERSION 3.15)

project(trie_hard CXX)


# Set the C++ standard (C++17 is a great choice, or use 20)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_LINK)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    message(STATUS "This build is optimized for debugging: optimizations have been reduced to a strict minimum
    (only return value optimization is active). Debugger symbols including unexpanded macro debugger symbols are included")
    add_compile_options(-g3 -O0)
elseif (CMAKE_BUILD_TYPE STREQUAL "Release")
    message(STATUS "This build is optimized for releasing and not debugging: aggressive optimizations are turned on.
        No debugger symbols will be produced")
    add_compile_options(-O3 -DNDEBUG)
endif ()

# This is crucial for Neovim's LSP (clangd) to work correctly
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Find all your .hpp files in the src directory
file(GLOB SOURCES "*.hpp" "*.cpp")

# Tell CMake where to find header files
include_directories(include)

# Create the executable from your source files
add_executable(my_project ${SOURCES})

# We set the language
set_target_properties(my_project PROPERTIES LINKER_LANGUAGE CXX)
