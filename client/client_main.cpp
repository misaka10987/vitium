#define _LINUX_ // probably support macOS ?
// #define _WIN32_

#include "httplib.h"
// #include "json.hpp" //we might not use it yet since the rust-C port is built
#include <bits/stdc++.h> //very bad coding habit
#include <ncurses.h>
#include <curses.h>
using std::cin;
using std::cout;
using std::endl;
using std::ifstream;
using std::ofstream;
using std::string;
#include "data_structure.hpp" //<bits/stdc++> is already in this

void clear_cli() // a simple clear screen ...
{
#ifdef _WIN32_
    system("cls");
#endif
#ifdef _LINUX_
    system("clear");
#endif
}

class err_handle_pack
{
private:
public:
    int err_code;
    string err_message;
    err_handle_pack();
    ~err_handle_pack();
};

err_handle_pack::err_handle_pack()
{
    err_code = 0;
    err_message = "";
}

err_handle_pack::~err_handle_pack()
{
}

// universal vaiables are declared here
err_handle_pack err_package_major;
ifstream local_r;
ofstream local_w;
string ip_address;
int maxX = 0;
int maxY = 0;
void http_get_file(char server_path[], char local_path[])
{
    try
    {
        httplib::Client cli(ip_address);
        auto res = cli.Get(server_path);
        if (res->status / 100 == 2)
        {
        }
        else
        {
            err_package_major.err_code = 20000 + res->status;
            err_package_major.err_message = httplib::to_string(res.error());
            throw res->status;
        }
        if (local_r.is_open() || local_w.is_open())
        {
            std::cerr << "[WARNING] A .local file stream was not closed. This may cause fatal errors." << '\n';
            if (local_r.is_open())
                local_r.close();
            if (local_w.is_open())
                local_w.close();
        }
        local_w.open(local_path);
        local_w << res->body; // load the data directly into the local file
        local_w.close();
    }
    catch (int e)
    {
        std::cerr << "HTTP connection error ";
        std::cerr << e + 20000 << '\n';
    }
}

void http_post_file(char server_path[], char local_path[])
{
    try
    {
        if (local_r.is_open() || local_w.is_open())
        {
            std::cerr << "[WARNING] A .local file stream was not closed. This may cause fatal errors." << '\n';
            if (local_r.is_open())
                local_r.close();
            if (local_w.is_open())
                local_w.close();
        }
        httplib::Client cli(ip_address);
        /*         string filename;
                filename = server_path;
                for (int i = filename.size() - 1; i > 0; i--)
                {
                    if (filename[i] != '/')
                    {
                        continue;
                    }
                    else
                    {
                        filename.erase(0, i);
                    }
                }
         */
        auto res = cli.Post(server_path, local_path, "json"); // I don't know if this actually works
        if (res->status / 100 == 2)
        {
        }
        else
        {
            err_package_major.err_code = 20000 + res->status;
            err_package_major.err_message = httplib::to_string(res.error());
            throw res->status;
        }
    }
    catch (int e)
    {
        std::cerr << "HTTP connection error ";
        std::cerr << e + 20000 << '\n';
    }
}

void sync_data()
{
    // todo, can't do since we don't have data structure yet...
}

int main()
{
    cout << "Please input your server IP : ";
    cin >> ip_address;
    local_w.open(".data/.local");
    clear_cli();
    try
    {
        httplib::Client cli(ip_address);
        auto res = cli.Get("/hello"); // this init should provide the list of player names and stuff, actual name depends on the server-side development (^w^)
        if (res->status / 100 == 2)
        {
        }
        else
        {
            err_package_major.err_code = 21000 + res->status;
            err_package_major.err_message = httplib::to_string(res.error());
            throw res->status;
        }
        local_w << res->body; // load the init directly into the local file
        local_w.close();
        std::cerr << "connection built to server." << '\n';
    }
    catch (int e)
    {
        std::cerr << err_package_major.err_message << '\n';
        std::cerr << "HTTP connection initialize error ";
        std::cerr << e + 21000 << '\n';
    }
    initscr();
    raw();
    keypad(stdscr, TRUE);
    getmaxyx(stdscr, maxY, maxX);
    mvwprintw(stdscr, 0, maxX / 2 - 3,"vitium");
    mvwprintw(stdscr,0,0,"IP: %s",ip_address.c_str());
    char a;
    cin>>a;
    // now we have the init, we can start working
    // we will use the hash of the "username+salt" as the password, at least for now...
}