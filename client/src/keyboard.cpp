#include "libs/ncurses_utils.cpp"
#include <thread>
#include <future>
#include "frontend.cpp"
namespace keyboard
{
    void keyboard_event_handler(int key)
    {
        if (key == 3) // Ctrl + C
        {
            frontend::Exit_Flag = true;
        }
        else if (key == (int)';') // semicolon
        {
            frontend::switch_hotwindow();
        }
        else if (!frontend::hot_window_chat)
        {
            switch (key)
            {
            case (int)'a':
                break;
            case (int)'w':
                break;
            case (int)'s':
                break;
            case (int)'d':
                break;
            case KEY_UP:
                break;
            case KEY_DOWN:
                break;
            case KEY_LEFT:
                break;
            case KEY_RIGHT:
                break;
            case 10: // Enter
                break;
            case 27: // Escape
                break;
            default:
                break;
            }
        }
        else
        {
            // we should put it as an echo
        }
        frontend::fresh_all();
    }

    void keyboard_event_listener() // This function should be run on a single thread
    {
        while (!frontend::Exit_Flag)
        {
            int key = getch();
            std::thread(keyboard_event_handler, key).detach();
        }
    }
} // namespace keyboard
