# Set the minimum required version of CMake
cmake_minimum_required(VERSION 3.15)

# Define the project name and language
project(my_project CXX)

# Set the C++ standard (C++17 is a great choice, or use 20)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# This is crucial for Neovim's LSP (clangd) to work correctly
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Find all your .cpp files in the src directory
file(GLOB SOURCES "src/*.cpp")

# Tell CMake where to find header files
include_directories(include)

# Create the executable from your source files
add_executable(my_project ${SOURCES})
