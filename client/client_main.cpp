#include <bits/stdc++.h> //very bad habit
#include "cxxcurses/cxxcurses.hpp"
namespace cc = cxxcurses;

// global vars here
int maxX,maxY;
std::ostringstream my_input_stream;

char input_func()
{
}

int main()
{
    // init
    // std::thread input_proc(input_func);
    const auto& main_win = cc::terminal::main_win;
    cc::terminal init;
    // main_win.max_yx();
    main_win << cc::format(1)("vitium");
}