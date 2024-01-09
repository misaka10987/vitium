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
    // std::thread input_proc(input_func);
    const auto &main_win = cc::terminal::main_win;
    cc::terminal init;
    maxX = main_win.max_yx().second;
    maxY = main_win.max_yx().first;
    main_win << cc::format(0)("vitium");
    auto info_window{cc::widget::window{{1, 0, 15, maxX - 21}, cc::terminal::main_win}}; //{int curserY,int curserX,int heightY,int widthX}
    info_window << cc::format(2, 2)("HelloWorld from info-window!");
    info_window.get_char();
    return 0;
}