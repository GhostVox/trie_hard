#TODO(brinhasavlin): fix this whole file. it is not good

# Set the minimum required version of CMake
cmake_minimum_required(VERSION 3.15)

project(trie_hard CXX)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# This is crucial for many IDEs and tools like clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# -----------------------------------------------------------------------------
# LIBRARY TARGET (This is what gets packaged)
# -----------------------------------------------------------------------------

# Create an INTERFACE library target. This is for header-only libraries.
add_library(trie_hard INTERFACE)

# Tell the library target where its public header files are located.
# This uses generator expressions to handle paths correctly for both
# local builds and for the final installed package.
target_include_directories(trie_hard INTERFACE
  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
  $<INSTALL_INTERFACE:include>
)


# -----------------------------------------------------------------------------
# EXECUTABLE TARGET (For local testing, not part of the final package)
# -----------------------------------------------------------------------------

# Create an executable from main.cpp for you to test your code.
add_executable(trie_hard_runner src/main.cpp)

# Link the executable against the library to give it access to the headers.
target_link_libraries(trie_hard_runner PRIVATE trie_hard)


# -----------------------------------------------------------------------------
# INSTALL RULES (Crucial for creating a Conan package)
# -----------------------------------------------------------------------------

# This section tells CMake what to include in the final package.

# 1. Install the header files.
# This copies the entire 'include/' directory into the package.
install(DIRECTORY include/ DESTINATION include)

# 2. Install the library's CMake target information.
# This allows other CMake projects to find and use your library with find_package().
install(TARGETS trie_hard EXPORT trie_hard-targets)

# 3. Generate and install the necessary CMake configuration files for the package.
install(EXPORT trie_hard-targets
  FILE      trie_hard-targets.cmake
  NAMESPACE trie_hard::
  DESTINATION lib/cmake/trie_hard
)


