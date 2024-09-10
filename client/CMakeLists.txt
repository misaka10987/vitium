cmake_minimum_required(VERSION 3.20)
project(Vitium)
set(CMAKE_CXX_STANDARD 20)

set(CMAKE_BUILD_TYPE Debug)

add_executable(Vitium src/main.cpp)

if(UNIX)
    set(CURSES_NEED_NCURSES TRUE)
    set(CURSES_NEED_WIDE TRUE)
    find_package(Curses)
    include_directories(${CURSES_INCLUDE_DIR})
    target_link_libraries(Vitium PRIVATE ${CURSES_LIBRARIES})
elseif(WIN32)
    target_link_libraries(Vitium PRIVATE ../../PDCurses/wincon/pdcurses.a)
else()
    message("This platform is still not supported. You may try to compile with pdcurses and link it manually.")
endif()

include(FetchContent)
FetchContent_Declare(cpr GIT_REPOSITORY https://github.com/libcpr/cpr.git
                         GIT_TAG 3b15fa82ea74739b574d705fea44959b58142eb8) # Replace with your desired git commit, we are using the ver 1.10.5
FetchContent_MakeAvailable(cpr)
target_link_libraries(Vitium PRIVATE cpr::cpr)
