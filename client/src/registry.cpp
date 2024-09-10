/// This file is incharge of maintaining the object registery system of vitium.
#include "libs/json_nlohmann_headerlib/json.hpp"
#include "libs/json_nlohmann_headerlib/json_fwd.hpp"
#include <optional>
#include <string>

namespace object
{
    class obj // @warning : I believe this should be implemented in a better way. @todo : rewrite this.
    {
    public:
        std::string regit_id;
        std::optional<std::string> name;
        /// @brief use "name.value_or(regit::regit_lookup(regit_id).name)" to get the value.
        // @todo : add more.
        obj(std::string id)
        {
            this->regit_id = id;
        }
        ~obj() {}
    };
} // namespace object

namespace regit
{

} // namespace regit
