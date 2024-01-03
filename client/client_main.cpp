#define _LINUX_
//#define _WIN32_

#include "httplib.h"
#include <bits/stdc++.h> //very bad coding habit
using std::cin;
using std::cout;
using std::endl;
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
        res->body;
    }
    catch (int e)
    {
        err_package_major.err_code = 20000 + e;
        err_package_major.err_message = "HTTP connection initialize error ";
        std::cerr << err_package_major.err_message;
        std::cerr << e + 20000 << '\n';
    }
}