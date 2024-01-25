#include "cxxcurses/cxxcurses.hpp"
#include <curses.h>
#include <mutex>
#include <iostream>
namespace cc = cxxcurses;

#include "cursed.cpp"

#define OS unix

#if OS == unix
#include <thread>
void my_sleep(double seconds) {
    int timetosleep;
    timetosleep = (int)(seconds * 1000);
    std::this_thread::sleep_for(std::chrono::milliseconds(timetosleep));
}
#elif OS == windows
#include <Windows.h>
void my_sleep(double seconds) {
    Sleep(seconds * 1000);
}
#endif

// global vars here
std::string ip_address;
std::stringstream my_input_stream;
cc::widget::stdscr_wrapper* main_win;
cc::widget::window* info_window;
cc::widget::window* stat_window;
cc::widget::window* story_window;
cc::widget::window* buffer_window;
cc::widget::window* life_window;
bool exit_loop = 0;
std::mutex input_stream_lock;

void window_init() {
    using cursed::Window;
    info_window = (new Window<1, 1, 15, -21>())->raw();
    stat_window = (new Window<1, -20, 15, 20>())->raw();
    story_window = (new Window<16, 1, -19, -1>())->raw();
    buffer_window = (new Window<-3, 1, 3, -31>())->raw();
    life_window = (new Window<-3, -30, 3, 30>())->raw();
    main_win = &cc::terminal::main_win;
}

void init() {
    std::cout << "Server IP: ";
    std::cin >> ip_address;
}

void input_func() {
    char buff;
    while (!exit_loop) {
        buff = wgetch(main_win->get());
        input_stream_lock.lock();
        my_input_stream << buff;
        input_stream_lock.unlock();
        my_sleep(0.1);
    }
}

void window_refresh_all() {
    info_window->refresh();
    stat_window->refresh();
    story_window->refresh();
    life_window->refresh();
    buffer_window->refresh();
    //main_win->refresh();
}

void client_welcome_page() {
    *info_window << cc::format(1, 1)("HelloWorld from info-window!");
    *stat_window << cc::format(1, 1)("Hello !"); // do not use \n to change line because you will lose part of the frame
    *stat_window << cc::format(2, 1)("From Stat !");
    *story_window << cc::format(5)("Welcome to vitium client (under dev) !");
    window_refresh_all();
}

void main_loop() {
    std::string buffer_storage;
    int counter = 0;
    bool got_data = 0;
    while (counter < 3 && !input_stream_lock.try_lock()) // this will try to get the lock for three times, else skip
    {
        my_sleep(0.2);
        counter++;
    }
    if (counter < 3) {
        my_input_stream >> buffer_storage;
        input_stream_lock.unlock();
        got_data = 1;
    }
    if (got_data) {
        *buffer_window << cc::format(0, 0)(buffer_storage);
    }

    // TODO

    buffer_storage = "";
    window_refresh_all();
}

int main() {
    init();
    cursed::init();
    window_init();
    client_welcome_page();
    for (;;);
    // declares end here
    //*main_win << cc::format(0)("vitium");
    //ip_address = "IP: " + ip_address;
    //*main_win << cc::format(0, 1)(ip_address.c_str());
    // std::thread input_proc(input_func);
    // exit_loop = 0;
    // while (!exit_loop) {
    //     main_loop();
    // }
    //input_proc.join();
    //main_win->get_char();
    return 0;
}