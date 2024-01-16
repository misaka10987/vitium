cmake_minimum_required(VERSION 3.10) 
project(vitium CXX)

# specify the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# aux_source_directory(client DIR_SRCS)
find_package(Curses REQUIRED)
include_directories(vitium vitium/client ${CURSES_INCLUDE_DIR})

add_executable(vitium client-main/client_main.cpp)
target_link_libraries(vitium ${CURSES_LIBRARIES})
