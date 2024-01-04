#define _LINUX_ // probably support macOS ?
// #define _WIN32_

#include "httplib.h"
// #include <bits/stdc++.h> //very bad coding habit
using std::cin;
using std::cout;
using std::endl;
using std::ifstream;
using std::ofstream;
using std::string;
#include "data_structure.cpp" //<bits/stdc++> is already in this

void clear() // a simple clear screen ...
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

void http_get(char server_path[], char local_path[])
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
            throw res->status;
        }
        if (local_w.is_open()){
            local_w.close();
            local_w.open(local_path);
        }
        local_w << res->body; // load the data directly into the local file
        local_w.close();
    }
    catch (int e)
    {
        err_package_major.err_code = 20000 + e;
        err_package_major.err_message = "HTTP connection error ";
        std::cerr << err_package_major.err_message;
        std::cerr << e + 20000 << '\n';
    }
}

int main()
{
    cout << "Please input your server IP : ";
    cin >> ip_address;
    local_w.open(".local");
    clear();
    try
    {
        httplib::Client cli(ip_address);
        auto res = cli.Get("/user/init.json"); // this init should provide the list of player names and stuff, actual name depends on the server-side development (^w^)
        if (res->status / 100 == 2)
        {
        }
        else
        {
            throw res->status;
        }
        local_w << res->body; // load the init directly into the local file
        local_w.close();
        std::cerr << "connection built to server." << '\n';
    }
    catch (int e)
    {
        err_package_major.err_code = 21000 + e;
        err_package_major.err_message = "HTTP connection initialize error ";
        std::cerr << err_package_major.err_message;
        std::cerr << e + 21000 << '\n';
    }
}