#define _LINUX_ // probably support macOS ?
// #define _WIN32_

#include "httplib.h"
#include <bits/stdc++.h> //very bad coding habit
using std::cin;
using std::cout;
using std::endl;
using std::ifstream;
using std::ofstream;
using std::string;

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

int main()
{
    string ip_address;
    cout << "Please input your server IP : ";
    cin >> ip_address;
    ifstream init_local_r;
    ofstream init_local_w;
    init_local_w.open(".init_local");
#ifdef _WIN32_
    system("cls");
#endif
#ifdef _LINUX_
    system("clear");
#endif
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
        init_local_w << res->body; // load the init directly into the local file
        init_local_w.close();
    }
    catch (int e)
    {
        err_package_major.err_code = 20000 + e;
        err_package_major.err_message = "HTTP connection initialize error ";
        std::cerr << err_package_major.err_message;
        std::cerr << e + 20000 << '\n';
    }
}