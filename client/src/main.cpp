#include "connect.cpp"
#include "registry.cpp"
#include <thread>
#include "frontend.cpp" // This file have to be included last because "curses.h" has some shit macros that would crash into the lib-cpr.

bool Exit_Flag = false;
const bool DEBUG = true;

int main()
{
    initscr(); // Start curses mode
    raw();
    keypad(stdscr, true);
    noecho();

    /// here, we will fork out threads for the registry, the timer and the frontend. Nothing should run under this main thread.

    while (!Exit_Flag) // main loop
    {
        printw("Hello World !!!");
        refresh(); // Move the 'window' on to the screen
        getch();
        Exit_Flag = true;
    }

    endwin(); // end curses mode
    return 0;
}