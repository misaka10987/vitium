#include "cxxcurses/cxxcurses.hpp"
#include <curses.h>
#include <mutex>
#include <iostream>
namespace cc = cxxcurses;

#define OS unix

#if OS == unix
#include <thread>
void my_sleep(double seconds)
{
    int timetosleep;
    timetosleep = (int)(seconds * 1000);
    std::this_thread::sleep_for(std::chrono::milliseconds(timetosleep));
}
#elif OS == windows
#include <Windows.h>
void my_sleep(double seconds)
{
    Sleep(seconds * 1000);
}
#endif

// global vars here
int maxX, maxY;
std::string ip_address;
std::stringstream my_input_stream;
cc::widget::stdscr_wrapper *main_win;
cc::widget::window *info_window;
cc::widget::window *stat_window;
cc::widget::window *story_window;
cc::widget::window *buffer_window;
cc::widget::window *life_window;
bool exit_loop = 0;
std::mutex input_stream_lock;

void window_init()
{
    auto _info_window{cc::widget::window{{1, 0, 15, maxX - 21}, cc::terminal::main_win}}; //{int curserY,int curserX,int heightY,int widthX}
    auto _stat_window{cc::widget::window{{1, maxX - 20, 15, 20}, cc::terminal::main_win}};
    auto _story_window{cc::widget::window{{16, 0, maxY - 19, maxX}, cc::terminal::main_win}};
    auto _buffer_window{cc::widget::window{{maxY - 3, 0, 3, maxX - 31}, cc::terminal::main_win}};
    auto _life_window{cc::widget::window{{maxY - 3, maxX - 30, 3, 30}, cc::terminal::main_win}};
    // 浪费我这么多时间去看了头文件，再看这个示例真的是...
    // 但也不敢乱改，鄙人认为只需要 cc::widget::window _info_window({1,2,3,4},cc::terminal::main_win); 就行了。
    // 注：{1,2,3,4} 其实已经隐式类型转换调用过 cxxcurses::widget::window::dimensions 的构造函数了。
    // 这里面的回环调用真的是...
    // 原作者重复调用构造函数应该是为了使用auto关键字，何意呢？ 完全没必要好吧。
    info_window = &_info_window;
    stat_window = &_stat_window;
    story_window = &_story_window;
    buffer_window = &_buffer_window;
    life_window = &_life_window;
    // 这里依然有持续性问题，widget不支持new也是服了
    // 使用中文以表达我的愤恨...clang应该不会管注释里的中文的
}

void init()
{
    std::cout << "Server IP: ";
    std::cin >> ip_address;
}

void input_func()
{
    char buff;
    while (!exit_loop)
    {
        buff = wgetch(main_win->get());
        input_stream_lock.lock();
        my_input_stream << buff;
        input_stream_lock.unlock();
        my_sleep(0.1);
    }
}

void window_refresh_all()
{
    info_window->refresh();
    stat_window->refresh();
    story_window->refresh();
    life_window->refresh();
    buffer_window->refresh();
    main_win->refresh();
}

void client_welcome_page()
{
    *info_window << cc::format(1, 1)("HelloWorld from info-window!");
    *stat_window << cc::format(1, 1)("Hello !"); // do not use \n to change line because you will lose part of the frame
    *stat_window << cc::format(2, 1)("From Stat !");
    *story_window << cc::format(5)("Welcome to vitium client (under dev) !");
    window_refresh_all();
}

void main_loop()
{
    std::string buffer_storage;
    int counter = 0;
    bool got_data = 0;
    while (counter < 3 && !input_stream_lock.try_lock()) // this will try to get the lock for three times, else skip
    {
        my_sleep(0.2);
        counter++;
    }
    if (counter < 3)
    {
        my_input_stream >> buffer_storage;
        input_stream_lock.unlock();
        got_data = 1;
    }
    if (got_data)
    {
        *buffer_window << cc::format(0, 0)(buffer_storage);
    }

    // TODO

    buffer_storage = "";
    window_refresh_all();
}

int main()
{
    main_win = &cc::terminal::main_win;
    init();
    cc::terminal init;
    maxX = main_win->max_yx().second;
    maxY = main_win->max_yx().first;

    // declares end here

    *main_win << cc::format(0)("vitium");
    ip_address = "IP: " + ip_address;
    *main_win << cc::format(0, 1)(ip_address.c_str());
    std::thread input_proc(input_func);
    exit_loop = 0;
    window_refresh_all();
    client_welcome_page();
    while (!exit_loop)
    {
        main_loop();
    }
    input_proc.join();

    main_win->get_char();
    return 0;
}