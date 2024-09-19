/// This file is meant to implement the web functions for server connection.
#include "libs/json_nlohmann_headerlib/json.hpp"
#include "libs/json_nlohmann_headerlib/json_fwd.hpp"
#include "libs/timer.cpp"
#include "registry.cpp"
#include "map.cpp"
#include <cpr/cpr.h>
#include <string>
#include <mutex>

namespace connct
{
    struct Player
    {
        std::string display_name = "";
        bool is_root = false;
        int64_t token = 0;
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
        nlohmann::json j;
        j["username"] = main_player.user_name;
        j["password"] = main_player.password;
        auto r = cpr::Post(cpr::Url{"http://localhost:8080/api/auth/login"}, cpr::Body{j.dump()}, cpr::Header{{"content-type", "application/json"}});
        if (r.status_code == 200)
        {
            main_player.token = std::stoll(r.text);
        }
        else
        {
            std::cerr << "[Error] Failed to get token from server." << std::endl;
        }
    }
}