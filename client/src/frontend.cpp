/// this file is meant to provide an interface and tools to show messeges on the screen, as well as maintain the vitium ui.
#include "libs/ncurses_utils.cpp"
#include <iostream>
#include "keyboard.cpp"

namespace frontend
{
    inline volatile bool Exit_Flag = false;

    inline void curses_init()
    {
        initscr(); // Start curses mode
        cbreak();
        keypad(stdscr, true);
        start_color();
        noecho();
    } // intialize the ncurses and configure a few of the display settings

    inline void pop_up(int height, int width, std::string title, std::string message)
    {
        if (height < 3 || width < 3)
        {
            std::cerr << "[Warning] Pop up box too small." << std::endl;
            return;
        }
        if (height > LINES || width > COLS)
        {
            std::cerr << "[Warning] Pop up box too large." << std::endl;
        }

        int starty = (LINES - height) / 2;
        int startx = (COLS - width) / 2;
        WINDOW *pop_box = newwin(height, width, starty, startx); // this create a pop up window at the center of the screen
        box(pop_box, 0, 0);                                      // wrap the window with a box
        mvwprintw(pop_box, 0, 1, title.c_str());
        wrefresh(pop_box);
        delwin(pop_box);
        // finish making the box
        WINDOW *pop_box_up = newwin(height - 2, width - 2, starty + 1, startx + 1);
        if (message.size() <= width - 2)
        {
            mvwprintw(pop_box_up, height / 2 - 1, width / 2 - message.size() / 2, message.c_str());
        }
        else if (message.size() <= (height - 2) * (width - 2))
        {
            mvwprintw(pop_box_up, 0, 0, message.c_str());
        }
        else
        {
            std::cerr << "[Warning] Message too long for the pop box." << std::endl;
        }
        wrefresh(pop_box_up);
        delwin(pop_box_up);
    }

    inline void hello_world() noexcept
    {
        printw("Hello World !!!");
        refresh(); // Move the 'window' on to the screen
        getch();
        pop_up(10, 30, "Hello World", "This is a pop up box.");
        refresh();
        getch();
        Exit_Flag = true;
    }
} // namespace frontend
