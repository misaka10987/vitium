#include <chrono>
namespace vtimer
{
    class timer
    {
    private:
        std::chrono::steady_clock::time_point start;

    public:
        int64_t get();
        timer();
        ~timer();
    };

    int64_t timer::get()
    {
        std::chrono::milliseconds duration = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now() - start);
        return duration.count();
    } // return milisecs passed since timer creation.

    timer::timer()
    {
        start = std::chrono::steady_clock::now();
    }

    timer::~timer()
    {
    }

} // namespace vtimer
