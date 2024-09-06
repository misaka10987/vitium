cmake_minimum_required(VERSION 3.20)
project(Vitium-client-ng)
set(CMAKE_CXX_STANDARD 17)

add_executable(Vitium src/main.cpp)

include(FetchContent)
FetchContent_Declare(cpr GIT_REPOSITORY https://github.com/libcpr/cpr.git
                         GIT_TAG 3b15fa82ea74739b574d705fea44959b58142eb8) # Replace with your desired git commit, we are using the ver 1.10.5
FetchContent_MakeAvailable(cpr)
target_link_libraries(Vitium PRIVATE cpr::cpr)
