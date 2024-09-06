/// this file is meant to provide an interface and tools to show messeges on the screen, as well as maintain the vitium ui.
#include "libs/ncurses_utils.cpp"
namespace frontend
{
    bool Exit_Flag = false;

    void hello_world()
    {
        printw("Hello World !!!");
        refresh(); // Move the 'window' on to the screen
        getch();
        Exit_Flag = true;
    }
} // namespace frontend
