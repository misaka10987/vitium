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



    while (!Exit_Flag) // main loop
    {
        printw("Hello World !!!");
        refresh(); // Move the 'window' on to the screen
        getch();
        Exit_Flag = true;
    }

    endwin(); // end curses mode
}