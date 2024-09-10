#pragma once
#include <iostream>
/// @brief This file is a class only file. You use it to access the maintained scene cache.
namespace map
{
    class block
    {
    };

    class scene
    {
    private:
        block *scene_base[65535][65535];

    public:
        auto get(int x, int y) -> block *
        {
            if (x < 0 || y < 0 || x >= 65535 || y >= 65535)
            {
                std::cerr << "[Warning] getting block out of scene bound." << std::endl;
                return nullptr;
            }
            if (scene_base[x][y] == nullptr)
                std::cerr << "[Warning] getting block that is not initialized." << std::endl;
            return scene_base[x][y];
        }

        void set(int x, int y, block *blk)
        {
            if (x < 0 || y < 0 || x >= 65535 || y >= 65535)
            {
                std::cerr << "[Warning] setting block out of scene bound at (" << x << "," << y << ")" << std::endl;
                return;
            }
            if (scene_base[x][y] != nullptr)
            {
                std::cerr << "[Warning] setting initialized block at (" << x << "," << y << ") , removed previous block data" << std::endl;
                delete scene_base[x][y];
            }
            scene_base[x][y] = blk;
        } // usage: scene.set(x, y, new block(...));

        scene()
        {
            for (int i = 0; i < 65535; i++)
            {
                for (int j = 0; j < 65535; j++)
                {
                    scene_base[i][j] = nullptr;
                }
            }
        }
        ~scene()
        {
            for (int i = 0; i < 65535; i++)
            {
                for (int j = 0; j < 65535; j++)
                {
                    if (scene_base[i][j] != nullptr)
                    {
                        delete scene_base[i][j];
                    }
                }
            }
        }
    };
} // namespace map
