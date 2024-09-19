/// This file is meant to implement the web functions for server connection.
#include "libs/json_nlohmann_headerlib/json.hpp"
#include "libs/json_nlohmann_headerlib/json_fwd.hpp"
#include "libs/timer.cpp"
#include "registry.cpp"
#include "map.cpp"
#include <cpr/cpr.h>
#include <string>
#include <mutex>

namespace connect
{
    struct Player
    {
        std::string display_name = "";
        bool is_root = false;
        std::string token = 0;
        std::mutex token_lock;
        std::string user_name = "";
        std::string password = "";
    };

    struct Player main_player;

    std::string server_address = "http://localhost:8080";

    void player_init(std::string dispname, std::string usrname, std::string pswd)
    {
        main_player.display_name = dispname;
        main_player.user_name = usrname;
        main_player.password = pswd;
    }

    void token_get()
    {
        std::lock_guard<std::mutex> lock(main_player.token_lock);
        auto r = cpr::Get(cpr::Url{server_address + "/api/auth/login"},
                          cpr::Authentication{main_player.user_name, main_player.password, cpr::AuthMode::BASIC});
        if (r.status_code == 200)
        {
            main_player.token = r.header["Set-Cookie"];
        }
        else
        {
            std::cerr << "[Error] Failed to get token from server." << std::endl;
        }
    }
}