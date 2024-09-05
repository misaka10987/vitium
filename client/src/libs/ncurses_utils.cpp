#pragma once
#include "curses_include.hpp"
#include <tuple>
#include <cstdint>
namespace nutils
{
    struct vec2
    {
        int x;
        int y;
    };

    auto update_screen_size()
    {
        struct vec2 screen_size = {getmaxx(stdscr), getmaxy(stdscr)};
        return screen_size;
    }
} // namespace nutils, a few useful tool functions, structs and definitions
