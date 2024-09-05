#include "curses_include.hpp"
#include <utility>
#include <cstdint>
namespace nutils
{
    auto screen_size()
    {
        struct
        {
            int16_t x = getmaxx(stdscr);
            int16_t y = getmaxy(stdscr);
        } screen;
        return screen;
    }
} // namespace nutils
