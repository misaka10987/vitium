#include "registry.cpp"
#include "connect.cpp"
#include <thread>
#include "frontend.cpp" // This file have to be included last because "curses.h" has some shit macros that would crash into the lib-cpr.

const bool DEBUG = true;

int main()
{
    initscr(); // Start curses mode
    raw();
    keypad(stdscr, true);
    noecho();

    /// here, we will fork out threads for the registry, the timer. The frontend should run under this main thread.

    while (!frontend::Exit_Flag) // main loop
    {
        frontend::hello_world();
    }

    endwin(); // end curses mode
    return 0;
}