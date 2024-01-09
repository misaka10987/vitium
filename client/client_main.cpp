#include <bits/stdc++.h> //very bad habit
#include "cxxcurses/cxxcurses.hpp"
namespace cc = cxxcurses;

// global vars here
int maxX, maxY;
std::ostringstream my_input_stream;

char input_func()
{
}

int main()
{
    // init
    const auto &main_win = cc::terminal::main_win;
    cc::terminal init;
    maxX = main_win.max_yx().second;
    maxY = main_win.max_yx().first;
    main_win << cc::format(0)("vitium");
    auto info_window{cc::widget::window{{1, 0, 15, maxX - 21}, cc::terminal::main_win}}; //{int curserY,int curserX,int heightY,int widthX}
    auto stat_window{cc::widget::window{{1, maxX - 20, 15, 20}, cc::terminal::main_win}};
    auto story_window{cc::widget::window{{17, 0, maxY - 17, maxX}, cc::terminal::main_win}};
    auto buffer_window{cc::widget::window{{maxY, 0, 1, maxX}, cc::terminal::main_win}};
    // std::thread input_proc(input_func);
    info_window << cc::format(2, 2)("HelloWorld from info-window!");
    stat_window << cc::format("Hello !", '\n', "From Stat");
    story_window << cc::format(5)("Welcome to vitium client (under dev) !");
    buffer_window.get_char();
    return 0;
}