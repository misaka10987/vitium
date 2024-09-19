#define DEBUG

#include "registry.cpp"
#include "connect.cpp"
#include <thread>
#include "keyboard.cpp" // This file have to be included last because "curses.h" from "frontend.cpp" has some shit macros that would crash into the lib-cpr.

int main()
{
    freopen("vitium_client.log", "w", stderr);

    /// here, we will fork out threads for the registry, the timer. The frontend should run under this main thread.
    std::thread(keyboard::keyboard_event_listener).detach();

    frontend::curses_init();
    frontend::empty_base();

    while (!frontend::Exit_Flag) // main loop
    {
        frontend::hello_world();
    }

    endwin(); // end curses mode
    return 0;
}