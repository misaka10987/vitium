#pragma once
#include <iostream>
#include <vector>
/// @brief This file is a class only file. You use it to access the maintained scene cache.
/// @note All pointers should be either 0 (not initialized) or nullptr or a valid block pointer. DO NOT USE "NULL".
namespace map
{
    class block
    {
    };

    class scene
    {
    private:
        std::vector<std::vector<block *>> scene_base;

    public:
        auto get(int x, int y) -> block *
        {
            if (scene_base[x % 65535][y % 65535] == 0)
            {
                std::cerr << "[Warning] getting block that is not initialized at (" << x << "," << y << ")" << std::endl;
                return nullptr;
            }
            return scene_base[x % 65535][y % 65535];
        }

        void set(int x, int y, block *blk)
        {
            if (x % 65535 > scene_base.size())
            {
                scene_base.resize(x % 65535);
            }
            if (y % 65535 > scene_base[x % 65535].size())
            {
                scene_base[x % 65535].resize(y % 65535);
            }
            if (scene_base[x % 65535][y % 65535] != 0)
            {
                std::cerr << "[Warning] setting initialized block at (" << x << "," << y << ") , removed previous block data" << std::endl;
                delete scene_base[x % 65535][y % 65535];
            }
            scene_base[x % 65535][y % 65535] = blk;
        } // usage: scene.set(x, y, new block(...));

        scene()
        {
        }
        ~scene()
        {
            for (int i = 0; i < 65535; i++)
            {
                for (int j = 0; j < 65535; j++)
                {
                    if (scene_base[i][j] != 0 && scene_base[i][j] != nullptr)
                    {
                        delete scene_base[i][j];
                    }
                }
            }
        }
    };
} // namespace map
