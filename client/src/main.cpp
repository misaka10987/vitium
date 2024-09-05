#include "libs/ncurses_utils.cpp"

int main()
{
    initscr(); //Start curses mode
    printw("Hello World !!!");
    refresh(); //Move the 'window' on to the screen
    getch();
    endwin(); //end curses mode
}