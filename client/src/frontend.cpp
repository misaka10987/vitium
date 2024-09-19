/// @brief this file is meant to provide an interface and tools to show messeges on the screen, as well as maintain the vitium ui.
#pragma once
#include "libs/ncurses_utils.cpp"
#include <iostream>
#include "registry.cpp"
#include "map.cpp"

namespace frontend
{
    volatile bool Exit_Flag = false; // only in keyboard event handler could this flag be set.

    const int CHAT_HISTORY_SIZE = 50;

    std::string chat_history[CHAT_HISTORY_SIZE];
    volatile int chat_history_index = 0; // this is the counter for the chat history
                                         // the most recent message is at chat_history[chat_history_index]
                                         // the oldest message is at chat_history[(chat_history_index + 1) % CHAT_HISTORY_SIZE]

    WINDOW *chat_window;
    WINDOW *map_window;

    void curses_init()
    {
        initscr(); // Start curses mode
        cbreak();
        keypad(stdscr, true);
        start_color();
        noecho();
    } // intialize the ncurses and configure a few of the display settings

    void fresh_chat_win()
    {
        int line_count = 0;
        for (int i = chat_history_index; i > chat_history_index - CHAT_HISTORY_SIZE; i--)
        {
            line_count += chat_history[i].size() / (COLS / 2 - 2) + 1;
            if (line_count > LINES - 2)
            {
                line_count = i + 1; // now for the count of messages
                break;
            }
        }
        wmove(chat_window, 0, 0);
        for (int i = chat_history_index - line_count + 1; i <= chat_history_index; i++)
        {
            wprintw(chat_window, chat_history[i].c_str());
            wprintw(chat_window, "\n");
        }

        wrefresh(chat_window);
    } // @todo

    void pop_up(int height, int width, std::string title, std::string message)
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

    void empty() // @brief clean the windows and recreate the base
    {
        erase();
        WINDOW *chat_window_base = newwin(LINES - 1, COLS / 2, 1, 0);
        chat_window = newwin(LINES - 3, COLS / 2 - 2, 2, 1);
        WINDOW *map_window_base = newwin(LINES - 1, COLS - COLS / 2, 1, COLS / 2 + 1);
        map_window = newwin(LINES - 3, COLS - COLS / 2 - 2, 2, COLS / 2 + 2);
        box(chat_window_base, 0, 0);
        box(map_window_base, 0, 0);
    }

    void fresh_all()
    {
        fresh_chat_win();
        refresh();
    } // @todo

    void hello_world() noexcept
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
