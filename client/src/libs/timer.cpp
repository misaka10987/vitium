#include <chrono>
#include <thread>
namespace vtimer
{
    void timer_call(int milisecs, auto (*callback)())
    {
        std::this_thread::sleep_for(std::chrono::milliseconds(milisecs));
        callback();
    } // sleep for milisecs and then call the callback function.

} // namespace vtimer
