#define _LINUX_

#include <bits/stdc++.h> //very bad habit
#include "cxxcurses/cxxcurses.hpp"
#include <curses.h>
namespace cc = cxxcurses;

#ifdef _LINUX_
void my_sleep(double milliseconds)
{
    int timetosleep;
    timetosleep = (int)(milliseconds * 1000);
    std ::this_thread ::sleep_for(std::chrono::milliseconds(timetosleep));
}
#endif

#ifdef _WIN32_
#include <Windows.h>
void my_sleep(int milliseconds)
{
    Sleep(milliseconds);
}
#endif

// global vars here
int maxX, maxY;
std::string ip_address;
std::stringstream my_input_stream;
const auto &main_win = cc::terminal::main_win;
auto info_window{cc::widget::window{{1, 0, 15, maxX - 21}, cc::terminal::main_win}}; //{int curserY,int curserX,int heightY,int widthX}
auto stat_window{cc::widget::window{{1, maxX - 20, 15, 20}, cc::terminal::main_win}};
auto story_window{cc::widget::window{{17, 0, maxY - 17, maxX}, cc::terminal::main_win}};
auto buffer_window{cc::widget::window{{maxY, 0, 1, maxX - 21}, cc::terminal::main_win}};
auto life_window{cc::widget::window{{maxY, maxX - 20, 1, 20}, cc::terminal::main_win}};
bool exit_loop = 0;
std::mutex input_stream_lock;

void init()
{
    std::cout << "Server IP: ";
    std::cin >> ip_address;
}

void input_func()
{
    char buff[4096];
    while (!exit_loop)
    {
        wscanw(main_win.get(), "%s", buff);
        input_stream_lock.lock();
        my_input_stream << buff;
        input_stream_lock.unlock();
        my_sleep(0.05);
    }
}

void window_refresh_all()
{
    info_window.refresh();
    stat_window.refresh();
    story_window.refresh();
    life_window.refresh();
    buffer_window.refresh();
}

void client_welcome_page()
{
    info_window << cc::format(1, 1)("HelloWorld from info-window!");
    stat_window << cc::format(1, 1)("Hello !"); // do not use \n to change line because you will lose part of the frame
    stat_window << cc::format(2, 1)("From Stat !");
    story_window << cc::format(5)("Welcome to vitium client (under dev) !");
    window_refresh_all();
}

int main_loop()
{
    std::string buffer_storage;
    int counter = 0;
    bool got_data = 0;
    while (counter < 3 && !input_stream_lock.try_lock()) // this will try to get the lock for three times, else skip
    {
        my_sleep(0.1);
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
        // todo
    }
    client_welcome_page();
    window_refresh_all();
}

int main()
{
    init();
    cc::terminal init;
    maxX = main_win.max_yx().second;
    maxY = main_win.max_yx().first;
    main_win << cc::format(0)("vitium");
    main_win << cc::format(1, 1)("IP: " + ip_address);
    std::thread input_proc(input_func);
    exit_loop = 0;
    while (!exit_loop)
    {
        main_loop();
    }
    input_proc.join();
    main_win.get_char();
    return 0;
}