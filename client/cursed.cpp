#include "cxxcurses/cxxcurses.hpp"

namespace cursed
{
    typedef cxxcurses::widget::window RawWindow;
    int MAX_X = 0;
    int MAX_Y = 0;
    auto init() -> int;
    template<int ori_y, int ori_x, int height, int width>
    class Window;
}

auto cursed::init() -> int {
    cxxcurses::terminal _init;
    auto main_win = &cxxcurses::terminal::main_win;
    MAX_X = main_win->max_yx().second;
    MAX_Y = main_win->max_yx().first;
    return MAX_X * MAX_Y != 0;
}

template<int ori_y, int ori_x, int height, int width>
class cursed::Window {
private:
    RawWindow _w{
        {
            ori_y,
            ori_x,
            height > 0 ? height : MAX_Y + height,
            width > 0 ? width : MAX_X + width,
        },
        cxxcurses::terminal::main_win
    };
public:
    Window() {}
    auto raw() -> RawWindow* {
        return &this->_w;
    }
    auto refresh() -> int {
        this->raw()->refresh();
        return 0;
    }
};

auto example() -> void {
    cursed::init();
    auto w = cursed::Window<1, 1, 5, 20>();
    *w.raw() << cxxcurses::format(1, 1)("Hello, world!");
    w.refresh();
    for (;;);
}