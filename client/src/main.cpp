#include "frontend.cpp"
#include <thread>

bool Exit_Flag = false;
const bool DEBUG = true;

int main()
{
    initscr(); // Start curses mode
    raw();
    keypad(stdscr, true);
    noecho();

    /// here, we will fork out threads for the registry, the timer. Frontend will run under this main thread.

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